# Phase 0A Definition

**Project:** `prime-shell`  
**Repository:** `prime-shell`

## 1. Purpose and scope

This document fixes only the product constraints and risk definitions needed to run the lean Phase 0B technical spike. It defines the representative workload, data classification, trust-boundary flows, initial platform/package matrix, Python assumptions, network posture, signing feasibility, ownership, and evidence responsibilities.

It does not implement the application or approve later product work.

## 2. Representative workload statement

The Phase 0B representative workload is a synthetic, local, product-neutral document-analysis flow:

1. The user selects a small UTF-8 plaintext file.
2. Rust brokers the native picker, validates the selected file, and issues an opaque reference.
3. A packaged Python sidecar performs deterministic CPU-only analysis.
4. The operation emits bounded/coalesced progress, supports cooperative cancellation, and returns a small structured result.
5. No database, remote API, account, GPU, model runtime, external Python installation, or runtime plugin is involved.

This is sufficient to test the selected Tauri + Rust + packaged-Python model because it crosses every risky boundary relevant to the spike: native file intent, typed frontend/native IPC, bounded Rust/Python transport, sidecar packaging and lifecycle, progress, cancellation, crash/hang handling, and safe result rendering. It is not intended to prove future native scientific or GPU dependencies.

## 3. Explicit non-goals

- Application or repository implementation
- Real product/domain functionality
- Product database or durable task resume
- Runtime plugins or a public module SDK
- Multiple Python workers or per-task worker processes
- GPU, scientific native stack, or model runtime validation
- Production updater, telemetry, or crash upload
- Production signing credentials or public distribution
- Broad Linux distribution support, stores, or extra architectures
- Enterprise deployment, policy, or compliance systems

## 4. Data classification

| Data | Classification | Phase 0B handling |
|---|---|---|
| User-selected UTF-8 test file | Local user-authored content; non-regulated and non-secret by spike rule | Treat as untrusted display data; operation-scoped access only |
| File metadata | Potentially sensitive, especially full path | Frontend receives display name, size, media type, and opaque ID; no full path by default |
| Analysis request/result | Small structured local data | Schema-validated at each trust boundary |
| Progress/task events | Operational metadata | Bounded, coalesced, no document content |
| Protocol identifiers and timings | Low-sensitivity diagnostics | Allowed in sanitized structured logs |
| Credentials, tokens, regulated records | Prohibited spike input | Must not be requested, bundled, transmitted, or logged |

Spike test inputs must not contain credentials, health data, financial data, biometric data, regulated personal records, or secrets.

## 5. Trust-boundary data-flow table

| Flow | Data entering | Enforcement/validation | Data leaving |
|---|---|---|---|
| User → Rust native picker | User selection intent | File type/size/path policy; reject invalid or unavailable files | Opaque `DocumentRef`, safe display metadata |
| Rust → React | Backend status, safe file metadata, task events, result, safe errors | Typed Tauri commands/channels; per-window capability; output schema; progress rate limit | Renderable text/structured state, never arbitrary native paths or commands |
| React → Rust | Typed operation request, opaque document ID, cancellation intent | Compile-time/generated operation registry; request schema; payload, timeout, and window checks | Accepted/rejected request and safe status |
| Rust → Python | Versioned operation envelope, scoped input/path, request/task/trace IDs | Exact bundled executable; no shell; minimal environment; operation allowlist; frame limits | Only data required for the selected operation |
| Python → Rust | `hello`, result/error, progress/terminal events, structured stderr logs | UTF-8 JSON Lines framing; schema/version/hash checks; size/queue limits; stdout protocol-only | Validated native state or deterministic failure |
| Rust → local diagnostics | Allowlisted operational fields | Sanitization, truncation, bounded retention | Local-only diagnostic evidence |
| CI/test harness → packaged app | Synthetic fixtures and test actions | Non-production test capability; known fixtures | Build, package, E2E, lifecycle, and measurement artifacts |

Data that must never appear in logs or frontend bundles: document contents, raw input payloads, credentials/tokens, authorization headers, environment dumps, signing material, unrestricted native paths, and secrets. Full paths are excluded from default logs and frontend state.

## 6. Threat model

### Assets

- Integrity of the trusted Rust host and packaged Python sidecar
- User-selected document confidentiality
- Process and host stability
- Protocol/task state integrity
- Signing keys and release identity, when later introduced
- Sanitized diagnostic evidence

### Trust boundaries

- User-controlled file to Rust
- Least-trusted React webview to Rust
- Rust to trusted packaged Python over stdio
- Application/sidecar to local filesystem and process environment
- CI artifacts to clean-install test environments

