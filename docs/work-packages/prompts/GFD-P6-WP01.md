# GFD-P6-WP01 — Settings, Persistence, and Single-Instance Behavior

## 1. Package identity and prompt status

**Package ID:** `GFD-P6-WP01`
**Phase:** `Phase 6`
**Title:** `Settings, Persistence, and Single-Instance Behavior`
**Task ID:** `GFD-P6-WP01`
**Prompt ID:** `PRIME-SHELL-GFD-P6-WP01-PROMPT`
**Prompt version:** `R1`
**Prompt lifecycle state:** `Provisional`
**Implementation status at authoring:** `Not started`
**Recommended future implementation model:** `GPT-5.6 Sol`
**Recommended future implementation reasoning/intelligence:** `High`
**Authorization boundary:** Exactly one package, `GFD-P6-WP01`
**Execution status:** `Execution is not authorized`

High is the roadmap minimum because deterministic settings migration,
atomicity and recovery, Rust single-writer ownership, cross-platform
filesystem behavior, and primary/secondary launch routing are correctness and
data-integrity boundaries. A future user-selected higher reasoning setting
does not change this roadmap minimum.

This provisional prompt is complete for review but non-executable. Chat
Session must first accept it as an Approved provisional prompt and later issue
a separately activated revision after every prerequisite is accepted and
merged. Prompt acceptance is not activation, implementation authorization,
PR or merge authorization, permission to begin `GFD-P6-WP02`, or permission
to begin diagnostics, export, repair, product, customer, or release work.

**Direct prerequisite:** Accepted and merged `GFD-P5-WP01`, including its
accepted two-feature contract, contribution registry, settings contributions,
conflict behavior, implementation report, evidence, and source.

**Direct dependent:** `GFD-P6-WP02`, which remains unauthorized.

### Activation metadata

```text
Activation ID: Not activated — Chat Session must refresh and supply this exact value.
Activated by: Not activated — Chat Session must refresh and supply this exact value.
Activation UTC: Not activated — Chat Session must refresh and supply this exact value.
Authoritative main SHA: Not activated — Chat Session must refresh and supply this exact value.
Required fresh implementation branch: Not activated — Chat Session must verify absence and supply this exact value.
Accepted predecessors through P5-WP01: Not activated — Chat Session must supply exact merged commits, reports, artifacts, hashes, CI runs/jobs, native/manual evidence, deviations, fallbacks, limitations, and blockers.
Accepted P5 feature/command/settings contribution contract: Not activated — Chat Session must supply its exact source paths, types, owners, validators, consumers, report, evidence, and accepted status.
Accepted P2 layout bridge and durable-settings facts: Not activated — Chat Session must supply the exact schema or successor, paths, bounds, writer, debounce/coalescing behavior, recovery behavior, and accepted evidence.
Current settings schema/version/path and section inventory: Not activated — Chat Session must verify and supply exact facts.
Current settings bounds: Not activated — Chat Session must supply exact value, string, collection, document-size, temporary-file, and previous-copy limits.
Current settings owner and write behavior: Not activated — Chat Session must supply exact reader/writer, revision, queue, debounce/coalescing, locking, and stale-write facts.
Current migration and recovery policy: Not activated — Chat Session must supply current, oldest supported, future-version, section-isolation, reset, and recovery facts.
Current settings host/search/reset surfaces: Not activated — Chat Session must supply exact routes, hosts, state owners, contribution paths, and accessibility behavior.
Per-target durability semantics: Not activated — Chat Session must supply exact temp, flush/sync, replace, parent-directory, permissions, locking, and previous-copy primitives and limitations.
Current single-instance mechanism: Not activated — Chat Session must verify the exact Tauri/plugin/API version, initialization point, ownership, and target behavior.
Current window focus/show/restore behavior: Not activated — Chat Session must supply exact window labels, minimized/hidden/modal/startup behavior, platform limits, and owners.
Primary/secondary startup ordering: Not activated — Chat Session must supply exact lock/registration, settings hydration, contribution validation, backend launch, forwarding, acknowledgement, and exit ordering.
Allowlisted secondary-launch intents: Not activated — Chat Session must supply exact names, argument forms, bounds, normalization, rejection behavior, and Rust native-intent integration.
Accepted targets and evidence matrix: Not activated — Chat Session must refresh OS/version/architecture, engine, package, filesystem, tools, runners, native/manual capabilities, and owners.
Accepted targeted amendments: Not activated — Chat Session must refresh and supply the exact list or None.
Predecessor deviations incorporated: Not activated — Chat Session must refresh and supply the exact list or None.
Unresolved blockers and assumptions: Not activated — Chat Session must refresh and supply the exact list or None.
Authorization boundary: Not activated — a future activation may authorize only GFD-P6-WP01.
Authorization invalidates when: Not activated — a future activation must invalidate on any base-SHA, predecessor acceptance, evidence, contract, schema/version, migration, writer, durability, filesystem, single-instance, forwarding, target, tool, repository-layout, or authorization-boundary change.
```

## 2. Repository and exact starting state

**Repository:** `Mirarsoft/prime-shell`
**Authoritative base branch:** `main`
**Required future starting commit:** Not activated — Chat Session must refresh
and supply the exact accepted and merged `main` SHA after `GFD-P5-WP01`
acceptance and merge.
**Required future implementation branch:** Not activated — Chat Session must
verify absence and supply one exact fresh branch name.

Authoring-time documentation facts, for review only:

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
- approved-provisional P3-WP02 prompt documentation head:
  `1157616413408b3b2fb596f52493ed3026188afc`;
- approved-provisional P4-WP01 prompt documentation head:
  `a5cd16a31841b0089c904680dac3bc40229071fc`;
- approved-provisional P5-WP01 prompt documentation head:
  `d60bc9efe3398ec0e4c872359e60ebb9c897d517`;
- P5-WP01 prompt tree:
  `213a745f00624d24a64ee99c3f777abe672314f4`;
- P5-WP01 prompt blob:
  `53e9db1950f8bd72ac2e6838dcb30ab481f0cf1b`;
- P5-WP01 prompt SHA-256:
  `edfb47436449bdd88aa35d813e30beb86bdfdd7fa7f2280802d8c8ed970126f8`;
- prompt-pack documentation branch:
  `docs/gfd-work-package-prompt-pack-v1`;
- predecessor activation and implementation through P5-WP01: `Not started`;
- P6-WP01 implementation: `Not started`.

The authoring-time merged source remains the accepted WP01 spike. It contains
no implemented Phase 2 shell layout bridge, Phase 4 or Phase 5 feature
settings, durable settings repository, migration chain, settings UI, or
single-instance integration. Those future paths and facts must come from
accepted merged predecessor implementations at activation; this prompt does
not invent them.

Required activation and Work Session preflight:

1. Verify exact access and write permission for `Mirarsoft/prime-shell`.
2. Record the default branch and its exact 40-character SHA.
3. Verify WP01 and every package through P5-WP01 are accepted and merged at
   that SHA.
4. Verify every accepted implementation report, evidence index, manifest,
   source snapshot, digest, CI run/job/artifact, native/manual result,
   measurement, deviation, fallback, limitation, amendment, and blocker.
5. Verify the accepted P2 layout bridge, P4/P5 settings contributions, P5
   contract and validator, P3 native-intent/task authority, and current
   settings ownership.
6. Inventory current schemas, files, migrations, read/write/reset commands,
   frontend settings state, browser persistence, startup, Tauri lifecycle,
   single-instance API, focus/restore behavior, forwarded launch inputs,
   capabilities, CSP, packages, tests, workflows, tools, and support claims.
