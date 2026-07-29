# Generic Fluent Desktop Cross-Package Dependency Matrix

**Document role:** Dependency, sequencing, ownership, and stale-fact authority
for the package inventory in [`roadmap-index.md`](roadmap-index.md)
**Prompt status:** Every future package prompt remains provisional until
explicit Chat Session activation

## 1. Matrix

| Package ID | Phase | Direct prerequisites | Required predecessor outputs/evidence | Major repository ownership | Inherited architecture/security/lifecycle/platform gates | Direct dependents | Parallelism and sequencing | Conflict risks | Activation facts most likely to become stale | Minimum suitable model |
|---|---|---|---|---|---|---|---|---|---|---|
| `GFD-P0B-WP02` | Phase 0B | Accepted WP01 at the activation base | WP01 merged source; Unicode/native release-CSP/backend/sidecar-cleanup evidence; accepted contract and toolchain state | Task schemas/fixtures; Rust backend lifecycle; Python test operations; bounded task UI/state; lifecycle/process tests | Rust authority; bounded protocol/queues/logs; cancellation terminal precedence; timeout/crash/hang; one restart/circuit; no replay; zero descendants | `GFD-P0B-WP03` | First remaining package; no parallel implementation touching backend/task/runtime areas | Overlap with WP03 evidence scripts; accidental productization; multiple-worker or durable-task expansion | Current `main`; WP01 paths/contracts; tool versions; CI capability; process-containment facilities | GPT-5.6 Sol / Extra High |
| `GFD-P0B-WP03` | Phase 0B | `GFD-P0B-WP02` | Accepted WP02 lifecycle/fault/process evidence plus WP01 native baseline | Native E2E and platform-smoke tests; packaging verification; target workflows; spike evidence/report; focused demonstrated fixes only | Title-bar promotion/fallback; native WebdriverIO authority; Windows/macOS/Ubuntu matrix; `.deb` Linux scope; release CSP/cross-engine accessibility; measurements; truthful blocked manual evidence | `GFD-P1-WP01` | Executes only after WP02 acceptance; platform jobs may run in parallel within this package, but results converge before closure | Broad application changes; confusing build with runtime/native UX; adding formats/architectures; production signing claims | Runner labels/images; WebdriverIO/Tauri service; target hardware/manual owners; package tooling; accepted WP02 head | GPT-5.6 Sol / Extra High |
| `GFD-P1-WP01` | Phase 1 | `GFD-P0B-WP03` | Final accepted spike report, measurements, platform result matrix, targeted deviations, merged Phase 0B head | Root toolchains/config; targeted authority/ADRs; contract drift checks; CSP/capability baseline; contribution/security/release guidance; minimal CI | Amend architecture only from accepted evidence; pin/lock dependencies; least privilege; support matrix and smoke-build truth | `GFD-P2-WP01` | Begins after Phase 0B closure; no implementation package runs against an unreconciled baseline | Broad redesign; duplicating spike work; overbuilt governance/CI; premature UI/backend/release implementation | Final spike deviations; current tools and runner capabilities; repository layout; accepted amendments | GPT-5.6 Sol / High |
| `GFD-P2-WP01` | Phase 2 | `GFD-P1-WP01` | Accepted baseline/toolchains/CSP/capabilities and platform rendering evidence | `packages/design-tokens/` when consumed; theme providers/bootstrap; semantic/accent/material/forced-color assets; focused accessibility/visual tests | Fluent v9; semantic tokens; release CSP/Griffel; theme before paint; forced colors; contrast; reduced motion/transparency; cross-engine equivalence | `GFD-P2-WP02` | Must precede shell composition; no parallel shell implementation against unstable tokens | Premature `packages/ui`; raw colors; broad Storybook; shell or persistence scope leakage | Pinned Fluent/Tauri versions; CSP nonce behavior; WebView engines; token provenance; platform material support | GPT-5.6 Sol / High |
| `GFD-P2-WP02` | Phase 2 | `GFD-P2-WP01` | Accepted theme/token contract, accessibility evidence, title-bar fallback facts | Application shell, routes, UI/layout state, selected title-bar/platform adapters, narrow layout/settings bridge, shell tests | Responsive 500 px minimum; native title-bar reliability/fallback; keyboard splitters/focus; screen reader; scaling/RTL; forced colors/reduced motion; bounded layout persistence | `GFD-P3-WP01` | Must follow theme foundation; backend productization remains sequenced after the coherent shell | Future comprehensive settings ownership; title-bar re-litigation; feature UI; public UI package extraction | Actual shell paths; platform title-bar results; webview behavior; accessibility tooling; layout persistence boundary | GPT-5.6 Sol / High |
| `GFD-P3-WP01` | Phase 3 | `GFD-P2-WP02` | Accepted shell integration points plus Phase 1 contract/security baseline | Productized contracts; Rust commands/registry/security/file intent; typed frontend boundary; shared fixtures/tests | React least trusted; Rust operation authority; Python trusted native; no generic tunnel; bounded schemas/payloads; opaque file/artifact references; safe rendering; single writer | `GFD-P3-WP02` | Begins after shell contract is stable; no task-runtime expansion before native intent/operation boundaries | Ownership overlap with shell settings bridge or later persistence; generic filesystem/process API; product database; premature SDK | Current schemas/generated types; repository module paths; Tauri capabilities; file APIs; accepted data classification | GPT-5.6 Sol / Extra High |
| `GFD-P3-WP02` | Phase 3 | `GFD-P3-WP01` | Accepted operation/file boundary and retained WP02 lifecycle evidence | Rust lifecycle/task/runtime; Python protocol/runtime; task frontend state; build/package scripts; fault fixtures/platform tests | Bounded framing/queues/progress; deterministic cancellation/races; timeout/crash/circuit; no replay; process-tree containment; structured redacted logs; exact bundled sidecar | `GFD-P4-WP01` | Executes after contract/intent boundary; platform test jobs may parallelize only inside this package | Duplicating Phase 0B instead of productizing; multiple workers; durable tasks; remote diagnostics; release-signing creep | Runtime operation set; process APIs; packaging layout; tool versions; target CI/hardware; performance limits | GPT-5.6 Sol / Extra High |
| `GFD-P4-WP01` | Phase 4 | `GFD-P2-WP02`, `GFD-P3-WP02` | Accepted real shell and productized native/task boundaries | One internal document-analysis feature; scoped operation/schema; shell contributions; focused E2E/accessibility/performance evidence | Native file intent; untrusted text rendering; real Python analysis; progress/cancel/recovery; empty/loading/error/cancelled/backend-unavailable states; no product claims | `GFD-P5-WP01` | UI and backend prerequisites must both be accepted; feature work is one integrated vertical slice | Hiding architecture friction with mocks; database/network/product scope; extracting contracts before evidence | Shell contribution APIs; operation registry; file-intent contract; task runtime; performance budgets | GPT-5.6 Sol / High |
| `GFD-P5-WP01` | Phase 5 | `GFD-P4-WP01` | Accepted reference feature, friction record, real route/command/settings contributions | Second tiny internal feature; registration/conflict validation; only proven feature/command/settings contract | Static first-party features; metadata is not authorization; conflict detection; two real consumers before extraction; no runtime plugins | `GFD-P6-WP01` | Add/prove second consumer before extracting common contract within this package; no broad SDK/UI extraction in parallel | False module security boundary; `packages/ui`; speculative migrations/generator; shell refactor; one-consumer abstractions | Reference feature shape; shell registry paths; command/route/shortcut rules; Rust authorization; second-consumer viability | GPT-5.6 Sol / High |
| `GFD-P6-WP01` | Phase 6 | `GFD-P5-WP01` | Accepted two-feature contract and existing narrow layout/settings behavior | Rust settings/migrations/single-instance; settings UI/routes; schemas; atomic persistence and recovery tests | Rust single writer; schema versions; atomic replace/previous valid copy; section recovery; no plaintext secret fallback; deterministic second-launch behavior | `GFD-P6-WP02` | Follows feature contract extraction; diagnostics/recovery builds on accepted durable state | Overlap with Phase 2 layout bridge; product database; multiple workspaces/windows; separate settings window; remote sync | Persisted schema and paths; platform atomic-file semantics; single-instance APIs; settings contributions; product data classification | GPT-5.6 Sol / High |
| `GFD-P6-WP02` | Phase 6 | `GFD-P6-WP01` | Accepted settings migrations/recovery and productized structured logging/lifecycle | Diagnostics/repair commands and UI; Rust/Python log boundaries; local export/retention; privacy and recovery tests | Allowlisted local diagnostics; bounded retention/permissions; no payload/secrets/full paths; preview before export; reset/repair preserves unrelated data; no replay | `GFD-P7-WP01` | Executes after durable settings foundation; no release hardening until recovery behavior is accepted | Telemetry/crash upload; raw environment/path export; enterprise support tooling; uncertain-work replay | Log schema/locations; OS permission behavior; retention policy; backend recovery states; user-selected export APIs | GPT-5.6 Sol / Extra High |
| `GFD-P7-WP01` | Phase 7 | `GFD-P6-WP02` | Accepted diagnostics/recovery, target matrix, current package/runtime evidence | Production capabilities/CSP; release/package workflows; installed-artifact tests; SBOM/license rules; security/performance/recovery docs | Least privilege; test-driver exclusion; clean-machine install; supply-chain inventory; target-specific artifacts; critical/high finding gate; no unsupported platform claims | `GFD-P7-WP02` | Must precede signing/updater enablement; target build jobs may parallelize only within one controlled package | Publishing or signing too early; adding formats/stores; embedding test capability; weakening CSP; untracked dependencies | Package formats/dependencies; target OS/webviews; release tools; supply-chain data; security findings; performance measurements | GPT-5.6 Sol / Extra High |
| `GFD-P7-WP02` | Phase 7 | `GFD-P7-WP01` | Accepted hardened clean-install artifacts, SBOM/security result, migration/repair readiness | Protected signing/notarization/update workflows; verification scripts; channel metadata; release/key/recovery documentation | App/update signature separation; macOS notarization/Gatekeeper; protected secret handling; stable/beta isolation; key backup/rotation; update/downgrade/repair proof; explicit publish authorization | `GFD-P8-WP01` | Requires credentials and target evidence at activation; signing and update verification sequence before any publication | Credential leakage; unsupported hardware; channel crossing; store/enterprise rollout; publication without authorization | Certificates/identities; CI secret permissions; Apple/Windows tooling; updater versions/endpoints; manual target access | GPT-5.6 Sol / Extra High |
| `GFD-P8-WP01` | Phase 8 | `GFD-P7-WP02` | Accepted releasable application; two feature consumers; real second-application consumer and extraction evidence at activation | Evidence-justified shared packages; template/config or narrow generator; second branded app; branding/conformance/build/release guidance | Template extraction last; every public/shared surface has real consumers; shell internals remain replaceable; static features; no runtime plugins/universal UI framework | None | Final package; extraction and second-app proof are one controlled vertical slice, with no successor assumed | Premature `packages/ui` or public SDK; speculative options; copying release secrets; reference-product leakage; one-consumer abstraction | Second application requirements; package ownership; branding/config boundary; release tooling; proven duplicated surfaces | GPT-5.6 Sol / High |

