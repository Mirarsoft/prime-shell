use std::{
    env,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use prime_shell_desktop_lib::backend::{
    BackendLifecycle, BackendRuntime, LaunchSpec, TaskEventSink, TaskLifecycle, TaskSnapshot,
};
use serde_json::json;

fn packaged_runtime() -> BackendRuntime {
    let executable = PathBuf::from(
        env::var_os("PRIME_SHELL_PACKAGED_SIDECAR")
            .expect("PRIME_SHELL_PACKAGED_SIDECAR must be set"),
    );
    let bundle_directory = executable.parent().expect("bundle directory");
    let target_root = bundle_directory
        .parent()
        .expect("target root")
        .to_path_buf();
    BackendRuntime::launch(LaunchSpec::from_paths(executable, target_root))
}

fn event_sink() -> (TaskEventSink, Arc<Mutex<Vec<TaskSnapshot>>>) {
    let events = Arc::new(Mutex::new(Vec::<TaskSnapshot>::new()));
    let captured = Arc::clone(&events);
    let sink: TaskEventSink = Arc::new(move |event| {
        captured.lock().expect("event lock").push(event);
    });
    (sink, events)
}

fn wait_for_terminal(runtime: &BackendRuntime, task_id: &str, timeout: Duration) -> TaskSnapshot {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        let snapshot = runtime
            .task_status(task_id.to_owned())
            .expect("task status");
        if snapshot.state.is_terminal() {
            return snapshot;
        }
        thread::sleep(Duration::from_millis(10));
    }
    panic!("task did not reach a terminal state");
}

fn assert_one_terminal(events: &Arc<Mutex<Vec<TaskSnapshot>>>) {
    assert_eq!(
        events
            .lock()
            .expect("event lock")
            .iter()
            .filter(|event| event.state.is_terminal())
            .count(),
        1
    );
}

#[test]
#[ignore = "requires PRIME_SHELL_PACKAGED_SIDECAR after the PyInstaller build"]
fn rust_to_packaged_python_unicode_echo() {
    let runtime = packaged_runtime();
    assert_eq!(
        runtime.status().expect("runtime status").state,
        BackendLifecycle::Ready
    );
    let text = "Hello — مرحبا — こんにちは 👋";
    let result = runtime.echo(text.to_owned()).expect("echo must succeed");
    assert_eq!(result.text, text);
}

#[test]
#[ignore = "requires PRIME_SHELL_PACKAGED_SIDECAR after the PyInstaller build"]
fn rust_to_packaged_python_count_cancel_is_terminal_once() {
    let runtime = packaged_runtime();
    let (sink, events) = event_sink();
    let accepted = runtime
        .start(
            "spike.count",
            json!({"countTo": 100, "intervalMs": 20}),
            Duration::from_secs(10),
            sink,
        )
        .expect("count must be accepted");
    thread::sleep(Duration::from_millis(30));
    let acknowledgement = runtime
        .cancel(accepted.task_id.clone())
        .expect("cancel must be acknowledged");
    assert!(acknowledgement.accepted);

    let terminal = wait_for_terminal(&runtime, &accepted.task_id, Duration::from_secs(3));
    assert_eq!(terminal.state, TaskLifecycle::Cancelled);

    let observed = events.lock().expect("event lock");
    let terminals: Vec<_> = observed
        .iter()
        .filter(|event| event.state.is_terminal())
        .collect();
    assert_eq!(terminals.len(), 1);
    assert_eq!(terminals[0].state, TaskLifecycle::Cancelled);
    assert!(observed
        .windows(2)
        .all(|pair| pair[0].sequence <= pair[1].sequence));
}

