# Generic Fluent Desktop Remaining Roadmap Index

**Document role:** Authoritative planning index for work remaining after accepted WP01
**Prompt-pack stage:** Stage 1 foundation
**Prompt status:** Future package prompts are provisional until explicitly activated
**Repository:** `Mirarsoft/prime-shell`
**Planning base:** `main` at `35d53bffec6e5b05aefdf8c7fed39a9bfdf288a2`

## 1. Current accepted position

- Phase 0A is accepted as `READY WITH ASSUMPTIONS`.
- Phase 0B WP01 is `Implemented` and merged. It is an accepted predecessor, not future work.
- Phase 0B WP02 and WP03 are `Not started`.
- No application implementation later than WP01 is authorized by this index.
- This index does not redo Phase 0A, restart WP01, amend frozen architecture, or authorize a package.
- All future package prompts are **provisional until activated** by Chat Session.

Only these implementation-status labels are valid throughout the prompt pack:
`Implemented`, `Partially implemented`, `Stub`, `Mock-only`, `Not started`, and
`Blocked`.

## 2. Package inventory

| Package ID | Phase | Title | Status | Minimum suitable model |
|---|---|---|---|---|
| `GFD-P0B-WP02` | Phase 0B | Lifecycle and Task Resilience Spike | `Not started` | GPT-5.6 Sol / Extra High |
| `GFD-P0B-WP03` | Phase 0B | Cross-Platform Evidence and Spike Closure | `Not started` | GPT-5.6 Sol / Extra High |
| `GFD-P1-WP01` | Phase 1 | Post-Spike Architecture and Repository Baseline | `Not started` | GPT-5.6 Sol / High |
| `GFD-P2-WP01` | Phase 2 | Theme, Tokens, and Accessibility Foundation | `Not started` | GPT-5.6 Sol / High |
| `GFD-P2-WP02` | Phase 2 | Responsive Application Shell | `Not started` | GPT-5.6 Sol / High |
| `GFD-P3-WP01` | Phase 3 | Productized Contracts and Native Intent Boundary | `Not started` | GPT-5.6 Sol / Extra High |
| `GFD-P3-WP02` | Phase 3 | Productized Task Runtime and Sidecar Operations | `Not started` | GPT-5.6 Sol / Extra High |
| `GFD-P4-WP01` | Phase 4 | Local Document-Analysis Reference Feature | `Not started` | GPT-5.6 Sol / High |
| `GFD-P5-WP01` | Phase 5 | Second Consumer and Proven Feature Contracts | `Not started` | GPT-5.6 Sol / High |
| `GFD-P6-WP01` | Phase 6 | Settings, Persistence, and Single-Instance Behavior | `Not started` | GPT-5.6 Sol / High |
| `GFD-P6-WP02` | Phase 6 | Diagnostics, Repair, and Recovery | `Not started` | GPT-5.6 Sol / Extra High |
| `GFD-P7-WP01` | Phase 7 | Production Security and Release Artifact Hardening | `Not started` | GPT-5.6 Sol / Extra High |
| `GFD-P7-WP02` | Phase 7 | Signing, Notarization, and Signed Update Channels | `Not started` | GPT-5.6 Sol / Extra High |
| `GFD-P8-WP01` | Phase 8 | Template Extraction and Second Branded Application | `Not started` | GPT-5.6 Sol / High |

The package IDs and order above must match
[`dependency-matrix.md`](dependency-matrix.md). Package prompts do not exist
yet and may be authored only through the controlled Stage 2 workflow.

## 3. Phase 0B completion packages

### GFD-P0B-WP02 — Lifecycle and Task Resilience Spike

- **Objective:** Complete the high-risk lifecycle slice left after WP01 by
  proving bounded progress, cancellation, timeout, deliberate crash and hang
  handling, one bounded restart, circuit behavior, no replay, and
  process-tree containment.
- **Prerequisites:** Accepted WP01 implementation and evidence at the activation
  base; the accepted Phase 0A lifecycle, protocol, and measurement rules.
- **Major in-scope outcome:** A real Tauri/Rust/packaged-Python synthetic task
  path with deterministic terminal-state races, bounded queues and progress,
  safe escalation, and zero surviving descendants after normal and forced
  closure.
- **Explicit exclusions:** Title-bar promotion, native WebdriverIO authority,
  the full cross-platform package matrix, product features, durable task
  resume, multiple Python workers, automatic replay, database, telemetry, and
  updater work.
- **Acceptance summary:** Lifecycle and fault-injection evidence demonstrates
  the accepted state machine and limits without false rollback, replay, orphan,
  protocol/log mixing, or unbounded growth.