7. Verify no P6 settings store, second durable writer, single-instance
   mechanism, diagnostics/export/repair surface, or successor work exists
   unexpectedly.
8. Verify the named fresh implementation branch is absent.
9. Use a clean fresh clone or worktree at the activated starting commit.
10. Narrow every allowed and protected path against that exact merged tree.
11. Stop if repository state differs from activation, is ambiguous, would
    overwrite unrelated work, or would require rebasing or rewriting history.

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
6. all accepted Markdown under `docs/phase-0a/`
7. `docs/work-packages/roadmap-index.md`
8. `docs/work-packages/shared-ground-rules.md`
9. `docs/work-packages/dependency-matrix.md`
10. accepted WP01 source, review, implementation report, evidence index,
    manifest, snapshot, hashes, CI jobs, and native evidence named by
    activation
11. accepted P0B-WP02 and P0B-WP03 prompts, activations, sources, reviews,
    reports, manifests, snapshots, lifecycle/process evidence, measurements,
    deviations, fallbacks, and blockers
12. accepted P1-WP01 prompt, activation, source, review, baseline report,
    manifest, snapshot, pins, locks, support claims, deviations, and blockers
13. accepted P2-WP01 and P2-WP02 prompts, activations, sources, reports,
    manifests, snapshots, theme, layout, responsive, focus, settings bridge,
    native, accessibility, fallback, deviation, and blocker evidence
14. accepted P3-WP01 and P3-WP02 prompts, activations, sources, reports,
    manifests, snapshots, schemas, operations, native intent, references,
    task races, process/package evidence, measurements, deviations, and
    blockers
15. accepted P4-WP01 prompt, activation, source, report, evidence, manifest,
    snapshot, feature setting, native/package journey, friction, measurements,
    deviations, and blockers
16. accepted P5-WP01 prompt, activation, source, report, evidence, manifest,
    snapshot, two-feature contract, settings contributions, registry,
    validators, conflict behavior, measurements, deviations, and blockers
17. every separately accepted targeted architecture amendment named by
    activation
18. current layout, feature, command, settings contribution, frontend state,
    Rust settings, migration, file, lifecycle, single-instance, window,
    launch-intent, backend, capability, CSP, package, test, workflow,
    toolchain, manifest, lockfile, and support-claim surfaces
19. the exact activated revision of this prompt

The first implementation progress message must state:

> `AGENTS.md, repository authority, accepted Phase 0A, accepted and merged
> predecessors through GFD-P5-WP01, their accepted reports and artifacts, the
> accepted P2 layout bridge, P5 feature/command/settings contribution
> contract, and the activated GFD-P6-WP01 prompt have been read and are
> active. Executing only GFD-P6-WP01 with GPT-5.6 Sol / High. GFD-P6-WP02,
> diagnostics/export/repair, product/customer work, PR creation, and merge
> remain unauthorized.`

If a required source is missing, unreadable, stale, or materially
inconsistent, stop before writing. Do not substitute a broad architecture
review, rerun predecessors as acceptance evidence, or invent future P5 paths,
settings schemas, operating-system atomicity guarantees, or single-instance
APIs.

## 4. Dependencies, predecessor outputs, and prerequisite evidence

**Direct prerequisite:** Accepted and merged `GFD-P5-WP01`, including the real
second consumer, accepted P4/P5 feature definitions, settings contributions,
static registry, deterministic validators, command and shortcut behavior,
complete implementation report, evidence index, manifest, source snapshot,
hashes, native/package evidence, measurements, deviations, and blockers.

**Inherited prerequisites:** Accepted and merged WP01, P0B-WP02, P0B-WP03,
P1-WP01, P2-WP01, P2-WP02, P3-WP01, P3-WP02, and P4-WP01 source and evidence.

**Direct dependent:** `GFD-P6-WP02`, which remains unauthorized and owns
diagnostics, export, and repair work after this durable-state foundation is
accepted.

P6-WP01 cannot activate while any prerequisite exists only as a provisional
prompt, an unmerged branch, an unaccepted report, missing settings or layout
evidence, stale platform evidence, or an unresolved blocker affecting this
scope.

Activation must supply exact:

- accepted merged commits and tree identities;
- reports, evidence indexes, manifests, snapshots, byte lengths, and SHA-256
  digests;
- CI run/job/artifact identities and per-target native/manual evidence;
- accepted P2 layout preference schema or successor, bounds, writer,
  debounce/coalescing, clamping, reset, migration, and recovery truth;
- accepted P4/P5 setting IDs, domains, defaults, reset/apply behavior,
  contribution source, registry ownership, validation, and current
  persistence truth;
- accepted P3 native-intent, task/no-replay, sidecar ownership, path secrecy,
  safe-error, process-containment, and startup rules;
- current settings paths, document versions, sections, bounds, commands,
  readers/writers, browser storage, temp/previous-copy behavior, frontend
  hydration, and error states;
- current Tauri and target-specific single-instance APIs, app lifecycle,
  window labels, focus/show/restore behavior, startup ordering, and launch
  inputs;
- supported targets, filesystems, engines, package formats, tools, runners,
  manual owners, and unavailable evidence;
- exact performance budgets, limitations, deviations, fallbacks, amendments,
  and blockers.

Acceptance of this prompt does not accept predecessor implementation by
inference, activate this package, authorize P6-WP02, or approve diagnostics,
export, repair, product, customer, remote-sync, multiple-workspace, or release
work.

## 5. Objective and measurable runnable outcome

Complete the durable local settings and application-instance model around the
accepted shell and two proven feature consumers.

The future package must prove this exact order in one real installed or
packaged Tauri journey:

```text
accepted P5 feature/command/settings contribution contract
→ canonical schema-versioned Rust-owned settings document
→ accepted layout plus P4/P5 settings materialized in one in-window settings UI
→ searchable accessible settings with bounded typed values
→ atomic durable write and clean application restart
→ deterministic migration from each supported prior version
→ previous-valid-copy recovery from corrupt or interrupted primary write
→ section-scoped reset/recovery preserving unrelated valid sections
→ one primary instance owns settings and backend lifecycle
→ second launch focuses/restores primary and forwards one allowlisted bounded intent
→ secondary exits without starting another backend or writing settings
```

Planning, types, mock storage, browser `localStorage`, in-memory-only
persistence, a unit-only file helper, a window-focus mock, two browser tabs, or
a second process that does not exercise real OS/application-instance behavior
cannot satisfy the outcome.

Activation must fix exact:

- canonical schema name, current version, oldest supported version, and
  top-level sections;
- value, string, collection, document, temporary-file, and previous-copy
  bounds;
- primary and previous-valid-copy locations without exposing user paths in UI
  or evidence;
- deterministic migration chain, unknown-field rule, unsupported
  future-version rule, and section-isolation boundary;
- target-specific temp, permissions, flush/sync, replace, parent-directory,
  locking, and durability procedure;
- write revision, bounded queue, debounce/coalescing, stale-write, reset, and
  recovery semantics;
- settings search fields, matching, result bounds, and ordering;
- one exact single-instance mechanism per target;
- primary/secondary startup ordering, forwarding deadline, acknowledgement,
  and exit behavior;
- allowlisted launch intents, parsing, normalization, and safe failures;
- focus/show/restore/minimized/hidden behavior;
- target matrix, evidence owners, and performance budgets.

## 6. Explicit in-scope work

The activated Work Session may perform only the smallest coherent change set
needed for the following work.

### 6.1 Evidence-driven consolidation

Inventory and classify:

