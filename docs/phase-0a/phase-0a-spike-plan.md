# Phase 0A — Lean Phase 0B Spike Plan

**Project:** `prime-shell`  
**Repository:** `prime-shell`

## 1. Spike objective

Prove, with retained cross-platform evidence, that a minimal Tauri 2 + React/TypeScript + Fluent UI application can launch and control a packaged PyInstaller `onedir` Python sidecar through a bounded typed protocol, handle progress/cancellation/crash/hang lifecycle correctly, package on the exact target matrix, and fall back safely when custom title-bar behavior is not reliable.

The spike is a risk-reduction vertical slice, not a mini-product or reusable framework.

## 2. In-scope vertical slice

1. Launch a minimal themed Fluent window.
2. Rust launches the exact bundled sidecar resource without a shell.
3. Validate sidecar handshake and schema/build metadata.
4. Execute one typed Unicode echo round trip.
5. Run one synthetic bounded-progress count task.
6. Cancel it to a deterministic terminal state.
7. Trigger deliberate crash and hang paths.
8. Detect failure, fail in-flight work, perform at most one bounded restart, and never replay work.
9. Reject an oversized request/frame deterministically.
10. Close the host and prove zero surviving sidecar/descendant processes.
11. Drive one native Tauri smoke flow with WebdriverIO.
12. Build/package and retain measurements on the declared target matrix.

## 3. Explicit exclusions

- Real document-analysis product feature or database
- Product persistence, durable tasks, or automatic replay
- Runtime plugins, module SDK, app generator, or `packages/ui`
- Multiple Python workers or per-task worker processes
- GPU, model, scientific native stack, database, or runtime network client
- Production updater, telemetry, crash upload, or enterprise deployment
- Production signing/notarization completion
- Broad component library, full settings system, or final design system
- Additional OS, Linux distribution, package format, or CPU architecture

## 4. Required test-only operations

| Operation | Purpose | Required result |
|---|---|---|
| `spike.echo` | Typed Unicode request/response and schema validation | Exact validated Unicode echo; bounded payload |
| `spike.count` | One synthetic long task with bounded/coalesced progress | Monotonic sequence, terminal event retained, UI ≤10 progress updates/s |
| `spike.crash` | Unexpected sidecar exit | In-flight work fails; crash detected; one restart; no replay |
| `spike.hang` | Timeout/cancellation escalation | Bounded acknowledgement/stop handling; escalation marks affected work `Interrupted` |
| `spike.largeRejected` | Oversized input/frame defense | Rust or protocol boundary rejects deterministically without unbounded allocation or sidecar corruption |

These operations are test-only and must not be presented as product functionality.

## 5. Proposed repository areas that Phase 0B may create

```text
apps/desktop/                 # minimal React/Fluent/Tauri host
apps/desktop/src-tauri/       # Rust commands, operation registry, sidecar lifecycle
packages/app-contracts/       # JSON Schema 2020-12 and shared fixtures
services/python-backend/      # test-only Python sidecar and PyInstaller spec/config
tests/contract/               # cross-language valid/invalid fixtures
tests/e2e/                    # one native WebdriverIO smoke path
tests/platform-smoke/         # package/install/process evidence scripts
scripts/build-sidecar/        # target-specific sidecar build entry points
scripts/package/              # narrow target package commands
docs/spike/                   # measurements, deviations, evidence index
.github/workflows/            # exact target build/test jobs
```

Create only areas with an immediate spike consumer. Do not create `packages/ui`, `packages/module-sdk`, broad shared test utilities, generators, product feature folders, or production release systems.

## 6. Ordered implementation sequence

