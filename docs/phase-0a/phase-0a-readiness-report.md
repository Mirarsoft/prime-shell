# Phase 0A Readiness Report

**Project:** `prime-shell`  
**Repository:** `prime-shell`

## 1. Executive result

**READY WITH ASSUMPTIONS**

No remaining known unknown currently invalidates Tauri 2 + Rust + a packaged Python `onedir` sidecar for the lean Phase 0B spike. The assumptions concern runner/tool availability and target-specific manual/runtime evidence; they are explicit, testable during Phase 0B, and do not justify delaying the spike.

Phase 0A documentation is ready for Chat Session review. Phase 0B application implementation has not started.

## 2. Files read

Mandatory execution controls:

- `AGENTS.md`
- `Core-Functionality-DeliveryRules.md`

Authority files, read in required order:

1. `generic-fluent-desktop-project-handoff.md`
2. `generic-fluent-desktop-app-architecture-v0.2.md`
3. `architecture-review-consolidation-decision-log.md`

Task prompt:

- `phase-0a-work-session-prompt.md`

The architecture and consolidation-log SHA-256 values match the optional identities in the handoff.

## 3. Frozen decisions respected

- React + TypeScript + Vite + Fluent UI React v9 in Tauri 2
- Rust as native policy/process authority
- Packaged Python sidecar; no external Python
- PyInstaller `onedir` first
- JSON Schema 2020-12 + fixtures
- WebdriverIO with `@wdio/tauri-service`
- Platform-adaptive title bars with native fallback
- Rust operation registry; feature metadata is not authorization
- Python trusted, not sandboxed
- Single writer and single app instance defaults
- In-window settings
- Offline/network-denied default; no telemetry/crash upload
- WCAG 2.2 AA plus native checks
- Static first-party features; no runtime plugins
- One active long task initially
- No replay after uncertain backend failure
- Reference feature before SDK; template extraction last

No alternative stack, broad architecture review, or speculative platform expansion was introduced.

## 4. Defaults adopted

- Synthetic local UTF-8 plaintext analysis as the representative workload
- Non-regulated/non-secret spike inputs, treated as untrusted display content
- CPU-only, standard-library-first Python profile
- Exact Windows 11 x64, macOS arm64, and Ubuntu 24.04 x64 matrix
- GitHub Actions for automated target evidence and role-based manual owners
- Network denied, all UI assets local, no telemetry/crash upload/updater
- Production signing credentials not required for Phase 0A
- Public macOS release blocked until signing/notarization succeeds once

## 5. Decisions made

1. The representative workload is sufficient to exercise every architecture risk relevant to Phase 0B without implementing a real product.
2. The Linux spike package is exactly one Ubuntu 24.04 `.deb`; it favors explicit dependencies and clean install/remove evidence over broad portability.
3. Windows uses an NSIS candidate; macOS uses unsigned `.app` and DMG-layout feasibility in CI; Python uses per-target `onedir`.
4. Logs/frontends exclude document contents, raw inputs, credentials, environment dumps, signing material, secrets, and full native paths by default.
5. Initial ADRs are limited to ADR-0001 through ADR-0005. No blocker justifies ADRs 6–10 now.
6. Phase 0B must retain raw measurements and platform context; hardware-sensitive budget misses are analyzed, not fabricated or hidden.
7. A failed Windows custom-title-bar candidate selects native fallback unless evidence proves the adapter strategy itself invalid.

## 6. Remaining unknowns

| Unknown | Why it does not block entry | Required resolution |
|---|---|---|
| Exact GitHub Actions runner images/labels and installed tool versions at execution time | Operational and measurable; no architecture change assumed | Pin/record runners and tools at Phase 0B start |
| macOS runtime coverage on current and previous major versions without user-owned hardware | CI can establish build/automation; final native/signed evidence already has an external owner | Retain CI evidence; schedule external Mac testing before public release |
| Windows custom title-bar native behavior | Explicit spike candidate with safe native fallback | Run promotion checklist on Windows 11 |
| Wayland/X11 compositor behavior | Ubuntu target is fixed and native decorations reduce risk | Run automated/manual smoke in both sessions where available |
| Exact startup/memory/package values | Measurement is the spike’s purpose | Record raw per-platform evidence |
| Future GPU/native/scientific dependency impact | Not part of representative workload or first spike | Revisit when a real product introduces it |
| Production signing providers/credentials | Production release is not authorized | Resolve before release hardening/public distribution |

## 7. Blockers

**None for beginning the authorized lean Phase 0B spike after Chat Session approval.**

If Phase 0B cannot obtain runtime/manual evidence on a declared target, the affected gate must remain `Blocked`; a successful build alone must not be presented as runtime or native UX proof.

## 8. Risks carried into Phase 0B

- PyInstaller `onedir` resource layout and clean launch can vary per OS.
- Windows custom chrome may fail native promotion gates and require fallback.
- Process-tree containment differs between Windows Job Objects and Unix process groups/session handling.
- Strict release CSP may expose Griffel/style integration issues.
- Native WebdriverIO service support can vary with runner/window environment.
- macOS unsigned CI evidence cannot prove signing, notarization, Gatekeeper, or final UX.
- Linux WebKitGTK and compositor behavior may differ between Wayland and X11.
- Hardware-sensitive targets may need refinement after raw measurements.

## 9. Phase 0B entry checklist

- [ ] Chat Session explicitly authorizes Phase 0B with a controlled execution prompt.
- [ ] Use a fresh/confirmed repository and record branch/commit/state.
- [ ] Re-read `AGENTS.md`, delivery rules, and authority files.
- [ ] Confirm exact target matrix and `.deb`-only Linux package scope.
- [ ] Pin and record Node/pnpm, Rust, Python, PyInstaller, Tauri, Fluent, WebdriverIO, and runner versions.
- [ ] Establish JSON Schema/fixture authority before operation implementation.
- [ ] Enable/test release-mode CSP at the start, not at release.
- [ ] Implement only the five test operations.
- [ ] Define frame, queue, timeout, cancellation, restart, and process-tree limits explicitly.
- [ ] Keep stdout protocol-only and stderr structured/bounded.
- [ ] Verify no external Python is required.
- [ ] Run real Tauri WebdriverIO smoke and exclude test capability from production artifacts.
- [ ] Retain raw measurements, commands, environment, artifacts, and hashes per target.
- [ ] Mark missing manual/runtime/signing evidence truthfully.
- [ ] Package one complete authoritative repository snapshot for review.

## 10. Scope exclusions confirmed

No application source, repository scaffold, CI workflow, package, installer, sidecar, UI implementation, test, production configuration, database, product feature, runtime plugin, module SDK, multiple-worker design, updater, telemetry, enterprise policy, additional package format, or later-phase implementation was created.

Only the nine authorized Phase 0A Markdown deliverables were created.

## 11. Recommended exact next controlled action

Return all Phase 0A artifacts to the Chat Session. The Chat Session should verify the `READY WITH ASSUMPTIONS` determination, `.deb` selection, measurement gates, and remaining unknowns, then issue either a focused correction prompt or an authorized Phase 0B execution prompt. Do not begin Phase 0B before that prompt is supplied.