- the accepted P2-WP02 `ShellLayoutPreferencesV1` bridge or successor;
- accepted theme, density, motion, transparency, and accessibility preference
  ownership;
- P4 and P5 setting contributions and their durable or session-only truth;
- current Rust settings commands, path helpers, temporary and previous-copy
  behavior, frontend settings caches, and state owners;
- every `localStorage`, `sessionStorage`, IndexedDB, or other browser
  persistence use;
- settings-dependent bootstrap/hydration behavior;
- Tauri lifecycle, window, argument, deep-link, file-open, and
  single-instance code.

Classify every piece as retained, migrated, absorbed, replaced, removed,
test-only, or blocked. The final repository may have one durable settings
writer and one canonical settings document authority only. Do not leave
parallel P2 and P6 stores, browser/native duplication, or last-write-wins
races.

### 6.2 Canonical schema-versioned settings document

Implement one bounded versioned settings document owned by Rust. Activation
must define its exact schema and include only evidence-backed sections such as:

- accepted appearance, theme, density, motion, transparency, or
  accessibility preferences;
- bounded shell layout preferences;
- proven P4/P5 feature settings;
- narrowly necessary application settings for this package.

The schema must:

- carry one explicit document schema version;
- use stable section and setting IDs;
- validate types, enums, ranges, lengths, collections, finite numeric values,
  Unicode/serialization, and total document size;
- distinguish missing/defaulted, invalid, migrated, reset, recovered,
  unsupported-future, and unavailable states;
- preserve unrelated valid sections under section-scoped recovery;
- define exact unknown-field and future-version behavior;
- serialize deterministically where tests or digests require it;
- exclude derived runtime/task state, document content, native paths, opaque
  reference values, raw launch inputs, logs, secrets, credentials, and
  backend internals.

Do not turn settings into a product database, recent-files store, task journal,
diagnostic log, or general key-value repository.

### 6.3 Rust sole writer and typed frontend boundary

- Rust is the only durable writer and authoritative revision owner.
- React owns transient drafts, validation display, settings search, and
  pending UI intent only.
- Python does not read or write application settings. One exact typed
  non-secret value may be passed as P3-authorized operation input only when
  accepted predecessor evidence requires it.
- Reads, writes, resets, and recovery decisions use finite named typed
  commands.
- Mutations carry a Rust-issued revision or accepted equivalent, and stale
  writes fail safely.
- Use one bounded write queue with deterministic debounce/coalescing and
  acknowledgement behavior.
- Expose no direct filesystem command, arbitrary key path, generic JSON patch,
  caller-supplied whole settings file, or generic persistence tunnel.
- Remove canonical-value duplication in browser persistence or another native
  store.
- Complete hydration before settings-dependent feature behavior becomes
  authoritative, while safe bounded defaults cover startup.

### 6.4 Searchable accessible in-window settings

- Render accepted shell and P4/P5 settings through the proven P5 contribution
  contract.
- Show trusted label, description, current/default value, section/group,
  validation, reset, and apply semantics.
- Add bounded local search over trusted static label, description, and keyword
  metadata only.
- Define deterministic matching, ordering, query bounds, result bounds,
  visible context, and explicit no-result behavior.
- Never search document content, native paths, logs, secrets, raw launch
  inputs, or diagnostics.
- Preserve keyboard operation, logical headings, screen-reader names/status,
  deterministic focus restoration, non-color states, forced colors, reduced
  motion/transparency, 200% scaling, text expansion, RTL, and the 500-pixel
  compact shell.
- Use one settings state authority and the one Rust writer.
- Keep destructive resets scoped, explicit, and confirmed where appropriate.
- Add no separate native settings window.

Settings search is not a broad command/search framework and must not enter
P6-WP02 diagnostics.

### 6.5 Atomic persistence and target durability

Activation must choose and document exact target-specific primitives. At
minimum:

- serialize only validated bounded bytes;
- create the temporary file in the same directory as the primary;
- apply restrictive user-only permissions where supported;
- flush userspace buffers and perform the accepted file durability call;
- atomically replace the primary under the documented target filesystem
  semantics;
- perform accepted parent-directory durability where meaningful and
  supported;
- update a bounded previous-valid-copy only from a fully validated known-good
  document;
- never accept a partial temporary file as valid;
- clean stale temporary files under a deterministic bounded policy;
- serialize or reject concurrent requests under the single-writer policy;
- record truthful Windows, macOS, Linux, and filesystem limitations.

Prove simulated interruption, crash boundaries, disk-full/write failure,
permission failure, rename/replace failure, and concurrent write pressure.
Never claim universal power-loss guarantees beyond retained evidence.

### 6.6 Previous-valid-copy and section-scoped recovery

- Validate the primary fully on normal load.
- If it is unreadable, corrupt, or unsupported under the activated policy,
  validate the previous known-good copy.
- Report recovery explicitly and path-free; never label it a normal load.
- Do not overwrite corruption evidence merely because a previous copy loaded.
  Persist only under the accepted new-valid-write policy.
- Recover or reset only an isolated invalid section when schema and migration
  rules prove isolation.
- Preserve unrelated valid sections byte-for-byte or semantically under the
  activated rule.
- Use whole-document fallback only when isolation is unsafe.
- Provide finite typed reset-one-setting, reset-one-section, and reset-all
  commands with explicit defaults and results.
- Keep corrupt bytes, user paths, and sensitive values out of UI, logs,
  screenshots, CI artifacts, and handoff deliverables.

Do not add a corrupt-file viewer, backup browser, repair wizard, diagnostics
bundle, or export flow. Those belong to separately authorized P6-WP02 work.

### 6.7 Deterministic migration chain

Migrations must be:

- explicit ordered source-version-to-target-version functions;
- derived from accepted merged versions rather than speculative releases;
- deterministic and tested from every supported historic version;
- idempotent at the current version;
- bounded in step count, input bytes, and output bytes;
- section-aware where isolation is demonstrably safe;
- validated after every step or at an explicitly justified boundary;
- free of network, clocks, random values, environment-derived secrets, native
  paths, and side effects outside the settings document;
- forward-version safe so unsupported newer settings are not automatically
  downgraded or overwritten.

Provide fixtures for every valid supported version, malformed old versions,
partially invalid sections, unknown fields under policy, and unsupported
future versions.

### 6.8 Accepted layout bridge integration

Absorb or formally retain the accepted bounded P2 layout bridge without
changing shell behavior. Preserve:

- sidebar, inspector, and bottom-panel bounds and clamping;
- keyboard and pointer splitters;
- responsive-band reclamping;
- focus restoration;
- title-bar/native-decoration fallback;
- theme, density, motion, transparency, and forced-color behavior;
- accepted debounce and crash-safe persistence semantics.

Do not redesign the shell, alter responsive breakpoints, broaden layout
history, or introduce workspace/window profiles without accepted evidence.

### 6.9 P4/P5 feature-setting integration

Use the accepted P5 contribution contract and one Rust persistence path.
Validate:

- unique setting IDs and section ownership;
- bounded domains and deterministic default/reset/apply semantics;
- conflict rejection independent of registration order;
- current-schema mapping;
- migration ownership and failure isolation;
- feature-disabled or removed behavior without orphan writes;
- independence of Rust authorization from frontend metadata;
- absence of commands, scripts, paths, URLs, capabilities, or arbitrary
  payloads in setting definitions.

Do not add a public settings SDK, runtime plugin contract, generator, or
cross-application settings package.

### 6.10 Single-instance authority and lifecycle

Use one accepted target-specific single-instance mechanism, preferably the
current supported Tauri mechanism verified at activation. Define:

