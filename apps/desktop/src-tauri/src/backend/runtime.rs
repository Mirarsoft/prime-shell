use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        mpsc::{self, Receiver, SyncSender},
        Arc,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use serde::Serialize;
use serde_json::{json, Value};

use super::{
    client::{map_backend_error, BackendClient, LaunchSpec},
    error::{AppError, AppResult},
    protocol::PENDING_REQUEST_MAX,
    registry::BackendOperation,
};

const ACTOR_TICK: Duration = Duration::from_millis(10);
const ACCEPT_TIMEOUT: Duration = Duration::from_secs(3);
const CANCEL_ACK_TIMEOUT: Duration = Duration::from_millis(250);
const CANCEL_STOP_TIMEOUT: Duration = Duration::from_secs(2);
const RESTART_BACKOFF: Duration = Duration::from_millis(100);
const FAILURE_WINDOW: Duration = Duration::from_secs(60);
const UI_PROGRESS_INTERVAL: Duration = Duration::from_millis(100);

pub type TaskEventSink = Arc<dyn Fn(TaskSnapshot) + Send + Sync + 'static>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum BackendLifecycle {
    Stopped,
    Starting,
    Ready,
    Busy,
    Restarting,
    Faulted,
    Stopping,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum TaskLifecycle {
    Queued,
    Running,
    Cancelling,
    Succeeded,
    Failed,
    Cancelled,
    TimedOut,
    Interrupted,
}

