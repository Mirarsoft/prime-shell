pub mod backend;

use std::sync::{
    atomic::{AtomicU64, Ordering},
    Mutex,
};

use backend::{AppError, AppResult, BackendClient, BackendStatus, EchoResponse, LaunchSpec};
use tauri::{Manager, State};

struct BackendState {
    client: Mutex<Option<BackendClient>>,
}

static REQUEST_SEQUENCE: AtomicU64 = AtomicU64::new(1);

#[tauri::command]
fn backend_status(state: State<'_, BackendState>) -> AppResult<BackendStatus> {
    let client = state
        .client
        .lock()
        .map_err(|_| AppError::internal("backend-status"))?;
    Ok(match client.as_ref() {
        Some(client) => client.status(),
        None => BackendStatus {
            ready: false,
            backend_version: None,
        },
    })
}

#[tauri::command]
fn echo_text(text: String, state: State<'_, BackendState>) -> AppResult<EchoResponse> {
    let sequence = REQUEST_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let request_id = format!("request-{sequence}");
    let trace_id = format!("trace-{}-{sequence}", std::process::id());
    let mut client = state
        .client
        .lock()
        .map_err(|_| AppError::internal(&trace_id))?;
    client
        .as_mut()
        .ok_or_else(|| AppError::unavailable(&trace_id))?
        .echo(&text, &request_id, &trace_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let client = app
                .path()
                .resource_dir()
                .ok()
                .and_then(|resource_dir| {
                    BackendClient::launch(LaunchSpec::from_resource_dir(&resource_dir)).ok()
                });
            app.manage(BackendState {
                client: Mutex::new(client),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![backend_status, echo_text])
        .run(tauri::generate_context!())
        .expect("error while running Prime Shell Echo Spike");
}

