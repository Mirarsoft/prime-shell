use std::{
    env, fs,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use super::{
    error::{AppError, AppResult},
    protocol::{
        validate_hello, BundleManifest, CancelEnvelope, EchoResponse, Hello, RequestEnvelope,
        BACKEND_EVENT_MAX, FRAME_MAX_BYTES, HANDSHAKE_MAX_BYTES, LOG_MAX_BYTES,
        TEXT_MAX_CHARACTERS,
    },
    registry::BackendOperation,
};

const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(3);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(2);
const FORCE_TERM_TIMEOUT: Duration = Duration::from_millis(250);

#[derive(Debug)]
enum FrameReadError {
    Io,
    Eof,
    TooLarge,
}

#[derive(Clone)]
pub struct LaunchSpec {
    executable: PathBuf,
    target_root: PathBuf,
}

impl LaunchSpec {
    pub fn from_resource_dir(resource_dir: &Path) -> Self {
        let target_root = resource_dir.join("sidecar").join(target_identity());
        let executable_name = if cfg!(windows) {
            "prime-shell-python-backend.exe"
        } else {
            "prime-shell-python-backend"
        };
        let executable = target_root
            .join("prime-shell-python-backend")
            .join(executable_name);
        Self {
            executable,
            target_root,
        }
    }

    pub fn from_paths(executable: PathBuf, target_root: PathBuf) -> Self {
        Self {
            executable,
            target_root,
        }
    }
}

pub struct BackendClient {
    child: Child,
    stdin: Option<ChildStdin>,
    frames: Receiver<Result<Vec<u8>, FrameReadError>>,
    _logs: Receiver<Vec<u8>>,
    backend_version: String,
    pid: u32,
    terminated: bool,
}

impl BackendClient {
    pub fn launch(spec: LaunchSpec) -> AppResult<Self> {
        let trace = "backend-launch";
        let target_root = spec
            .target_root
            .canonicalize()
            .map_err(|_| AppError::unavailable(trace))?;
        let executable = spec
            .executable
            .canonicalize()
            .map_err(|_| AppError::unavailable(trace))?;
        if !executable.starts_with(&target_root) || !executable.is_file() {
            return Err(AppError::unavailable(trace));
        }

        let manifest_bytes = fs::read(target_root.join("sidecar-manifest.json"))
            .map_err(|_| AppError::unavailable(trace))?;
        let manifest: BundleManifest = serde_json::from_slice(&manifest_bytes)
            .map_err(|_| AppError::mismatch("Sidecar manifest is invalid.", trace))?;
        verify_bundle(
            working_directory_from(&executable, trace)?,
            &manifest,
            trace,
        )?;

        let working_directory = working_directory_from(&executable, trace)?;
        let mut command = Command::new(&executable);
        command
            .current_dir(working_directory)
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        apply_minimal_environment(&mut command);
        configure_process_containment(&mut command);

        let mut child = command.spawn().map_err(|_| AppError::unavailable(trace))?;
        let pid = child.id();
        if let Err(error) = record_forced_close_process_id(pid) {
            let _ = child.kill();
            let _ = child.wait();
            return Err(error);
        }
        let stdin = child.stdin.take().ok_or_else(|| AppError::io(trace))?;
        let stdout = child.stdout.take().ok_or_else(|| AppError::io(trace))?;
        let stderr = child.stderr.take().ok_or_else(|| AppError::io(trace))?;

        let (sender, frames) = mpsc::sync_channel(BACKEND_EVENT_MAX);
        thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            let mut first = true;
            loop {
                let maximum = if first {
                    HANDSHAKE_MAX_BYTES
                } else {
                    FRAME_MAX_BYTES
                };
                first = false;
                let frame = read_bounded_line(&mut reader, maximum);
                let terminal = frame.is_err();
                if sender.send(frame).is_err() || terminal {
                    break;
                }
            }
        });

        let (log_sender, logs) = mpsc::sync_channel(BACKEND_EVENT_MAX);
        thread::spawn(move || {
            let mut reader = BufReader::new(stderr);
            while let Some(line) = read_bounded_log_line(&mut reader, LOG_MAX_BYTES) {
                let _ = log_sender.try_send(line);
            }
        });

        let hello_frame = frames
            .recv_timeout(HANDSHAKE_TIMEOUT)
            .map_err(|_| AppError::unavailable(trace))?
            .map_err(|error| map_frame_error(error, trace))?;
        let hello: Hello = serde_json::from_slice(&hello_frame)
            .map_err(|_| AppError::mismatch("Backend hello is malformed.", trace))?;
        validate_hello(&hello, &manifest, env!("PRIME_SHELL_SCHEMA_HASH"))?;

        Ok(Self {
            child,
            stdin: Some(stdin),
            frames,
            _logs: logs,
            backend_version: hello.backend_version,
            pid,
            terminated: false,
        })
    }

    pub fn backend_version(&self) -> &str {
        &self.backend_version
    }

    pub fn send_request(
        &mut self,
        operation: BackendOperation,
        payload: &Value,
        request_id: &str,
        trace_id: &str,
    ) -> AppResult<()> {
        let request = RequestEnvelope {
            protocol: "generic-app",
            kind: "request",
            request_id,
            trace_id,
            operation: operation.name(),
            payload,
        };
        self.write_frame(&request, trace_id)
    }

    pub fn send_cancel(
        &mut self,
        task_id: &str,
        request_id: &str,
        trace_id: &str,
    ) -> AppResult<()> {
        let request = CancelEnvelope {
            protocol: "generic-app",
            kind: "cancel",
            request_id,
            trace_id,
            task_id,
        };
        self.write_frame(&request, trace_id)
    }

    pub fn receive(&mut self, timeout: Duration, trace_id: &str) -> AppResult<Option<Value>> {
        match self.frames.recv_timeout(timeout) {
            Ok(Ok(frame)) => serde_json::from_slice(&frame)
                .map(Some)
                .map_err(|_| AppError::protocol("Backend response is malformed.", trace_id)),
            Ok(Err(error)) => Err(map_frame_error(error, trace_id)),
            Err(mpsc::RecvTimeoutError::Timeout) => Ok(None),
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(AppError::crashed(trace_id)),
        }
    }

    pub fn echo(
        &mut self,
        text: &str,
        request_id: &str,
        trace_id: &str,
    ) -> AppResult<EchoResponse> {
        if text.chars().count() > TEXT_MAX_CHARACTERS {
            return Err(AppError::exhausted(trace_id));
        }
        let operation = BackendOperation::authorize("spike.echo", trace_id)?;
        let payload = json!({ "text": text });
        self.send_request(operation, &payload, request_id, trace_id)?;
        let frame = self
            .receive(REQUEST_TIMEOUT, trace_id)?
            .ok_or_else(|| AppError::timeout(trace_id))?;
        validate_echo_response(frame, request_id, trace_id)
    }

    pub fn terminate_now(&mut self) {
        if self.terminated {
            return;
        }
        self.stdin.take();
        signal_process_tree(self.pid, 15);
        let deadline = Instant::now() + FORCE_TERM_TIMEOUT;
        while Instant::now() < deadline {
            match self.child.try_wait() {
                Ok(Some(_)) => break,
                Ok(None) => thread::sleep(Duration::from_millis(10)),
                Err(_) => break,
            }
        }
        signal_process_tree(self.pid, 9);
        let _ = self.child.kill();
        let _ = self.child.wait();
        self.terminated = true;
    }

    pub fn shutdown(&mut self) {
        if self.terminated {
            return;
        }
        if let Some(stdin) = self.stdin.as_mut() {
            let _ = stdin.write_all(b"{\"protocol\":\"generic-app\",\"kind\":\"shutdown\"}\n");
            let _ = stdin.flush();
        }
        self.stdin.take();

        let deadline = Instant::now() + SHUTDOWN_TIMEOUT;
        while Instant::now() < deadline {
            match self.child.try_wait() {
                Ok(Some(_)) => {
                    signal_process_tree(self.pid, 9);
                    self.terminated = true;
                    return;
                }
                Ok(None) => thread::sleep(Duration::from_millis(20)),
                Err(_) => break,
            }
        }
        self.terminate_now();
    }

    fn write_frame<T: Serialize>(&mut self, value: &T, trace_id: &str) -> AppResult<()> {
        let mut encoded =
            serde_json::to_vec(value).map_err(|_| AppError::internal(trace_id.to_owned()))?;
        if encoded.len() > FRAME_MAX_BYTES {
            return Err(AppError::exhausted(trace_id));
        }
        encoded.push(b'\n');
        let stdin = self
            .stdin
            .as_mut()
            .ok_or_else(|| AppError::unavailable(trace_id))?;
        stdin
            .write_all(&encoded)
            .map_err(|_| AppError::crashed(trace_id))?;
        stdin.flush().map_err(|_| AppError::crashed(trace_id))
    }
}