- the earliest authoritative primary lock or registration point;
- exactly one primary app lifecycle and window owner under the accepted shell
  model;
- exactly one backend/sidecar lifecycle owner;
- secondary behavior before settings read/migration/write, backend launch, or
  visible duplicate window;
- bounded forwarding deadline and acknowledgement;
- deterministic exit status for forwarded, rejected, timed-out, and
  unsupported launches;
- stale-lock behavior under exact accepted platform semantics;
- primary shutdown and lock/registration cleanup.

Do not add multiple workspaces/windows, a background daemon, resident helper,
or silent stale-lock takeover. A secondary instance must never become a
settings writer or sidecar owner.

### 6.11 Allowlisted launch-intent forwarding

Rust must parse OS-provided arguments, deep links, and file-open data before
anything reaches React or Python. Activation must define the smallest
allowlist:

- focus/show/restore only; and
- at most one already accepted named open/import intent when real merged
  predecessor evidence justifies it.

Require:

- no raw argv, file path, URI, environment, working directory, or native
  handle reaches React or Python;
- argument count and bytes are bounded before allocation or use;
- unknown flags, mutually exclusive duplicates, oversized input, malformed
  Unicode, unsupported schemes, and ambiguous targets fail safely;
- any accepted file-open input enters the existing Rust-owned native
  intent/opaque-reference path, never path forwarding;
- focus/restore policy remains deterministic when content intent is rejected;
- no automatic command execution, task replay, or authorization bypass;
- sensitive launch values and paths are not logged.

### 6.12 Focus, restore, and launch routing

Require real native evidence for:

- primary visible and focused;
- primary minimized;
- primary hidden where supported;
- primary on another virtual desktop or space where tooling permits;
- primary still starting or hydrating;
- primary showing a modal dialog;
- primary with an active P3 task;
- backend faulted or unavailable;
- malformed or unsupported launch intent;
- rapid repeated secondary launches.

Use one bounded launch-intent queue/coalescing policy. A user-triggered
secondary launch may request focus/show/restore, but forwarding must not cause
unbounded focus theft, duplicate navigation, or loss of the final accepted
intent.

### 6.13 Startup ordering, failure states, and no replay

Preserve this explicit ordering:

```text
single-instance authority
→ primary process identity
→ settings read, validate, migrate, and recover
→ accepted UI bootstrap and contribution validation
→ backend lazy readiness according to P3
→ normal interaction
```

Secondary launches short-circuit before settings migration/write and backend
launch. Preserve:

- no replay of interrupted P3 tasks;
- no restoration of stale native references;
- no automatic execution of forwarded commands;
- safe defaults and explicit settings-unavailable, recovered, and
  unsupported-newer-version states;
- bounded startup and forwarding deadlines with truthful failure
  classifications;
- normal primary shutdown and zero surviving sidecar descendants.

### 6.14 Secrets and sensitive data

Secret persistence is `Not applicable` unless activation proves an accepted
OS-secure-storage boundary already exists and is necessary. Always require:

- no plaintext secret fallback;
- no credential, token, API key, native path, user content, opaque-reference
  value, raw launch input, or environment dump in settings;
- no secret setting type silently degrading to an ordinary string;
- bounded safe errors and logs using section/setting IDs rather than values;
- diagnostics or settings export deferred to P6-WP02.

### 6.15 Cross-platform and packaged-runtime truth

Activation must refresh exact target OS/version/architecture, engine, package,
filesystem semantics, single-instance API, tools, runner, and manual owner.
Keep separate:

- schema, unit, and integration evidence;
- filesystem atomicity and recovery harness evidence;
- source-run Tauri behavior;
- host package creation;
- installed or packaged primary/secondary behavior;
- WebView2, WKWebView, and WebKitGTK behavior;
- native window focus/show/restore behavior;
- argument/deep-link/file-open behavior;
- accessibility/manual behavior;
- backend/process cleanup;
- CI and unavailable/blocked evidence.

One target cannot substitute for another. A mock lock, two browser tabs, unit
test, source build, package file, or screenshot is not installed/native
single-instance evidence.

### 6.16 Performance and reliability measurements

Retain method, sample count, target, engine, filesystem, hardware/runner,
activated budget, raw values, and classification for:

- settings read, parse, validate, and hydrate;
- each supported migration;
- atomic write and previous-copy update;
- coalesced rapid writes and stale-write rejection;
- corrupt-primary recovery;
- setting, section, and full reset;
- settings search, filter, and render;
- primary cold and warm launch;
- secondary launch to primary acknowledgement;
- focus/show/restore;
- forwarded-intent routing;
- rapid secondary-launch bursts;
- host memory and settings-document high-water size;
- package-size delta;
- shutdown and zero-descendant cleanup.

Measure rather than invent universal service levels. Do not hide a miss by
changing the workload, target, filesystem, or classification.

### 6.17 Focused documentation and evidence

Document only:

- canonical settings ownership and schema;
- supported versions and migration chain;
- atomic write, previous-copy, recovery procedure, and limitations;
- layout and feature-setting integration;
- settings UI and search behavior;
- single-instance lifecycle and launch-intent allowlist;
- exact commands/tests and per-target evidence;
- privacy, secret, and diagnostics exclusions;
- known limitations and deferred P6-WP02 work.

Do not rewrite architecture authority, publish a settings API, or create
diagnostics/export/customer-support documentation.

## 7. Explicit exclusions and prohibited adjacent work

The activated package must not:

- activate, repair, reimplement, or rerun predecessors as substitute work;
- begin `GFD-P6-WP02` or another package;
- add diagnostics collection, preview, export, repair, recovery wizard,
  corrupt-file viewer, backup browser, telemetry, crash upload, or support
  bundle behavior;
- add product/customer logic, accounts, collaboration, cloud service,
  product database, remote API, remote sync, or customer data;
- create multiple workspace/window profiles, multi-window synchronization, a
  separate settings window, hidden daemon, resident helper, or background
  agent;
- create a general key-value store, generic file API, arbitrary JSON patch,
  arbitrary key-path mutation, import/export settings format, or caller-owned
  whole-file write;
- create a recent-files database, task journal, checkpoint, automatic replay,
  durable task, or stale native-reference restoration;
- add runtime plugins, public settings/feature SDK, `packages/ui`, generator,
  marketplace, dynamic loading, or compatibility promise;
- add native-intent paths beyond the one accepted forwarded allowlist;
- expose raw argv, path, URI, working directory, environment, native handle,
  opaque-reference value, secret, credential, or user content;
- store secrets as plaintext or ordinary strings;
- weaken Rust single-writer authority, P3 operation authorization, path
  secrecy, no-replay, process containment, P5 validation, CSP, capabilities,
  or production exclusions;
- redesign shell, theme, responsive breakpoints, route/navigation/command
  ownership, task runtime, or feature contract;
- invent unshipped migration versions or universal filesystem guarantees;
- broaden platform, filesystem, engine, package, accessibility, performance,
  or support claims beyond accepted evidence;
- perform opportunistic dependency upgrades, formatting sweeps, broad
  refactors, or unrelated cleanup;
- add signing, notarization, updater, release, publication, deployment, store,
  or repository-setting work;
- open a PR, enable auto-merge, merge, force-push, rewrite history, or delete a
  branch;
- author or modify another work-package prompt.

A demonstrated predecessor defect, missing accepted durability decision, or
required architecture change is a stop condition unless Chat Session
separately accepts and activation names one exact focused correction.

## 8. Allowed repository areas and expected changes

Future activation must replace these categories with an exact narrow path
allowlist from the accepted merged tree. Expected areas are no broader than:

