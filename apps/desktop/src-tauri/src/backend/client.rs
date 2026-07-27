use std::{
    env, fs,
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

use serde::Serialize;
use sha2::{Digest, Sha256};

use super::{
    error::{AppError, AppResult},
    protocol::{
        validate_hello, BackendStatus, BundleManifest, EchoPayload, EchoResponse, Hello,
        RequestEnvelope, ResponseEnvelope, FRAME_MAX_BYTES, HANDSHAKE_MAX_BYTES,
    },
    registry::BackendOperation,
};

const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(3);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Debug)]
enum FrameReadError {
    Io,
    Eof,
    TooLarge,
}

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
    backend_version: String,
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

        let mut child = command.spawn().map_err(|_| AppError::unavailable(trace))?;
        let stdin = child.stdin.take().ok_or_else(|| AppError::io(trace))?;
        let stdout = child.stdout.take().ok_or_else(|| AppError::io(trace))?;
        let stderr = child.stderr.take().ok_or_else(|| AppError::io(trace))?;

        let (sender, frames) = mpsc::sync_channel(64);
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

        thread::spawn(move || {
            let mut reader = BufReader::new(stderr);
            let mut chunk = [0_u8; 8192];
            while let Ok(count) = reader.read(&mut chunk) {
                if count == 0 {
                    break;
                }
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
            backend_version: hello.backend_version,
        })
    }

    pub fn status(&self) -> BackendStatus {
        BackendStatus {
            ready: true,
            backend_version: Some(self.backend_version.clone()),
        }
    }

    pub fn echo(
        &mut self,
        text: &str,
        request_id: &str,
        trace_id: &str,
    ) -> AppResult<EchoResponse> {
        if text.chars().count() > super::protocol::TEXT_MAX_CHARACTERS {
            return Err(AppError::exhausted(trace_id));
        }
        let operation = BackendOperation::authorize("spike.echo", trace_id)?;
        let request = RequestEnvelope {
            protocol: "generic-app",
            kind: "request",
            request_id,
            trace_id,
            operation: operation.name(),
            payload: EchoPayload { text },
        };
        self.write_frame(&request, trace_id)?;

        let frame = self
            .frames
            .recv_timeout(REQUEST_TIMEOUT)
            .map_err(|_| AppError::unavailable(trace_id))?
            .map_err(|error| map_frame_error(error, trace_id))?;
        let response: ResponseEnvelope = serde_json::from_slice(&frame)
            .map_err(|_| AppError::protocol("Backend response is malformed.", trace_id))?;
        validate_response(response, request_id, trace_id)
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
            .map_err(|_| AppError::io(trace_id))?;
        stdin.flush().map_err(|_| AppError::io(trace_id))
    }
}

impl Drop for BackendClient {
    fn drop(&mut self) {
        if let Some(stdin) = self.stdin.as_mut() {
            let _ = stdin.write_all(b"{\"protocol\":\"generic-app\",\"kind\":\"shutdown\"}\n");
            let _ = stdin.flush();
        }
        self.stdin.take();

        let deadline = Instant::now() + SHUTDOWN_TIMEOUT;
        while Instant::now() < deadline {
            match self.child.try_wait() {
                Ok(Some(_)) => return,
                Ok(None) => thread::sleep(Duration::from_millis(20)),
                Err(_) => break,
            }
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn validate_response(
    response: ResponseEnvelope,
    request_id: &str,
    trace_id: &str,
) -> AppResult<EchoResponse> {
    if response.protocol != "generic-app"
        || response.request_id.as_deref() != Some(request_id)
        || response.trace_id != trace_id
    {
        return Err(AppError::protocol(
            "Backend response identity mismatch.",
            trace_id,
        ));
    }
    match response.kind.as_str() {
        "result" => {
            if response.operation.as_deref() != Some("spike.echo") || response.error.is_some() {
                return Err(AppError::protocol("Backend result is invalid.", trace_id));
            }
            let payload = response
                .payload
                .ok_or_else(|| AppError::protocol("Backend result is missing.", trace_id))?;
            Ok(EchoResponse {
                text: payload.text,
                trace_id: trace_id.to_owned(),
            })
        }
        "error" => {
            let error = response
                .error
                .ok_or_else(|| AppError::protocol("Backend error is missing.", trace_id))?;
            match error.code.as_str() {
                "VALIDATION_ERROR" => Err(AppError::validation(error.message, trace_id)),
                "RESOURCE_EXHAUSTED" => Err(AppError::exhausted(trace_id)),
                _ => Err(AppError::protocol(
                    "Backend rejected the request.",
                    trace_id,
                )),
            }
        }
        _ => Err(AppError::protocol(
            "Backend response kind is invalid.",
            trace_id,
        )),
    }
}

fn map_frame_error(error: FrameReadError, trace_id: &str) -> AppError {
    match error {
        FrameReadError::TooLarge => AppError::exhausted(trace_id),
        FrameReadError::Io | FrameReadError::Eof => AppError::io(trace_id),
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
            return Err(FrameReadError::TooLarge);
        }
        let count = available.len();
        line.extend_from_slice(available);
        reader.consume(count);
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

    use super::{read_bounded_line, FrameReadError};

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
}