fn record_forced_close_process_id(pid: u32) -> AppResult<()> {
    if env::var_os("PRIME_SHELL_NATIVE_FORCED_CLOSE_VERIFY").is_none() {
        return Ok(());
    }
    let path = env::var_os("PRIME_SHELL_FORCED_CLOSE_SIDECAR_PID_FILE").ok_or_else(|| {
        AppError::validation(
            "Forced-close sidecar PID evidence path is not configured.",
            "backend-launch",
        )
    })?;
    fs::write(path, format!("{pid}\n")).map_err(|_| AppError::io("backend-launch"))
}

impl Drop for BackendClient {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn validate_echo_response(
    response: Value,
    request_id: &str,
    trace_id: &str,
) -> AppResult<EchoResponse> {
    if response.get("protocol").and_then(Value::as_str) != Some("generic-app")
        || response.get("requestId").and_then(Value::as_str) != Some(request_id)
        || response.get("traceId").and_then(Value::as_str) != Some(trace_id)
    {
        return Err(AppError::protocol(
            "Backend response identity mismatch.",
            trace_id,
        ));
    }
    match response.get("kind").and_then(Value::as_str) {
        Some("result") => {
            if response.get("operation").and_then(Value::as_str) != Some("spike.echo") {
                return Err(AppError::protocol("Backend result is invalid.", trace_id));
            }
            let text = response
                .pointer("/payload/text")
                .and_then(Value::as_str)
                .ok_or_else(|| AppError::protocol("Backend result is missing.", trace_id))?;
            Ok(EchoResponse {
                text: text.to_owned(),
                trace_id: trace_id.to_owned(),
            })
        }
        Some("error") => Err(map_backend_error(&response, trace_id)),
        _ => Err(AppError::protocol(
            "Backend response kind is invalid.",
            trace_id,
        )),
    }
}

pub(crate) fn map_backend_error(value: &Value, trace_id: &str) -> AppError {
    let code = value
        .pointer("/error/code")
        .and_then(Value::as_str)
        .unwrap_or("PROTOCOL_ERROR");
    let message = value
        .pointer("/error/message")
        .and_then(Value::as_str)
        .unwrap_or("Backend rejected the request.");
    match code {
        "VALIDATION_ERROR" => AppError::validation(message, trace_id),
        "CONFLICT" => AppError::conflict(message, trace_id),
        "NOT_FOUND" => AppError::not_found(message, trace_id),
        "RESOURCE_EXHAUSTED" => AppError::exhausted(trace_id),
        "TIMEOUT" => AppError::timeout(trace_id),
        "CANCELLED" => AppError::cancelled(trace_id),
        "INTERRUPTED" => AppError::interrupted(trace_id),
        "BACKEND_CRASHED" => AppError::crashed(trace_id),
        _ => AppError::protocol("Backend rejected the request.", trace_id),
    }
}

fn map_frame_error(error: FrameReadError, trace_id: &str) -> AppError {
    match error {
        FrameReadError::TooLarge => AppError::exhausted(trace_id),
        FrameReadError::Io => AppError::io(trace_id),
        FrameReadError::Eof => AppError::crashed(trace_id),
    }
}

fn read_bounded_line<R: BufRead>(
    reader: &mut R,
    maximum: usize,
) -> Result<Vec<u8>, FrameReadError> {
    let mut line = Vec::with_capacity(maximum.min(8192));
    loop {
        let available = reader.fill_buf().map_err(|_| FrameReadError::Io)?;
        if available.is_empty() {
            return if line.is_empty() {
                Err(FrameReadError::Eof)
            } else {
                Ok(line)
            };
        }
        if let Some(index) = available.iter().position(|byte| *byte == b'\n') {
            if line.len() + index > maximum {
                reader.consume(index + 1);
                return Err(FrameReadError::TooLarge);
            }
            line.extend_from_slice(&available[..index]);
            reader.consume(index + 1);
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            return Ok(line);
        }
        if line.len() + available.len() > maximum {
            let count = available.len();
            reader.consume(count);
            return Err(FrameReadError::TooLarge);
        }
        let count = available.len();
        line.extend_from_slice(available);
        reader.consume(count);
    }
}

fn read_bounded_log_line<R: BufRead>(reader: &mut R, maximum: usize) -> Option<Vec<u8>> {
    let mut line = Vec::with_capacity(maximum.min(8192));
    let mut truncated = false;
    loop {
        let available = reader.fill_buf().ok()?;
        if available.is_empty() {
            if line.is_empty() && !truncated {
                return None;
            }
            break;
        }
        let newline = available.iter().position(|byte| *byte == b'\n');
        let count = newline.unwrap_or(available.len());
        if !truncated {
            let remaining = maximum.saturating_sub(line.len());
            line.extend_from_slice(&available[..count.min(remaining)]);
            truncated = count > remaining;
        }
        reader.consume(count + usize::from(newline.is_some()));
        if newline.is_some() {
            break;
        }
    }
    if truncated {
        Some(
            br#"{"component":"python-backend","event":"log_truncated","level":"warning"}"#.to_vec(),
        )
    } else {
        if line.last() == Some(&b'\r') {
            line.pop();
        }
        Some(line)
    }
}

fn target_identity() -> &'static str {
    if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        "windows-x86_64"
    } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        "macos-aarch64"
    } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        "linux-x86_64"
    } else {
        "unsupported-target"
    }
}

