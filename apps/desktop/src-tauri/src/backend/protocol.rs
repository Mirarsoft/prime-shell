use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::error::{AppError, AppResult};

pub const HANDSHAKE_MAX_BYTES: usize = 64 * 1024;
pub const FRAME_MAX_BYTES: usize = 1024 * 1024;
pub const LOG_MAX_BYTES: usize = 64 * 1024;
pub const TEXT_MAX_CHARACTERS: usize = 262_144;
pub const PENDING_REQUEST_MAX: usize = 64;
pub const BACKEND_EVENT_MAX: usize = 256;
pub const SUPPORTED_OPERATIONS: [&str; 5] = [
    "spike.echo",
    "spike.count",
    "spike.crash",
    "spike.hang",
    "spike.largeRejected",
];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Hello {
    pub protocol: String,
    pub kind: String,
    pub protocol_min: u32,
    pub protocol_max: u32,
    pub backend_version: String,
    pub build_id: String,
    pub target_triple: String,
    pub python_version: String,
    pub schema_hash: String,
    pub supported_operations: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BundleManifest {
    pub backend_version: String,
    pub build_id: String,
    pub schema_hash: String,
    pub target_triple: String,
    pub bundle_bytes: u64,
    pub files: Vec<BundleFile>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BundleFile {
    pub path: String,
    pub size: u64,
    pub sha256: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestEnvelope<'a> {
    pub protocol: &'static str,
    pub kind: &'static str,
    pub request_id: &'a str,
    pub trace_id: &'a str,
    pub operation: &'static str,
    pub payload: &'a Value,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CancelEnvelope<'a> {
    pub protocol: &'static str,
    pub kind: &'static str,
    pub request_id: &'a str,
    pub trace_id: &'a str,
    pub task_id: &'a str,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EchoResultPayload {
    pub text: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EchoResponse {
    pub text: String,
    pub trace_id: String,
}

pub fn validate_hello(
    hello: &Hello,
    manifest: &BundleManifest,
    expected_schema_hash: &str,
) -> AppResult<()> {
    let trace = "backend-handshake";
    if hello.protocol != "generic-app" || hello.kind != "hello" {
        return Err(AppError::mismatch(
            "Backend protocol identity mismatch.",
            trace,
        ));
    }
    if hello.protocol_min > 1 || hello.protocol_max < 1 {
        return Err(AppError::mismatch(
            "Backend protocol range mismatch.",
            trace,
        ));
    }
    if hello.backend_version != manifest.backend_version
        || hello.build_id != manifest.build_id
        || hello.schema_hash != manifest.schema_hash
        || hello.target_triple != manifest.target_triple
        || hello.schema_hash != expected_schema_hash
    {
        return Err(AppError::mismatch(
            "Packaged backend identity mismatch.",
            trace,
        ));
    }
    if hello.python_version.is_empty()
        || hello.supported_operations != SUPPORTED_OPERATIONS.map(str::to_owned).to_vec()
    {
        return Err(AppError::mismatch(
            "Packaged backend capabilities mismatch.",
            trace,
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{validate_hello, BundleManifest, Hello};

    fn manifest() -> BundleManifest {
        BundleManifest {
            backend_version: "0.1.0".to_owned(),
            build_id: "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                .to_owned(),
            schema_hash: "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                .to_owned(),
            target_triple: "linux-x86_64".to_owned(),
            bundle_bytes: 1,
            files: vec![],
        }
    }

    fn hello() -> Hello {
        Hello {
            protocol: "generic-app".to_owned(),
            kind: "hello".to_owned(),
            protocol_min: 1,
            protocol_max: 1,
            backend_version: "0.1.0".to_owned(),
            build_id: manifest().build_id,
            target_triple: "linux-x86_64".to_owned(),
            python_version: "3.12.13".to_owned(),
            schema_hash: manifest().schema_hash,
            supported_operations: super::SUPPORTED_OPERATIONS.map(str::to_owned).to_vec(),
        }
    }

    #[test]
    fn valid_handshake_passes() {
        let expected = manifest().schema_hash;
        validate_hello(&hello(), &manifest(), &expected).expect("handshake must pass");
    }

    #[test]
    fn stale_schema_is_rejected() {
        let error = validate_hello(
            &hello(),
            &manifest(),
            "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
        )
        .expect_err("stale schema must fail");
        assert_eq!(error.code, "BACKEND_PROTOCOL_MISMATCH");
    }
}
