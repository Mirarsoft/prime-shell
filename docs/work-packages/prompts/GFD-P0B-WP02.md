# GFD-P0B-WP02 — Lifecycle and Task Resilience Spike

## 1. Package identity and prompt status

**Package ID:** `GFD-P0B-WP02`
**Phase:** `Phase 0B`
**Title:** `Lifecycle and Task Resilience Spike`
**Task ID:** `GFD-P0B-WP02`
**Prompt ID:** `PRIME-SHELL-GFD-P0B-WP02-PROMPT`
**Prompt version:** `R1`
**Prompt lifecycle state:** `Provisional`
**Implementation status at authoring:** `Not started`
**Recommended implementation model:** `GPT-5.6 Sol`
**Reasoning/intelligence:** `Extra High`
**Authorization boundary:** Exactly one package, `GFD-P0B-WP02`
**Execution status:** `Execution is not authorized`

Extra High is the minimum suitable reasoning level because this package crosses
lifecycle-state, concurrency, cancellation-race, timeout, crash/hang recovery,
bounded-protocol, process-tree-containment, and no-replay boundaries.

This provisional prompt is complete for review but is non-executable. Chat
Session must first accept it as an approved provisional prompt and later issue a
separately activated revision against current repository and predecessor facts.

### Activation metadata

```text
Activation ID: Not activated — Chat Session must refresh and supply this exact value.
Activated by: Not activated — Chat Session must refresh and supply this exact value.
Activation UTC: Not activated — Chat Session must refresh and supply this exact value.
Authoritative main SHA: Not activated — Chat Session must refresh and supply this exact value.
Required fresh implementation branch: Not activated — Chat Session must refresh and supply this exact value.
Accepted predecessor heads/evidence: Not activated — Chat Session must refresh and supply exact refs, artifacts, hashes, and CI evidence.
Accepted targeted amendments: Not activated — Chat Session must refresh and supply the exact list or None.
Predecessor deviations incorporated: Not activated — Chat Session must refresh and supply the exact list or None.
Current tools/CI/platform facts: Not activated — Chat Session must verify and supply them.
Unresolved blockers/assumptions: Not activated — Chat Session must refresh and supply the exact list or None.
Authorization boundary: Not activated — the future activation may authorize only GFD-P0B-WP02.
Authorization expires/invalidates when: Not activated — the future activation must invalidate on any base-SHA, accepted-predecessor, material platform-capability, or authorization-boundary change.
```

## 2. Repository and exact starting state

**Repository:** `Mirarsoft/prime-shell`
**Authoritative base branch:** `main`
**Required starting commit:** Not activated — Chat Session must refresh and
supply the exact current accepted and merged `main` SHA.
**Required implementation branch:** Not activated — Chat Session must verify
absence and supply one exact fresh branch name.

Authoring-time facts, for review only:

- accepted WP01 implementation head before merge:
  `bd502ffcd4c2d80bd3cd8817ebea4dfb18a76107`;
- accepted WP01 squash-merged `main`:
  `35d53bffec6e5b05aefdf8c7fed39a9bfdf288a2`;
- accepted WP01 Ubuntu evidence: Actions run `30285523445`, job
  `90042226699`, successful;
- Stage 1 documentation branch at authoring:
  `docs/gfd-work-package-prompt-pack-v1`;
- Stage 1 foundation head:
  `cf2c2613d1facca1e61ba03eab88078a0fc1ffbf`.

The documentation-branch refs above do not authorize implementation from that
branch. Activation and implementation must start from the latest accepted and
merged `main`.

Required activation and Work Session preflight:

1. Verify exact access and write permission for `Mirarsoft/prime-shell`.
2. Verify the default branch is `main` and record its exact 40-character SHA.
3. Verify the accepted WP01 merge, files, evidence, deviations, and relevant
   status inventory.
4. Verify no later application package has been implemented or merged
   unexpectedly.
5. Verify the exact fresh implementation branch does not already exist.
6. Use a clean fresh clone/worktree at the activated starting commit.
7. Verify every allowed and protected repository path against the current
   tree.
8. Stop if repository state differs from activation, is ambiguous, would
   overwrite unrelated work, or would require silently rebasing the package.

The merged repository is authoritative over old snapshots, unmerged branches,
chat summaries, and superseded handoffs.

## 3. Mandatory authority and reading order

Read completely, in this order, before changing repository files:

1. `AGENTS.md`
2. `docs/authority/Core-Functionality-DeliveryRules.md`
3. `docs/authority/generic-fluent-desktop-project-handoff.md`
4. `docs/authority/generic-fluent-desktop-app-architecture-v0.2.md`
5. `docs/authority/architecture-review-consolidation-decision-log.md`
6. all accepted Markdown files under `docs/phase-0a/`, including:
   - `ADR-0001-tauri-react-fluent-stack.md`
   - `ADR-0002-rust-python-trust-model.md`
   - `ADR-0003-python-sidecar-onedir-spike.md`
   - `ADR-0004-bounded-json-lines-contract.md`
   - `ADR-0005-platform-adaptive-title-bar.md`
   - `phase-0a-definition.md`
   - `phase-0a-readiness-report.md`
   - `phase-0a-spike-plan.md`
   - `work-session-handoff-manifest.md`
7. `docs/work-packages/roadmap-index.md`
8. `docs/work-packages/shared-ground-rules.md`
9. `docs/work-packages/dependency-matrix.md`
10. the accepted WP01/FIX01 review, implementation report, evidence index,
    handoff manifest, and source/artifact hash records named by activation
11. every accepted targeted architecture amendment named by activation
12. current contracts, Rust backend, Python sidecar, frontend boundary,
    verification scripts, lockfiles, and relevant CI workflow
13. the exact activated revision of this execution prompt

In the first implementation progress message, state:

> `AGENTS.md, repository authority, accepted Phase 0A, accepted WP01/FIX01
> predecessor evidence, and the activated GFD-P0B-WP02 prompt have been read
> and are active. Executing only GFD-P0B-WP02 with GPT-5.6 Sol / Extra High.
> GFD-P0B-WP03 and later work remain unauthorized.`

If a required source is unavailable, unreadable, stale, or materially
inconsistent, stop before writing. Do not conduct a broad architecture review.

## 4. Dependencies, predecessor outputs, and prerequisite evidence

**Direct package prerequisite:** Accepted, merged Phase 0B WP01/FIX01 at the
activation base.

**Authoring-time accepted predecessor commits:**

- accepted WP01 implementation head before squash merge:
  `bd502ffcd4c2d80bd3cd8817ebea4dfb18a76107`;
- accepted merged `main`:
  `35d53bffec6e5b05aefdf8c7fed39a9bfdf288a2`.

**Required predecessor outputs:**

- merged WP01 source implementing the minimal React/Fluent/Tauri host,
  `spike.echo`, Rust operation authorization, packaged PyInstaller `onedir`
  sidecar, bounded JSON Lines baseline, and release CSP;
- the accepted WP01/FIX01 review and the exact implementation report,
  review-evidence index, handoff manifest, source snapshot, and hashes supplied
  by activation;
- current repository contract, toolchain, lockfile, validation-script, and
  workflow state.

**Required prerequisite evidence:**

- accepted exact Unicode echo through the real native Tauri path;
- visible release-mode Fluent rendering with the accepted CSP;
- backend readiness and safe error rendering;
- Rust-to-packaged-sidecar execution;
- normal native-host closure with zero surviving sidecar processes;
- successful Ubuntu 24.04 Actions run `30285523445`, job `90042226699`, or the
  exact accepted replacement evidence supplied by activation.

**Accepted deviations:** None are inferred by this provisional prompt.
Activation must list every accepted predecessor deviation or state `None`.

**Known blockers carried forward:** No implementation blocker is asserted at
authoring. Implementation itself remains `Blocked` until Chat Session supplies
an exact activated prompt.

If WP01 evidence is not accepted and merged, if a build is the only available
substitute for required runtime evidence, or if current paths materially
conflict with this package boundary, activation or execution must stop.

## 5. Objective and measurable runnable outcome

**Objective:** Extend the accepted WP01 echo baseline with the smallest real
lifecycle and task-resilience vertical slice, without restarting WP01 or
entering WP03.

**Required runnable outcome:**

```text
React minimal task controls/status
→ typed Tauri commands/channels
→ Rust operation registry and lifecycle authority
→ packaged PyInstaller onedir Python sidecar
→ bounded ordered task events
→ deterministic cancel/timeout/crash/hang/restart outcomes
→ visible terminal state and zero surviving descendants
```

An operator or test must be able to start one synthetic `spike.count` task in
the real native Tauri application, observe ordered/coalesced bounded progress,
cancel it, and see one deterministic terminal state. Focused fault paths must
prove timeout behavior without a rollback claim, deliberate crash detection,
deliberate hang escalation, one bounded restart, repeated-failure circuit
behavior, no automatic replay, bounded frame/queue/log handling, and zero
surviving descendants after both normal and forced host closure.

