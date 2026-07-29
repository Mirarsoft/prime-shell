# GFD-P3-WP02 — Productized Task Runtime and Sidecar Operations

## 1. Package identity and prompt status

**Package ID:** `GFD-P3-WP02`
**Phase:** `Phase 3`
**Title:** `Productized Task Runtime and Sidecar Operations`
**Task ID:** `GFD-P3-WP02`
**Prompt ID:** `PRIME-SHELL-GFD-P3-WP02-PROMPT`
**Prompt version:** `R1`
**Prompt lifecycle state:** `Provisional`
**Implementation status at authoring:** `Not started`
**Recommended implementation model:** `GPT-5.6 Sol`
**Reasoning/intelligence:** `Extra High`
**Authorization boundary:** Exactly one package, `GFD-P3-WP02`
**Execution status:** `Execution is not authorized`

Extra High is required because this package crosses task acceptance and
terminal-state races, bounded framing and queues, ordered and coalesced
progress, cancellation escalation, timeouts, crash and hang handling, bounded
restart and circuit behavior, process-tree containment, structured redacted
logging, per-target packaged-sidecar resolution, and strict no-replay
guarantees. A small inconsistency could create an orphan process, duplicate
uncertain work, lose a terminal state, mix logs with protocol, or permit
unbounded growth.

This provisional prompt is complete for review but is non-executable. Chat
Session must first accept it as an approved provisional prompt and later issue
a separately activated revision only after every prerequisite is accepted and
merged. Prompt acceptance is not activation and grants no implementation
authority.

### Activation metadata

```text
Activation ID: Not activated — Chat Session must refresh and supply this exact value.
Activated by: Not activated — Chat Session must refresh and supply this exact value.
Activation UTC: Not activated — Chat Session must refresh and supply this exact value.
Authoritative main SHA: Not activated — Chat Session must refresh and supply this exact value.
Required fresh implementation branch: Not activated — Chat Session must refresh and supply this exact value.
Accepted WP01, P0B-WP02, P0B-WP03, P1-WP01, P2-WP01, P2-WP02, and P3-WP01 heads/evidence: Not activated — Chat Session must supply exact merged commits, reports, artifacts, hashes, CI runs/jobs, native/manual evidence, deviations, fallbacks, and blockers.
Final accepted Phase 0B closure report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Final accepted Phase 1 baseline report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Final accepted P2-WP01 theme-foundation report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Final accepted P2-WP02 responsive-shell report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Final accepted P3-WP01 contracts/native-intent report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Accepted targeted amendments: Not activated — Chat Session must refresh and supply the exact list or None.
Predecessor deviations incorporated: Not activated — Chat Session must refresh and supply the exact list or None.
Current operation IDs, schema hash, task/race policy, frame/queue limits, process APIs, sidecar resource layout, toolchain, capabilities, CSP, support matrix, and manual owners: Not activated — Chat Session must verify and supply them.
Current target packages, native/process evidence, and accepted limitations: Not activated — Chat Session must refresh and supply the exact list or None.
Unresolved blockers/assumptions: Not activated — Chat Session must refresh and supply the exact list or None.
Authorization boundary: Not activated — the future activation may authorize only GFD-P3-WP02.
Authorization expires/invalidates when: Not activated — the future activation must invalidate on any base-SHA, predecessor acceptance, targeted amendment, material contract/tool/engine/platform/process/packaging/capability/CSP, repository-layout, or authorization-boundary change.
```

## 2. Repository and exact starting state

**Repository:** `Mirarsoft/prime-shell`
**Authoritative base branch:** `main`
**Required starting commit:** Not activated — Chat Session must refresh and
supply the exact current accepted and merged `main` SHA after P3-WP01
acceptance and merge.
**Required implementation branch:** Not activated — Chat Session must verify
absence and supply one exact fresh branch name.

Authoring-time facts, for review only:

- accepted WP01 implementation head before squash merge:
  `bd502ffcd4c2d80bd3cd8817ebea4dfb18a76107`;
- accepted WP01 squash-merged `main`:
  `35d53bffec6e5b05aefdf8c7fed39a9bfdf288a2`;
- accepted WP01 Ubuntu evidence: Actions run `30285523445`, job
  `90042226699`, successful;
- accepted Stage 1 foundation head:
  `cf2c2613d1facca1e61ba03eab88078a0fc1ffbf`;
- approved-provisional P0B-WP02 prompt documentation head:
  `4195d6588afa78448bb0a909e78ce973c0ab1b24`;
- approved-provisional P0B-WP03 prompt documentation head:
  `074bb2c8bd117e17477ac97ee859e5e89cfe4f82`;
- approved-provisional P1-WP01 prompt documentation head:
  `511f80f60f2bf367a725d96742deba6cb79f6551`;
- approved-provisional P2-WP01 prompt documentation head:
  `96acc196814c802a267b661f79e34295cd927ef1`;
- approved-provisional P2-WP02 prompt documentation head:
  `e861e93b1fc1057a0d3d6cfdded8c52908a870fc`;
- approved-provisional corrected P3-WP01 prompt documentation head:
  `6d880bc6cc95489f84c09ddb76ec60497a3583d5`;
- corrected P3-WP01 prompt SHA-256:
  `465757faf44562afe19e1c561433ddea9e6a058d2a97acdab5a668c4ba8ddfd7`;
- prompt-pack documentation branch:
  `docs/gfd-work-package-prompt-pack-v1`;
- P0B-WP02, P0B-WP03, Phase 1, Phase 2, and P3-WP01 activation and
  implementation: `Not started`;
- P3-WP02 implementation: `Not started`.

The documentation refs do not authorize implementation. P3-WP02 activation
and implementation must start from the latest accepted and merged `main`, only
after P0B-WP02, P0B-WP03, P1-WP01, P2-WP01, P2-WP02, and P3-WP01 are accepted
and merged and their required closure, baseline, foundation, shell, and
contracts/native-intent reports are accepted.

Required activation and Work Session preflight:

1. Verify exact access and write permission for `Mirarsoft/prime-shell`.
2. Verify the default branch is `main` and record its exact 40-character SHA.
3. Verify WP01, P0B-WP02, P0B-WP03, P1-WP01, P2-WP01, P2-WP02, and P3-WP01
   are accepted and merged at that SHA.
4. Verify all accepted predecessor reports, source snapshots, evidence
   indexes, manifests, hashes, CI jobs/artifacts, native/manual evidence,
   measurements, deviations, fallbacks, amendments, and blockers.
5. Verify the accepted P3-WP01 schema bundle, finite operation registry,
   authorization metadata, safe errors, opaque-reference rules, and
   single-writer boundaries.
6. Verify current task schemas/events, request/task/trace identifiers,
   frontend state, Rust lifecycle/client/process ownership, Python protocol and
   control loop, sidecar resource packaging, fault fixtures, logging,
   capabilities, CSP, toolchain, dependencies, locks, workflows, support
   targets, and manual owners.
7. Verify one authoritative productized task/runtime path does not already
   exist unexpectedly and identify all retained spike-era runtime pieces.
8. Verify the exact fresh implementation branch does not already exist.
9. Use a clean fresh clone or worktree at the activated starting commit.
10. Verify and narrow every allowed and protected path against that exact tree.
11. Stop if repository state differs from activation, is ambiguous, would
    overwrite unrelated work, or would require silently rebasing the package.

The merged repository is authoritative over snapshots, documentation branches,
unmerged implementation branches, chat summaries, provisional prompts, and
superseded handoffs.

## 3. Mandatory authority and reading order

Read completely, in this order, before changing repository files:

1. `AGENTS.md`
2. `docs/authority/Core-Functionality-DeliveryRules.md`
3. `docs/authority/generic-fluent-desktop-project-handoff.md`
4. `docs/authority/generic-fluent-desktop-app-architecture-v0.2.md`
5. `docs/authority/architecture-review-consolidation-decision-log.md`
6. all accepted Markdown files under `docs/phase-0a/`
7. `docs/work-packages/roadmap-index.md`
8. `docs/work-packages/shared-ground-rules.md`
9. `docs/work-packages/dependency-matrix.md`
10. the accepted WP01/FIX01 review, source, implementation report, evidence
    index, handoff manifest, source snapshot, hashes, CI run/job, and native
    evidence named by activation
11. the accepted P0B-WP02 prompt, activation, source, review, implementation
    report, evidence index, handoff manifest, source snapshot, hashes, CI and
    native evidence, measurements, deviations, and blockers named by
    activation
12. the accepted P0B-WP03 prompt, activation, source, review, closure report,
    evidence index, handoff manifest, source snapshot, hashes, per-platform
    CI/runtime/native/manual evidence, measurements, deviations, fallbacks, and
    blockers named by activation
13. the accepted P1-WP01 prompt, activation, source, review, baseline report,
    evidence index, handoff manifest, source snapshot, hashes, CI/native/manual
    evidence, pins, locks, support claims, deviations, and blockers named by
    activation
14. the accepted P2-WP01 prompt, activation, source, review,
    theme-foundation report, evidence index, handoff manifest, source snapshot,
    hashes, native evidence, token provenance, contrast matrix, material and
    title-bar facts, deviations, and blockers named by activation
15. the accepted P2-WP02 prompt, activation, source, review,
    responsive-shell report, evidence index, handoff manifest, source
    snapshot, hashes, native evidence, responsive bands, splitter/focus
    behavior, layout schema, title-bar facts, deviations, and blockers named by
    activation
16. the accepted corrected P3-WP01 prompt, activation, source, review,
    contracts/native-intent report, evidence index, handoff manifest, source
    snapshot, hashes, schema and operation identities, authorization policy,
    opaque-reference rules, capabilities, native evidence, deviations, and
    blockers named by activation
17. every separately accepted targeted architecture amendment named by
    activation
18. current shared schemas/fixtures/generated bindings, frontend task adapter
    and state, Rust transport/lifecycle/tasks/process modules, Python
    protocol/control runtime, sidecar build/resource paths, Tauri commands and
    channels, capabilities/permissions, CSP, fault fixtures, tests, workflows,
    toolchain/version files, manifests, lockfiles, and support claims
19. the exact activated revision of this execution prompt

In the first implementation progress message, state:

> `AGENTS.md, repository authority, accepted Phase 0A, accepted and merged
> WP01/P0B-WP02/P0B-WP03, P1-WP01, P2-WP01, P2-WP02, and P3-WP01 evidence,
> the final accepted Phase 0B closure report, the final accepted Phase 1
> baseline report, the final accepted P2-WP01 theme-foundation report, the
> final accepted P2-WP02 responsive-shell report, the final accepted P3-WP01
> contracts/native-intent report, and the activated GFD-P3-WP02 prompt have
> been read and are active. Executing only GFD-P3-WP02 with GPT-5.6 Sol /
> Extra High. GFD-P4-WP01 and product features remain unauthorized.`

If a required source is unavailable, unreadable, stale, or materially
inconsistent, stop before writing. Do not conduct a broad architecture review,
rerun predecessor packages as substitutes for accepted evidence, or improvise
new product, runtime, platform, process, packaging, or support claims.

## 4. Dependencies, predecessor outputs, and prerequisite evidence

**Direct prerequisite:** accepted and merged `GFD-P3-WP01` contracts and
native-intent boundary.

**Required retained lifecycle authority:** accepted and merged
`GFD-P0B-WP02` lifecycle implementation and accepted `GFD-P0B-WP03` closure
evidence.

**Inherited baseline:** accepted and merged WP01, P1-WP01, P2-WP01, and
P2-WP02 source and evidence.

**Direct dependent:** `GFD-P4-WP01`, which remains unauthorized.

P3-WP02 cannot activate while P3-WP01 is only an approved provisional prompt,
an unmerged implementation branch, or an unaccepted contracts/native-intent
report. It also cannot activate while the retained P0B-WP02 lifecycle or
P0B-WP03 closure evidence is absent, stale, unaccepted, or materially
conflicting.

Activation must supply exact:

- accepted merged commits and tree identities;
- implementation reports, evidence indexes, manifests, source snapshots, and
  SHA-256 digests;
- CI run/job/artifact identities and per-target native/manual evidence;
- task/request/trace/event schemas and operation-policy metadata;
- frame, queue, log, subscription, snapshot, timeout, restart, and circuit
  limits;
- process-containment APIs and zero-descendant evidence;
- sidecar build ID, schema hash, protocol range, operation list, resource
  layout, executable identity, package formats, and build commands;
- toolchain and dependency pins/locks;
- support matrix, engine/OS/architecture versions, manual owners, and
  unavailable evidence;
- measurements, deviations, fallbacks, amendments, and unresolved blockers.

Activation must reconcile P3-WP01 policy metadata with retained P0B-WP02
runtime behavior without creating a second registry or contract authority. It
must name the exact retained, moved, replaced, test-only, obsolete, and blocked
runtime pieces and one authoritative destination for each.

Acceptance of this prompt does not accept predecessor implementation by
inference, does not activate this package, and does not authorize P4-WP01.

## 5. Objective and measurable runnable outcome

Productize the accepted Phase 0B task/lifecycle evidence behind the accepted
P3-WP01 operation and native-intent boundary so later real feature consumers
have one bounded, deterministic, packaged task runtime.

The future runnable result must be a real packaged Tauri path proving:

```text
accepted P3-WP01 typed operation authorization
→ named product-neutral long-running task acceptance
→ Rust-owned request/task/trace identity and lifecycle state
→ exact bundled PyInstaller onedir sidecar launch
→ bounded UTF-8 JSON Lines framing, pending requests, events, and logs
→ ordered/coalesced progress through an authorized Tauri channel
→ deterministic cancel, timeout, crash, hang, restart, and circuit outcomes
→ queryable bounded task snapshot and safe frontend state
→ graceful shutdown or process-tree termination
→ zero automatic replay and zero surviving descendants
```

The smallest acceptable real path is a clearly labeled internal runtime
verification route, or activation-verified equivalent, integrated with the
accepted shell and P3-WP01 typed adapters. It must exercise one
product-neutral cancellable long-running operation and only the accepted
synthetic fault paths needed to prove lifecycle behavior.

It must not implement document analysis, product/domain output, Phase 4 UX, a
generic background-service framework, a public task SDK, or any real product
feature. Activation must refresh the exact finite operation IDs. Prefer
productized internal/test equivalents of the accepted Phase 0B count, crash,
hang, and oversize paths. Fault injection must remain test/evidence-gated and
absent from ordinary production capabilities.

## 6. Explicit in-scope work

The activated Work Session may perform only the smallest coherent change set
needed for the following items.

### 6.1 Productization rather than spike duplication

- Inventory accepted Phase 0B lifecycle source and evidence before editing.
- Classify each part as retained unchanged; moved or renamed; replaced by a
  narrower implementation; test-only fault infrastructure; obsolete and
  removed with evidence; or blocked pending an accepted amendment.
- Convert proven behavior into stable internal runtime ownership, named
  commands, bounded state, exact packaging paths, and focused regression/fault
  evidence.
- Remove or explicitly isolate obsolete spike-only runtime pieces.
- Leave exactly one authoritative productized runtime path. Parallel spike and
  product runtime implementations are prohibited.

### 6.2 Sidecar trust, launch, and exact bundled identity

- Keep Rust as sidecar lifecycle and process authority.
- Launch only the exact bundled first-party Python sidecar resource/executable
  resolved by Rust.
- Use no shell and accept no caller-supplied executable, script, argument
  vector, working directory, or environment.