```text
packages/app-contracts/schemas/             # bounded settings commands/results when cross-language types are justified
packages/app-contracts/fixtures/            # valid/current/old/corrupt/future-version fixtures
accepted generated binding locations        # mechanically required outputs only
apps/desktop/src/settings/ or accepted path  # in-window host, search, drafts, validation, reset UI
apps/desktop/src/shell/ or accepted path     # exact P2 layout bridge integration only
apps/desktop/src/features/ or accepted path  # P4/P5 contribution consumption only
apps/desktop/src/state/ or accepted path     # one bounded settings-facing state authority
apps/desktop/src/backend/                    # finite typed settings and launch adapters
apps/desktop/src-tauri/src/settings/         # schema, load, migration, writer, recovery, reset
apps/desktop/src-tauri/src/instance/         # single-instance authority and forwarding
apps/desktop/src-tauri/src/windows/          # exact focus/show/restore behavior
apps/desktop/src-tauri/src/native/           # accepted forwarded native intent only
apps/desktop/src-tauri/src/commands/         # exact named settings commands
apps/desktop/src-tauri/capabilities/         # least-privilege exact commands
apps/desktop/src-tauri/permissions/          # least-privilege exact commands
scripts/verify/                              # bounded persistence/instance/package checks
tests/e2e/ and tests/platform-smoke/         # focused settings and primary/secondary journeys
.github/workflows/                           # minimal activated target evidence only
manifests, lockfiles, build configs          # mechanically required changes only
focused settings/instance documentation      # ownership, versions, limits, evidence
```

Activation must remove nonexistent or unused entries, identify actual accepted
owners instead of creating duplicate modules, and list every allowed and
protected path exactly.

Protected by default:

- architecture authority, Stage 1 files, prompt files, accepted decisions,
  predecessor reports, evidence, and snapshots;
- accepted P2 shell/theme behavior except the exact settings-store bridge;
- accepted P3 operation, native-intent, task, process, no-replay, and path
  authority;
- accepted P4/P5 feature behavior and P5 contribution contract except exact
  settings persistence integration;
- Python sidecar behavior;
- unrelated routes, features, commands, data, diagnostics, and recovery UI;
- release, signing, notarization, updater, publication, deployment, and
  repository settings;
- public API and package boundaries.

Every changed path must appear in the implementation report with its purpose
and authority. A path outside the activated allowlist is a stop condition
unless it is a mechanically required generated or locked companion explicitly
authorized. Generated caches, build products, installers, native captures,
local environments, secrets, user data, and temporary settings files must not
be committed.

## 9. Ordered implementation procedure

After a separate activation, execute exactly in this order:

1. Read all authority and accepted predecessor evidence; issue the exact
   acknowledgement.
2. Verify repository, exact accepted merged `main`, fresh branch, permissions,
   clean tree, paths, and activation freshness.
3. Generate exactly one task-start run ID after authority and preflight and
   before any repository or deliverable write; resolve all four
   implementation filenames and stop on collision.
4. Inventory P2 layout persistence, P4/P5 settings, current settings commands,
   browser storage, startup ordering, and single-instance behavior.
5. Run accepted P2/P3/P4/P5 regressions and current settings/lifecycle
   baselines before editing.
6. Freeze exact schema, version, sections, bounds, migration chain,
   file/previous-copy policy, atomic primitives, search behavior,
   single-instance mechanism, forwarded-intent allowlist, target matrix, and
   evidence budgets.
7. Produce a minimal consolidation map with one final owner for every retained,
   migrated, absorbed, replaced, removed, test-only, and blocked piece.
8. Implement canonical schemas, fixtures, generated bindings where required,
   and deterministic bounded validation.
9. Implement Rust-owned load, default, migration, recovery, revision, and
   status logic.
10. Implement atomic write, previous-valid-copy, restrictive permissions,
    stale-temp cleanup, bounded queue/coalescing, acknowledgement, and
    stale-write rejection.
11. Absorb or formally integrate the accepted P2 layout bridge without
    changing shell behavior.
12. Integrate P4/P5 setting contributions through the accepted contract and
    one writer.
13. Implement searchable accessible in-window settings plus reset-one,
    reset-section, and reset-all flows.
14. Establish target-specific single-instance authority before settings read
    or write, migration, duplicate window creation, or backend launch.
15. Implement bounded allowlisted secondary-launch parsing, forwarding,
    acknowledgement, focus/show/restore, and deterministic secondary exit.
16. Add safe startup, recovery, future-version, permission, disk/write,
    atomic-replace, forwarding, and timeout states without diagnostics scope.
17. Add proportional schema, migration, Rust, TypeScript, filesystem-fault,
    concurrency, UI, accessibility, native, packaging, and platform tests.
18. Run fast checks continuously, then packaged primary/secondary journeys on
    every claimed target.
19. Measure and retain settings, migration, write, recovery, search, startup,
    forwarding, focus, burst, memory, package, and cleanup results.
20. Inspect production source and packages for duplicate writers, browser
    persistence, raw argv/paths, plaintext secrets, generic storage commands,
    test controls, multiple instances/backends, P6-WP02 diagnostics, network,
    product, and release creep.
21. Reconcile the final diff to the activated allowlist; remove dead
    scaffolding, duplicate stores, unrelated refactors, and hidden stubs; rerun
    all checks.
22. Commit and push only the activated implementation branch, verify remote
    parity, and leave `main` unchanged without opening a PR.
23. Build the source snapshot from the final commit; verify digest, CRC, safe
    paths and one root; extract into a fresh empty directory; rerun complete
    activated verification.
24. Finalize exactly four deliverables, verify ordinary hashes and canonical
    manifest self-hash, return the section 18 handoff, and stop without
    entering P6-WP02.

Never solve a failed gate by entering diagnostics, repair/export, product
data, multiple workspaces/windows, remote sync, or release work.

## 10. Cross-cutting constraints

### Ownership and authority

- Rust owns the canonical settings document, revisions, validation,
  migrations, durable writes, recovery, resets, native paths,
  single-instance authority, launch parsing, window policy, and
  backend/sidecar lifecycle.
- React owns transient settings drafts, validation presentation, trusted
  search query, and user intent only.
- Python owns no durable application setting.
- Accepted P5 definitions remain static trusted presentation/availability
  metadata, never native authorization.
- Accepted P3 operation and native-intent authority remains independent.
- Every mutable datum has one writer and one authoritative revision.

### Bounds and deterministic behavior

- Activation fixes every schema, ID, version, bound, migration, queue,
  debounce, timeout, normalizer, reset, recovery, and safe result.
- Enforce byte/count/time bounds before allocation, parse, migration, search,
  queue admission, forwarding, or retention.
- Serialize and migrate deterministically.
- Reject stale revisions and invalid, unsupported, oversized, ambiguous, or
  conflicting inputs safely.
- Use no last-write-wins race, silent downgrade, hidden auto-repair,
  auto-renaming, or priority override.

### Durability and recovery truth

- Use documented target-specific atomic and durability primitives.
- A validated primary, temporary file, and previous-valid-copy have distinct
  roles.
- Promote only known-good validated bytes.
- Preserve corruption evidence until accepted policy permits a new write.
- Keep section recovery isolated only when schema and migration rules prove
  safety.
- Record power-loss, filesystem, permission, locking, and parent-directory
  limitations without extrapolation.

### Instance and task safety

- Establish primary authority before settings/backend side effects.
- A secondary process cannot become a writer, backend owner, or visible
  duplicate window.
- Forward finite allowlisted intents only after Rust parsing and bounds.
- Focus/show/restore behavior is deterministic and bounded.
- Never replay interrupted tasks, restore stale references, or auto-execute a
  forwarded command.