### Credible threats and required spike mitigations

| Threat | Required Phase 0B mitigation/evidence |
|---|---|
| Webview requests unknown/native-dangerous action | Rust operation registry rejects unknown operations before Python; no generic shell/filesystem API |
| Malformed or oversized protocol data exhausts memory or corrupts state | 64 KiB handshake, 1 MiB normal frame, bounded pending/event queues, deterministic rejection |
| User content becomes privileged HTML/SVG or leaks into logs | Render as text; no raw HTML; structured-log allowlist; fixture checks for redaction |
| Stray stdout/log data corrupts protocol | Protocol-only stdout, structured bounded stderr, continuous concurrent draining |
| Sidecar hang or cancellation race leaves false success state | Defined terminal precedence, acknowledgement versus stop distinction, bounded escalation, no rollback claim |
| Sidecar crash causes silent replay or duplicate effects | Fail in-flight work, one bounded restart, no automatic replay |
| Host close leaves a Python descendant | Windows Job Object or Unix process group/session candidate; verify zero orphans after normal and forced close |
| Wrong or stale sidecar is launched | Exact contained resource path, expected build/schema hash and target checked during handshake |
| Excess inherited environment exposes secrets | Construct minimal environment; no automatic credential/proxy inheritance |
| CSP relaxation permits unsafe content/script execution | Release-mode CSP test with bundled assets and Griffel nonce/configuration; no broad `unsafe-inline` workaround |
| Test driver ships in production | Inspect release artifacts and verify test-only WebDriver capability is absent |

### Threats explicitly deferred

- Untrusted third-party plugins
- Remote content execution and product network endpoints
- Production update delivery and key rotation
- Database encryption and regulated-data controls
- OS sandboxing of the trusted sidecar
- GPU/model/native scientific supply-chain risks
- Enterprise deployment and remote diagnostics

### Residual risk

Python remains trusted native code with current-user privileges, not a sandbox. Platform process-tree and title-bar behavior can only be proven on target systems. Unsigned macOS CI artifacts do not prove Gatekeeper, signing, notarization, or final native UX. Future native/GPU dependencies can materially change bundle and signing risk and require targeted revalidation.

## 7. Security assumptions

- React is the least-trusted application layer.
- Rust is the native policy and process-lifecycle authority.
- Python and its dependencies are reviewed first-party release artifacts.
- The sidecar is launched from the exact bundled resource without a shell.
- The spike uses only controlled synthetic/non-secret input.
- All cross-boundary messages are validated and bounded.
- Unknown operations are rejected in Rust.
- No uncertain operation is automatically replayed.
- All UI assets are local and the network is denied by default.
- Test-only driver capability is excluded from production artifacts.

## 8. Python runtime and dependency profile

| Assumption | Phase 0B validation |
|---|---|
| Packaged runtime; no external Python | Clean environment launches packaged `onedir` sidecar |
| CPU-only, standard library preferred | Implement test operations without GPU/scientific/model/database/network dependencies |
| One long cancellable task | `spike.count` stays responsive to cancellation/control input |
| Short operations may coexist only if simple | Demonstrate safe echo/status behavior or retain serialized behavior |
| Target-specific build | Build Python bundle separately on Windows x64, macOS arm64, and Ubuntu x64 |
| UTF-8/unbuffered stdio | Unicode, CRLF, fragmentation, and timely progress evidence |
| Locked supply chain | Pin Python and PyInstaller; record versions and dependency/build metadata |
| Inspectable bundle | Record contents, size, startup, antivirus/Gatekeeper limitations, and native library layout |

The spike does not validate future native scientific, GPU, model, database, or network-client dependencies. Their introduction is a revisit trigger.

## 9. Exact support/build/test matrix

| Target | Architecture | Webview/window system | Spike package candidate | Automated/build owner | Manual evidence owner |
|---|---|---|---|---|---|
| Windows 11 | x64 | WebView2 / DWM | NSIS candidate | Work Session / CI | User or designated Windows tester |
| Current and previous major macOS | arm64 | WKWebView | Unsigned `.app`; DMG layout candidate | GitHub Actions macOS runner | External Mac tester before public release |
| Ubuntu 24.04 LTS | x64 | WebKitGTK on Wayland and X11 | `.deb` | GitHub Actions Ubuntu runner | Designated Ubuntu tester |

No other OS, distribution, architecture, store, webview, or package format is in the initial matrix. GitHub Actions can establish macOS build/automated evidence but cannot replace final signed/notarized clean-install and native UX evidence.