- Use controlled fixed arguments, a deterministic working directory, a
  minimal constructed environment, explicit UTF-8/unbuffered protocol
  configuration, and piped stdin/stdout/stderr.
- Drain stdout and stderr concurrently and continuously, independent of task
  completion and webview speed.
- Apply target-specific process-tree containment.
- Do not inherit credentials, proxy variables, tokens, secrets, or unrelated
  environment entries.
- Validate expected build ID, target triple, protocol range, schema hash,
  operation list, bundle inventory, and hashes before readiness.
- Keep Python classified as trusted native code, not a sandbox. Tauri and
  PyInstaller do not sandbox Python.
- Reject source-tree Python, a system interpreter, PATH lookup, editable
  environments, shell commands, and developer-only sidecars as
  packaged-runtime evidence.

### 6.3 Packaging baseline and target ownership

- Use PyInstaller `onedir`, or the exact accepted successor named by an
  accepted targeted amendment, built separately on each accepted
  OS/architecture.
- Refresh the accepted resource directory, executable name, runtime-library
  inventory, package format, and build command at activation.
- Add only the per-target build/resource wiring needed by accepted consumers
  and tests.
- Keep target results separate; one platform cannot pass on another
  platform's runtime evidence.
- Exclude unsupported architectures, Linux distributions, package formats,
  stores, signing, notarization, updater, publication, and release channels.
- Do not convert to `onefile` without an accepted targeted amendment and
  measured startup, extraction, and security evidence.
- Do not claim production release readiness.
- Keep fault controls, Webdriver/test commands, and evidence writers out of
  ordinary production capabilities and artifacts.

### 6.4 Bounded transport and framing

- Use UTF-8 JSON Lines over stdin/stdout unless activation names an accepted
  replacement.
- Preserve or activation-refresh these accepted baselines:
  - handshake frame maximum: `64 KiB`;
  - normal frame maximum: `1 MiB`;
  - structured log-line maximum: `64 KiB`, followed by deterministic
    truncation or rejection with a marker;
  - bounded pending requests: initial accepted limit `64`;
  - bounded backend event queue: initial accepted limit `256`;
  - UI progress delivery: at most `10 updates/second/task`;
  - one JSON value per line;
  - protocol-only stdout;
  - structured bounded logs only on stderr;
  - continuous draining independent of task completion.
- Handle invalid UTF-8, malformed JSON, oversized frames, unknown kinds,
  unknown operations, duplicate identifiers, impossible transitions, queue
  exhaustion, slow consumers, closed channels, and stream corruption
  deterministically.
- Terminate a corrupted sidecar instance, fail affected work safely, and
  follow the bounded restart/circuit policy.
- Never allocate from an untrusted declared size without enforcing the actual
  cap.

### 6.5 Request, task, trace, and event contracts

Consume the accepted P3-WP01 schema bundle and operation registry. Preserve
distinct identifiers:

- `requestId`: one invocation;
- `taskId`: minted only after long-running work is accepted;
- `traceId`: minted authoritatively by Rust and propagated through safe logs
  and Python.

Require:

- one synchronous terminal result, or one `accepted` result followed by
  exactly one terminal task result, for each request;
- a monotonically increasing per-task event sequence;
- rejection and safe recording of duplicate, decreasing, or post-terminal
  events;
- progress coalescing when necessary, while terminal events are never
  coalesced, dropped, or reordered;
- Rust-owned authoritative current snapshots and terminal decisions;
- queryable terminal state despite navigation, remount, or a slow webview;
- protection against an old subscription mutating a newer task instance;
- bounded path-, payload-, and secret-free task/trace metadata.

No frontend-provided task ID, trace ID, sequence, terminal state, or backend
process ID is authoritative.

### 6.6 Task states and deterministic terminal races

Use only these version-1 task states:

- `Queued`
- `Running`
- `Cancelling`
- `Succeeded`
- `Failed`
- `Cancelled`
- `TimedOut`
- `Interrupted`

`Paused`, `Resuming`, durable pending, and replay states are prohibited.

Define an executable transition table and tests proving:

- cancellation acknowledgement means the cancel request was accepted, not
  that work already stopped or rolled back;
- a success accepted by Rust wins over a later cancellation race;
- only the first valid terminal state wins;
- timeout requests cancellation and may escalate, but does not claim rollback;
- unexpected sidecar exit marks affected in-flight work `Interrupted`;
- restart restores service availability only and never replays uncertain work;
- shutdown interruption is explicit and is not relabeled as cancellation or
  success;
- terminal state is retained only within a bounded authorized history/snapshot
  policy.

Every race rule must exist in source and tests, not only narrative.

### 6.7 Concurrency, queues, and backpressure

- Permit one active long-running sidecar task at a time unless activation
  names an accepted evidence-backed amendment.
- Permit short status/read operations to coexist only while the Python control
  loop remains responsive.
- Do not add multiple Python workers, a generic worker pool, per-task
  processes, or a parallel long-task scheduler.
- Explicitly bound pending requests, accepted tasks, event queues,
  subscriptions, logs, and retained snapshots.
- Return a stable safe admission failure rather than hidden waiting or
  unbounded allocation.
- Coalesce progress before crossing to the webview.
- Give terminal events priority over progress.
- Prevent slow/disconnected consumers from blocking sidecar drains or Rust
  lifecycle control.
- Expose truthful saturation and queue high-water evidence.
- Use accepted capacities or name and justify an accepted targeted change; do
  not silently tune architectural limits.

### 6.8 Cancellation and timeout model

- Make Python read control messages continuously and independently from
  long-running task computation.
- Require cooperative operations to check cancellation at documented bounded
  checkpoints.
- Map cancellation only for operations P3-WP01 marks cancellable.
- Enforce accepted per-operation deadlines in Rust.
- Route timeout through the accepted cancellation and escalation path.
- Measure cancellation acknowledgement against the architecture-time goal
  `≤ 250 ms`.
- Measure cooperative stop against the architecture-time goal `≤ 2 s`.
- If work does not stop by the cancellation deadline, permit Rust to terminate
  and restart the sidecar under the accepted policy and mark affected work
  `Interrupted`.
- Never imply rollback for destructive or write operations without a specific
  transactional guarantee.
- Never retry automatically unless P3-WP01 marks an operation idempotent and
  the accepted caller policy permits it.
- Never automatically replay uncertain or side-effecting work after
  escalation, crash, or restart.
- Retain raw values and exact classification for hardware-sensitive misses.

### 6.9 Sidecar lifecycle, restart, and circuit

Refresh the accepted lifecycle from final Phase 0B evidence:

```text
Stopped
  → Starting
  → Ready
  → Busy
  → Ready

Starting → Faulted
Ready/Busy → Restarting
Restarting → Ready
Restarting → Faulted
Ready/Busy → Stopping
Stopping → Stopped
```

Preserve:

- lazy start unless accepted product evidence changes it;
- bounded startup and handshake deadlines;
- safe failure of all in-flight work on unexpected exit;
- one bounded restart after an accepted short backoff;
- zero automatic replay;
- a circuit requiring explicit user action after repeated failures within the
  accepted window;
- the architecture-time baseline that repeated failures within `60 seconds`
  open the circuit, subject to activation refresh;
- authorization, version, queue, and circuit enforcement on manual retry;
- bounded restart counters and React-safe status;
- no infinite restart loop and no background daemon behavior.

Add finite test-only fault injection for invalid/late handshake, startup
failure, deliberate crash, deliberate hang, repeated crash, and
restart-budget exhaustion, then prove its absence from production.

### 6.10 Process-tree containment and shutdown

- Use Windows Job Object kill-on-close or the accepted equivalent on Windows.
- Use the accepted Unix process group/session and parent-death behavior on
  macOS and Linux.
- Attempt bounded graceful shutdown, then terminate the full process tree.
- Continue draining stdout/stderr throughout shutdown.
- Cover normal close, active-task close, cancellation escalation, crash, hang,
  and forced host termination.
