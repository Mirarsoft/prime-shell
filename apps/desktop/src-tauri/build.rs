use sha2::{Digest, Sha256};
use std::{fs, path::Path};

fn collect_json(root: &Path, current: &Path, paths: &mut Vec<std::path::PathBuf>) {
    let mut entries: Vec<_> = fs::read_dir(current)
        .expect("schema directory must be readable")
        .map(|entry| entry.expect("schema entry must be readable").path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_json(root, &path, paths);
        } else if path.extension().and_then(|value| value.to_str()) == Some("json") {
            let _ = root;
            paths.push(path);
        }
    }
}

fn main() {
    let schema_root = Path::new("../../../packages/app-contracts/schemas");
    let mut paths = Vec::new();
    collect_json(schema_root, schema_root, &mut paths);

    let mut digest = Sha256::new();
    for path in paths {
        let relative = path
            .strip_prefix(schema_root)
            .expect("schema must remain under schema root");
        digest.update(relative.to_string_lossy().replace('\\', "/").as_bytes());
        digest.update([0]);
        digest.update(fs::read(&path).expect("schema must be readable"));
        println!("cargo:rerun-if-changed={}", path.display());
    }
    println!(
        "cargo:rustc-env=PRIME_SHELL_SCHEMA_HASH=sha256:{:x}",
        digest.finalize()
    );
    tauri_build::build();
}