The existing `spike.echo` path is a regression baseline. It must remain
functional and must not be reimplemented as substitute WP02 progress.

## 6. Explicit in-scope work

The package authorizes only:

1. Extend JSON Schema and shared valid/invalid fixtures for the existing
   `spike.echo` regression and the test-only `spike.count`, `spike.crash`,
   `spike.hang`, and `spike.largeRejected` operations.
2. Add the minimum task-event, cancellation, terminal-result, safe-error, and
   backend-status contracts required by this package.
3. Extend the compile-time Rust operation registry so all five test operations
   are explicitly known and unknown operations still stop in Rust.
4. Implement a Rust-owned sidecar lifecycle and operation registry with
   bounded pending work, bounded events, ordered task sequencing, deadlines,
   crash/hang detection, one restart budget, a 60-second repeated-failure
   circuit, and no replay.
5. Extend the single packaged Python sidecar with the synthetic count, crash,
   hang, and oversized-rejection test paths plus cooperative cancellation and
   independently responsive control-message reading.
6. Add typed Tauri commands/channels or the accepted ordered equivalent for
   task start, cancel, status, backend state, progress, and terminal events.
7. Add only the minimal React controls and state needed to start count, show
   bounded progress, cancel, display terminal/error/trace state, show backend
   ready/restarting/faulted state, and expose a safe explicit recovery action.
8. Preserve continuous stdout and stderr draining, protocol-only stdout,
   structured-log-only stderr, frame/log bounds, and safe stream-corruption
   handling.
9. Implement and verify cooperative cancellation, success/cancel/timeout race
   precedence, crash interruption, hang escalation, circuit behavior, and
   graceful/forced process-tree termination.
10. Add proportional contract, Python, Rust, packaged-sidecar, frontend, native
    Tauri, and process-containment tests for this slice.
11. Add or update one narrowly scoped Ubuntu 24.04 WP02 validation workflow only
    when activation confirms it is required for retained real evidence.
12. Produce the required authoritative source snapshot and three Markdown
    handoff artifacts after implementation validation.

This section authorizes one package only. It grants no WP03, successor, merge,
publication, release, or repository-setting authority.

## 7. Explicit exclusions and prohibited adjacent work

Do not:

- implement `GFD-P0B-WP03` title-bar promotion/fallback, native WebdriverIO
  authority, complete Windows/macOS/Ubuntu package matrix, clean-install
  matrix, cross-engine gallery, final measurements, or Phase 0B closure report;
- implement real document analysis, file import, a product feature, database,
  account, or runtime network service;
- introduce multiple Python workers, a worker pool, or a per-task worker
  process;
- introduce durable tasks, pause/resume, resume after restart, or automatic
  replay;
- add GPU, model, scientific-native, or uninterruptible workload support;
- add production updater, signing, notarization, telemetry, crash upload,
  diagnostics export, publishing, stores, or enterprise deployment;
- add runtime plugins, a module SDK, `packages/ui`, a component library, a
  broad application shell, settings, persistence, template extraction, or
  speculative extension points;
- redesign or broadly refactor accepted WP01 code;
- weaken Rust operation authorization, frame/queue/log limits, process
  containment, CSP, or safe-error/redaction behavior;
- add an unsupported OS, architecture, Linux distribution, or package-format
  claim;
- present unit, fixture, browser-mock, or build-only evidence as real native
  runtime proof;
- alter authority, Phase 0A, prompt-pack, release, or unrelated documentation;
- implement a predecessor correction without first stopping for Chat Session
  review;
- begin Phase 1 or any dependent/later package.

If a per-task process, architecture amendment, additional platform, or broader
repository area appears necessary, stop with evidence and request a focused
prompt correction. Do not add it speculatively.

## 8. Allowed repository areas and expected changes

Activation must verify current paths and narrow this list where possible.
Subject to that refresh, only these responsibility areas may change:

```text
packages/app-contracts/schemas/
packages/app-contracts/fixtures/
packages/app-contracts/generated/                 # only if already used or mechanically required
apps/desktop/src-tauri/src/backend/
apps/desktop/src-tauri/src/lib.rs
apps/desktop/src-tauri/tests/
apps/desktop/src/backend.ts
apps/desktop/src/contracts.ts
apps/desktop/src/App.tsx                          # or the current minimal task-facing equivalent
apps/desktop/src/App.test.tsx                     # or focused current task-facing tests
apps/desktop/src/app.css                          # minimal task-state/accessibility styling only
services/python-backend/src/prime_shell_backend/
services/python-backend/tests/
scripts/verify/                                   # focused WP02 lifecycle/process verification only
tests/                                            # focused WP02 tests only, if current layout requires them
.github/workflows/                                # at most one narrow Ubuntu 24.04 WP02 workflow
package.json                                      # script wiring only when mechanically required
apps/desktop/package.json                         # script/dependency wiring only when mechanically required
apps/desktop/src-tauri/Cargo.toml                 # only a directly required lifecycle/process dependency
apps/desktop/src-tauri/Cargo.lock                 # mechanically resulting locked resolution only
pnpm-lock.yaml                                    # mechanically resulting locked resolution only
services/python-backend/requirements-build.*      # only if packaging mechanics require a locked change
```

Expected changes are task schemas/fixtures, explicit Rust/Python task and
lifecycle modules, minimal typed frontend adapters/UI, focused lifecycle and
containment verification, and only mechanically required lock/generated
companions.

The following must remain unchanged:

- `AGENTS.md`;
- `docs/authority/` and `docs/phase-0a/`;
- `docs/work-packages/`, including this prompt;
- WP03 native-E2E, platform-matrix, title-bar, measurement, and spike-closure
  areas;
- Phase 1+ shell, product, settings, diagnostics, release, signing, updater,
  SDK, UI-package, and template areas;
- unrelated workflows, dependencies, tests, scripts, and documentation.

A necessary path outside the activated allowed list is a stop condition unless
it is a mechanically generated/locked companion explicitly authorized at
activation.

## 9. Ordered implementation procedure

1. Complete the authority, repository, activation, clean-tree, and predecessor
   preflight at the exact activated `main` SHA.
2. Record current paths, tool versions, commands, accepted WP01 behavior, and
   the exact initial status inventory before writing.
3. Run the accepted WP01 contract, frontend, Python, packaged-sidecar, Rust,
   Tauri-build, and native-runtime baselines relevant to files that WP02 may
   change. Stop on a predecessor regression.
4. Extend the schema authority and shared fixtures for the five exact test
   operations, task events, cancellation, terminal results, backend status,
   and safe errors before widening implementation.
5. Implement the Python `spike.count`, `spike.crash`, `spike.hang`, and
   `spike.largeRejected` test behavior inside the single sidecar, with
   cooperative checkpoints and responsive control reading.
6. Implement the Rust lifecycle authority, explicit operation authorization,
   bounded queues/framing/log handling, task registry, monotonic sequences,
   progress coalescing, deadlines, cancellation races, crash/hang escalation,
   restart budget, circuit behavior, and no replay.
7. Connect only the minimum typed Tauri task commands/channels and React
   controls/status needed to operate and observe the slice.
8. Add focused tests continuously for contracts, Python protocol/task faults,
   Rust lifecycle/queue/races/restart/circuit/containment, frontend state and
   accessibility, and packaged-sidecar behavior.
9. Prove stderr flooding cannot block stdout protocol progress and that
   malformed input, oversize, queue exhaustion, unknown kinds/operations, and
   stream corruption fail deterministically.
10. Prove normal close, graceful-timeout escalation, forced host close, and
    crash/hang paths leave zero sidecar descendants.
11. Build the release `.deb` and exercise the real Ubuntu 24.04 Tauri path under
    Xvfb, observing count progress, cancel, terminal state, backend lifecycle,
    safe errors, accepted echo/CSP behavior, and cleanup.
12. Run or add the activated narrow Ubuntu validation workflow, retain exact
    run/job/log/artifact evidence, and distinguish CI from local evidence.
13. Run the complete proportional baseline and WP02 checks from a clean tree.
14. Commit only the coherent WP02 implementation, create a complete source
    snapshot of the final commit, verify CRC/path safety/one root/hash, extract
    it into a fresh empty directory, and rerun every command required by
    activation.
15. Verify the exact diff, status inventory, descendants, deliverables, hashes,
    and branch/PR state; then return the handoff without entering WP03.

Prefer a direct implementation over a reusable task framework. Stop rather than
expanding scope to make an unrelated check pass.

## 10. Cross-cutting constraints

### Contract