- Prove zero surviving sidecar children or descendants after the declared
  observation window.
- Record the enumeration method, sanitized parent/child identities, deadlines,
  raw results, and platform limitations.

A host exit with surviving descendants fails the gate. Killing only the direct
child is insufficient evidence.

### 6.11 Rust-to-UI task streaming and frontend state

Use a named Tauri channel or the exact accepted ordered Rust-managed
subscription:

- deliver only to authorized/requesting windows;
- use one central typed frontend adapter consuming P3-WP01 contracts;
- coalesce progress before webview delivery;
- prioritize terminal events and expose a queryable Rust-owned snapshot;
- bound frontend task state and retained history;
- clean up subscriptions deterministically on route/window close;
- do not store high-frequency progress as TanStack Query data;
- do not create duplicate Zustand/query/local-state authorities;
- retain active/terminal state across route navigation and remount;
- expose accessible status, progress, cancel, timeout, interrupted,
  backend-unavailable, and circuit-open states;
- throttle screen-reader progress announcements and prioritize terminal
  announcements;
- preserve Phase 2 reduced-motion, forced-colors, scaling, keyboard, focus, and
  compact-shell behavior.

The UI is a product-neutral runtime verification surface only, never a real
feature or Phase 4 document-analysis flow.

### 6.12 Structured logs, traces, and redaction

- Propagate Rust-minted `traceId`.
- Log only safe event kind, component, state transition, duration, result
  code, and bounded context.
- Truncate oversized lines deterministically with a marker.
- Preserve stdout protocol and stderr log separation.
- Bound log queues and prevent logging from blocking lifecycle control.
- Exclude request/response payloads, document contents, native paths, secrets,
  credentials, environment dumps, raw stderr exposure, unrestricted Python
  traceback, and opaque-reference backing values.
- Record crash/hang/restart/circuit outcomes without personal paths or user
  data.
- Add no telemetry, remote upload, crash-upload endpoint, diagnostics export,
  or user-facing collection/repair surface.

### 6.13 Security, capabilities, and operation authority

- Keep React least trusted.
- Keep Rust authoritative for operation, lifecycle, process, file intent,
  capability, and native policy.
- Give Python only already-authorized typed operations; Python cannot choose
  its executable/runtime path or launch arbitrary processes.
- Expose only finite named typed least-privilege start, cancel, snapshot, and
  restart commands.
- Enforce P3-WP01 risk, timeout, cancellability, idempotency, payload/ref, and
  window metadata as executable policy.
- Reject unknown, unauthorized, malformed, oversized, stale-version, and
  wrong-window requests before Python.
- Add no generic process, shell, command, method, script, environment, raw
  protocol, queue, log, path, or payload tunnel.
- Keep network denied by default and release CSP intact.
- Exclude fault injectors, evidence writers, Webdriver/test drivers, debug
  restart bypasses, and test-only commands from production capabilities and
  packages.
- Inspect final production source, capabilities, and binaries; tests alone do
  not prove exclusion.

### 6.14 Data ownership, artifacts, and no durable tasks

- Enforce inline frame limits.
- Mediate binary or multi-megabyte values through accepted opaque artifacts.
- Give task/runtime temporary files one named owner with bounded size/count and
  deterministic cleanup.
- Add no durable queue, journal, resume token, replay log, checkpoint store, or
  product database.
- Never resume or replay interrupted tasks on application restart.
- Use the accepted shell to warn or request confirmation on active-task
  shutdown where appropriate.
- Let sidecar restart restore readiness only.
- Add no second writer for settings, artifacts, cache, logs, or future domain
  data.
- Keep synthetic fault fixtures free of user files and sensitive data.

### 6.15 Cross-platform and packaging truth

Classify each accepted target separately, with activation-refreshed versions:

- Windows 11 x64 / WebView2 / accepted Windows package candidate;
- current and previous declared macOS major versions on arm64 / WKWebView /
  accepted app/package candidate;
- Ubuntu 24.04 x64 / WebKitGTK / `.deb`, with Wayland and X11 separated where
  actual evidence exists.

Separate source build, sidecar build, host package creation, installed/packaged
runtime, automated native journey, process-containment evidence, manual
platform behavior, and unavailable/blocked evidence. Never infer runtime from
a build, installed behavior from a package, descendant cleanup from direct
child exit, or one platform from another.

### 6.16 Performance and reliability measurements

Retain raw measurements, method, sample count, runner/hardware, target,
accepted predecessor baseline, and classification for:

- sidecar cold and warm start/handshake;
- task acceptance latency;
- progress production rate, coalesced webview rate, and sequence gaps;
- cancellation acknowledgement;
- cooperative stop and forced escalation;
- crash detection and in-flight interruption;
- restart backoff and return to readiness;
- repeated-failure circuit opening;
- graceful shutdown and forced process-tree cleanup;
- pending-request, event, and log queue high-water marks;
- host and sidecar idle/active memory;
- package and bundled-sidecar size deltas;
- zero descendants after normal and forced close.

The architecture-time goals `≤ 3 seconds` cold handshake,
`≤ 10 progress updates/second/task`, `≤ 250 ms` cancellation acknowledgement,
`≤ 2 seconds` cooperative stop, bounded frame/queue growth, and zero orphans
are measured initial goals, not fabricated universal service levels.

### 6.17 Focused documentation and evidence

- Document the final authoritative runtime ownership, task/race model,
  packaging identity, process containment, limits, commands, and known
  constraints.
- Add proportional local and CI evidence for the activated target matrix.
- Record exact command/method, exit status, runner, hardware,
  OS/architecture, engine, tool, commit, artifact, duration, result, and
  limitation for every claim.
- Keep schema, Rust transport, task/race, cancellation, lifecycle, containment,
  Python, frontend, logging, packaging, native, engine, CI, manual, security,
  documentation, and unavailable evidence layers separate.

## 7. Explicit exclusions and prohibited adjacent work

Do not:

- activate, repair, reimplement, or rerun predecessors as substitute work;
- change P3-WP01 schema, operation, native-intent, safe-error, opaque-reference,
  capability, or single-writer authority except for one exact demonstrated
  defect named by an accepted targeted correction;
- implement `GFD-P4-WP01`, document analysis, search, domain output, product
  workflow, account, database, or real feature UI;
- add automatic replay, durable resume, durable tasks, task journals,
  checkpointing, uncertain-work retry, or background synchronization;
- add multiple Python workers, worker pools, per-task child workers, GPU/model
  orchestration, distributed execution, or arbitrary subprocess launch;
- add runtime plugins, dynamic operation registration, a public task/module
  SDK, generic job framework, extension marketplace, or remote service API;
- expose generic shell, process, filesystem, window, environment, secret,
  backend, protocol, queue, log, operation, method, path, or payload access;
- broaden file-intent, artifact, settings, migration, single-instance,
  diagnostics, repair, recovery-export, telemetry, crash upload, cloud sync,
  or enterprise-support work;
- redesign the shell, theme, title bar, responsive system, or public UI
  architecture;
- create a public `packages/ui` or unrelated UI;
- weaken CSP, load remote content, add broad Tauri permissions, log
  paths/payloads/secrets, or ship production test controls;
- add unsupported platforms, architectures, Linux distributions, package
  formats, signing, notarization, updater, publication, store, release, or
  deployment claims;
- perform opportunistic dependency upgrades, formatting sweeps, broad
  refactors, or repository cleanup;
- author or modify another work-package prompt;
- open a PR, enable auto-merge, merge, delete branches, release, publish, or
  deploy.

A demonstrated predecessor defect, missing accepted runtime decision, or
required architecture change is a stop condition unless Chat Session has
separately accepted and activation names one exact focused correction.

## 8. Allowed repository areas and expected changes

Activation must replace this provisional list with an exact path inventory
from the accepted merged tree. Expected allowed areas are no broader than:

```text
packages/app-contracts/schemas/             # task events/runtime status only when consumed
packages/app-contracts/fixtures/            # shared lifecycle/fault fixtures
apps/desktop/src/backend/                    # typed task/runtime adapter
apps/desktop/src/state/ or accepted path     # bounded task-facing UI state
apps/desktop/src/routes/ or shell path       # product-neutral runtime verification only
apps/desktop/src-tauri/src/backend/          # sidecar client, transport, lifecycle
apps/desktop/src-tauri/src/tasks/            # task state, sequencing, cancellation, snapshots
apps/desktop/src-tauri/src/process/          # target containment only if current ownership
apps/desktop/src-tauri/src/commands/         # exact named runtime commands
apps/desktop/src-tauri/capabilities/         # least-privilege exact commands
apps/desktop/src-tauri/permissions/          # least-privilege exact commands
services/python-backend/                     # protocol loop, typed task/control runtime
scripts/build-sidecar/                       # exact per-target onedir build/resource assembly
scripts/verify/                              # bounded runtime/package/process checks only
tests/e2e/ and tests/platform-smoke/         # focused accepted native/runtime evidence
.github/workflows/                           # minimal P3-WP02 target evidence when activated
package manifests, lockfiles, build configs  # mechanically required changes only
focused runtime documentation                # operation/lifecycle/package usage and limits
```

Activation must substitute actual accepted paths, remove unused entries,
identify retained Phase 0B source instead of duplicating it, and list every
allowed and protected path exactly.

Protected by default:

- architecture authority, Stage 1 files, accepted decisions, and predecessor
  reports/evidence/snapshots;
- accepted P3-WP01 schemas, operation/native-intent policy, opaque-reference
  rules, safe errors, and single-writer ownership except for an explicitly
  accepted focused correction;
- unrelated routes, product features, shell regions, theme, title-bar,
  responsive layout, settings, diagnostics, and data ownership;
- release, signing, notarization, updater, publication, deployment, and
  repository settings;
- all work-package prompts.

Every changed path must be listed in the implementation report with purpose
and authority. A path outside the activated allowlist is a stop condition
unless it is a mechanically required generated or locked companion explicitly
authorized. Generated caches, build products, packages/installers, local
environments, native captures, secrets, user content, and temporary files must
not be committed.

## 9. Ordered implementation procedure

After activation, execute exactly this order:

1. Complete all mandatory reads and the authority, repository, activation,
   clean-tree, branch, path, capability, target, and predecessor preflight at
   the exact activated `main`.
2. Create exactly one implementation `RUN_ID` after reads/preflight and before
   any repository or deliverable write; resolve all four deliverable names and
   stop on any collision.
3. Build a prerequisite ledger of accepted commits, reports, source snapshots,
   schema/operation identities, lifecycle limits, process-containment results,
   evidence, measurements, deviations, fallbacks, amendments, and blockers.
4. Run the relevant accepted predecessor contract, lifecycle, process,
   packaging, native, CSP/capability, and no-replay baselines before editing.
5. Inventory spike/product runtime duplication, task schemas/events, frontend
   state, Rust transport/lifecycle/tasks/process modules, Python
   protocol/control loop, sidecar build/resources, fault fixtures,
   capabilities, workflows, and logs.
6. Produce a minimal productization plan mapping every retained, moved,
   replaced, removed, test-only, and blocked component to one authoritative
   final runtime path.
7. Reconcile task/request/trace/event schemas and runtime status with accepted
   P3-WP01 contract authority and drift checks.
8. Implement or consolidate bounded framing, pending requests, event/log
   queues, stream validation, backpressure, and continuous drains.
9. Implement or consolidate task acceptance, sequence validation, snapshots,
   terminal precedence, cancellation, timeout, and no-retry/no-replay policy.
10. Implement or consolidate sidecar lifecycle, one bounded restart, circuit,
    explicit user retry, startup/handshake validation, and safe status.
11. Implement target-specific process-tree containment and bounded
    graceful/forced shutdown without orphan descendants.
12. Implement the exact packaged `onedir` sidecar build/resource resolution
    for accepted targets, excluding source/developer fallbacks from acceptance.
13. Implement the central typed Tauri channel/frontend task adapter, bounded UI
    state, accessibility, controlled announcements, and product-neutral
    verification surface.
14. Add structured trace-correlated redacted logs without diagnostics export
    or remote telemetry.
15. Add finite test-only fault injection and prove its absence from production
    commands, capabilities, and artifacts.
16. Add proportional schema, unit, integration, fault, package, native, and
    platform tests plus exact negative checks.
17. Run fast contract/TypeScript/Rust/Python checks continuously, followed by
    source, sidecar, package, installed/native, containment, and accepted
    platform jobs.
18. Measure and retain raw startup, progress, cancellation, crash/restart,
    circuit, queue, memory, package, and cleanup results.
19. Inspect final source and production artifacts for duplicate runtimes,
    generic commands, broad permissions, test controls, system-Python
    fallbacks, inherited secrets, paths/payloads, remote content,
    replay/durable state, and unsupported target claims.
20. Run relevant WP01/P0B-WP02/P0B-WP03/P1/P2/P3-WP01 regressions and
    classify every unavailable/manual result truthfully.
21. Reconcile the final diff to the activated inventory and remove
    speculative, duplicate, obsolete, or unused work.
22. Commit one coherent P3-WP02 implementation, push only the activated branch,
    and leave `main` unchanged without opening a PR.
23. Build the complete source snapshot from the final commit; verify SHA-256,
    CRC, path safety, one expected root, and exact inventory; extract into a
    fresh empty directory; and rerun the complete activated verification.
24. Finalize exactly four deliverables, verify ordinary hashes and any
    canonical manifest self-hash, return the section 18 handoff, and stop
    without entering P4-WP01.

Never use a Phase 4 feature, additional worker, durable queue, generic process
framework, diagnostics surface, or release change as a workaround for a
failed runtime gate.

## 10. Cross-cutting constraints

### Architecture and authority

- React owns presentation and user intent only.
- Rust owns operation authorization, task IDs and terminal decisions,
  lifecycle/process control, native policy, capabilities, bounded queues,
  snapshots, sidecar launch, and persistent writes.
- Python owns only typed authorized computation and cooperative task control.
- Accepted P3-WP01 JSON Schema and finite operation metadata remain canonical.
- Do not create a second operation registry, state authority, writer, or
  runtime path.

### Bounds and protocol

- Enforce actual byte/count/time bounds before allocation or admission.
- Preserve protocol-only stdout and structured bounded stderr.
- Continuously drain both streams independent of task completion and consumer
  speed.
- Bound frames, pending requests, tasks, queues, logs, subscriptions,
  snapshots, history, temporary files, and retained metadata.
- Treat saturation, corruption, closed channels, duplicates, invalid
  transitions, and post-terminal events as explicit safe outcomes.

### Task and failure semantics

- Rust mints authoritative request correlations, task IDs, trace IDs,
  sequences, snapshots, and terminal state.
- First valid terminal wins; accepted success beats later cancel.
- Cancellation acknowledgement is not completion or rollback.
- Timeout may cancel and escalate but never implies rollback.
- Crash, hang, forced termination, or restart never replays uncertain work.
- Restart restores availability only.
- One active long task is the baseline; short control/status work may remain
  responsive without adding workers.

### Least privilege and production exclusion

- Use finite named typed commands and authorized window delivery only.
- Keep network denied and release CSP least privilege.
- Exclude generic process/protocol/operation/path/payload tunnels.
- Construct the sidecar environment explicitly and omit credentials, proxies,
  tokens, and unrelated variables.
- Prove production packages omit fault injection, evidence writers, drivers,
  developer-sidecar fallbacks, and debug bypasses.

### UI and accessibility