- Preserve accepted shutdown and zero-descendant behavior.

### Security and privacy

- Expose no generic settings, filesystem, JSON patch, key-path, process,
  shell, URI, launch, capability, or payload tunnel.
- Persist no secret, credential, user content, path, raw launch input,
  environment value, log, reference backing value, or backend internal.
- Keep network denied and release CSP least privilege.
- Exclude test controls, fault injectors, evidence writers, debug commands,
  and alternate writers from production.
- Treat secret persistence as not applicable without accepted OS-secure
  storage; never use plaintext fallback.

### UI and accessibility

- Use the accepted in-window settings host and proven contributions.
- Use one settings state authority and no duplicated browser persistence.
- Preserve semantic tokens, portals, title-bar fallback, responsive shell,
  forced colors, reduced effects, visible focus, keyboard operation,
  screen-reader status, 200% scaling, text expansion, RTL, and 500-pixel
  behavior.
- Keep automated browser, native focus, screen-reader, and manual evidence
  classifications separate.

### Reproducibility and evidence

- Use activated pins, locks, clean checkouts, deterministic fixtures, and
  exact commands.
- Build the snapshot from the reviewed commit and prove a fresh extraction
  rerun.
- Record command, exit status, tool, runner/hardware, OS/architecture,
  filesystem, engine, package, commit, artifact, capture time, expected and
  actual result, measurement, limitation, and classification.
- One target or evidence class cannot substitute for another.
- Perform no opportunistic upgrade, broad refactor, formatting sweep, or
  unrelated cleanup.

## 11. Proportional tests and exact evidence

Future activation must replace command families with exact current commands,
working directories, versions, target identities, environment controls,
expected results, retained artifacts, and manual owners.

### Schema and migration evidence

Verify:

- valid current settings;
- defaults and missing sections;
- every supported prior version to current;
- current-version migration idempotence;
- malformed old versions;
- unsupported future versions;
- unknown fields under the activated policy;
- invalid types, enums, ranges, lengths, collections, non-finite numbers,
  malformed Unicode, and oversized documents;
- partially invalid isolated sections;
- migration step failure and bounded output;
- deterministic serialization;
- cross-language fixture/binding parity where applicable;
- absence of speculative unshipped migrations.

### Rust writer and concurrency evidence

Verify:

- Rust sole-writer enforcement;
- finite typed read/write/reset commands;
- authoritative revision and stale-write rejection;
- rapid writes and bounded coalescing;
- concurrent UI intents;
- primary/secondary startup and mutation races;
- no browser/native duplicate persistence;
- no Python durable write;
- no arbitrary key/path/whole-file payload tunnel;
- exactly one settings document and writer after consolidation.

### Atomicity and previous-copy fault evidence

For every claimed target, cover:

- normal write;
- same-directory temporary creation;
- restrictive permissions;
- userspace flush and accepted durability call;
- atomic replace and parent-directory durability where applicable;
- interruption at every safe injected fault point;
- process crash after temporary write and after replacement;
- disk-full or write failure;
- permission denial;
- rename/replace failure;
- stale temporary cleanup;
- corrupt primary with valid previous copy;
- corrupt primary and corrupt previous copy;
- valid primary with stale or corrupt previous copy;
- no unvalidated backup promotion;
- truthful filesystem and power-loss limitations.

### Section recovery and reset evidence

Cover:

- reset one setting;
- reset one section;
- reset all;
- invalid P4 section preserving P2 and P5;
- invalid P5 section preserving P2 and P4;
- invalid layout section preserving feature settings;
- isolated migration failure when safe;
- whole-document fallback only when isolation is unsafe;
- explicit recovered, unavailable, and unsupported-newer-version states;
- no corrupt-byte or native-path exposure.

### Settings UI and accessibility evidence

Cover:

- accepted contributions rendered through one host;
- bounded trusted search, deterministic results, visible context, and no
  result;
- typed value editing, validation, apply, and reset;
- keyboard navigation and deterministic focus after search, apply, reset,
  recovery, and error;
- screen-reader labels and status;
- non-color validation/recovery states;
- forced colors;
- reduced motion and transparency;
- 200% scaling and text expansion;
- RTL;
- 500-pixel compact shell;
- layout/settings regression;
- no separate settings window.

### Single-instance and forwarding evidence

On each claimed platform, separately prove:

- first launch becomes primary;
- second launch before primary hydration completes;
- second launch after primary readiness;
- primary visible/focused;
- primary minimized;
- primary hidden where supported;
- primary modal and active-task behavior;
- backend-fault behavior;
- rapid repeated secondary launches;
- forwarded focus-only intent;
- one allowlisted content/open intent when activated;
- unknown, oversized, malformed, duplicate, and conflicting inputs;
- no raw launch input or path to React, Python, logs, or artifacts;
- deterministic acknowledgement, timeout, rejection, and exit;
- secondary performs no settings write, migration, backend launch, or
  duplicate-window creation;
- one primary lifecycle/window/backend owner;
- normal close and zero descendants.

### Production exclusion and security evidence

Inspect final source, capabilities, and packages for:

- no plaintext secret fallback;
- no user content, native path, raw launch input, opaque-reference backing
  value, secret, credential, environment dump, or raw exception;
- no generic filesystem, JSON patch, key-path, process, shell, URI, or payload
  tunnel;
- no browser duplicate canonical store;
- no test fault control, evidence writer, or alternate production writer;
- no second backend/sidecar owner;
- no diagnostics export, repair wizard, telemetry, network, remote sync,
  database, multiple-workspace framework, signing, updater, or release work.

### Packaged/native and platform-truth evidence

Require real packaged or installed Tauri runs for every claimed target where
tooling permits. Keep schema, filesystem harness, source run, package
creation, installed runtime, engine, native window behavior, CI, manual, and
unavailable evidence separate. A unit test, source run, mock lock, screenshot,
or package file cannot substitute for packaged single-instance behavior.

### Performance and snapshot evidence

Retain all raw measurements from section 6.16. Prove exactly four
implementation deliverables, a source snapshot from the exact reviewed
commit, one safe expected root, CRC and path-safety success, clean extraction,
complete rerun, reproducible digests, and absence of sensitive data.

The review evidence index must map every acceptance gate to exact source and
tests, command and exit status, CI run/job/artifact, native/manual evidence,
measurement, limitation, classification, and deliverable digest.

## 12. Measurable acceptance gates

All 27 gates are unsatisfied while this prompt remains provisional:

1. Exact accepted and merged predecessors through P5-WP01, including the
   proven settings contribution contract, were used from exact clean
   activated `main`.
2. The final diff stays inside the activated allowlist and contains no
   P6-WP02, diagnostics/export, product database, multiple workspace/window
   framework, release, prompt-pack, or unrelated work.
3. Exactly one canonical bounded schema-versioned settings document and one
   Rust durable writer remain.
4. The accepted P2 layout bridge is integrated without changing responsive,
   splitter, clamping, focus, title-bar, theme, density, motion,
   transparency, or forced-color behavior.
5. P4 and P5 settings use the accepted contribution contract, unique IDs,
   bounded domains, deterministic defaults/reset/apply semantics, and one
   writer.
6. Current settings load/default/validate status is explicit, bounded, and
   path-free.
7. Every supported prior version migrates deterministically to current and
   current-version migration is idempotent.
8. Unsupported future versions are not downgraded or overwritten
   automatically.
9. Atomic write uses the activated target-specific temp, permissions,
   flush/sync, replace, and parent-directory procedure and records truthful
   limitations.