- JSON Schema 2020-12 plus shared valid/invalid fixtures remains authoritative.
- Cover only `spike.echo`, `spike.count`, `spike.crash`, `spike.hang`, and
  `spike.largeRejected`.
- Every operation name is schema-bound and explicitly authorized in Rust.
- Unknown operations are rejected in Rust before Python receives them.
- Use UTF-8 JSON Lines and accept CRLF.
- Handshake frames are at most `64 KiB`; normal frames are at most `1 MiB`.
- Structured log lines are at most `64 KiB`, with deterministic safe truncation
  or rejection behavior that does not block the protocol.
- Pending requests are bounded initially to `64`; the backend event queue is
  bounded initially to `256`.
- UI progress delivery is coalesced to no more than
  `10 updates/second/task`; terminal events are never coalesced or dropped.
- Invalid UTF-8/JSON, unknown kinds, unknown operations, oversize, queue
  exhaustion, and stream corruption fail deterministically.
- Keep stdout protocol-only, stderr structured-log-only, and drain both
  continuously. No protocol/log mixing or unbounded buffering is permitted.
- Activation must refresh exact schema paths and generation/drift commands.

### Security and trust

- React remains least trusted; Rust remains the native policy, operation, task,
  and process-lifecycle authority; Python remains trusted first-party native
  code, not a sandbox.
- Do not expose generic shell, process, arbitrary filesystem, secret, native
  path, or unrestricted operation access to React.
- Rust launches only the exact verified bundled resource without a shell,
  constructs a minimal environment, and checks expected build/schema/target
  identity.
- Do not log raw payloads, user content, credentials, authorization headers,
  environment dumps, signing material, secrets, or full paths by default.
- Preserve local-only assets and the accepted release CSP. A demonstrated
  Griffel/CSP regression must be fixed narrowly without broad `unsafe-inline`.
- Errors crossing into React must use stable safe codes, bounded messages, a
  trace identifier, and no internal path or payload leakage.

### Lifecycle and concurrency

Required backend lifecycle:

```text
Stopped → Starting → Ready → Busy → Ready
Starting → Faulted
Ready/Busy → Restarting → Ready or Faulted
Ready/Busy → Stopping → Stopped
```

Required task states:

```text
Queued
Running
Cancelling
Succeeded
Failed
Cancelled
TimedOut
Interrupted
```

Do not introduce `Paused`.

- Permit one active long-running sidecar task initially.
- Short status/echo operations may coexist only if the same sidecar remains
  responsive without a worker pool or per-task process.
- Task-event sequence is monotonically increasing.
- Progress may be coalesced; terminal events may not be lost or duplicated.
- Cancellation acknowledgement confirms receipt only. It does not confirm
  completed stop or rollback.
- Target cancellation acknowledgement is `≤250 ms` under normal load.
- Target cooperative stop is `≤2 s` for `spike.count`.
- A success already accepted by Rust wins over a later cancellation race.
- Timeout requests cancellation and may escalate, but never claims rollback.
- Every success/cancel/timeout race produces exactly one terminal state.
- Unexpected sidecar exit fails every in-flight request with
  `BACKEND_CRASHED` at the service boundary and marks affected accepted tasks
  `Interrupted`.
- Perform at most one automatic restart after a short bounded backoff. Never
  replay in-flight work.
- Repeated failure within `60 seconds` opens a circuit and requires explicit
  user/test action before another start.
- An unresponsive task reaches a bounded cancellation deadline, then Rust
  terminates the contained sidecar tree, marks affected work `Interrupted`,
  and either completes the one allowed restart or enters `Faulted`.
- No operation is retried unless it is explicitly idempotent.
- No task is durable or resumed after application restart.
- Do not create a per-task worker process unless focused implementation
  evidence first blocks this model and Chat Session approves a corrected
  prompt.
- Graceful host shutdown has a bounded deadline followed by process-tree
  termination. Normal and forced host close must leave zero descendants.

### Accessibility

- Start count, cancel, safe recovery, and task-status controls must be keyboard
  operable and have accessible names.
- Preserve visible focus and logical focus order.
- Announce backend state and terminal task changes through controlled live
  regions.
- Do not announce every progress tick; throttle/coalesce announcements so
  assistive technology is not flooded.
- Progress remains understandable without motion or color alone.
- Reduced-motion preference must disable nonessential progress animation while
  preserving state.
- Error and terminal states include text, safe code, and trace identifier.
- Use proportional frontend accessibility tests plus real native observation;
  do not expand into Phase 2 shell accessibility work.

