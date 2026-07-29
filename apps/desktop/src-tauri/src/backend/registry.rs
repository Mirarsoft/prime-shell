use super::error::{AppError, AppResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendOperation {
    Echo,
    Count,
    Crash,
    Hang,
    LargeRejected,
}

impl BackendOperation {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Echo => "spike.echo",
            Self::Count => "spike.count",
            Self::Crash => "spike.crash",
            Self::Hang => "spike.hang",
            Self::LargeRejected => "spike.largeRejected",
        }
    }

    pub const fn is_long_running(self) -> bool {
        matches!(self, Self::Count | Self::Crash | Self::Hang)
    }

    pub fn authorize(name: &str, trace_id: &str) -> AppResult<Self> {
        match name {
            "spike.echo" => Ok(Self::Echo),
            "spike.count" => Ok(Self::Count),
            "spike.crash" => Ok(Self::Crash),
            "spike.hang" => Ok(Self::Hang),
            "spike.largeRejected" => Ok(Self::LargeRejected),
            _ => Err(AppError::validation(
                "The requested operation is not authorized.",
                trace_id,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BackendOperation;

    #[test]
    fn unknown_operation_is_rejected_before_backend() {
        let error = BackendOperation::authorize("spike.future", "trace-registry")
            .expect_err("unknown operation must fail");
        assert_eq!(error.code, "VALIDATION_ERROR");
    }

    #[test]
    fn exact_spike_operations_are_authorized() {
        for name in [
            "spike.echo",
            "spike.count",
            "spike.crash",
            "spike.hang",
            "spike.largeRejected",
        ] {
            assert_eq!(
                BackendOperation::authorize(name, "trace-registry")
                    .expect("operation must be authorized")
                    .name(),
                name
            );
        }
    }

    #[test]
    fn shared_unknown_operation_fixture_is_rejected() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../../packages/app-contracts/fixtures/invalid/unknown-operation.json"
        ))
        .expect("fixture must be valid JSON");
        let operation = fixture["operation"]
            .as_str()
            .expect("fixture operation must be a string");
        assert!(BackendOperation::authorize(operation, "fixture-trace").is_err());
    }
}