10. Previous-valid-copy is updated only from known-good data and explicitly
    recovers a corrupt primary without silently destroying evidence.
11. Section-scoped recovery and reset preserve unrelated valid sections
    whenever the activated isolation policy says it is safe.
12. Stale, concurrent, and rapid UI writes are bounded, coalesced
    deterministically, and cannot overwrite a newer Rust revision.
13. No browser store, Python writer, generic key/path patch, or second native
    store can mutate canonical settings.
14. Searchable in-window settings render accepted contributions accessibly
    with bounded trusted search and no separate settings window.
15. No secret, credential, token, native path, user content, raw launch input,
    opaque-reference backing value, environment dump, or raw exception is
    persisted or exposed; no plaintext secret fallback exists.
16. Single-instance authority is established before secondary settings
    reads/writes, migrations, duplicate window creation, or backend/sidecar
    launch.
17. A secondary launch focuses, shows, or restores the primary
    deterministically and exits without becoming a writer or backend owner.
18. Only activation-allowlisted bounded launch intents are forwarded;
    malformed, oversized, unknown, conflicting, or unsupported inputs fail
    safely.
19. Raw argv, paths, URIs, working directory, environment, and native handles
    never reach React or Python.
20. Forwarding during startup, modal, minimized/hidden, active-task,
    backend-fault, and rapid-launch states follows the executable accepted
    policy without replay or duplicate navigation.
21. Primary restart proves valid settings survive, migrated and recovered
    settings remain truthful, and stale task or reference state is not
    silently restored.
22. Keyboard, focus, screen-reader, non-color, forced-color, reduced-effect,
    200% scaling, text expansion, RTL, and 500-pixel compact evidence passes
    proportionally.
23. Schema, filesystem harness, source-run, package creation,
    installed/native, engine, window, CI, manual, and unavailable evidence
    remains separate and truthful per target.
24. Raw settings, migration, write, recovery, search, startup, forwarding,
    focus, burst, memory, package, and cleanup measurements are retained
    without fabricated service-level claims.
25. Production source, capabilities, and packages exclude duplicate
    stores/writers, test controls, generic tunnels, diagnostics/export,
    network, telemetry, database, multiple-workspace framework, signing,
    updater, and release work.
26. Exactly four implementation deliverables use one task-start run ID, have
    verified hashes, contain no sensitive data, and reproduce the reviewed
    commit through a safe fresh snapshot extraction and complete rerun.
27. The implementation branch matches its remote, `main` is unchanged, no PR
    or auto-merge exists, and the handoff stops at
    `READY FOR CHAT SESSION REVIEW` without starting P6-WP02.

If a gate is unmet, report `Blocked`, `Not run`, `Partial`, or the exact
accepted limitation as an evidence outcome. Do not claim implementation
success by inference.

## 13. Stop conditions and blocker reporting

Stop before writing if:

- activation is absent, provisional, stale, incomplete, or overbroad;
- repository, base, branch, clean-tree, path, or permission state differs;
- P5-WP01 or another required predecessor is unaccepted or unmerged;
- accepted P2 layout, P5 settings, contract, source, report, evidence, or
  platform facts are missing or inconsistent;
- exact schema, version, migration, section, bound, writer, revision, queue,
  file, previous-copy, atomicity, recovery, single-instance, forwarding,
  window, target, tool, or manual-owner facts cannot be resolved safely;
- current source and accepted authority materially disagree;
- implementation would weaken Rust single-writer ownership, P3 authorization,
  path secrecy, no replay, process containment, P5 validation, or CSP;
- work requires diagnostics/export/repair, product database, remote sync,
  multiple workspace/window architecture, plaintext secrets, generic storage,
  runtime plugins, public SDK, unsupported target, signing, updater, release,
  or another package;
- target-specific atomicity or recovery semantics cannot be proven
  truthfully;
- section isolation cannot be proven and unrelated valid data would be at
  risk;
- real packaged primary/secondary single-instance behavior cannot be proven
  where a pass would be claimed;
- sensitive data could enter source, settings, UI, logs, errors, screenshots,
  CI, or handoff artifacts;
- CI, tooling, runner, filesystem, engine, package, or manual evidence is
  blocked and another target would have to substitute;
- snapshot integrity or clean extraction rerun fails;
- unauthorized or unrelated files enter the diff;
- remote state changes, push is non-fast-forward, or history rewriting would
  be required.

When blocked:

1. Stop at the last clean non-destructive state.
2. Do not broaden scope, weaken a gate, or enter P6-WP02 as a workaround.
3. Record the exact failed fact, command or path, expected value, observed
   value, affected gates, changed files, tree state, and unchanged exclusions.
4. Distinguish repository defect, stale activation, missing authority or
   evidence, environment limitation, permission failure, platform limitation,
   filesystem limitation, and design decision.
5. Use `Blocked` or `Partially implemented` for implementation status. Use
   `Partial` only for an explicitly labeled evidence outcome.