### Platform and packaging

- Primary real evidence environment for WP02 is Ubuntu 24.04 x64 with
  WebKitGTK, native Tauri runtime, Xvfb where needed, `.deb`, and the packaged
  PyInstaller `onedir` sidecar.
- Keep lifecycle/process code portable where required by current architecture,
  but do not claim Windows or macOS runtime proof without evidence.
- Build the sidecar on the target OS/architecture from locked dependencies.
- Preserve exact resource containment and manifest/hash verification.
- Do not require WebdriverIO in WP02.
- Do not claim title-bar promotion/fallback, cross-platform packaging,
  clean-install matrix, complete measurements, or final spike closure; those
  remain WP03.

### Data ownership and privacy

- WP02 introduces no product database, durable task store, settings store,
  file import, network state, or new durable-data writer.
- In-memory task state is Rust-owned; Python owns only current test-operation
  execution inside the sidecar.
- Task traces and logs contain only allowlisted operational fields and are
  bounded/redacted.
- Evidence must not retain raw payloads, credentials, environment dumps,
  signing material, secrets, user document content, or unrestricted full
  paths.
- Source snapshots and handoff reports contain repository source and sanitized
  execution evidence only.

## 11. Proportional tests and exact evidence

Activation must verify every command against the current repository and replace
any stale command before changing the lifecycle state to `Activated`.

### Current accepted baseline commands, subject to activation refresh

From the repository root:

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

From `apps/desktop/src-tauri`:

```text
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked -- --include-ignored
```

From the repository root:

```text
xvfb-run -a pnpm --filter @prime-shell/desktop tauri build --bundles deb -- --locked
```

### Required focused WP02 commands

This provisional revision does not authorize execution. Before activation,
Chat Session must inspect the final accepted scripts and supply exact runnable
commands for:

- contract/fixture validation for all five test operations and task events;
- Python protocol, control, count, cancel, crash, hang, log-flood, and bound
  tests;
- Rust lifecycle, registry, queue, ordering, terminal-race, timeout, restart,
  circuit, and process-containment tests;
- direct packaged-sidecar lifecycle and fault verification;
- frontend task-state and accessibility tests;
- real native Tauri lifecycle verification under Ubuntu 24.04/Xvfb;
- normal-close and forced-close descendant checks;
- frame, queue, progress, and structured-log bounds;
- source-snapshot CRC/path/root validation, fresh empty extraction, and full
  rerun.

Until those exact focused commands are supplied in an activated revision, this
prompt remains non-executable.

### Required evidence layers

For every check, record the command/method, expected behavior, actual result,
OS/architecture/tool identity, duration where relevant, retained artifact or
log, and blocker.

Keep these evidence classes separate:

1. schema/fixture and unit evidence;
2. Python and Rust integration evidence;
3. direct packaged-sidecar evidence;
4. real native Tauri runtime evidence;
5. Ubuntu 24.04 CI evidence;
6. unavailable Windows/macOS/manual evidence, which remains unclaimed.

### Required CI evidence

Use at most one narrowly scoped Ubuntu 24.04 WP02 workflow/job when required.
Activation must supply the exact workflow path/name, runner label, trigger
policy, commands, permissions, artifact-retention behavior, and expected
evidence. Retain the final run ID, job ID, commit SHA, conclusion, logs, and
artifact hashes.

No WP02 gate may be satisfied only by browser mocks, build success, fixture
success, or a platform that was not actually exercised.

## 12. Measurable acceptance gates

The package passes only when retained evidence proves:

1. All five test operations are schema-bound and Rust-authorized; unknown
   operations still stop in Rust.
2. The real packaged Python sidecar performs `spike.count` with monotonically
   ordered progress and one retained terminal event.
3. UI-observed progress is bounded to `≤10 updates/second/task`, while terminal
   events are never coalesced, dropped, or duplicated.
4. Cancellation acknowledgement and cooperative stop meet the initial
   `≤250 ms` and `≤2 s` targets, or truthful measured deviation is retained
   without fabricating a pass.
5. Success/cancel/timeout races produce exactly one deterministic terminal
   state and timeout does not claim rollback.
6. Deliberate crash fails all in-flight work, marks accepted tasks
   `Interrupted`, performs at most one bounded restart, and replays no work.
7. Deliberate hang escalation interrupts affected work and restores one ready
   sidecar or leaves the backend explicitly `Faulted` according to the restart
   budget.