1. Pin the minimum Node/pnpm, Rust, Python, Tauri, Fluent, PyInstaller, and test versions; create the minimal host.
2. Establish release-mode CSP early and render a minimal Fluent/Griffel surface.
3. Define JSON Schema envelopes, handshake, errors, task events, and five test operations with shared valid/invalid fixtures.
4. Implement Python protocol-only stdout, structured stderr, handshake, echo, count, crash, hang, and oversized-input behavior.
5. Package Python as target-specific `onedir`; bundle the full directory as Tauri resources.
6. Implement Rust exact-resource launch, minimal environment, concurrent pipe draining, bounded framing/queues, handshake validation, and operation registry.
7. Add task progress, cancellation races, deadlines, crash detection, one-restart budget, circuit behavior, and no-replay semantics.
8. Add process-tree containment and normal/forced-close verification.
9. Implement the Windows title-bar candidate and platform-native fallbacks; retain macOS traffic lights and Linux native decorations.
10. Add one WebdriverIO native smoke journey and verify test capability exclusion from production artifacts.
11. Build/package on Windows, macOS, and Ubuntu; run applicable automated and clean-install smoke checks.
12. Collect measurements, manual-evidence gaps, package hashes, and targeted deviations; stop for Chat Session review.

## 7. Contract and protocol boundaries

- JSON Schema 2020-12 plus shared valid/invalid fixtures is authoritative.
- TypeScript runtime validation is generated or equivalent; Rust/Python boundary models may be idiomatic if both consume the same fixtures.
- UTF-8 JSON Lines over stdin/stdout; CRLF accepted.
- `stdout` is protocol-only; `stderr` is structured logs only.
- Handshake maximum: 64 KiB; normal frame maximum: 1 MiB; log line maximum: 64 KiB with truncation marker.
- Initial pending-request limit: 64; backend event queue: 256.
- UI progress: at most 10 events/second/task; terminal events are never coalesced away.
- Rust mints trace IDs and rejects unknown operations before Python.
- Handshake checks protocol overlap, target, backend/build identity, schema hash, and supported operations.
- Large/binary values use no inline path in the spike; oversized input is rejected.
- Invalid UTF-8/JSON, unknown kinds/operations, corruption, and size violations fail deterministically.

## 8. Process lifecycle and cancellation evidence

Retain evidence for:

- Valid cold handshake and invalid/late handshake.
- One active long task with independently responsive control-message reading.
- Cancel before acceptance, during execution, and after success.
- Cancellation acknowledgement distinguished from actual stop.
- Success accepted by Rust wins a later cancellation race.
- Cooperative stop target and escalation deadline.
- `spike.hang` escalation terminates/restarts the affected sidecar and marks in-flight work `Interrupted`.
- `spike.crash` fails in-flight work with a stable error, restarts once, and does not replay.
- Repeated crash opens a circuit/requires explicit action.
- Normal close and forced host close leave zero descendants.
- Windows Job Object and Unix process group/session behavior are documented per target.

Evidence artifacts: structured sanitized logs, task-event traces, test reports, process listings/counts, and a platform result table. No evidence may include document contents, raw payloads, credentials, environment dumps, or full paths by default.

## 9. Title-bar candidate and fallback evidence

| Platform | Candidate/default | Evidence |
|---|---|---|
| Windows 11 | Custom Fluent candidate | Drag/non-drag regions; minimize/maximize/restore/close; double-click; system menu; `Win+Z`/Snap Layouts; keyboard/accessibility; 100/125/150/200% scale; mixed DPI; forced colors; fallback |
| macOS | Native traffic lights with overlay/transparent area if safe | Movement, full screen, focus, theme background, safe areas; retain native behavior |
| Ubuntu | Native decorations | Wayland and X11 launch/resize/focus; no custom Linux chrome |

If the Windows candidate fails any native reliability/promotion gate, the spike passes only by using and documenting native decorations plus a styled in-app header. A fallback is an intended result, not a hidden failure.

## 10. CSP/Fluent/Griffel evidence

- Test a release-mode build with strict CSP active.
- All assets are local; runtime network remains denied.
- Verify theme/background before first visible paint where measurable.
- Exercise Fluent controls, portals/dialog/menu/tooltip styles, and Griffel injection.
- Use the pinned Tauri-compatible nonce/configuration; do not introduce broad `unsafe-inline` without a targeted architecture decision.
- Confirm no duplicate Griffel runtime breaks the configured renderer.
- Retain per-platform screenshots, console/error capture, CSP violation output, and pass/fail result.

## 11. Native WebdriverIO evidence