impl TaskLifecycle {
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Succeeded | Self::Failed | Self::Cancelled | Self::TimedOut | Self::Interrupted
        )
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskProgress {
    pub current: u64,
    pub target: u64,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSnapshot {
    pub task_id: String,
    pub trace_id: String,
    pub operation: String,
    pub state: TaskLifecycle,
    pub sequence: u64,
    pub progress: Option<TaskProgress>,
    pub result: Option<Value>,
    pub error: Option<AppError>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendStatus {
    pub state: BackendLifecycle,
    pub backend_version: Option<String>,
    pub restart_count: u32,
    pub circuit_open: bool,
    pub active_task_id: Option<String>,
    pub last_error: Option<AppError>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelReceipt {
    pub task_id: String,
    pub accepted: bool,
}

pub struct BackendRuntime {
    commands: SyncSender<Command>,
    sequence: AtomicU64,
    actor: Option<JoinHandle<()>>,
}

enum Command {
    Status(mpsc::Sender<AppResult<BackendStatus>>),
    Echo {
        text: String,
        request_id: String,
        trace_id: String,
        response: mpsc::Sender<AppResult<super::protocol::EchoResponse>>,
    },
    Start {
        operation: BackendOperation,
        payload: Value,
        timeout: Duration,
        request_id: String,
        trace_id: String,
        sink: TaskEventSink,
        response: mpsc::Sender<AppResult<TaskSnapshot>>,
    },
    Cancel {
        task_id: String,
        request_id: String,
        trace_id: String,
        response: mpsc::Sender<AppResult<CancelReceipt>>,
    },
    TaskStatus {
        task_id: String,
        response: mpsc::Sender<AppResult<TaskSnapshot>>,
    },
    Recover {
        response: mpsc::Sender<AppResult<BackendStatus>>,
    },
    Shutdown,
}

struct ActiveTask {
    snapshot: TaskSnapshot,
    request_id: String,
    deadline: Instant,
    cancellation_deadline: Option<Instant>,
    timeout_requested: bool,
    sink: TaskEventSink,
    last_ui_progress: Option<Instant>,
}

struct Actor {
    spec: LaunchSpec,
    client: Option<BackendClient>,
    state: BackendLifecycle,
    restart_count: u32,
    circuit_open: bool,
    last_failure: Option<Instant>,
    last_error: Option<AppError>,
    active: Option<ActiveTask>,
    last_task: Option<TaskSnapshot>,
    pending_internal_cancel: Option<(String, String)>,
    request_sequence: u64,
}

impl BackendRuntime {
    pub fn launch(spec: LaunchSpec) -> Self {
        let (commands, receiver) = mpsc::sync_channel(PENDING_REQUEST_MAX);
        let actor = thread::Builder::new()
            .name("prime-shell-backend-runtime".to_owned())
            .spawn(move || Actor::new(spec).run(receiver))
            .expect("backend runtime actor must start");
        Self {
            commands,
            sequence: AtomicU64::new(1),
            actor: Some(actor),
        }
    }

    pub fn status(&self) -> AppResult<BackendStatus> {
        let (sender, receiver) = mpsc::channel();
        self.send(Command::Status(sender), "backend-status")?;
        receive_response(receiver, "backend-status")
    }

    pub fn echo(&self, text: String) -> AppResult<super::protocol::EchoResponse> {
        let (request_id, trace_id) = self.identifiers("echo");
        let (sender, receiver) = mpsc::channel();
        self.send(
            Command::Echo {
                text,
                request_id,
                trace_id: trace_id.clone(),
                response: sender,
            },
            &trace_id,
        )?;
        receive_response(receiver, &trace_id)
    }

    pub fn start(
        &self,
        operation_name: &str,
        payload: Value,
        timeout: Duration,
        sink: TaskEventSink,
    ) -> AppResult<TaskSnapshot> {
        let (request_id, trace_id) = self.identifiers("task");
        let operation = BackendOperation::authorize(operation_name, &trace_id)?;
        if !operation.is_long_running() {
            return Err(AppError::validation(
                "The operation is not a long-running task.",
                trace_id,
            ));
        }
        let (sender, receiver) = mpsc::channel();
        self.send(
            Command::Start {
                operation,
                payload,
                timeout,
                request_id,
                trace_id: trace_id.clone(),
                sink,
                response: sender,
            },
            &trace_id,
        )?;
        receive_response(receiver, &trace_id)
    }

    pub fn cancel(&self, task_id: String) -> AppResult<CancelReceipt> {
        let (request_id, trace_id) = self.identifiers("cancel");
        let (sender, receiver) = mpsc::channel();
        self.send(
            Command::Cancel {
                task_id,
                request_id,
                trace_id: trace_id.clone(),
                response: sender,
            },
            &trace_id,
        )?;
        receive_response(receiver, &trace_id)
    }

    pub fn task_status(&self, task_id: String) -> AppResult<TaskSnapshot> {
        let (sender, receiver) = mpsc::channel();
        self.send(
            Command::TaskStatus {
                task_id,
                response: sender,
            },
            "task-status",
        )?;
        receive_response(receiver, "task-status")
    }

    pub fn recover(&self) -> AppResult<BackendStatus> {
        let (sender, receiver) = mpsc::channel();
        self.send(Command::Recover { response: sender }, "backend-recover")?;
        receive_response(receiver, "backend-recover")
    }

    fn identifiers(&self, prefix: &str) -> (String, String) {
        let sequence = self.sequence.fetch_add(1, Ordering::Relaxed);
        (
            format!("{prefix}-request-{sequence}"),
            format!("{prefix}-trace-{}-{sequence}", std::process::id()),
        )
    }

    fn send(&self, command: Command, trace_id: &str) -> AppResult<()> {
        self.commands
            .try_send(command)
            .map_err(|error| match error {
                mpsc::TrySendError::Full(_) => AppError::exhausted(trace_id),
                mpsc::TrySendError::Disconnected(_) => AppError::unavailable(trace_id),
            })
    }
}

impl Drop for BackendRuntime {
    fn drop(&mut self) {
        let _ = self.commands.try_send(Command::Shutdown);
        if let Some(actor) = self.actor.take() {
            let _ = actor.join();
        }
    }
}

fn receive_response<T>(receiver: Receiver<AppResult<T>>, trace_id: &str) -> AppResult<T> {
    receiver
        .recv()
        .map_err(|_| AppError::unavailable(trace_id))?
}

impl Actor {
    fn new(spec: LaunchSpec) -> Self {
        let mut actor = Self {
            spec,
            client: None,
            state: BackendLifecycle::Stopped,
            restart_count: 0,
            circuit_open: false,
            last_failure: None,
            last_error: None,
            active: None,
            last_task: None,
            pending_internal_cancel: None,
            request_sequence: 1,
        };
        actor.state = BackendLifecycle::Starting;
        match BackendClient::launch(actor.spec.clone()) {
            Ok(client) => {
                actor.client = Some(client);
                actor.state = BackendLifecycle::Ready;
            }
            Err(error) => {
                actor.last_error = Some(error);
                actor.state = BackendLifecycle::Faulted;
            }
        }
        actor
    }

    fn run(mut self, commands: Receiver<Command>) {
        loop {
            match commands.recv_timeout(ACTOR_TICK) {
                Ok(Command::Shutdown) | Err(mpsc::RecvTimeoutError::Disconnected) => {
                    self.state = BackendLifecycle::Stopping;
                    if let Some(mut client) = self.client.take() {
                        client.shutdown();
                    }
                    self.state = BackendLifecycle::Stopped;
                    return;
                }
                Ok(command) => self.handle_command(command),
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
            self.drain_backend();
            self.enforce_deadline();
        }
    }

    fn handle_command(&mut self, command: Command) {
        match command {
            Command::Status(response) => {
                let _ = response.send(Ok(self.status()));
            }
            Command::Echo {
                text,
                request_id,
                trace_id,
                response,
            } => {
                let result = if self.active.is_some() {
                    Err(AppError::conflict(
                        "Echo is unavailable while a long task is active.",
                        &trace_id,
                    ))
                } else if self.circuit_open {
                    Err(AppError::unavailable(&trace_id))
                } else {
                    self.client
                        .as_mut()
                        .ok_or_else(|| AppError::unavailable(&trace_id))
                        .and_then(|client| client.echo(&text, &request_id, &trace_id))
                };
                if matches!(result, Err(ref error) if error.code == "BACKEND_CRASHED") {
                    self.backend_failed(AppError::crashed(&trace_id));
                }
                let _ = response.send(result);
            }
            Command::Start {
                operation,
                payload,
                timeout,
                request_id,
                trace_id,
                sink,
                response,
            } => {
                let result =
                    self.start_task(operation, payload, timeout, request_id, trace_id, sink);
                if matches!(
                    result,
                    Err(ref error)
                        if matches!(
                            error.code,
                            "BACKEND_CRASHED" | "PROTOCOL_ERROR" | "IO_ERROR"
                        )
                ) {
                    let failure = result.as_ref().expect_err("failed result").clone();
                    self.backend_failed(failure);
                }
                let _ = response.send(result);
            }
            Command::Cancel {
                task_id,
                request_id,
                trace_id,
                response,
            } => {
                let result = self.cancel_task(&task_id, &request_id, &trace_id);
                let _ = response.send(result);
            }
            Command::TaskStatus { task_id, response } => {
                let result = self
                    .active
                    .as_ref()
                    .map(|task| &task.snapshot)
                    .or(self.last_task.as_ref())
                    .filter(|task| task.task_id == task_id)
                    .cloned()
                    .ok_or_else(|| AppError::not_found("The task was not found.", "task-status"));
                let _ = response.send(result);
            }
            Command::Recover { response } => {
                let result = self.recover_backend();
                let _ = response.send(result);
            }
            Command::Shutdown => {}
        }
    }

    fn status(&self) -> BackendStatus {
        BackendStatus {
            state: self.state,
            backend_version: self
                .client
                .as_ref()
                .map(|client| client.backend_version().to_owned()),
            restart_count: self.restart_count,
            circuit_open: self.circuit_open,
            active_task_id: self
                .active
                .as_ref()
                .map(|task| task.snapshot.task_id.clone()),
            last_error: self.last_error.clone(),
        }
    }

    fn start_task(
        &mut self,
        operation: BackendOperation,
        payload: Value,
        timeout: Duration,
        request_id: String,
        trace_id: String,
        sink: TaskEventSink,
    ) -> AppResult<TaskSnapshot> {
        if self.circuit_open || self.state == BackendLifecycle::Faulted {
            return Err(AppError::unavailable(&trace_id));
        }
        if self.active.is_some() {
            return Err(AppError::conflict(
                "A long-running task is already active.",
                &trace_id,
            ));
        }
        if timeout.is_zero() {
            return Err(AppError::validation(
                "The task timeout must be positive.",
                &trace_id,
            ));
        }

        let client = self
            .client
            .as_mut()
            .ok_or_else(|| AppError::unavailable(&trace_id))?;
        client.send_request(operation, &payload, &request_id, &trace_id)?;
        let accepted = wait_for_frame(client, ACCEPT_TIMEOUT, &trace_id)?;
        if accepted.get("kind").and_then(Value::as_str) == Some("error") {
            return Err(map_backend_error(&accepted, &trace_id));
        }
        validate_identity(&accepted, &request_id, &trace_id)?;
        if accepted.get("kind").and_then(Value::as_str) != Some("accepted")
            || accepted.get("operation").and_then(Value::as_str) != Some(operation.name())
        {
            return Err(AppError::protocol(
                "Backend task acceptance is invalid.",
                &trace_id,
            ));
        }
        let task_id = required_text(&accepted, "taskId", &trace_id)?.to_owned();
        let snapshot = TaskSnapshot {
            task_id,
            trace_id,
            operation: operation.name().to_owned(),
            state: TaskLifecycle::Running,
            sequence: 0,
            progress: None,
            result: None,
            error: None,
        };
        sink(snapshot.clone());
        self.active = Some(ActiveTask {
            snapshot: snapshot.clone(),
            request_id,
            deadline: Instant::now() + timeout,
            cancellation_deadline: None,
            timeout_requested: false,
            sink,
            last_ui_progress: None,
        });
        self.state = BackendLifecycle::Busy;
        Ok(snapshot)
    }

    fn cancel_task(
        &mut self,
        task_id: &str,
        request_id: &str,
        trace_id: &str,
    ) -> AppResult<CancelReceipt> {
        let active = self
            .active
            .as_mut()
            .filter(|task| task.snapshot.task_id == task_id)
            .ok_or_else(|| AppError::not_found("The active task was not found.", trace_id))?;
        if active.snapshot.state.is_terminal() {
            return Ok(CancelReceipt {
                task_id: task_id.to_owned(),
                accepted: false,
            });
        }
        active.snapshot.state = TaskLifecycle::Cancelling;
        active.cancellation_deadline = Some(Instant::now() + CANCEL_STOP_TIMEOUT);
        let client = self
            .client
            .as_mut()
            .ok_or_else(|| AppError::unavailable(trace_id))?;
        client.send_cancel(task_id, request_id, trace_id)?;

        let deadline = Instant::now() + CANCEL_ACK_TIMEOUT;
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err(AppError::timeout(trace_id));
            }
            let value = wait_for_frame(
                self.client
                    .as_mut()
                    .ok_or_else(|| AppError::unavailable(trace_id))?,
                remaining,
                trace_id,
            )?;
            match value.get("kind").and_then(Value::as_str) {
                Some("taskEvent") => self.process_event(value)?,
                Some("cancelAck") => {
                    validate_identity(&value, request_id, trace_id)?;
                    if value.get("taskId").and_then(Value::as_str) != Some(task_id) {
                        return Err(AppError::protocol(
                            "Backend cancellation acknowledgement is invalid.",
                            trace_id,
                        ));
                    }
                    let accepted =
                        value
                            .get("accepted")
                            .and_then(Value::as_bool)
                            .ok_or_else(|| {
                                AppError::protocol(
                                    "Backend cancellation acknowledgement is invalid.",
                                    trace_id,
                                )
                            })?;
                    return Ok(CancelReceipt {
                        task_id: task_id.to_owned(),
                        accepted,
                    });
                }
                _ => {
                    return Err(AppError::protocol(
                        "Backend emitted an unexpected cancellation response.",
                        trace_id,
                    ));
                }
            }
        }
    }

    fn drain_backend(&mut self) {
        loop {
            let result = match self.client.as_mut() {
                Some(client) => client.receive(Duration::ZERO, "backend-event"),
                None => return,
            };
            match result {
                Ok(Some(value)) => {
                    if let Err(error) = self.process_backend_frame(value) {
                        self.backend_failed(error);
                        return;
                    }
                }
                Ok(None) => return,
                Err(error) => {
                    self.backend_failed(error);
                    return;
                }
            }
        }
    }

    fn process_backend_frame(&mut self, value: Value) -> AppResult<()> {
        if value.get("kind").and_then(Value::as_str) != Some("cancelAck") {
            return self.process_event(value);
        }
        let (request_id, task_id) = self.pending_internal_cancel.as_ref().ok_or_else(|| {
            AppError::protocol(
                "Backend emitted an unexpected cancellation acknowledgement.",
                "backend-event",
            )
        })?;
        if value.get("protocol").and_then(Value::as_str) != Some("generic-app")
            || value.get("requestId").and_then(Value::as_str) != Some(request_id)
            || value.get("taskId").and_then(Value::as_str) != Some(task_id)
            || value.get("accepted").and_then(Value::as_bool) != Some(true)
        {
            return Err(AppError::protocol(
                "Backend cancellation acknowledgement is invalid.",
                "backend-event",
            ));
        }
        self.pending_internal_cancel = None;
        Ok(())
    }

    fn process_event(&mut self, value: Value) -> AppResult<()> {
        if value.get("protocol").and_then(Value::as_str) != Some("generic-app")
            || value.get("kind").and_then(Value::as_str) != Some("taskEvent")
        {
            return Err(AppError::protocol(
                "Backend emitted an unexpected event.",
                "backend-event",
            ));
        }
        let task = self.active.as_mut().ok_or_else(|| {
            AppError::protocol("Backend emitted an orphan event.", "backend-event")
        })?;
        if value.get("requestId").and_then(Value::as_str) != Some(&task.request_id)
            || value.get("traceId").and_then(Value::as_str) != Some(&task.snapshot.trace_id)
            || value.get("taskId").and_then(Value::as_str) != Some(&task.snapshot.task_id)
        {
            return Err(AppError::protocol(
                "Backend task event identity mismatch.",
                &task.snapshot.trace_id,
            ));
        }
        let sequence = value
            .get("sequence")
            .and_then(Value::as_u64)
            .filter(|sequence| *sequence > task.snapshot.sequence)
            .ok_or_else(|| {
                AppError::protocol(
                    "Backend task event sequence is invalid.",
                    &task.snapshot.trace_id,
                )
            })?;
        task.snapshot.sequence = sequence;

        match value.get("state").and_then(Value::as_str) {
            Some("Running") => {
                let current = value
                    .pointer("/progress/current")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| {
                        AppError::protocol("Backend progress is invalid.", &task.snapshot.trace_id)
                    })?;
                let target = value
                    .pointer("/progress/target")
                    .and_then(Value::as_u64)
                    .filter(|target| current <= *target)
                    .ok_or_else(|| {
                        AppError::protocol("Backend progress is invalid.", &task.snapshot.trace_id)
                    })?;
                task.snapshot.progress = Some(TaskProgress { current, target });
                if task.snapshot.state != TaskLifecycle::Cancelling {
                    task.snapshot.state = TaskLifecycle::Running;
                }
                let now = Instant::now();
                if task
                    .last_ui_progress
                    .is_none_or(|previous| now.duration_since(previous) >= UI_PROGRESS_INTERVAL)
                {
                    (task.sink)(task.snapshot.clone());
                    task.last_ui_progress = Some(now);
                }
                Ok(())
            }
            Some("Succeeded") => {
                task.snapshot.state = TaskLifecycle::Succeeded;
                task.snapshot.result = value.get("result").cloned();
                self.finish_active()
            }
            Some("Failed") => {
                task.snapshot.state = TaskLifecycle::Failed;
                task.snapshot.error = Some(map_backend_error(
                    &json!({"error": value.get("error")}),
                    &task.snapshot.trace_id,
                ));
                self.finish_active()
            }
            Some("Cancelled") => {
                task.snapshot.state = if task.timeout_requested {
                    TaskLifecycle::TimedOut
                } else {
                    TaskLifecycle::Cancelled
                };
                task.snapshot.error = Some(if task.timeout_requested {
                    AppError::timeout(&task.snapshot.trace_id)
                } else {
                    AppError::cancelled(&task.snapshot.trace_id)
                });
                self.finish_active()
            }
            _ => Err(AppError::protocol(
                "Backend task event state is invalid.",
                &task.snapshot.trace_id,
            )),
        }
    }

    fn finish_active(&mut self) -> AppResult<()> {
        let task = self
            .active
            .take()
            .ok_or_else(|| AppError::protocol("No active task exists.", "backend-event"))?;
        (task.sink)(task.snapshot.clone());
        self.last_task = Some(task.snapshot);
        self.state = BackendLifecycle::Ready;
        Ok(())
    }

    fn enforce_deadline(&mut self) {
        let now = Instant::now();
        let should_request_timeout = self.active.as_ref().is_some_and(|task| {
            now >= task.deadline
                && task.cancellation_deadline.is_none()
                && !task.snapshot.state.is_terminal()
        });
        if should_request_timeout {
            let (task_id, trace_id) = {
                let task = self.active.as_ref().expect("active task");
                (
                    task.snapshot.task_id.clone(),
                    task.snapshot.trace_id.clone(),
                )
            };
            let request_id = self.next_request_id("timeout-cancel");
            if let Some(task) = self.active.as_mut() {
                task.timeout_requested = true;
                task.snapshot.state = TaskLifecycle::Cancelling;
                task.cancellation_deadline = Some(now + CANCEL_STOP_TIMEOUT);
            }
            if let Some(client) = self.client.as_mut() {
                self.pending_internal_cancel = Some((request_id.clone(), task_id.clone()));
                if client
                    .send_cancel(&task_id, &request_id, &trace_id)
                    .is_err()
                {
                    self.pending_internal_cancel = None;
                    self.escalate_interruption();
                    return;
                }
            }
        }

        if self
            .active
            .as_ref()
            .and_then(|task| task.cancellation_deadline)
            .is_some_and(|deadline| now >= deadline)
        {
            self.escalate_interruption();
        }
    }

    fn escalate_interruption(&mut self) {
        if let Some(mut client) = self.client.take() {
            client.terminate_now();
        }
        let trace_id = self.active.as_ref().map_or_else(
            || "task-interrupted".to_owned(),
            |task| task.snapshot.trace_id.clone(),
        );
        self.interrupt_active(AppError::interrupted(trace_id));
        self.restart_after_failure(AppError::crashed("backend-escalation"));
    }

    fn backend_failed(&mut self, error: AppError) {
        if let Some(mut client) = self.client.take() {
            client.terminate_now();
        }
        let trace_id = self.active.as_ref().map_or_else(
            || "backend-crash".to_owned(),
            |task| task.snapshot.trace_id.clone(),
        );
        self.interrupt_active(AppError::interrupted(trace_id));
        self.restart_after_failure(error);
    }

    fn interrupt_active(&mut self, error: AppError) {
        if let Some(mut task) = self.active.take() {
            task.snapshot.state = TaskLifecycle::Interrupted;
            task.snapshot.sequence = task.snapshot.sequence.saturating_add(1);
            task.snapshot.error = Some(error);
            (task.sink)(task.snapshot.clone());
            self.last_task = Some(task.snapshot);
        }
    }

    fn restart_after_failure(&mut self, error: AppError) {
        let now = Instant::now();
        let repeated = self
            .last_failure
            .is_some_and(|previous| now.duration_since(previous) <= FAILURE_WINDOW);
        self.last_failure = Some(now);
        self.last_error = Some(error);
        self.pending_internal_cancel = None;
        if self.restart_count >= 1 || repeated {
            self.circuit_open = true;
            self.state = BackendLifecycle::Faulted;
            return;
        }

        self.restart_count += 1;
        self.state = BackendLifecycle::Restarting;
        thread::sleep(RESTART_BACKOFF);
        match BackendClient::launch(self.spec.clone()) {
            Ok(client) => {
                self.client = Some(client);
                self.state = BackendLifecycle::Ready;
            }
            Err(launch_error) => {
                self.last_error = Some(launch_error);
                self.circuit_open = true;
                self.state = BackendLifecycle::Faulted;
            }
        }
    }

    fn recover_backend(&mut self) -> AppResult<BackendStatus> {
        if self.active.is_some() {
            return Err(AppError::conflict(
                "Recovery is unavailable while a task is active.",
                "backend-recover",
            ));
        }
        if let Some(mut client) = self.client.take() {
            client.shutdown();
        }
        self.circuit_open = false;
        self.restart_count = 0;
        self.last_failure = None;
        self.last_error = None;
        self.pending_internal_cancel = None;
        self.state = BackendLifecycle::Starting;
        match BackendClient::launch(self.spec.clone()) {
            Ok(client) => {
                self.client = Some(client);
                self.state = BackendLifecycle::Ready;
                Ok(self.status())
            }
            Err(error) => {
                self.last_error = Some(error.clone());
                self.state = BackendLifecycle::Faulted;
                Err(error)
            }
        }
    }

    fn next_request_id(&mut self, prefix: &str) -> String {
        let sequence = self.request_sequence;
        self.request_sequence = self.request_sequence.saturating_add(1);
        format!("{prefix}-{sequence}")
    }
}

fn wait_for_frame(
    client: &mut BackendClient,
    timeout: Duration,
    trace_id: &str,
) -> AppResult<Value> {
    client
        .receive(timeout, trace_id)?
        .ok_or_else(|| AppError::timeout(trace_id))
}

fn validate_identity(value: &Value, request_id: &str, trace_id: &str) -> AppResult<()> {
    if value.get("protocol").and_then(Value::as_str) == Some("generic-app")
        && value.get("requestId").and_then(Value::as_str) == Some(request_id)
        && value.get("traceId").and_then(Value::as_str) == Some(trace_id)
    {
        Ok(())
    } else {
        Err(AppError::protocol(
            "Backend response identity mismatch.",
            trace_id,
        ))
    }
}

fn required_text<'a>(value: &'a Value, key: &str, trace_id: &str) -> AppResult<&'a str> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|text| !text.is_empty() && text.len() <= 128)
        .ok_or_else(|| AppError::protocol("Backend response field is invalid.", trace_id))
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use super::{BackendLifecycle, TaskLifecycle, FAILURE_WINDOW, UI_PROGRESS_INTERVAL};
    use crate::backend::protocol::{BACKEND_EVENT_MAX, PENDING_REQUEST_MAX};

    #[test]
    fn terminal_states_are_exact_and_paused_is_absent() {
        assert!(!TaskLifecycle::Queued.is_terminal());
        assert!(!TaskLifecycle::Running.is_terminal());
        assert!(!TaskLifecycle::Cancelling.is_terminal());
        for state in [
            TaskLifecycle::Succeeded,
            TaskLifecycle::Failed,
            TaskLifecycle::Cancelled,
            TaskLifecycle::TimedOut,
            TaskLifecycle::Interrupted,
        ] {
            assert!(state.is_terminal());
        }
    }

    #[test]
    fn lifecycle_contains_required_recovery_states() {
        let states = [
            BackendLifecycle::Stopped,
            BackendLifecycle::Starting,
            BackendLifecycle::Ready,
            BackendLifecycle::Busy,
            BackendLifecycle::Restarting,
            BackendLifecycle::Faulted,
            BackendLifecycle::Stopping,
        ];
        assert_eq!(states.len(), 7);
    }

    #[test]
    fn progress_and_failure_windows_are_bounded() {
        assert_eq!(UI_PROGRESS_INTERVAL.as_millis(), 100);
        assert_eq!(FAILURE_WINDOW.as_secs(), 60);
    }

    #[test]
    fn command_and_backend_event_queues_reject_exhaustion() {
        let (command_sender, _command_receiver) = mpsc::sync_channel(PENDING_REQUEST_MAX);
        for value in 0..PENDING_REQUEST_MAX {
            command_sender
                .try_send(value)
                .expect("command queue capacity");
        }
        assert!(matches!(
            command_sender.try_send(PENDING_REQUEST_MAX),
            Err(mpsc::TrySendError::Full(_))
        ));

        let (event_sender, _event_receiver) = mpsc::sync_channel(BACKEND_EVENT_MAX);
        for value in 0..BACKEND_EVENT_MAX {
            event_sender.try_send(value).expect("event queue capacity");
        }
        assert!(matches!(
            event_sender.try_send(BACKEND_EVENT_MAX),
            Err(mpsc::TrySendError::Full(_))
        ));
    }
}