- Use one typed frontend adapter and one task-state authority.
- Bound state/history and clean subscriptions on route/window close.
- Preserve terminal state across navigation/remount through Rust snapshots.
- Throttle progress and screen-reader announcements; prioritize terminal
  events.
- Preserve Phase 2 semantic theme, forced colors, reduced motion/transparency,
  zoom, responsive bands, keyboard, focus, splitter, portals, and title-bar
  fallbacks.
- Keep the route clearly product-neutral and internal.

### Platform and evidence truth

- Refresh exact OS/architecture/engine/package identities at activation.
- Keep source, build, package, installed runtime, engine, native, process,
  manual, CI, and unavailable evidence separate.
- Record raw measurements and limitations without inferring one platform or
  evidence class from another.
- No source build, mock, harness, screenshot, package file, or prior spike
  result substitutes for current installed/native runtime and containment
  evidence.

### Reproducibility and hygiene

- Use activated pins, lockfiles, clean checkouts, deterministic fixtures, and
  exact commands.
- Keep payloads, user content, paths, secrets, environment dumps, raw
  exceptions, and opaque backing values out of logs and artifacts.
- Build the final snapshot from the reviewed commit and prove a clean
  extraction rerun.
- Do not make opportunistic upgrades, broad refactors, formatting sweeps, or
  unrelated cleanup.

## 11. Proportional tests and exact evidence

Activation must replace command families with exact current commands, working
directories, tool versions, target identities, environment controls, expected
results, retained artifacts, and manual owners.

### Required automated evidence

Collect proportional evidence for:

- canonical task/runtime schemas, generated types, fixture parity, and drift;
- request/task/trace identifiers and accepted/terminal envelopes;
- the task transition table and invalid-transition rejection;
- monotonic sequences, duplicate/post-terminal rejection, progress
  coalescing, and terminal priority;
- bounded frame, request, event, subscription, snapshot, history, and log
  queues;
- slow-consumer and closed-channel behavior;
- malformed UTF-8/JSON, oversized frames/logs, unknown kind/operation,
  duplicate IDs, and stream corruption;
- one active long task, responsive allowed short operations, and explicit
  admission saturation;
- cancellation acknowledgement, cooperative stop, timeout, and forced
  escalation;
- cancel/timeout/success/crash/shutdown race matrices;
- invalid/late handshake, startup failure, crash, hang, one restart, repeated
  failure, circuit, explicit retry, and restart-budget exhaustion;
- no replay after crash/restart and no automatic retry of non-idempotent work;
- exact bundled sidecar resolution, target/build/schema/operation handshake,
  no shell, minimal environment, and no system-Python/PATH fallback;
- continuous stdout/stderr drains and protocol/log separation;
- log redaction/truncation and trace correlation;
- process-tree containment and zero descendants after normal, active,
  cancellation-escalated, crash/hang, and forced close;
- typed frontend channel/state/snapshot integration, navigation retention,
  cleanup, accessibility, focus, announcements, compact shell, forced colors,
  reduced motion, and scaling;
- production exclusion of fault injectors, test drivers, evidence writers,
  developer sidecars, generic commands, remote content, broad permissions,
  paths/payloads, and secrets;
- source-snapshot fresh extraction and complete rerun.

### Required real runnable evidence

According to the accepted support matrix, require:

- source frontend and locked Rust/Python checks;
- per-target sidecar build and host package creation;
- installed or packaged native Tauri launch resolving the bundled sidecar;
- a valid handshake and safe backend-ready state;
- one product-neutral long task with ordered bounded progress;
- real cancel acknowledgement and cooperative or escalated stop;
- deliberate crash and interruption without replay;
- deliberate hang and bounded escalation;
- one valid restart and repeated-failure circuit;
- explicit authorized recovery from circuit;
- navigation/remount without task-state or terminal loss;
- safe unavailable, faulted, and circuit-open UI states;
- orderly and forced close with zero descendants;
- target-specific resource, package, and architecture identity;
- accepted WebView2, WKWebView, and WebKitGTK evidence where available;
- truthful native/manual/process evidence separation.

A browser mock, process harness, unit test, source build, package artifact,
screenshot, documentation statement, or prior spike result cannot be
presented as current installed/native packaged-sidecar, containment,
cancellation, restart, circuit, or manual platform evidence.

### Required focused scenarios

At minimum, cover:

1. cold start and valid handshake;
2. warm start or already-ready status;
3. invalid, late, mismatched, or oversized handshake;
4. synchronous short operation while ready;
5. one long task accepted with unique request/task/trace IDs;
6. ordered progress with coalescing and terminal priority;
7. duplicate, decreasing, and post-terminal sequence rejection;
8. navigation/remount and late subscription;
9. one-active-long-task admission and saturation;
10. slow or disconnected webview consumer;
11. pending-request and event-queue saturation;
12. malformed UTF-8/JSON and oversized frames/logs;
13. unknown kind/operation and invalid backend response;
14. cancellation before start, during running, near success, and after terminal;
15. timeout with cooperative stop;
16. timeout/cancel escalation to termination;
17. deliberate crash during active work;
18. deliberate hang during active work;
19. one bounded restart with no replay;
20. repeated failure opening the circuit;
21. explicit retry after circuit according to accepted policy;
22. application close with no task;
23. close with active task and confirmation behavior;
24. forced host close and descendant cleanup;
25. log redaction, truncation, and stdout/stderr separation;
26. wrong sidecar architecture/build/schema/operation identity;
27. missing or corrupt bundled sidecar;
28. production exclusion of fault/test/developer surfaces;
29. source-snapshot extraction and rerun.

### Evidence classification

Keep these layers separate:

1. schema/fixture/generated-contract evidence;
2. Rust transport/framing/queue evidence;
3. task state/sequence/race evidence;
4. cancellation/timeout/escalation evidence;
5. sidecar lifecycle/restart/circuit evidence;
6. process-tree containment/shutdown evidence;
7. Python protocol/control/runtime evidence;
8. frontend channel/state/accessibility evidence;
9. structured logging/redaction evidence;
10. sidecar build/resource/package evidence;
11. source build and host package-creation evidence;
12. real installed/native packaged-runtime evidence;
13. WebView2/WKWebView/WebKitGTK evidence;
14. CI workflow/run/job/artifact evidence;
15. manual native/process/accessibility evidence;
16. security/capability/production-exclusion evidence;
17. signing/notarization/update evidence;
18. documentation accuracy evidence;
19. unavailable or blocked evidence.

For every check, retain exact command/method, exit status, expected and actual
behavior, commit, OS/version/architecture, engine, tool, runner/hardware,
duration where relevant, artifact/log, limitation, and classification.

The review evidence index must map every acceptance gate to exact source and
tests, command and exit status, CI run/job/artifact, native/manual artifact,
measurement, classification, limitation, and external deliverable digest.

## 12. Measurable acceptance gates

All gates are future implementation requirements. None is satisfied by this
provisional document.

1. Exact accepted and merged WP01, P0B-WP02, P0B-WP03, P1-WP01, P2-WP01,
   P2-WP02, and P3-WP01 predecessors plus accepted reports/evidence were used.
2. The final diff stays within the activated path allowlist and contains no
   P4-WP01, product feature, public SDK, release, prompt-pack, or unrelated
   work.
3. One authoritative productized task/runtime path remains; duplicate spike
   and product runtimes do not coexist.
4. Accepted P3-WP01 schemas, operation registry, authorization, safe errors,
   opaque references, and capability boundaries remain authoritative and
   drift-free.
5. The exact bundled target-matched sidecar launches with no shell, controlled
   arguments, working directory and environment, continuous drains, and valid
   handshake identity.
6. Framing, pending requests, event queues, logs, subscriptions, snapshots,
   and retained task state are bounded and saturation fails safely.
7. Malformed, oversized, unknown, or corrupt protocol input fails
   deterministically without unbounded allocation or protocol/log mixing.