- Use WebdriverIO with `@wdio/tauri-service`.
- Drive a real Tauri app, not only browser-mocked IPC.
- Minimum journey: window visible → backend ready → Unicode echo → count progress → cancel/terminal state.
- Add focused crash/restart coverage at a lower integration layer if native E2E would become brittle; do not inflate journey count.
- Retain test report/log and target/platform identity.
- Inspect the production/release artifact and prove embedded WebDriver/testing capability is absent.

## 12. Cross-platform build/package evidence

| Target | Required evidence |
|---|---|
| Windows 11 x64 | Build, PyInstaller `onedir`, Tauri/NSIS candidate artifact, hash/size, launch, native E2E where supported, process cleanup, title-bar/manual result |
| macOS arm64 | GitHub Actions build, unsigned `.app` and DMG-layout feasibility, bundled sidecar launch where runner permits, automated smoke, layout/signing-readiness notes |
| Ubuntu 24.04 x64 | Build, `.deb`, dependency metadata, install/remove in clean runner/container/VM as appropriate, WebKitGTK launch, Wayland/X11 evidence split, process cleanup |

Every result must identify runner/OS version, architecture, webview/runtime, tool versions, command, artifact, actual outcome, and any manual evidence still missing. A build alone does not satisfy a runtime or native UX gate.

## 13. Measurement table

Phase 0A defines targets and methods only. Phase 0B records actual values without fabricating missing measurements.

| Metric | Initial target | Measurement method | Evidence artifact | Pass/fail interpretation | Platforms |
|---|---:|---|---|---|---|
| Themed window visible time | ≤700 ms on representative hardware | Monotonic timestamp from process start to first themed visible marker; ≥5 cold runs | CSV/JSON timing series plus environment | Median meets target; deviations recorded, not hidden | All runtime targets |
| Shell interactive time | ≤2 s | Start to enabled UI readiness marker; ≥5 cold runs | Timing series | Median ≤2 s | All |
| Cold sidecar handshake | ≤3 s | Rust launch timestamp to validated `hello`; ≥5 cold starts | Structured timing log | Median ≤3 s; no timeout | All |
| Echo round-trip | Record distribution; planning target ≤100 ms after ready | Rust send to validated result, ≥20 local runs | Timing CSV/JSON | Median ≤100 ms and no validation failure; otherwise targeted review | All |
| Progress event rate seen by UI | ≤10/s/task | Count UI-consumed progress events over stable interval | Task trace | Never exceeds cap; terminal retained | All |
| Cancellation acknowledgement | ≤250 ms | User/test cancel timestamp to Rust acknowledgement | Task trace | ≤250 ms under normal load | All |
| Cooperative stop time | ≤2 s | Cancel request to terminal `Cancelled`/confirmed stop | Task trace | ≤2 s for cooperative count operation | All |
| Crash detection | Planning target ≤1 s after process exit | Sidecar exit timestamp to `BACKEND_CRASHED` state | Lifecycle trace | Detected within target; pending work fails | All |
| Bounded restart result | Exactly one automatic attempt; ready within handshake deadline | Crash then observe lifecycle/restart counter | Lifecycle trace | One valid restart; no replay; excess crash opens circuit | All |
| Orphans after normal close | 0 | Capture process tree before/after bounded close | Process-list artifact | Zero sidecar/descendants | All |
| Orphans after forced close | 0 | Kill/force-close host under controlled test; inspect tree | Process-list artifact | Zero after containment deadline | All |
| Package size | Record, no universal pass ceiling in spike | Byte size of app/package and sidecar directory | Hash/size manifest | Recorded per target; unexplained regression blocks acceptance | All package targets |
| App idle memory | Record baseline after 60 s idle | Platform-native RSS/private-memory sampling, 3 runs | Memory CSV/JSON | Recorded with method/environment; extreme anomaly requires review | All |
| Sidecar idle memory | Record baseline after ready/60 s | Sample sidecar process memory | Memory CSV/JSON | Recorded; unexplained platform anomaly requires review | All |
| Sidecar active memory | Bounded/no sustained growth across repeated count runs | Sample during fixed workload and after recovery | Memory/time series | Returns near baseline; no monotonic leak pattern | All |
| Release-mode CSP rendering | No CSP error that breaks Fluent/Griffel; UI usable | Launch release build, inspect violations and exercise portals | Screenshot + console/CSP log | Required surfaces render; no broad unsafe bypass | All engines |
| WebdriverIO smoke | Required pass on supported CI runtime | Execute native Tauri journey | JUnit/report + log | Journey passes; production artifact excludes test driver | All where service supports runner |
| Per-platform build/package | All declared build/package jobs succeed | Exact CI commands and artifact inspection | CI logs + artifact hashes | Build/package gate passes separately per target | All |
| Title-bar fallback/deviation | No lost native behavior | Automated/manual platform checklist | Versioned checklist/screenshots | Candidate passes promotion gates or native fallback is active/documented | All; Windows focus |