## 2. Authoritative sequencing

The direct dependency graph is acyclic:

```text
accepted WP01
  → GFD-P0B-WP02
  → GFD-P0B-WP03
  → GFD-P1-WP01
  → GFD-P2-WP01
  → GFD-P2-WP02
  → GFD-P3-WP01
  → GFD-P3-WP02
  → GFD-P4-WP01
  → GFD-P5-WP01
  → GFD-P6-WP01
  → GFD-P6-WP02
  → GFD-P7-WP01
  → GFD-P7-WP02
  → GFD-P8-WP01
```

`GFD-P4-WP01` also requires the accepted Phase 2 shell and Phase 3 runtime, as
shown in the matrix. The linear activation order deliberately preserves the
approved architecture roadmap and prevents concurrent ownership of immature
boundaries. Parallel target jobs or independent tests inside one activated
package do not authorize parallel package implementation.

## 3. Cross-package consistency rules

Before accepting a roadmap, provisional prompt, activation, or final prompt
pack, verify:

1. **Inventory parity:** The matrix and roadmap have identical package row IDs,
   phases, order, titles, statuses, and model recommendations.
2. **No missing targets:** Every prerequisite and dependent names an inventory
   package or the accepted WP01 predecessor.
3. **No cycles:** Topological sorting must include every package exactly once.
4. **No duplicated scope:** Each real capability has one primary owner.
   Predecessors may expose spike evidence and successors may productize or
   harden it only when that transition is explicit.