#[test]
#[ignore = "requires PRIME_SHELL_PACKAGED_SIDECAR after the PyInstaller build"]
fn cooperative_timeout_and_late_cancel_have_deterministic_precedence() {
    let runtime = packaged_runtime();
    let (timeout_sink, timeout_events) = event_sink();
    let timed = runtime
        .start(
            "spike.count",
            json!({"countTo": 100, "intervalMs": 50}),
            Duration::from_millis(100),
            timeout_sink,
        )
        .expect("timed count must start");
    let timed_terminal = wait_for_terminal(&runtime, &timed.task_id, Duration::from_secs(3));
    assert_eq!(timed_terminal.state, TaskLifecycle::TimedOut);
    assert_eq!(timed_terminal.error.expect("timeout error").code, "TIMEOUT");
    assert_one_terminal(&timeout_events);

    let (success_sink, success_events) = event_sink();
    let short = runtime
        .start(
            "spike.count",
            json!({"countTo": 1, "intervalMs": 10}),
            Duration::from_secs(2),
            success_sink,
        )
        .expect("short count must start");
    let success = wait_for_terminal(&runtime, &short.task_id, Duration::from_secs(2));
    assert_eq!(success.state, TaskLifecycle::Succeeded);
    let late_cancel = runtime
        .cancel(short.task_id)
        .expect_err("accepted success must win a later cancel");
    assert_eq!(late_cancel.code, "NOT_FOUND");
    assert_one_terminal(&success_events);
}

#[test]
#[ignore = "requires PRIME_SHELL_PACKAGED_SIDECAR after the PyInstaller build"]
fn crash_restart_circuit_recovery_and_no_replay_are_bounded() {
    let runtime = packaged_runtime();
    let (first_sink, first_events) = event_sink();
    let first = runtime
        .start("spike.crash", json!({}), Duration::from_secs(3), first_sink)
        .expect("first crash task must start");
    let first_terminal = wait_for_terminal(&runtime, &first.task_id, Duration::from_secs(3));
    assert_eq!(first_terminal.state, TaskLifecycle::Interrupted);
    assert_one_terminal(&first_events);
    let restarted = runtime.status().expect("restarted status");
    assert_eq!(restarted.state, BackendLifecycle::Ready);
    assert_eq!(restarted.restart_count, 1);
    assert!(!restarted.circuit_open);
    assert!(restarted.active_task_id.is_none());

    let (second_sink, second_events) = event_sink();
    let second = runtime
        .start(
            "spike.crash",
            json!({}),
            Duration::from_secs(3),
            second_sink,
        )
        .expect("second crash task must start");
    let second_terminal = wait_for_terminal(&runtime, &second.task_id, Duration::from_secs(3));
    assert_eq!(second_terminal.state, TaskLifecycle::Interrupted);
    assert_one_terminal(&second_events);
    let faulted = runtime.status().expect("faulted status");
    assert_eq!(faulted.state, BackendLifecycle::Faulted);
    assert_eq!(faulted.restart_count, 1);
    assert!(faulted.circuit_open);
    assert!(faulted.active_task_id.is_none());

    let recovered = runtime.recover().expect("explicit recovery must succeed");
    assert_eq!(recovered.state, BackendLifecycle::Ready);
    assert_eq!(recovered.restart_count, 0);
    assert!(!recovered.circuit_open);
}

#[test]
#[ignore = "requires PRIME_SHELL_PACKAGED_SIDECAR after the PyInstaller build"]
fn hang_escalates_to_interruption_and_one_restart() {
    let runtime = packaged_runtime();
    let (sink, events) = event_sink();
    let hung = runtime
        .start("spike.hang", json!({}), Duration::from_millis(100), sink)
        .expect("hang task must start");
    let terminal = wait_for_terminal(&runtime, &hung.task_id, Duration::from_secs(4));
    assert_eq!(terminal.state, TaskLifecycle::Interrupted);
    assert_one_terminal(&events);
    let status = runtime.status().expect("post-hang status");
    assert_eq!(status.state, BackendLifecycle::Ready);
    assert_eq!(status.restart_count, 1);
    assert!(!status.circuit_open);
}