8. Request/task/trace IDs remain distinct, Rust-authoritative, bounded, and
   trace-correlated without sensitive data.
9. Each accepted long-running request produces one accepted result and exactly
   one valid terminal task result with a monotonic sequence.
10. Progress may coalesce, but terminal events are never lost, reordered, or
    overwritten; slow consumers cannot block lifecycle control.
11. Only one active long task exists under the accepted model, allowed short
    operations remain responsive, and admission saturation is explicit.
12. Task transitions and cancel/timeout/success/crash/shutdown races follow the
    executable accepted precedence table.
13. Cancellation acknowledgement and cooperative-stop behavior are measured;
    escalation is bounded and never claims rollback.
14. Crash and hang fault paths interrupt affected work safely, do not replay
    it, and do not leave uncertain terminal state.
15. One bounded restart restores availability only; repeated failures open the
    circuit and require explicit user action without an infinite loop.
16. No non-idempotent or uncertain work is automatically retried or replayed.
17. Process-tree containment and shutdown prove zero surviving descendants on
    every claimed target and declared close/fault path.
18. Frontend task state uses the typed central adapter, keeps bounded
    snapshots, survives navigation/remount, cleans subscriptions, and presents
    accessible deterministic states.
19. Structured logs are bounded, trace-correlated, stdout/stderr separated, and
    contain no payloads, native paths, secrets, raw exceptions, or user data.
20. Production source, capabilities, and packages exclude fault injectors,
    test drivers, evidence writers, generic commands, developer/system-Python
    fallbacks, remote content, broad permissions, and debug bypasses.
21. Per-target sidecar/host build, package, installed/native runtime,
    containment, engine, CI, and manual evidence remain separate and truthful.
22. Raw startup, progress, cancellation, crash, restart, circuit, queue,
    memory, package, and cleanup measurements and deviations are retained
    without fabricated service-level claims.
23. Accepted predecessor behavior shows no relevant regression, including
    Unicode, CSP, operation authorization, file-intent secrecy, shell, theme,
    title-bar fallback, and zero-descendant behavior.
24. No durable task/resume/journal, automatic replay, multiple worker,
    GPU/model, diagnostics export, telemetry, product database, generic service
    framework, or Phase 4 work entered the implementation.
25. Exactly four implementation deliverables use one task-start `RUN_ID`, have
    verified hashes, contain no sensitive data, and represent the reviewed
    commit.
26. The source snapshot contains one safe expected root, passes CRC/path
    checks, extracts into a fresh empty directory, and passes the complete
    rerun.
27. The implementation branch matches its remote, `main` is unchanged, no PR
    or auto-merge exists, and the handoff stops at Chat Session review.

If a gate is not met, report `Blocked`, `Not run`, `Partial`, or the exact
accepted limitation as an evidence outcome. Do not mark implementation
successful by inference.

## 13. Stop conditions and blocker reporting

Stop before writing if:

- the activation block is absent, provisional, stale, incomplete, or
  authorizes more than `GFD-P3-WP02`;
- repository, base, branch, clean-tree, path, or permission state differs;
- P3-WP01 is not accepted and merged;
- retained P0B-WP02 lifecycle or P0B-WP03 closure evidence is missing, stale,
  unaccepted, or materially conflicting;
- required predecessor reports, snapshots, hashes, CI/native/manual evidence,
  deviations, amendments, or support claims are unavailable or inconsistent;
- exact schemas, operation IDs, task/race rules, frame/queue limits, process
  APIs, sidecar paths, target matrix, tools, or manual owners cannot be
  determined safely;
- accepted spike evidence and current source disagree and one authoritative
  runtime cannot be selected without an architecture decision;
- P3-WP01 authorization, file-intent, safe-error, schema, capability, or
  single-writer boundaries would need to weaken;
- deterministic terminal precedence, bounded queues, continuous drains,
  cancellation, escalation, restart/circuit, no replay, or containment cannot
  be proven;
- real packaged/native sidecar or descendant-process evidence is unavailable
  and would have to be silently passed;
- production exclusion of fault/test/developer/generic surfaces cannot be
  proven;
- the work requires multiple workers, durable tasks, replay, GPU/model
  orchestration, a product feature, generic service, diagnostics, unsupported
  target, signing/updater/release, credentials, or architecture amendment;
- CI is blocked by policy, permission, quota, billing, runner, architecture,
  engine, package tool, credential, or artifact retention;
- a target result is unavailable and another target would have to substitute;
- source-snapshot integrity or fresh-extraction rerun fails;
- unauthorized or unrelated files enter the diff;
- a secret, native path, user payload, credential, or sensitive data could
  enter source, logs, evidence, or artifacts;
- remote state changes, push is non-fast-forward, or history rewriting would
  be required.

When blocked:

1. stop at the last clean, non-destructive state;
2. do not broaden scope or weaken a gate;
3. record the exact failed fact, command, path, expected value, observed value,
   affected gates, changed files, and unchanged exclusions;
4. distinguish repository defect, stale activation, missing authority/evidence,
   environment limitation, permission failure, platform limitation, or design
   decision;
5. report `Blocked` or `Partially implemented` using the approved
   implementation vocabulary; use `Partial` only for an explicitly identified
   evidence outcome;
6. return the smallest safe Chat Session decision, focused correction, or
   refreshed activation required to continue.

Do not enter P4-WP01 or broaden the runtime as a workaround.

## 14. Required functional and status inventory

Before activation, the truthful inventory is:

```text
Stage 1 foundation: Implemented and accepted
GFD-P0B-WP02 provisional prompt: Implemented and accepted as Approved provisional
GFD-P0B-WP02 activation/implementation: Not started
GFD-P0B-WP03 provisional prompt: Implemented and accepted as Approved provisional
GFD-P0B-WP03 activation/implementation: Not started
GFD-P1-WP01 provisional prompt: Implemented and accepted as Approved provisional
GFD-P1-WP01 activation/implementation: Not started
GFD-P2-WP01 provisional prompt: Implemented and accepted as Approved provisional
GFD-P2-WP01 activation/implementation: Not started
GFD-P2-WP02 provisional prompt: Implemented and accepted as Approved provisional
GFD-P2-WP02 activation/implementation: Not started
GFD-P3-WP01 provisional prompt: Implemented and accepted as Approved provisional
GFD-P3-WP01 activation/implementation: Not started
GFD-P3-WP02 package outcome: Not started
Productized authoritative task/runtime path: Not started
Accepted P3-WP01 task contracts integration: Not started
Bundled target-matched sidecar launch: Not started
Bounded framing and stream validation: Not started
Bounded pending requests and event/log queues: Not started
Request/task/trace identity and event sequencing: Not started
Task state machine and terminal precedence: Not started
One-long-task concurrency and backpressure: Not started
Cancellation and timeout escalation: Not started
Crash/hang interruption and no replay: Not started
Bounded restart and circuit behavior: Not started
Process-tree containment and zero-descendant shutdown: Not started
Typed Tauri channel and frontend task state: Not started
Structured trace logs and redaction: Not started
Per-target sidecar build/resource packaging: Not started
Production test/fault/developer-surface exclusion: Not started
Cross-platform native/process evidence: Not started
Performance and reliability measurements: Not started
Supporting infrastructure: Not started
Tests: Not started
Documentation for implementation: Not started
Generated code: Not started
Fixtures/mocks: Not started
Stubs/placeholders: Not started
Incomplete work: Not started
Blocked work: Blocked — P3-WP01 is not accepted and merged and P3-WP02 has no activated prompt
GFD-P4-WP01 and later work: Not started
Product features: Not started
Public SDK or plugin system: Not started
PR creation: Not started
Merge: Not started
Branch deletion: Not started
```

At future completion, state every line independently and use only these
implementation-status values:

- `Implemented`
- `Partially implemented`
- `Stub`
- `Mock-only`
- `Not started`
- `Blocked`