- **Expected repository ownership:** `packages/app-contracts/` task schemas and
  fixtures; `apps/desktop/src-tauri/src/backend/`; bounded task-facing frontend
  state/UI; `services/python-backend/` test operations; focused lifecycle and
  process tests.
- **Risk boundary:** Lifecycle, concurrency, cancellation races, process-tree
  containment, protocol security, and recovery require Extra High review.

### GFD-P0B-WP03 — Cross-Platform Evidence and Spike Closure

- **Objective:** Close Phase 0B with truthful platform evidence rather than
  extending product functionality.
- **Prerequisites:** Accepted WP01 and accepted `GFD-P0B-WP02` outputs.
- **Major in-scope outcome:** Title-bar candidate/fallback evidence, one real
  native WebdriverIO journey, Windows/macOS/Ubuntu build and package results,
  declared measurements, release-CSP/cross-engine evidence, and the final
  Phase 0B spike report.
- **Explicit exclusions:** New product behavior, broad shell implementation,
  production signing claims, production updater, extra Linux formats,
  additional architectures, store distribution, and silently passing missing
  native/manual evidence.
- **Acceptance summary:** Every declared target has separately classified
  build, package, runtime, native UX, and manual evidence; unavailable evidence
  remains `Blocked`; Windows custom chrome either passes its promotion gates or
  the accepted native fallback is active.
- **Expected repository ownership:** `tests/e2e/`,
  `tests/platform-smoke/`, narrow packaging verification scripts,
  `.github/workflows/`, and `docs/spike/`; application changes only where a
  demonstrated Phase 0B defect requires a focused correction.
- **Risk boundary:** Cross-platform behavior, packaging, native automation,
  release CSP, accessibility evidence, and measurement interpretation require
  Extra High review.

## 4. Phase 1 — Baseline hardening

### GFD-P1-WP01 — Post-Spike Architecture and Repository Baseline

- **Objective:** Reconcile accepted Phase 0B evidence with the repository and
  establish the smallest maintainable baseline for later implementation.
- **Prerequisites:** Accepted and merged `GFD-P0B-WP03` spike closure.
- **Major in-scope outcome:** Targeted evidence-backed architecture/ADR updates
  where needed; pinned toolchains and lockfiles; contract drift checks; CSP and
  capability baseline; contribution, security, and release rules; and minimal
  CI gates matching the accepted support matrix.
- **Explicit exclusions:** Broad architecture redesign, UI shell expansion,
  product features, production updater/signing completion, runtime plugins,
  module SDK, `packages/ui`, and speculative governance.
- **Acceptance summary:** Repository policy and CI claims match measured spike
  behavior; schema, capability, lint, type, Rust, Python, and declared smoke
  build gates are runnable and pass or are truthfully blocked.
- **Expected repository ownership:** Root toolchain/configuration files,
  `docs/authority/` only for targeted accepted amendments, `docs/adrs/` or the
  established ADR location, `SECURITY.md`, contribution/release guidance,
  contract verification, and minimal `.github/workflows/`.
- **Risk boundary:** Architecture amendments must be targeted and evidence
  backed; no package may silently change a frozen boundary.

## 5. Phase 2 — Design system and application shell

### GFD-P2-WP01 — Theme, Tokens, and Accessibility Foundation

- **Objective:** Establish the real Fluent theme and semantic-token foundation
  before composing the full shell.
- **Prerequisites:** Accepted `GFD-P1-WP01` baseline.
- **Major in-scope outcome:** System/light/dark/forced-colors behavior,
  semantic and accent tokens, material capability fallback, launch background,
  reduced motion/transparency, density, contrast/provenance checks, and focused
  accessible component evidence.
- **Explicit exclusions:** Full multi-pane shell, product feature UI,
  `packages/ui`, broad Storybook coverage, custom Linux chrome, arbitrary raw
  color use, and persistence beyond the minimum needed to demonstrate theme
  correctness.
- **Acceptance summary:** Theme is correct before first visible frame; token
  and contrast matrices pass; portals and Fluent/Griffel render under release
  CSP; forced colors, focus, scaling, and motion/transparency behavior have
  proportional automated and manual evidence.
- **Expected repository ownership:** `packages/design-tokens/` once immediately
  consumed; application provider/bootstrap/theme code; focused token,
  component, accessibility, and visual evidence.
- **Risk boundary:** Accessibility, forced colors, CSP styling, cross-engine
  behavior, and platform material fallback receive High review.

### GFD-P2-WP02 — Responsive Application Shell

