pub mod backend;

use std::{env, fs, sync::Arc, time::Duration};

use backend::{
    AppError, AppResult, BackendRuntime, BackendStatus, CancelReceipt, EchoResponse, LaunchSpec,
    TaskEventSink, TaskSnapshot,
};
use serde::Serialize;
use tauri::{ipc::Channel, AppHandle, Manager, State};

struct BackendState {
    runtime: BackendRuntime,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RuntimeProbeConfig {
    enabled: bool,
    evidence_path: Option<String>,
}

#[tauri::command]
fn backend_status(state: State<'_, BackendState>) -> AppResult<BackendStatus> {
    state.runtime.status()
}

#[tauri::command]
fn echo_text(text: String, state: State<'_, BackendState>) -> AppResult<EchoResponse> {
    state.runtime.echo(text)
}

#[tauri::command]
fn start_count(
    count_to: u64,
    interval_ms: u64,
    timeout_ms: u64,
    on_event: Channel<TaskSnapshot>,
    state: State<'_, BackendState>,
) -> AppResult<TaskSnapshot> {
    let trace = "start-count";
    if !(1..=10_000).contains(&count_to) {
        return Err(AppError::validation(
            "Count must be between 1 and 10000.",
            trace,
        ));
    }
    if !(10..=1_000).contains(&interval_ms) {
        return Err(AppError::validation(
            "Count interval must be between 10 and 1000 milliseconds.",
            trace,
        ));
    }
    if !(100..=300_000).contains(&timeout_ms) {
        return Err(AppError::validation(
            "Task timeout must be between 100 and 300000 milliseconds.",
            trace,
        ));
    }
    let sink: TaskEventSink = Arc::new(move |event| {
        let _ = on_event.send(event);
    });
    state.runtime.start(
        "spike.count",
        serde_json::json!({
            "countTo": count_to,
            "intervalMs": interval_ms,
        }),
        Duration::from_millis(timeout_ms),
        sink,
    )
}

#[tauri::command]
fn cancel_task(task_id: String, state: State<'_, BackendState>) -> AppResult<CancelReceipt> {
    state.runtime.cancel(task_id)
}

#[tauri::command]
fn task_status(task_id: String, state: State<'_, BackendState>) -> AppResult<TaskSnapshot> {
    state.runtime.task_status(task_id)
}

#[tauri::command]
fn recover_backend(state: State<'_, BackendState>) -> AppResult<BackendStatus> {
    state.runtime.recover()
}

#[tauri::command]
fn runtime_probe_config() -> RuntimeProbeConfig {
    let enabled = env::var_os("PRIME_SHELL_NATIVE_RUNTIME_VERIFY").is_some();
    RuntimeProbeConfig {
        enabled,
        evidence_path: env::var("PRIME_SHELL_RUNTIME_EVIDENCE").ok(),
    }
}

#[tauri::command]
fn write_runtime_evidence(evidence: serde_json::Value, app: AppHandle) -> AppResult<()> {
    let trace = "runtime-evidence";
    if env::var_os("PRIME_SHELL_NATIVE_RUNTIME_VERIFY").is_none() {
        return Err(AppError::validation(
            "Native runtime verification is disabled.",
            trace,
        ));
    }
    let path = env::var("PRIME_SHELL_RUNTIME_EVIDENCE")
        .map_err(|_| AppError::validation("Runtime evidence path is not configured.", trace))?;
    let bytes = serde_json::to_vec_pretty(&evidence).map_err(|_| AppError::internal(trace))?;
    fs::write(path, bytes).map_err(|_| AppError::io(trace))?;
    app.exit(0);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let resource_dir = app.path().resource_dir()?;
            app.manage(BackendState {
                runtime: BackendRuntime::launch(LaunchSpec::from_resource_dir(&resource_dir)),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            backend_status,
            echo_text,
            start_count,
            cancel_task,
            task_status,
            recover_backend,
            runtime_probe_config,
            write_runtime_evidence
        ])
        .run(tauri::generate_context!())
        .expect("error while running Prime Shell Echo Spike");
}