## 10. Initial package choices

- Windows: NSIS candidate.
- macOS: unsigned `.app` in CI with DMG layout candidate; public release later requires signing and notarization.
- Linux: exactly one `.deb` candidate.
- Python: PyInstaller `onedir` built per OS/architecture and bundled as Tauri resources.

`.deb` is selected for Ubuntu 24.04 because it provides repeatable CI artifact creation, explicit dependency metadata, straightforward installation/removal, and clear clean-install smoke evidence on the one declared distribution. The tradeoff is intentional lack of portability: this spike does not claim support for other Debian/Ubuntu releases or Linux distributions.

## 11. Network policy

- Network denied by default.
- UI assets, fonts, and application resources are bundled locally.
- No telemetry, crash upload, remote API, account, or runtime package installation.
- Privileged webviews do not load arbitrary remote HTML/scripts.
- CI dependency retrieval is a build concern, not runtime network permission.
- Update checking/signature transport is design-only during the spike and is not a production feature.

## 12. Signing and notarization feasibility

| Area | Phase 0A/0B feasible evidence | Deferred requirement |
|---|---|---|
| Windows developer/test signing | Document certificate/tool path; test-sign pipeline if credentials/environment permit; inspect artifact layout | Trusted production certificate, protected key custody, reputation/SmartScreen operational evidence |
| macOS CI | Build unsigned arm64 `.app`; inspect nested sidecar/native layout and signing order requirements | Apple Developer identity, hardened runtime/entitlements as needed, signing, notarization, stapling, Gatekeeper clean-install pass |
| Linux | Package integrity/hash and clean `.deb` install/remove evidence | Distribution signing/repository policy if a product later needs it |
| Updater signatures | Reserve public-key/configuration strategy and avoid layout choices that block later signing | Production updater, private-key custody/rotation, channel separation, signed end-to-end update test |

Production credentials are not a Phase 0A readiness blocker. Public macOS distribution is blocked until signing and notarization pass once on the packaged app.

## 13. Ownership and evidence responsibilities

| Responsibility | Owner |
|---|---|
| Windows build/automated evidence | Work Session / CI |
| macOS build/automated evidence | GitHub Actions macOS runner |
| Linux build/automated evidence | GitHub Actions Ubuntu runner |
| Windows native title-bar/manual evidence | User or designated Windows tester |
| macOS final native UX and signed-package evidence | External Mac tester before public release |
| Linux Wayland/X11 compositor/manual evidence | Designated Ubuntu tester |
| Architecture review and phase approval | Chat Session |
| Phase 0B implementation, measurements, and artifact inventory | Work Session |

## 14. Known assumptions

- Synthetic plaintext analysis adequately represents the initial architectural risks.
- GitHub Actions offers suitable Windows, macOS arm64-capable build strategy, and Ubuntu runners; exact runner labels/tool availability are verified when Phase 0B begins.
- Target test machines/runners can install required Tauri/webview build dependencies.
- The spike has no production signing credentials.
- A designated Windows tester and Ubuntu tester can provide native manual evidence; if unavailable, those manual gates remain unverified rather than silently passed.
- Current/previous macOS major-version runtime coverage may require external hardware beyond CI build evidence.

These assumptions do not invalidate beginning the spike, but missing target execution evidence prevents the affected gate from passing.

## 15. Deferred product decisions

- Real first product and real data sensitivity
- Additional CPU architectures or Linux distributions
- GPU/native scientific/model dependencies
- Enterprise deployment and managed/offline update policy
- Production signing providers and key custody
- Production updater enablement
- Database choice/encryption
- Telemetry, crash reporting, and remote diagnostics
- Store distribution
- Module SDK and template extraction

## 16. Phase 0A decision summary

- The representative workload is sufficient for the lean architecture spike.
- Data is local, user-selected, non-regulated plaintext, treated as untrusted display content.
- Rust owns native policy and process lifecycle; Python is trusted native code, not sandboxed.
- The exact matrix is Windows 11 x64, macOS arm64 current/previous major versions, and Ubuntu 24.04 x64.
- Linux uses one `.deb` candidate.
- Python is packaged `onedir`, CPU-only, standard-library-first, and single-long-task initially.
- Runtime network access, telemetry, crash upload, and production updater are absent.
- Production signing credentials are deferred; public macOS release still requires successful signing/notarization.
- No known unknown currently invalidates Tauri + sidecar before Phase 0B.
- Phase 0A status is `READY WITH ASSUMPTIONS`; target runner/tool availability and manual platform evidence must be verified in Phase 0B.