- **Objective:** Deliver the composable, keyboard-operable application shell
  around the accepted theme foundation.
- **Prerequisites:** Accepted `GFD-P2-WP01`.
- **Major in-scope outcome:** App rail, context sidebar, workspace, optional
  inspector, bottom panel, status/notification layer, responsive states,
  keyboard-resizable panes, focus restoration, selected title-bar adapters,
  in-window settings shell, and bounded layout-preference persistence.
- **Explicit exclusions:** Product/domain behavior, public UI package,
  feature/module SDK, comprehensive settings migrations, diagnostics/recovery,
  production release work, and title-bar claims beyond accepted platform
  evidence/fallbacks.
- **Acceptance summary:** The shell runs as a real desktop path, degrades to the
  500 px compact state, preserves native title-bar behavior, and passes
  keyboard, focus, screen-reader, 200% scaling, RTL/text-expansion,
  reduced-motion, and forced-colors checks appropriate to the package.
- **Expected repository ownership:** `apps/desktop/src/app/`,
  `apps/desktop/src/shell/`, routes and UI state, narrow Rust platform/settings
  adapters, and shell-focused component/native tests.
- **Risk boundary:** Shell accessibility and platform title-bar behavior must
  use accepted native fallbacks rather than visual uniformity at any cost.

## 6. Phase 3 — Backend and task infrastructure

### GFD-P3-WP01 — Productized Contracts and Native Intent Boundary

- **Objective:** Convert the spike boundary into product-ready, least-privilege
  contracts without creating a generic native tunnel.
- **Prerequisites:** Accepted `GFD-P2-WP02` and the Phase 1 baseline.
- **Major in-scope outcome:** Productized schemas and drift checks; explicit
  Rust operation registry; per-operation risk, payload, timeout,
  cancellability, and idempotency metadata; safe backend status/errors; and
  intent-centric file/artifact references with single-writer ownership.
- **Explicit exclusions:** Generic shell/process/filesystem access, direct
  native paths in React, product database, runtime plugins, public module SDK,
  multiple workers, durable tasks, and complete task lifecycle expansion.
- **Acceptance summary:** Unknown or unauthorized operations stop in Rust;
  inputs and outputs are bounded and schema validated; user file intent yields
  opaque references; ownership and safe-rendering rules are demonstrated on
  real boundaries.
- **Expected repository ownership:** `packages/app-contracts/`,
  Rust commands/registry/security/file-intent modules, typed frontend boundary
  adapters, and cross-language fixtures/tests.
- **Risk boundary:** Security, trust, file/path handling, schema evolution, and
  durable-data ownership require Extra High review.

### GFD-P3-WP02 — Productized Task Runtime and Sidecar Operations

- **Objective:** Productize the proven task/lifecycle infrastructure for later
  real feature consumers.
- **Prerequisites:** Accepted `GFD-P3-WP01` and retained Phase 0B lifecycle
  evidence.
- **Major in-scope outcome:** Sidecar lifecycle state machine, bounded framing
  and queues, ordered progress channels, cancel/timeout/crash/circuit behavior,
  sanitized trace-correlated logs, process-tree containment, and the
  per-platform sidecar/package build paths needed by current consumers.
- **Explicit exclusions:** Automatic replay, durable resume, multiple Python
  workers, GPU/model orchestration, remote diagnostics, product domain work,
  production signing/updater, and generic background-service frameworks.
- **Acceptance summary:** Fault-injection and target tests show no replay,
  orphan, protocol/log mixing, terminal loss, or unbounded buffer; recovery
  behavior is explicit and safe.
- **Expected repository ownership:** Rust backend/lifecycle/task modules,
  Python protocol/runtime, task-facing frontend state, packaging/build scripts,
  shared fault fixtures, and focused platform lifecycle tests.
- **Risk boundary:** Concurrency, cancellation races, crash recovery, packaging,
  and process containment require Extra High review.

## 7. Phase 4 — Reference feature

### GFD-P4-WP01 — Local Document-Analysis Reference Feature

- **Objective:** Prove the architecture with one realistic, product-neutral,
  end-to-end feature.
- **Prerequisites:** Accepted `GFD-P2-WP02` shell and `GFD-P3-WP02` task
  runtime.
- **Major in-scope outcome:** Import a small text document through native
  intent; show items, content/status, metadata, task output, search, one Python
  analysis, progress, cancellation, result and recovery states, and one small
  settings contribution.
- **Explicit exclusions:** Real customer product logic, regulated data,
  database, network service, runtime plugins, public SDK/package extraction,
  multiple workers, durable tasks, and release engineering.
