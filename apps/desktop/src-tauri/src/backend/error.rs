use serde::Serialize;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    pub code: &'static str,
    pub message: String,
    pub trace_id: String,
}

impl AppError {
    pub fn validation(message: impl Into<String>, trace_id: impl Into<String>) -> Self {
        Self {
            code: "VALIDATION_ERROR",
            message: message.into(),
            trace_id: trace_id.into(),
        }
    }

    pub fn unavailable(trace_id: impl Into<String>) -> Self {
        Self {
            code: "BACKEND_UNAVAILABLE",
            message: "The packaged backend is unavailable.".to_owned(),
            trace_id: trace_id.into(),
        }
    }

    pub fn protocol(message: impl Into<String>, trace_id: impl Into<String>) -> Self {
        Self {
            code: "PROTOCOL_ERROR",
            message: message.into(),
            trace_id: trace_id.into(),
        }
    }

    pub fn mismatch(message: impl Into<String>, trace_id: impl Into<String>) -> Self {
        Self {
            code: "BACKEND_PROTOCOL_MISMATCH",
            message: message.into(),
            trace_id: trace_id.into(),
        }
    }

    pub fn exhausted(trace_id: impl Into<String>) -> Self {
        Self {
            code: "RESOURCE_EXHAUSTED",
            message: "The request exceeds the configured protocol limit.".to_owned(),
            trace_id: trace_id.into(),
        }
    }

    pub fn io(trace_id: impl Into<String>) -> Self {
        Self {
            code: "IO_ERROR",
            message: "The packaged backend could not be reached.".to_owned(),
            trace_id: trace_id.into(),
        }
    }

    pub fn internal(trace_id: impl Into<String>) -> Self {
        Self {
            code: "INTERNAL_ERROR",
            message: "The echo request could not be completed.".to_owned(),
            trace_id: trace_id.into(),
        }
    }
}