fn apply_minimal_environment(command: &mut Command) {
    if cfg!(windows) {
        for key in ["SYSTEMROOT", "WINDIR", "TEMP", "TMP"] {
            if let Some(value) = env::var_os(key) {
                command.env(key, value);
            }
        }
    } else {
        command.env("LANG", "C.UTF-8").env("LC_ALL", "C.UTF-8");
    }
}

#[cfg(unix)]
fn configure_process_containment(command: &mut Command) {
    use std::os::unix::process::CommandExt;

    command.process_group(0);
    #[cfg(target_os = "linux")]
    {
        let parent_pid = std::process::id() as i32;
        // SAFETY: the closure calls only async-signal-safe Linux syscalls before exec.
        unsafe {
            command.pre_exec(move || {
                // PR_SET_PDEATHSIG asks the kernel to kill the sidecar if the host dies.
                if c_prctl(1, 9, 0, 0, 0) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                if c_getppid() != parent_pid {
                    return Err(std::io::Error::other(
                        "native host exited before sidecar exec",
                    ));
                }
                Ok(())
            });
        }
    }
}

#[cfg(not(unix))]
fn configure_process_containment(_command: &mut Command) {}

#[cfg(unix)]
fn signal_process_tree(pid: u32, signal: i32) {
    // SAFETY: a negative PID targets only the process group created for this child.
    unsafe {
        let _ = c_kill(-(pid as i32), signal);
    }
}