- **Acceptance summary:** The real user path covers empty, loading, success,
  error, cancelled, and backend-unavailable states using only approved
  UI/native/backend boundaries; accessibility and performance evidence is
  retained.
- **Expected repository ownership:** One internal feature under
  `apps/desktop/src/features/`, its scoped Rust/Python operation and contract,
  shell contributions, focused tests, and feature evidence.
- **Risk boundary:** The feature must expose architecture friction without
  hiding it behind mocks or prematurely extracting a framework.

## 8. Phase 5 — Proven feature and command contracts

### GFD-P5-WP01 — Second Consumer and Proven Feature Contracts

- **Objective:** Prove reusable feature and command boundaries with a second
  small real consumer, then extract only the extension points both consumers
  actually use.
- **Prerequisites:** Accepted `GFD-P4-WP01` and recorded architecture-friction
  evidence.
- **Major in-scope outcome:** A second tiny static first-party feature;
  route, command, ID, and shortcut conflict validation; and the smallest
  proven feature/command/settings contribution contract with Rust
  authorization remaining independent.
- **Explicit exclusions:** Runtime plugins, untrusted modules, false per-feature
  security boundaries, speculative migrations/generators, broad public SDK,
  `packages/ui`, and unrelated shell refactors.
- **Acceptance summary:** Both real features register without shell edits;
  conflicts fail deterministically; no metadata bypasses Rust authorization;
  every extracted contract has two concrete consumers.
- **Expected repository ownership:** Internal feature registration,
  the two feature consumers, conflict validation, and only an evidence-justified
  feature/command contract location.
- **Risk boundary:** The second feature satisfies the consumer gate only for
  feature/command contracts. It does not by itself justify `packages/ui` or a
  broad module SDK; those remain deferred until a second application or other
  real consumer proves them. A second real consumer is required before any
  broader extraction is accepted.

## 9. Phase 6 — Settings, persistence, diagnostics, and recovery

### GFD-P6-WP01 — Settings, Persistence, and Single-Instance Behavior

- **Objective:** Complete the durable local settings and application-instance
  model around proven shell and feature consumers.
- **Prerequisites:** Accepted `GFD-P5-WP01`.
- **Major in-scope outcome:** Searchable settings, schema-versioned atomic
  persistence, migrations, previous-valid-copy recovery, section-scoped reset,
  bounded layout writes, and single-instance launch/focus/argument forwarding.
- **Explicit exclusions:** Product database, cross-device sync, multiple
  workspaces without evidence, separate native settings window, secrets
  plaintext fallback, diagnostics export, remote services, and release updater.
- **Acceptance summary:** Valid settings survive restart; corrupt or old
  sections migrate/recover without destroying unrelated data; concurrent
  writing remains Rust-owned; second launch behavior is deterministic.
- **Expected repository ownership:** Rust settings/single-instance modules,
  in-window settings routes and state, schemas/migrations, and focused recovery
  and integration tests.
- **Risk boundary:** Migration, atomicity, single-writer ownership, and launch
  routing require High review.

### GFD-P6-WP02 — Diagnostics, Repair, and Recovery

- **Objective:** Add local, privacy-preserving diagnostics and bounded user
  recovery paths.
- **Prerequisites:** Accepted `GFD-P6-WP01` and productized lifecycle/logging.
- **Major in-scope outcome:** Sanitized bounded logs, diagnostics preview and
  user-selected export, allowlisted manifest, retention/permissions, recent
  safe errors, backend restart status, reset/repair flows, and recovery from
  corrupt settings or backend-unavailable states.
- **Explicit exclusions:** Telemetry, automatic crash upload, remote support
  service, raw payload/path/environment export, enterprise compliance export,
  automatic uncertain-work replay, and updater recovery.
- **Acceptance summary:** Export contains only allowlisted information; privacy
  and retention tests pass; repair/reset preserves unrelated valid data; user
  recovery never fabricates task success or replays uncertain work.
- **Expected repository ownership:** Rust/Python structured logging boundaries,
  diagnostics and repair commands/UI, export/retention policy, and privacy and
  recovery tests.
- **Risk boundary:** Privacy, data ownership, fault recovery, and no-replay
  guarantees require Extra High review.

## 10. Phase 7 — Release hardening

### GFD-P7-WP01 — Production Security and Release Artifact Hardening

- **Objective:** Harden installable artifacts and production security controls
  before signing and updates are enabled.
- **Prerequisites:** Accepted `GFD-P6-WP02`.
- **Major in-scope outcome:** Production capabilities/CSP, dependency and
  license inventory, SBOM, release/test-driver separation, clean-machine
  installed-artifact tests, performance/security review, package metadata, and
  backup/migration/repair readiness for the declared target matrix.
