# Architecture Review Consolidation Decision Log

**Architecture target:** `generic-fluent-desktop-app-architecture-v0.2.md`  
**Inputs:** v0.1.1, Cursor Opus review, Work Session review  
**Date:** 2026-07-27

## Overall result

Both reviewers approved the strategic stack with changes. Neither independent review found a reason to abandon React, Fluent UI, Tauri 2, Rust, or the packaged Python-sidecar model.

The consolidation preserves the stack and changes implementation authority in five areas:

1. Treat Python as trusted native code, not a sandbox.
2. Make Rust the enforceable operation-policy boundary.
3. Validate risky native behavior through an early cross-platform spike.
4. Use bounded, typed task/protocol semantics rather than a loosely described JSON-RPC tunnel.
5. Extract a module SDK only after a reference feature proves the extension points.

## Consensus findings accepted

| Topic | Cursor | Work Session | Decision |
|---|---|---|---|
| Native Tauri E2E | Replace packaged-app Playwright | Use WebdriverIO Tauri service | Accepted |
| Python trust model | Sidecar is high-privilege | Sidecar is trusted, not sandboxed | Accepted |
| Rust operation enforcement | Compile-time method registry | Typed operation enum/registry | Accepted |
| Module security | Manifest is not isolation | Features in one webview are not principals | Accepted |
| Protocol authority | Canonical generated schema | JSON Schema + fixtures | Accepted with staged generation |
| Bounded framing | Add size/queue/backpressure limits | Same | Accepted |
| Cancellation/crash | Define race/escalation/no replay | Same | Accepted |
| Process tree | Job/process-group containment | Same | Accepted |
| Title bar | Windows Snap/native risks | Spike with native fallback | Accepted |
| Platform matrix | Missing | Missing | Accepted |
| Packaging | Prefer `onedir` and spike | `onedir` first and measure | Accepted |
| State ownership | Single-writer/data ownership | Rust settings + Python domain owner | Accepted |
| Network/CSP | Define early | Network denied by default | Accepted |
| Diagnostics/privacy | Allowlist and consent | Same | Accepted |
| Accessibility | More desktop-specific | WCAG 2.2 AA + native tests | Accepted |
| Roadmap | Risk is too late | Split Phase 0 and spike first | Accepted |
| Module SDK | Premature | Reference feature first | Accepted |

## Accepted with simplification

### Python cancellation

Cursor proposed a worker pool and killable child process tier immediately. Work Session recommended one long-running sidecar task initially. v0.2 adopts:

- Independent control-message reading
- Cooperative cancellation
- One active long task initially
- Bounded deadline
- Sidecar termination/restart as escalation
- Separate worker processes only when real workload evidence requires them

### Contract generation

Cursor proposed generating TypeScript, Rust, and Python models immediately. Work Session allowed idiomatic handwritten Rust/Python models if all languages consume the same fixtures.

v0.2 uses JSON Schema and fixtures as authority, requires generated TypeScript/Zod where practical, and allows handwritten Rust/Python boundary types during the spike with CI fixture conformance.

### Release timing

Both reviewers correctly moved feasibility earlier. v0.2 distinguishes:

- Phase 0: accounts, package choices, test-signed pipeline feasibility
- Spike: package structure and signing layout evidence
- Pre-release: real signing/notarization and signed-update completion

Production credentials do not block writing the architecture, but production releases cannot bypass them.

## Claims not accepted as written

### “PyInstaller onefile is fundamentally incompatible with notarization”

Not accepted as a universal statement. Official PyInstaller guidance supports signing collected binaries in both `onefile` and `onedir`, while noting important limitations and recommending against some one-file app-bundle scenarios. The architecture selects `onedir` first because it is easier to inspect, sign, test, and diagnose—not because every one-file scenario is impossible.

### “CRLF mangles NDJSON”

Not accepted. CRLF is a valid and commonly handled line delimiter. The actual requirements are UTF-8, explicit framing, bounded reads, continuous pipe draining, and protection against stray stdout output.

### “All blockers must close before Phase 0 exits”

Not accepted. Findings are assigned to:

- Architecture decisions
- Phase 0B spike evidence
- Pre-implementation gates
- Pre-release gates

This prevents Phase 0 from becoming an entire enterprise release program.

### Immediate enterprise release features

Staged percentage rollouts, remote crash reporting, data-export/delete surfaces, silent enterprise deployment, and multiple installer formats are not generic version-1 requirements. They remain product decisions.

## v0.2 authority rule

The new architecture is implementation authority only after the final focused Work Session review confirms:

- No internal contradiction
- No unresolved blocker to Phase 0A/0B
- Security boundary is coherent
- Roadmap gates match the architecture
- No recommendation was incorporated without a concrete need

The final review should not redesign the document or add speculative features. It should return only blockers, major contradictions, and readiness for Phase 0A.
