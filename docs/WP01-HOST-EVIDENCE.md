# WP01 Host Evidence

## Result

**WP01 status: Partially implemented**

The packaged Python sidecar and release frontend build are verified on the Work
Session's Linux x86_64 host. The production Rust-to-sidecar path, Tauri native
host build, release GUI, and CSP runtime rendering are `Blocked` because the
host lacks Rust/Cargo, WebKitGTK/rsvg development dependencies, and a display
server.

This is not a claim that the WP01 completion gate passed.

## Verified commands

```text
pnpm install --frozen-lockfile
pnpm contract:test
pnpm typecheck
pnpm lint
pnpm test
pnpm build
pnpm sidecar:test
pnpm sidecar:build
pnpm sidecar:verify
```

Observed results:

- Supply-chain policy and frozen pnpm lock: passed.
- JSON Schema 2020-12: 5 schemas, 3 valid fixtures, 3 invalid fixtures passed.
- TypeScript typecheck: passed.
- ESLint: passed with zero warnings.
- Frontend tests: 2 passed.
- Vite release build: passed; 2,214 modules transformed.
- Python tests: 8 passed.
- PyInstaller 6.21.0 `onedir`: built successfully.
- Packaged handshake: 41.339 ms in the final retained run.
- Packaged bundle size: 34,873,410 bytes.
- Unicode echo: passed.
- Unknown operation: rejected.
- Malformed UTF-8 JSON frame: rejected.
- Oversized frame: rejected with `RESOURCE_EXHAUSTED`.
- Protocol-only stdout: passed.
- Structured stderr: passed.
- Bounded requested shutdown: passed.
- External Python path supplied to packaged process: false.

Packaged identities:

```text
buildId: sha256:908226bef71fb2cd8fd38216153f4fc7415a49708dffe1ec8c4739d64387fdb9
schemaHash: sha256:787d5fb68543ef7673d4b864d9bfe4f6d20ff078e836d5dc7b0315a695a8617f
targetTriple: linux-x86_64
```

## Blocked commands

```text
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
pnpm --filter @prime-shell/desktop tauri build
```

Observed blocker:

```text
cargo: command not found
rustc: not installed
webkit2gtk-4.1: not installed
rsvg2: not installed
```

The Tauri CLI successfully parsed the application and reported:

- Tauri Rust dependency configured as `=2.11.5`
- Tauri JavaScript API `2.11.1`
- Tauri CLI `2.11.4`
- React/Vite frontend detected
- bundle build type detected
- strict local CSP detected

## Unverified evidence

- Rust formatting, compilation, clippy, and unit/integration tests
- Rust launch of the PyInstaller sidecar
- Tauri native host build/package
- Visible Fluent window
- Runtime CSP/Griffel behavior
- Tauri resource-copy layout
- Host-exit sidecar cleanup through Rust
- Any Windows, macOS, Wayland, or X11 claim

Those items require a suitable Rust/Tauri host or a focused correction/re-run.
They do not authorize WP02.