- **Explicit exclusions:** Publishing, store distribution, additional Linux
  formats, enterprise deployment, automatic telemetry/crash upload, signing
  claims without credentials, and user-visible updater enablement.
- **Acceptance summary:** Declared artifacts install and run on clean targets;
  test capability is absent; capabilities and CSP are least privilege; supply
  chain and performance evidence is retained; critical/high findings are
  resolved or explicitly block progression.
- **Expected repository ownership:** Production Tauri capabilities/config,
  release/package workflows and scripts, installed-artifact tests,
  SBOM/license outputs or generation rules, and release/security documentation.
- **Risk boundary:** Security, supply chain, installed artifacts, packaging, and
  cross-platform evidence require Extra High review.

### GFD-P7-WP02 — Signing, Notarization, and Signed Update Channels

- **Objective:** Complete authenticated public-release mechanics for the
  declared platforms and prove the signed update path.
- **Prerequisites:** Accepted `GFD-P7-WP01`; available authorized signing
  identities/keys and target test ownership at activation.
- **Major in-scope outcome:** Windows signing, macOS signing/notarization/
  stapling and Gatekeeper evidence, platform integrity checks, stable/beta
  channel separation, updater signing and end-to-end verification, key custody/
  backup/rotation documentation, previous-installer retention, and update/
  downgrade recovery behavior.
- **Explicit exclusions:** Store submission, silent enterprise rollout,
  percentage rollout platform, extra packages/architectures, unapproved
  credential handling, telemetry, and publication without explicit user
  authorization.
- **Acceptance summary:** App and update signatures verify independently;
  notarized macOS clean installation passes; channels cannot cross; update and
  repair/rollback rules are proven; unavailable credentials or hardware remain
  `Blocked`.
- **Expected repository ownership:** Protected release workflows/configuration,
  signing/notarization/update verification scripts, release-channel metadata,
  and operational release/recovery documentation. Secrets never enter source.
- **Risk boundary:** Credentials, signing, notarization, updates, platform
  packaging, and recovery require Extra High review and explicit release
  authorization.

## 11. Phase 8 — Template extraction

### GFD-P8-WP01 — Template Extraction and Second Branded Application

- **Objective:** Extract only the reusable structure proven by the completed
  application and demonstrate it through a second branded application.
- **Prerequisites:** Accepted `GFD-P7-WP02`, two proven static feature
  consumers, and activation-time evidence identifying a real second
  application consumer.
- **Major in-scope outcome:** Remove reference-product naming, introduce only
  justified shared UI/feature packages, provide template configuration and a
  generator or documented copy workflow, create a second branded sample, and
  complete branding/conformance/build/release guidance.
- **Explicit exclusions:** Universal UI framework, runtime plugin marketplace,
  unsupported product options, speculative extension points, enterprise
  deployment, and any package with only one concrete consumer.
- **Acceptance summary:** A second application is created without modifying
  core shell internals; branding, static features, and Python domain logic can
  be replaced independently; build/release guidance works from a fresh copy.
- **Expected repository ownership:** Evidence-justified shared packages,
  template/configuration or narrow generator, second sample application, and
  branding/conformance documentation and tests.
- **Risk boundary:** `packages/ui`, public module/feature SDK surfaces, or
  broader extraction are permitted only when the second application or another
  real consumer proves each surface. Otherwise they remain `Not started`.

## 12. Ordering, activation, and deferred decisions

The authoritative dependency order is Phase 0B completion, Phase 1 baseline,
Phase 2 design/shell, Phase 3 backend/task infrastructure, Phase 4 reference
feature, Phase 5 proven contracts, Phase 6 settings/diagnostics/recovery,
Phase 7 release hardening, then Phase 8 template extraction.

Every future package prompt is provisional until Chat Session completes
[`activation-procedure.md`](activation-procedure.md) against the latest accepted
and merged `main`. Activation may refresh or narrow one package; it may not
silently broaden it or authorize a successor.

Activation-time facts include current repository paths and SHA, predecessor
deviations, pinned tools, CI/runner capabilities, platform/manual-test
availability, signing credentials, and targeted accepted architecture
amendments. Uncertainty is retained as a blocker or deferred decision rather
than resolved through speculative design.

The following remain outside this roadmap without a new accepted hard
requirement: runtime third-party plugins, untrusted code execution, multiple
Python workers, durable task resume, GPU/model orchestration, remote telemetry
or crash upload, broad Linux/store distribution, and enterprise deployment.