6. Return the smallest safe Chat Session decision, focused correction, or
   refreshed activation required.

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
GFD-P3-WP02 provisional prompt: Implemented and accepted as Approved provisional
GFD-P3-WP02 activation/implementation: Not started
GFD-P4-WP01 provisional prompt: Implemented and accepted as Approved provisional
GFD-P4-WP01 activation/implementation: Not started
GFD-P5-WP01 provisional prompt: Implemented and accepted as Approved provisional
GFD-P5-WP01 activation/implementation: Not started
GFD-P6-WP01 package outcome: Not started
Canonical schema-versioned settings document: Not started
Rust sole durable writer: Not started
P2 layout bridge integration: Not started
P4/P5 settings contribution integration: Not started
Searchable in-window settings: Not started
Settings validation/defaulting: Not started
Migration chain: Not started
Atomic persistence: Not started
Previous-valid-copy recovery: Not started
Section-scoped recovery/reset: Not started
Bounded write queue/coalescing: Not started
Stale-write rejection: Not started
Single-instance primary authority: Not started
Secondary launch focus/show/restore: Not started
Allowlisted launch-intent forwarding: Not started
Raw argv/path secrecy: Not started
Primary/secondary backend ownership: Not started
Cross-platform packaged/native evidence: Not started
Accessibility and responsive behavior: Not started
Performance and reliability measurements: Not started
Supporting infrastructure: Not started
Tests: Not started
Documentation for implementation: Not started
Generated code: Not started
Fixtures/mocks: Not started
Stubs/placeholders: Not started
Incomplete work: Not started
Blocked work: Blocked — P5-WP01 is not accepted and merged and P6-WP01 has no activated prompt
GFD-P6-WP02 and later work: Not started
Diagnostics/export/repair: Not started
Product/customer features: Not started
Product database or remote sync: Not started
Multiple workspace/window framework: Not started
Public SDK/plugin/packages-ui: Not started
Signing/updater/release: Not started
PR creation: Not started
Merge: Not started
Branch deletion: Not started
```

At future completion, update every line independently using only:

- `Implemented`
- `Partially implemented`
- `Stub`
- `Mock-only`
- `Not started`
- `Blocked`

Evidence outcomes may use `Passed`, `Failed`, `Partial`, `Blocked`, or
`Not run` only when explicitly labeled as evidence. Keep functionality,
infrastructure, tests, documentation, generated code, fixtures, stubs,
incomplete work, and blocked work separate. `Partially implemented` and
`Blocked` require a concise reason and evidence reference.

## 15. One RUN_ID and collision-resistant external naming

At the start of the future implementation task, after mandatory authority
reads and preflight but before any repository or deliverable write, generate
exactly one UTC run ID in basic ISO-8601 form:

```text
YYYYMMDDTHHMMSSZ
```

Resolve all four target filenames immediately from that one value and reuse it
unchanged for every implementation deliverable. Never generate or substitute
a second run ID, timestamp, branch label, local time, or random suffix in the
same task.

Before any write, verify none of the four resolved names exists in the
destination. A collision is a stop condition: report `Blocked` without
generating another run ID.

The run ID identifies the evidence set, not the implementation version. The
manifest must record the exact reviewed commit, tree, parent, branch, snapshot
root, and every deliverable digest.

## 16. Required deliverables, hashes, and evidence index

Produce exactly these four future implementation deliverables:

1. `prime-shell-work-gfd-p6-wp01-<RUN_ID>-source-snapshot-r1.zip`
2. `prime-shell-work-gfd-p6-wp01-<RUN_ID>-settings-persistence-single-instance-report-r1.md`
3. `prime-shell-work-gfd-p6-wp01-<RUN_ID>-review-evidence-index-r1.md`
4. `prime-shell-work-gfd-p6-wp01-<RUN_ID>-handoff-manifest-r1.md`

### Source snapshot

- Build it from the exact reviewed final commit.
- Use one top-level `prime-shell/` root and repository-relative paths.
- Include source, schemas, generated bindings, migrations, fixtures, tests,
  scripts, manifests, locks, workflows, and focused documentation needed for
  review.
- Exclude `.git`, environments, caches, build output, installers, native
  captures, temporary/primary/previous settings files, secrets, user data,
  and unrelated files.
- Verify SHA-256, ZIP CRC, path safety, expected root, and exact inventory.
- Extract into a fresh empty directory and rerun complete activated
  verification.

### Settings, persistence, and single-instance report

Record:

- activation, model, reasoning, repository, branch, base/final commit, parent,
  tree, and exact changed paths;
- every accepted predecessor commit, report, artifact, digest, evidence,
  deviation, fallback, limitation, amendment, and blocker;
- before/after settings, layout, contribution, writer, browser-storage,
  startup, instance, window, and backend ownership;
- exact schema, versions, sections, bounds, defaults, validation, migration,
  unknown/future-version behavior, and fixtures;
- exact file/previous-copy locations in path-free evidence form, permissions,
  temp, flush/sync, replace, parent-directory, locking, queue, coalescing,
  revision, recovery, reset, and limitations;
- settings UI/search, contribution integration, state ownership,
  accessibility, and shell regression;
- single-instance mechanism, authority point, startup order, forwarding
  allowlist, parsing, acknowledgement, focus/show/restore, secondary exits,
  and backend ownership;
- privacy, secret, CSP/capability, production-exclusion, and no-replay results;
- exact commands, raw measurements, per-target native/package evidence,
  unavailable results, all 27 gate outcomes, and full status inventory;
- confirmation that P6-WP02, diagnostics/export/repair, product/customer,
  remote-sync, multiple-workspace, release, PR, and merge work did not start.

### Review evidence index

- Include one row for each of the 27 acceptance gates.
- Map each gate to exact source/tests, command and exit status, CI
  run/job/artifact, native/manual evidence, measurement, classification,
  limitation, and digest.
- Keep schema/migration, Rust writer, atomicity/recovery, section reset,
  settings UI, accessibility, single-instance, forwarding, window,
  packaged/native, filesystem, security, performance, snapshot, and
  unavailable evidence separate.
- Record tool, runner/hardware, OS/version/architecture, filesystem, engine,
  package, commit, capture time, expected/actual result, and limitation.
- Use `None`, `Not run`, `Blocked`, or `Partial` explicitly rather than blank
  cells.

### Handoff manifest and hashing

- List all four files with exact filename, role, byte length, and ordinary
  SHA-256 where applicable.
- Record run ID, snapshot root, reviewed commit, tree, parent, branch,
  generation commands, target matrix, tools, paths, exact results, status
  inventory, and next controlled action.
- Record ordinary SHA-256 for the snapshot, report, and evidence index.
- If the manifest records its own digest, designate exactly one self-digest
  value. Copy the final manifest bytes, replace only that value with the
  literal `<SELF_SHA256>`, hash those exact canonical bytes with SHA-256, and
  record the lowercase hexadecimal result in the designated value.
- Reproduce the self-digest by repeating only that replacement without
  changing the field label, spacing, line endings, byte-length field, or any
  other byte.
- Recompute all digests after finalization.
- Never include settings values, corrupt bytes, user content, native paths,
  raw launch inputs, opaque-reference values, credentials, secrets, or other
  sensitive data.

## 17. Branch, draft-PR, review, merge, and deletion controls

This provisional prompt authorizes no implementation branch or PR. Future
activation must:

- start from the exact accepted merged `main` SHA supplied by Chat Session;
- use one fresh implementation branch named by Chat Session;
- never reuse the documentation branch or a predecessor implementation
  branch;
- keep changes limited to one coherent `GFD-P6-WP01` implementation;
- commit intentionally and push only after validation and an immediate remote
  race check;
- verify final remote commit, parent, tree, paths, hashes, and clean parity;
- leave `main` unchanged;
- stop at `READY FOR CHAT SESSION REVIEW`.

Do not open a draft or ready PR unless a later explicit Chat Session
instruction authorizes that separate action. Do not enable auto-merge. Prompt
acceptance is not activation, review acceptance does not authorize merge, and
P6-WP01 completion does not authorize P6-WP02.

Merge, squash, rebase, force-push, history rewriting, branch deletion,
diagnostics export, repair, release, publication, deployment, signing,
notarization, updater work, and repository-setting changes require separate
explicit authorization.

## 18. Completion response and return prompt

The future implementation response must lead with exactly one status:

- `READY FOR CHAT SESSION REVIEW`
- `BLOCKED`
- `NOT STARTED`

For `READY FOR CHAT SESSION REVIEW`, report concisely:

- model `GPT-5.6 Sol` and reasoning `High`;
- repository, starting commit, branch, final commit, parent, and tree;
- exact changed paths and source snapshot;
- all four deliverable filenames and digests;
- all 27 acceptance-gate outcomes;
- schema/migration, Rust writer, atomicity/recovery, section reset, settings
  UI, accessibility, single-instance, forwarding, packaged/native,
  filesystem, security, performance, snapshot, and unavailable evidence;
- raw measurements and full status inventory;
- confirmation that `main` is unchanged and no PR, merge, P6-WP02,
  diagnostics/export/repair, product/customer, remote-sync,
  multiple-workspace, signing/updater, or release work started.

For `BLOCKED`, report the exact stop condition, command or evidence,
repository state, changed files, unaffected scope, affected gates, and
smallest required Chat Session decision. Do not claim partial work as
implementation success.

End a successful future implementation response with:

```text
Chat Session: Review GFD-P6-WP01 on the exact implementation branch and commit
reported above. Read the settings/persistence/single-instance report, review
evidence index, handoff manifest, and verified source snapshot. Return
Accepted, Focused correction required, or Blocked. Confirm one canonical
schema-versioned settings document, Rust sole-writer authority, accepted P2
layout and P4/P5 contribution integration, deterministic migrations, atomic
write and previous-valid-copy recovery, section-scoped reset/recovery,
searchable accessible in-window settings, real packaged single-instance
behavior, safe focus/show/restore and allowlisted forwarding, path and launch
input secrecy, one backend owner, no replay, platform limitations, raw
measurements, status inventory, and artifact hashes. GFD-P6-WP02,
diagnostics/export/repair, product/customer work, PR creation, merge, release,
and branch deletion remain unauthorized.
```