5. **No conflicting ownership:** Two active packages may not make incompatible
   changes to the same contract, lifecycle, settings, release, or data-writing
   boundary.
6. **No future-dependent acceptance:** A package must pass using accepted
   predecessors plus its own work. Successor work cannot be an acceptance gate.
7. **Consumer before extraction:** Feature/command contracts require the real
   second feature in `GFD-P5-WP01`. `packages/ui`, a broader public module SDK,
   and template surfaces require a second application or other concrete
   consumer in `GFD-P8-WP01`.
8. **Platform evidence before release claims:** Phase 0B establishes truthful
   build/runtime/native evidence; Phase 7 hardens only declared artifacts;
   signing/updater claims wait for `GFD-P7-WP02`.
9. **One implementation package per prompt:** No activated prompt spans more
   than one matrix row.
10. **Status truth:** Only approved implementation-status labels are used;
    prompt lifecycle states are tracked separately.
11. **Model proportionality:** Extra High is reserved for lifecycle,
    concurrency, security, cross-platform packaging, signing/update, and
    recovery risk. High is used for substantial bounded UI, feature,
    persistence, extraction, and baseline work. Activation/housekeeping uses
    lower capacity unless a concrete risk requires more.
12. **Stale-fact refresh:** Activation rechecks every fact named in the last two
    columns of the applicable row and stops on material drift.

## 4. Ownership transition notes

- Phase 0B spike code is accepted evidence; Phase 3 may productize it without
  restarting the spike or broadening its support matrix.
- Phase 2 may persist only shell/layout preferences needed for the shell.
  `GFD-P6-WP01` owns comprehensive settings schemas, migrations, atomic
  recovery, and single-instance behavior.
- Phase 3 owns native operation, task, file-intent, and lifecycle boundaries.
  Feature packages consume them and may add only scoped domain operations.
- `GFD-P5-WP01` may extract feature/command contribution contracts only after
  its second feature is real. It does not justify `packages/ui`.
- Phase 7 owns production release hardening. Earlier package scripts must remain
  narrow build/test support and must not claim production signing or update
  readiness.
- `GFD-P8-WP01` is the only planned template-extraction package and must leave
  any one-consumer abstraction `Not started`.