8. Repeated failure within the declared 60-second window opens the circuit and
   requires explicit user/test recovery.
9. Oversized frames and exhausted request/event queues fail safely and
   deterministically without unbounded memory growth or sidecar corruption.
10. A bounded stderr flood cannot block stdout protocol progress; logs remain
    structured, bounded, and redacted.
11. Normal native-host close and forced native-host close each leave exactly
    zero sidecar child or descendant processes after the containment deadline.
12. Accepted WP01 Unicode echo, backend readiness, release CSP/Fluent
    rendering, safe-error path, packaged-sidecar, and relevant native baseline
    show no regression from WP02 changes.
13. The real Ubuntu 24.04 native Tauri application path is exercised under
    Xvfb or an accepted visible display; browser-only evidence is insufficient.
14. The final source snapshot represents the final commit, has valid SHA-256
    and CRC, contains one safe expected root, extracts into a fresh empty
    directory, and passes the activated required rerun.
15. `GFD-P0B-WP03` and every later package remain `Not started`, with no
    unauthorized path, dependency, feature, platform, PR-merge, or release
    work.

These are initial spike goals, not fabricated service-level results. Record raw
measurements and classify deviations truthfully.

## 13. Stop conditions and blocker reporting

Stop without improvising when:

- the repository, default branch, activated base SHA, predecessor state, or
  required fresh branch differs from activation;
- a required authority, accepted predecessor artifact, hash, CI run/job, or
  repository path is missing, stale, or materially conflicting;
- accepted WP01 behavior regresses before WP02 changes;
- a frozen trust, protocol, lifecycle, platform, data-ownership, or
  architecture boundary would need to change;
- the package requires an unapproved path, dependency, platform, credential,
  worker process, durable state, or adjacent package;
- deterministic cancellation/timeout terminal semantics, bounded queues/logs,
  one-restart/no-replay behavior, or process-tree containment cannot be
  achieved with the accepted model;
- zero descendants cannot be proven after normal and forced host closure;
- real packaged-sidecar or native Tauri evidence cannot be executed;
- the only available support for a claimed gate is mock-only, browser-only,
  build-only, unavailable, fabricated, or successor-dependent evidence;
- a test workflow is blocked by permissions, policy, quota, billing, runner,
  or artifact-retention limits;
- completing the work would require WP03, a broad WP01 redesign, an
  architecture amendment, or unsupported platform claims;
- source snapshot integrity or fresh-extraction rerun fails;
- the final diff includes an unauthorized path or unrelated existing changes
  cannot be preserved safely.

Return `Blocked` or `Partially implemented` using the approved labels. Name the
exact failed gate, observed evidence, unchanged scope, and smallest safe next
action. Do not enter later work as a workaround.

## 14. Required functional and status inventory

Before activation, the truthful inventory is:

```text
GFD-P0B-WP02 package outcome: Not started
Real lifecycle/task spike functionality: Not started
Supporting infrastructure: Not started
Tests: Not started
Documentation for implementation: Not started
Generated code: Not started
Fixtures/mocks: Not started
Stubs/placeholders: Not started
Incomplete work: Not started
Blocked work: Blocked — implementation has no activated prompt
Accepted WP01 regression baseline: Implemented
GFD-P0B-WP03: Not started
```

At implementation completion, report every line above with exactly one of:
`Implemented`, `Partially implemented`, `Stub`, `Mock-only`, `Not started`, or
`Blocked`. Add evidence references and separately report:

- schemas and fixtures;
- Python sidecar operations;
- Rust operation/lifecycle/task authority;
- typed Tauri channels/commands;
- minimal React task controls/status;
- packaged-sidecar evidence;
- real native Ubuntu evidence;
- cancellation/timeout/crash/hang/restart/circuit evidence;
- normal/forced-close containment;
- source snapshot and fresh-extraction rerun;
- CI evidence;
- Windows/macOS and WP03 work as `Not started` or truthfully unavailable.

Documentation or tests may be `Implemented` while real behavior is
`Partially implemented` or `Blocked`; never conflate them.

## 15. One `RUN_ID` and collision-resistant external naming

At the start of a future activated implementation, generate exactly one UTC
identifier:

```text
RUN_ID=YYYYMMDDTHHMMSSZ
```

Use that value unchanged for every external artifact and retained handoff name.
Do not reuse a prior run ID or generate separate IDs for a rerun, correction,
snapshot, or manifest within the same execution.