Hardware-sensitive timing misses do not become silent passes. The report must retain raw values and environment and classify whether a miss invalidates architecture, requires optimization, or only adjusts a provisional budget.

## 14. Pass/fail gates

| Gate | Pass condition | Fail/stop condition |
|---|---|---|
| Minimal host | Themed Fluent Tauri window launches on declared runtime targets | Stack cannot launch on a declared target without scope expansion |
| Packaged Python | `onedir` launches without external Python on each target | External Python required or target bundle cannot be produced/launched |
| Contract | Shared valid/invalid fixtures agree across boundaries | Schema/model disagreement or unknown operation reaches Python |
| Bounded protocol | Limits, malformed data, queue pressure, and oversized rejection are deterministic | Unbounded growth, protocol/log mixing, or nondeterministic corruption |
| Task/cancel | Ordered bounded progress and deterministic terminal races | UI unresponsive, terminal loss, or false rollback/success claim |
| Crash/restart | In-flight work fails, one restart succeeds, no replay | Silent replay, repeated loop, or uncertain work reported successful |
| Hang/shutdown | Escalation is bounded and zero descendants remain | Sidecar/descendant survives host containment deadline |
| CSP/Fluent | Release CSP preserves required rendering without broad unsafe bypass | Fluent/Griffel requires unacceptable CSP relaxation |
| Native E2E | One real Tauri smoke path passes; test driver absent from production | Only mocked browser evidence or production contains test capability |
| Packaging | Declared artifacts build with retained hashes/metadata | A declared target/package has no artifact or truthful blocker |
| Title bar | Native behavior passes or approved native fallback is active | Custom chrome ships despite failed native behavior |

Any failing assumption that changes a frozen boundary triggers a targeted `v0.2.1` proposal, not a broad redesign.

## 15. Required Phase 0B outputs

- Complete authoritative repository snapshot archive and SHA-256 hash
- Markdown handoff manifest with branch/commit, commands, inventory, evidence, limitations
- Per-platform build/package artifacts where feasible and hashes
- CI configuration and retained job results
- Shared schema/fixture conformance results
- Native WebdriverIO smoke report
- Lifecycle/cancellation/crash/hang/process-tree evidence
- Measurement dataset and concise spike report with pass/fail matrix
- Title-bar candidate/fallback checklist
- CSP/Fluent/Griffel release-mode evidence
- Signing/notarization feasibility notes
- Only targeted architecture amendment(s) supported by failed evidence

## 16. Stop conditions

Stop and report when:

- A declared target cannot build or run without adding an unapproved platform/stack.
- Packaged Python requires external Python.
- Bounded transport or no-replay semantics cannot be achieved with the selected model.
- Process-tree containment leaves descendants after bounded shutdown.
- Release CSP requires a material security-boundary change.
- Windows custom chrome cannot preserve native behavior and fallback is not available.
- Required credentials/hardware prevent a claim; mark that evidence blocked rather than inventing it.
- Fixing a result would require starting product work, multiple workers, a module SDK, or another later phase.
- Evidence requires changing a frozen architecture boundary; return a targeted amendment proposal for review.

## 17. No-go items

- No real product functionality or database
- No runtime plugins or public extension SDK
- No `packages/ui` or generator
- No multiple workers, durable tasks, or automatic replay
- No GPU/model/scientific runtime
- No runtime remote services, telemetry, or crash upload
- No production updater
- No production release claim without signing/notarization
- No additional Linux package, distribution, OS, or architecture
- No enterprise release/governance framework
- No broad architecture review
