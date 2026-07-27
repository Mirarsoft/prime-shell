# WP01 Status Inventory

| Category | Status | Primary evidence |
|---|---|---|
| Minimal React/Fluent UI | Implemented | `apps/desktop/src/App.tsx`; release build passed |
| Typed frontend command boundary | Implemented | `apps/desktop/src/backend.ts`; typecheck/tests passed |
| Tauri command permissions | Implemented, uncompiled | `src-tauri/capabilities/main.json`, `permissions/app.toml` |
| Rust operation registry | Implemented, uncompiled | `src-tauri/src/backend/registry.rs` |
| Rust bounded sidecar client | Implemented, uncompiled | `src-tauri/src/backend/client.rs` |
| Rust handshake/error models | Implemented, uncompiled | `src-tauri/src/backend/protocol.rs` |
| JSON Schema authority | Implemented | schema/fixture validation passed |
| Python protocol and echo | Implemented | 8 tests passed |
| PyInstaller onedir build | Implemented | packaged build and manifest passed |
| Packaged sidecar direct execution | Implemented | `packaged_echo.py` passed |
| Rust-to-packaged integration | Blocked | Cargo absent |
| Tauri native application build | Blocked | Cargo/WebKitGTK/rsvg absent |
| Visible GUI/CSP runtime evidence | Blocked | native prerequisites/display absent |
| Cargo lockfile | Blocked | Cargo unavailable; no fabricated lock created |
| Generated code | Not started | Not required; idiomatic boundary models used |
| Fixtures | Implemented | 3 valid/3 invalid shared fixtures |
| Mocks | Tests only | Tauri `invoke` mocked in 2 frontend tests |
| Production stubs/placeholders | Not started | None present |
| WP02 lifecycle functionality | Not started | Explicitly excluded |
| WP03 cross-platform evidence | Not started | Explicitly excluded |
| Product features | Not started | Explicitly excluded |

The full packaged Unicode echo vertical slice is `Partially implemented` until
Rust compilation and the Rust-to-packaged integration test pass.