Repository filenames remain conventional and do not receive timestamps unless
their existing repository contract requires versioning.

## 16. Required deliverables, hashes, and evidence index

The future activated implementation must produce exactly these external
deliverables:

```text
prime-shell-work-gfd-p0b-wp02-<RUN_ID>-source-snapshot-r1.zip
prime-shell-work-gfd-p0b-wp02-<RUN_ID>-implementation-report-r1.md
prime-shell-work-gfd-p0b-wp02-<RUN_ID>-review-evidence-index-r1.md
prime-shell-work-gfd-p0b-wp02-<RUN_ID>-handoff-manifest-r1.md
```

`RUN_ID` is the one runtime token defined in section 15.

The source snapshot must:

- represent the exact final implementation commit;
- contain one expected repository root;
- exclude repository metadata, caches, build output, credentials, and
  unrelated local files;
- pass ZIP CRC and path-safety checks;
- have an ordinary SHA-256;
- extract into a fresh empty directory;
- pass the exact activated required rerun from that extraction.

The implementation report must describe the real vertical slice, responsible
execution paths, observed behavior, measurements, tests, CI/native evidence,
limitations, status inventory, exclusions, and result.

The review evidence index must map every package requirement and acceptance
gate to a repository file/section, command/result, retained artifact, and
unresolved item without duplicating the implementation report.

The handoff manifest must record:

- activation ID, prompt ID/version, model/reasoning, repository, branch, exact
  base and final commit;
- accepted predecessor refs, artifacts, hashes, deviations, amendments, and
  CI evidence;
- changed files and real execution paths;
- tool/runner/platform versions;
- exact commands and actual results;
- local, packaged, native, CI, and unavailable platform evidence separately;
- SHA-256 of the source snapshot, prompt-required reports, and required
  repository artifacts;
- a reproducible canonical self-hash convention if the manifest contains its
  own digest;
- truthful functional/status inventory, limitations, files intentionally not
  created, and exact next controlled action.

Expose the four artifacts individually to Chat Session. Do not create
additional package prompts or present an incremental overlay as the
authoritative source.

## 17. Branch, draft-PR, review, merge, and deletion controls

This provisional prompt authorizes no implementation branch or pull request.

When Chat Session activates it:

1. Use only the exact fresh implementation branch named in the activation
   record, created from the exact activated `main` SHA.
2. Do not reuse the documentation branch or an earlier WP01/recovery branch.
3. Keep one coherent WP02 change and preserve unrelated user work.
4. Commit intentionally with a package-scoped message and push only the
   activated branch.
5. Open or update one draft PR only if the activated prompt explicitly
   authorizes it and supplies the exact base/head controls.
6. Keep any authorized PR draft until Chat Session accepts the exact head.
7. Do not enable or use auto-merge.
8. Do not merge, publish, release, change repository settings, or delete a
   branch without separate explicit user authorization after Chat Session
   acceptance.
9. After an explicitly authorized merge, verify `main` contains the accepted
   result before deleting only the exact merged implementation branch.
10. Prompt acceptance is not activation; implementation acceptance is not
    merge authorization; WP02 completion is not WP03 authorization.

Report exact local/remote branch, commit, PR, draft, auto-merge, merge, and
deletion state in the handoff.

## 18. Completion response and return prompt

Keep progress and completion messages concise. Lead with the truthful package
result and include:

- model and reasoning level;
- repository, base, branch, final commit, clean-tree, PR, and merge state;
- real functionality and responsible files;
- commands and actual results;
- packaged/native/CI evidence and limitations;
- full status inventory;
- four external artifact links and hashes;
- confirmation that WP03 and later work were not started.

At completion, define:

```text
FINAL_SHA=the exact 40-character final implementation commit
```

Then end with this short prompt, substituting the defined `FINAL_SHA` and the
section 15 `RUN_ID` with their exact values:

```text
Chat Session: Review GFD-P0B-WP02 on the activated implementation branch at FINAL_SHA.
Read the four prime-shell-work-gfd-p0b-wp02-RUN_ID deliverables. Verify bounded
progress, deterministic cancel/timeout/crash/hang/restart/circuit behavior, no
replay, zero descendants, native Ubuntu evidence, and snapshot rerun, then
return Accepted, Focused correction required, or Blocked. GFD-P0B-WP03 was not
started, and this handoff does not authorize merge or later implementation.
```

Do not include authorization for WP03 or any later package. Do not continue
after returning the handoff.