#[cfg(not(unix))]
fn signal_process_tree(_pid: u32, _signal: i32) {}

#[cfg(unix)]
extern "C" {
    #[link_name = "kill"]
    fn c_kill(pid: i32, signal: i32) -> i32;
}

#[cfg(target_os = "linux")]
extern "C" {
    #[link_name = "prctl"]
    fn c_prctl(option: i32, arg2: u64, arg3: u64, arg4: u64, arg5: u64) -> i32;
    #[link_name = "getppid"]
    fn c_getppid() -> i32;
}

fn working_directory_from<'a>(executable: &'a Path, trace_id: &str) -> AppResult<&'a Path> {
    executable
        .parent()
        .ok_or_else(|| AppError::unavailable(trace_id))
}

fn verify_bundle(bundle_root: &Path, manifest: &BundleManifest, trace_id: &str) -> AppResult<()> {
    let canonical_root = bundle_root
        .canonicalize()
        .map_err(|_| AppError::unavailable(trace_id))?;
    let mut total = 0_u64;
    if manifest.files.is_empty() {
        return Err(AppError::mismatch("Sidecar manifest is empty.", trace_id));
    }
    for entry in &manifest.files {
        let path = bundle_root.join(&entry.path);
        let canonical = path
            .canonicalize()
            .map_err(|_| AppError::mismatch("Sidecar bundle file is missing.", trace_id))?;
        if !canonical.starts_with(&canonical_root) {
            return Err(AppError::mismatch(
                "Sidecar bundle path escapes its resource directory.",
                trace_id,
            ));
        }
        let bytes = fs::read(&canonical)
            .map_err(|_| AppError::mismatch("Sidecar bundle file is unreadable.", trace_id))?;
        if bytes.len() as u64 != entry.size {
            return Err(AppError::mismatch(
                "Sidecar bundle size mismatch.",
                trace_id,
            ));
        }
        let hash = format!("{:x}", Sha256::digest(&bytes));
        if hash != entry.sha256 {
            return Err(AppError::mismatch(
                "Sidecar bundle hash mismatch.",
                trace_id,
            ));
        }
        total = total.saturating_add(entry.size);
    }
    if total != manifest.bundle_bytes {
        return Err(AppError::mismatch(
            "Sidecar bundle aggregate size mismatch.",
            trace_id,
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use super::{read_bounded_line, read_bounded_log_line, FrameReadError};

    #[test]
    fn crlf_is_accepted() {
        let mut reader = BufReader::new(Cursor::new(b"{\"ok\":true}\r\n"));
        assert_eq!(
            read_bounded_line(&mut reader, 64).expect("line must parse"),
            b"{\"ok\":true}"
        );
    }

    #[test]
    fn oversized_frame_is_rejected_without_unbounded_growth() {
        let data = vec![b'x'; 65];
        let mut reader = BufReader::new(Cursor::new(data));
        assert!(matches!(
            read_bounded_line(&mut reader, 64),
            Err(FrameReadError::TooLarge)
        ));
    }

    #[test]
    fn oversized_log_is_safely_truncated() {
        let mut data = vec![b'x'; 96];
        data.push(b'\n');
        let mut reader = BufReader::new(Cursor::new(data));
        let line = read_bounded_log_line(&mut reader, 64).expect("log line");
        assert!(String::from_utf8(line)
            .expect("marker must be UTF-8")
            .contains("log_truncated"));
    }
}
