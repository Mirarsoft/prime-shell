use super::error::{AppError, AppResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendOperation {
    SpikeEcho,
}

impl BackendOperation {
    pub const fn name(self) -> &'static str {
        match self {
            Self::SpikeEcho => "spike.echo",
        }
    }

    pub fn authorize(name: &str, trace_id: &str) -> AppResult<Self> {
        match name {
            "spike.echo" => Ok(Self::SpikeEcho),
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
    fn echo_is_the_only_authorized_operation() {
        assert_eq!(
            BackendOperation::authorize("spike.echo", "trace-registry")
                .expect("echo must be authorized")
                .name(),
            "spike.echo"
        );
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