Keep real functionality, supporting infrastructure, tests, documentation,
generated output, fixtures/mocks, stubs/placeholders, incomplete work, and
blocked work separate. `Partially implemented` and `Blocked` require a concise
reason and evidence reference. Evidence outcomes may use only `Passed`,
`Failed`, `Partial`, `Blocked`, or `Not run` when clearly labeled as evidence
rather than implementation status.

## 15. One `RUN_ID` and collision-resistant external naming

At the start of the future implementation task, after mandatory authority
reads and preflight but before any repository or deliverable write, create
exactly one UTC run ID in basic ISO-8601 form:

```text
YYYYMMDDTHHMMSSZ
```

Resolve all four target filenames immediately from that one value and reuse it
unchanged for every implementation deliverable. Never generate or substitute
a second run ID, timestamp, branch name, mutable label, local time, or random
suffix in the same task.

Before any write, verify that none of the four resolved target names exists in
the destination. A collision is a stop condition: report `Blocked` without
generating another run ID.

The run ID identifies the evidence set, not the implementation version. The
manifest must record the exact reviewed commit, tree, parent, branch, source
snapshot root, and every deliverable digest.

## 16. Required deliverables, hashes, and evidence index

Produce exactly these four future implementation deliverables:

1. `prime-shell-work-gfd-p3-wp02-<RUN_ID>-source-snapshot-r1.zip`
2. `prime-shell-work-gfd-p3-wp02-<RUN_ID>-task-runtime-sidecar-report-r1.md`
3. `prime-shell-work-gfd-p3-wp02-<RUN_ID>-review-evidence-index-r1.md`
4. `prime-shell-work-gfd-p3-wp02-<RUN_ID>-handoff-manifest-r1.md`

### Source snapshot

- Build from the exact reviewed final commit, not an uncommitted worktree.
- Use one top-level `prime-shell/` root and repository-relative paths.
- Exclude `.git`, local environments, dependency caches, build output,
  packages/installers, native captures, secrets, user content, and unrelated
  artifacts.
- Include the source, schemas, generated bindings, fixtures, tests, scripts,
  manifests, locks, workflows, and focused documentation needed to reproduce
  review.
- Verify SHA-256, ZIP CRC, safe paths, one expected root, and exact inventory.
- Extract into a fresh empty directory and rerun the complete activated
  verification.

### Task-runtime and sidecar report

Record:

- activation/model/reasoning, repository, branch, base/final commit, parent,
  tree, and exact changed paths;
- every accepted predecessor commit, report, artifact, hash, evidence,
  deviation, fallback, amendment, and blocker;
- retained/moved/replaced/removed/test-only Phase 0B runtime inventory;
- one authoritative final runtime architecture and ownership map;
- schema, operation, task, request, trace, event identities and limits;
- sidecar trust, launch, handshake, target, resource, environment, and package
  identity;
- framing, queues, backpressure, progress, sequencing, snapshots, and frontend
  state;
- state machine, terminal precedence, cancellation, timeout, crash, hang,
  restart, circuit, and no-replay results;
- process-tree containment and zero-descendant results per target;
- structured logs/redaction and production capability/test-surface exclusion;
- exact commands, raw measurements, CI/native/manual evidence, limitations,
  failures, and full independent status inventory;
- explicit confirmation that P4-WP01, product behavior, multiple workers,
  durable tasks, replay, diagnostics, and release work were not started.

### Review evidence index

- Include one row for each of the 27 acceptance gates.
- Map each gate to exact source/tests, command and exit status, CI
  run/job/artifact, native/manual artifact, measurement, classification,
  limitation, and digest.
- Keep all evidence layers from section 11 separate.
- Record tool, runner/hardware, OS/version/architecture, engine, commit,
  capture time, expected/actual result, and limitation.
- Use `None`, `Not run`, `Blocked`, or `Partial` explicitly; leave no ambiguous
  blanks.

### Handoff manifest and hashing

- List all four deliverables with exact filename, role, byte length, and
  ordinary SHA-256 where applicable.
- Record run ID, source snapshot root, reviewed commit, tree, parent, branch,
  generation commands, target matrix, tools, changed files, exact results,
  status inventory, and next controlled action.
- Record ordinary SHA-256 for the source snapshot, report, and evidence index.
- If the manifest records its own digest, designate exactly one self-digest
  value. Copy the final manifest bytes, replace only that self-digest value
  with the literal `<SELF_SHA256>`, and compute SHA-256 over those canonical
  bytes. Record the resulting lowercase hexadecimal digest in the designated
  value. Reproduce it by repeating only that replacement without changing the
  field label, spacing, line endings, or any other byte.
- After finalization, recompute every digest and verify that no deliverable
  changed.
- Never include credentials, secrets, native paths, sensitive user content, or
  opaque-reference backing values.

## 17. Branch, draft-PR, review, merge, and deletion controls

This provisional prompt authorizes no implementation branch or PR. A future
activation must:

- start from the exact accepted merged `main` SHA supplied by Chat Session;
- create exactly one fresh implementation branch named by Chat Session;
- never reuse the documentation branch, a predecessor branch, or a dirty
  worktree;
- keep changes limited to one coherent `GFD-P3-WP02` implementation;
- commit intentionally with a concise package-scoped message;
- push only after local validation and an immediate remote-race recheck;
- verify remote commit, parent, tree, paths, hashes, and clean local parity;
- leave `main` unchanged;
- stop at `READY FOR CHAT SESSION REVIEW`.

Do not open a draft or ready PR unless a later explicit Chat Session
instruction authorizes that separate action. Do not enable auto-merge.
Prompt acceptance is not activation. Review acceptance does not authorize
merge. P3-WP02 completion does not authorize P4-WP01 or product features.

Merge, squash, rebase, force-push, accepted-history rewriting, branch deletion,
signing, notarization, updater enablement, release, publication, deployment,
and repository-setting changes require separate explicit user authorization.

If a PR is later authorized, re-read the accepted review checklist and
activation procedure, verify exact base/head and evidence, create only the
authorized PR, and stop. PR review, merge authorization, merge execution,
post-merge verification, and branch deletion are distinct future decisions.

## 18. Completion response and return prompt

The future implementation response must lead with exactly one status:

- `READY FOR CHAT SESSION REVIEW`
- `BLOCKED`
- `NOT STARTED`

For `READY FOR CHAT SESSION REVIEW`, report concisely:

- model and reasoning level;
- repository, starting commit, branch, final commit, parent, and tree;
- exact changed-path inventory and source snapshot;
- all four deliverable filenames and digests;
- test, CI, native, process, engine, and manual evidence classifications with
  exact limitations;
- all 27 acceptance-gate outcomes;
- raw performance/reliability measurements;
- the full independent functional/status inventory;
- confirmation that `main` is unchanged and no PR, merge, branch deletion,
  P4-WP01, product feature, durable task, replay, multiple worker, diagnostics,
  or release work was started.

For `BLOCKED`, report the exact stop condition, command/evidence, repository
state, changed files, unaffected scope, affected gates, and smallest required
Chat Session decision. Do not claim partial work as implementation success.

End a successful response with this return prompt:

```text
Chat Session: Review GFD-P3-WP02 on the exact implementation branch and commit
reported above. Read the task-runtime/sidecar report, review evidence index,
handoff manifest, and verified source snapshot. Return Accepted, Focused
correction required, or Blocked. Confirm every acceptance gate, one
authoritative runtime path, P3-WP01 contract authority, bundled sidecar
identity, bounded framing/queues, task and terminal-race semantics,
cancellation/timeout escalation, crash/hang interruption, bounded
restart/circuit, no replay, process-tree containment, zero descendants,
frontend state/accessibility, log redaction, production exclusions, target
claim, limitation, measurement, and artifact hash. GFD-P4-WP01, product
features, PR creation, merge, release, and branch deletion remain unauthorized.
```
