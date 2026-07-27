# WP01 Toolchain Record

## Pinned versions

| Component | Version |
|---|---:|
| Node | 24.14.0 |
| pnpm | 11.7.0 |
| Rust | 1.88.0 |
| Tauri JavaScript API | 2.11.1 |
| Tauri CLI | 2.11.4 |
| Tauri Rust crate | 2.11.5 |
| tauri-build | 2.6.3 |
| Python | 3.12.13 |
| PyInstaller | 6.21.0 |
| React / React DOM | 19.2.8 |
| Fluent UI React components | 9.74.4 |
| TypeScript | 6.0.3 |
| Vite | 8.1.5 |
| Vitest | 4.1.10 |
| Zod | 4.4.3 |
| Ajv | 8.20.0 |

The npm versions were resolved from the npm registry during WP01 preflight.
The Tauri Rust versions were resolved from current official docs.rs metadata.
PyInstaller was resolved from the package index and its 6.21 documentation.

## Execution host

- OS/kernel: Linux x86_64, kernel 6.12.13
- Node: available
- pnpm: available
- Python: available
- Rust: unavailable on the Work Session host
- WebKitGTK/GTK development packages: unavailable on the Work Session host
- Display server: unavailable

The unavailable Rust and native GUI prerequisites block Rust compilation,
Tauri packaging, and visible-window evidence on this host. They do not alter the
repository's pinned target configuration.

Exact GitHub Actions runner labels, current/previous macOS runtime evidence, and
cross-platform packaging remain WP03 scope and are not claimed by WP01.
