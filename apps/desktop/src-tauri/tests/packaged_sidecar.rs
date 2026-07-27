use std::{env, path::PathBuf};

use prime_shell_desktop_lib::backend::{BackendClient, LaunchSpec};

#[test]
#[ignore = "requires PRIME_SHELL_PACKAGED_SIDECAR after the PyInstaller build"]
fn rust_to_packaged_python_unicode_echo() {
    let executable = PathBuf::from(
        env::var_os("PRIME_SHELL_PACKAGED_SIDECAR")
            .expect("PRIME_SHELL_PACKAGED_SIDECAR must be set"),
    );
    let bundle_directory = executable.parent().expect("bundle directory");
    let target_root = bundle_directory
        .parent()
        .expect("target root")
        .to_path_buf();
    let mut client = BackendClient::launch(LaunchSpec::from_paths(executable, target_root))
        .expect("packaged backend must launch");
    let text = "Hello — مرحبا — こんにちは 👋";
    let result = client
        .echo(text, "integration-request", "integration-trace")
        .expect("echo must succeed");
    assert_eq!(result.text, text);
}
