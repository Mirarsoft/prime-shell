# GFD-P6-WP02 — Diagnostics, Repair, and Recovery

## 1. Package identity and prompt status

**Package ID:** `GFD-P6-WP02`
**Phase:** `Phase 6`
**Title:** `Diagnostics, Repair, and Recovery`
**Task ID:** `GFD-P6-WP02`
**Prompt ID:** `PRIME-SHELL-GFD-P6-WP02-PROMPT`
**Prompt version:** `R1`
**Prompt lifecycle state:** `Provisional`
**Implementation status at authoring:** `Not started`
**Recommended future implementation model:** `GPT-5.6 Sol`
**Recommended future implementation reasoning/intelligence:** `Extra High`
**Authorization boundary:** Exactly one package, `GFD-P6-WP02`
**Execution status:** `Execution is not authorized`

Extra High is required because this package crosses local logging, bounded
retention, redaction, user-selected export, settings repair, backend recovery,
task no-replay, process lifecycle, native file intent, filesystem permissions,
accessibility, and production-exclusion boundaries. A small ambiguity could
export private content, expose a native path, retain unbounded logs, corrupt
unrelated settings, restart the backend outside its circuit policy, or imply
that interrupted work succeeded.

This provisional prompt is complete for review but non-executable. Chat
Session must first accept it as an Approved provisional prompt and later issue
a separately activated revision after every prerequisite is accepted and
merged. Prompt acceptance is not activation, implementation authorization,
PR or merge authorization, permission to begin `GFD-P7-WP01`, or permission
to begin product, customer, release-hardening, signing, updater, publication,
or deployment work.

**Direct prerequisite:** Accepted and merged `GFD-P6-WP01`, including its
canonical Rust-owned settings authority, migrations, atomic persistence,
previous-valid-copy and section recovery, searchable settings, single-instance
behavior, implementation report, evidence, and source.

**Direct dependent:** `GFD-P7-WP01`, which remains unauthorized.

### Activation metadata

```text
Activation ID: Not activated — Chat Session must refresh and supply this exact value.
Activated by: Not activated — Chat Session must refresh and supply this exact value.
Activation UTC: Not activated — Chat Session must refresh and supply this exact value.
Authoritative main SHA: Not activated — Chat Session must refresh and supply this exact value.
Required fresh implementation branch: Not activated — Chat Session must verify absence and supply this exact value.
Accepted predecessors through P6-WP01: Not activated — Chat Session must supply exact merged commits, reports, artifacts, hashes, CI runs/jobs, native/manual evidence, deviations, fallbacks, limitations, and blockers.
Accepted P3 logging/task/backend/process authority: Not activated — Chat Session must supply exact schemas, limits, safe errors, trace policy, runtime states, restart/circuit/no-replay rules, process evidence, and accepted owners.
Accepted P6-WP01 settings authority: Not activated — Chat Session must supply exact schema/version, sections, migrations, writer, revisions, atomic-write, previous-copy, recovery/reset commands, settings UI, and evidence.
Current structured logging surfaces: Not activated — Chat Session must supply exact Rust, Python, frontend, sidecar, package, and test paths plus current fields and bounds.
Diagnostic record schema and safe event-code allowlist: Not activated — Chat Session must verify and supply exact values.
Retention and rotation policy: Not activated — Chat Session must supply exact byte, file, record, age, queue, rotation, cleanup, and failure bounds.
Filesystem permissions and durability semantics: Not activated — Chat Session must supply target-specific create, permission, temp, flush/sync, replace, delete, and limitation facts.
Recent-safe-error and backend-status contracts: Not activated — Chat Session must supply exact fields, limits, state mapping, trace behavior, and owners.
Diagnostics UI route and shell host: Not activated — Chat Session must supply exact contribution, state, focus, responsive, and accessibility paths.
Export preview and allowlist: Not activated — Chat Session must supply exact categories, fields, exclusions, bounds, ordering, and frozen-snapshot behavior.
Export format and native save intent: Not activated — Chat Session must supply exact archive/container format, manifest version, file names, compression limits, picker purpose, extension, cancellation, and opaque-result behavior.
Backend recovery commands: Not activated — Chat Session must supply exact accepted status, restart, retry, circuit, concurrency, deadline, and process-cleanup rules.
Settings reset and repair commands: Not activated — Chat Session must supply exact setting/section/all reset and safe repair operations, preview, future-version behavior, and preservation rules.
Privacy marker and redaction corpus: Not activated — Chat Session must supply deterministic synthetic values, encoded variants, failure policy, and inspection commands.
Accepted target and evidence matrix: Not activated — Chat Session must refresh OS/version/architecture, engine, package, filesystem, archive tool, save dialog, runner, native/manual capability, and owner facts.
Accepted targeted amendments: Not activated — Chat Session must refresh and supply the exact list or None.
Predecessor deviations incorporated: Not activated — Chat Session must refresh and supply the exact list or None.
Unresolved blockers and assumptions: Not activated — Chat Session must refresh and supply the exact list or None.
Authorization boundary: Not activated — a future activation may authorize only GFD-P6-WP02.
Authorization invalidates when: Not activated — a future activation must invalidate on any base-SHA, predecessor acceptance, evidence, schema, logging, retention, redaction, export, repair, restart, process, settings, permission, target, tool, repository-layout, or authorization-boundary change.
```

## 2. Repository and exact starting state

**Repository:** `Mirarsoft/prime-shell`
**Authoritative base branch:** `main`
**Required future starting commit:** Not activated — Chat Session must refresh
and supply the exact accepted and merged `main` SHA after `GFD-P6-WP01`
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
- approved-provisional P6-WP01 prompt documentation head:
  `8f35f88dac96f9ab2921ecd1962a59f3102e40f9`;
- P6-WP01 prompt tree:
  `620210835bf5b6ee1b1f0db18e915534f575f0f5`;
- P6-WP01 prompt blob:
  `aff180f8454d44080823913c47393e7083aa0539`;
- P6-WP01 prompt SHA-256:
  `8cced912cfc0ecd3ff675f33732e0f9d8a3ceb9f1c7d00c2dddff4cbc4d2c438`;
- accepted P6-WP01 artifact-only correction run:
  `20260728T180059Z`;
- corrected P6-WP01 handoff manifest canonical self-hash:
  `1588e50a8ae8d41264238d69a6eb3503796495d3ce51c97ed35b123945ec6fd5`;
- prompt-pack documentation branch:
  `docs/gfd-work-package-prompt-pack-v1`;
- predecessor activation and implementation through P6-WP01: `Not started`;
- P6-WP02 implementation: `Not started`.

The authoring-time merged source remains the accepted WP01 echo spike. It has
bounded protocol frames, a small Python structured-stderr path, safe errors,
and a packaged sidecar lifecycle, but it has no accepted productized log
repository, diagnostic record schema, retention store, diagnostics UI,
preview/export flow, settings repair integration, or user-facing backend
recovery surface. Future activation must refresh these paths and facts from
accepted merged predecessor implementations rather than invent them.

Required activation and Work Session preflight:

1. Verify exact access and write permission for `Mirarsoft/prime-shell`.
2. Record the default branch and its exact 40-character SHA.
3. Verify WP01 and every package through P6-WP01 are accepted and merged.
4. Verify every accepted implementation report, evidence index, manifest,
   source snapshot, digest, CI run/job/artifact, native/manual result,
   measurement, deviation, fallback, limitation, amendment, and blocker.
5. Verify P3 structured logging, safe errors, task/backend state, restart,
   circuit, process containment, native intent, path secrecy, and no-replay
   authority.
6. Verify P6-WP01 settings schema, writer, migration, atomicity, recovery,
   reset, single-instance, startup, and window authority.
7. Inventory current logs, queues, storage, rotation, status, safe errors,
   diagnostic UI, native export, archive tooling, settings repair, backend
   recovery, capabilities, CSP, packages, tests, workflows, and target claims.
8. Verify no P6-WP02 diagnostics/export/repair implementation, telemetry,
   remote support, customer workflow, release hardening, or successor work
   exists unexpectedly.
9. Verify the named fresh implementation branch is absent.
10. Use a clean fresh clone or worktree at the activated starting commit.
11. Narrow every allowed and protected path against that exact merged tree.
12. Stop if state differs from activation, is ambiguous, would overwrite
    unrelated work, or would require rebasing or rewriting history.

The merged repository is authoritative over snapshots, documentation
branches, unmerged implementation branches, chat summaries, provisional
prompts, and superseded handoffs.

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
10. accepted WP01 source, review, reports, evidence, manifest, snapshot,
    hashes, CI jobs, and native evidence named by activation
11. accepted P0B-WP02 and P0B-WP03 prompts, activations, sources, reviews,
    reports, manifests, snapshots, lifecycle/process evidence, measurements,
    deviations, fallbacks, and blockers
12. accepted P1-WP01 through P5-WP01 prompts, activations, sources, reports,
    manifests, snapshots, platform/native evidence, measurements, deviations,
    fallbacks, and blockers
13. accepted P3-WP01 and P3-WP02 schemas, safe errors, operation authority,
    native intent, task races, structured logs, process containment,
    restart/circuit/no-replay policy, package identities, and evidence
14. accepted P6-WP01 prompt, activation, source, report, evidence, manifest,
    snapshot, schema, migrations, writer, atomicity, previous-copy,
    section-recovery, reset, single-instance, forwarding, measurements,
    deviations, and blockers
15. every separately accepted targeted architecture amendment named by
    activation
16. current logging, diagnostic record, safe-error, task/backend status,
    retention, rotation, settings repair, native export, archive, capability,
    CSP, production-exclusion, package, test, workflow, toolchain, manifest,
    lockfile, and support-claim surfaces
17. the exact activated revision of this prompt

The first implementation progress message must state:

> `AGENTS.md, repository authority, accepted Phase 0A, accepted and merged
> predecessors through GFD-P6-WP01, their accepted reports and artifacts, the
> accepted P3 logging/task/backend/process authority, the accepted P6-WP01
> settings/recovery/single-instance authority, and the activated GFD-P6-WP02
> prompt have been read and are active. Executing only GFD-P6-WP02 with
> GPT-5.6 Sol / Extra High. GFD-P7-WP01, product/customer work, release work,
> PR creation, and merge remain unauthorized.`

If a required source is missing, unreadable, stale, or materially
inconsistent, stop before writing. Do not substitute a broad architecture
review, rerun predecessors as acceptance evidence, or invent future log
formats, export APIs, filesystem permissions, repair semantics, restart
authority, target support, or release claims.

## 4. Dependencies, predecessor outputs, and prerequisite evidence

**Direct prerequisite:** Accepted and merged `GFD-P6-WP01`, including one
canonical settings document, Rust sole-writer authority, deterministic
migrations, atomic write and previous-valid-copy recovery, section-scoped
reset/recovery, searchable settings, single-instance behavior, safe
forwarding, reports, artifacts, native/package evidence, measurements,
deviations, and blockers.

**Inherited prerequisites:** Accepted and merged WP01, P0B-WP02, P0B-WP03,
P1-WP01, P2-WP01, P2-WP02, P3-WP01, P3-WP02, P4-WP01, and P5-WP01 source
and evidence.

**Direct dependent:** `GFD-P7-WP01`, which remains unauthorized.

P6-WP02 cannot activate while any prerequisite exists only as a provisional
prompt, an unmerged branch, an unaccepted report, stale platform evidence, or
an unresolved blocker affecting logging, privacy, settings recovery, backend
recovery, process containment, or native export.

Activation must supply exact:

- accepted merged commits and tree identities;
- implementation reports, evidence indexes, manifests, snapshots, byte
  lengths, digests, CI identities, native/manual evidence, and limitations;
- P3 trace IDs, safe errors, structured log fields and bounds, stderr policy,
  task/backend statuses, restart/circuit/no-replay, and process authority;
- P6-WP01 settings versions, sections, writer, revisions, migrations,
  previous copy, reset/recovery commands, future-version policy, startup, and
  single-instance authority;
- current Rust and Python log producers, queues, sinks, files, fields, levels,
  event codes, truncation, and retention behavior;
- diagnostic record schema, record and field bounds, allowlist, redaction,
  ordering, and status mapping;
- retention file/count/byte/age bounds, rotation, permissions, durability,
  cleanup, and failure behavior;
- exact diagnostics UI contribution, preview behavior, export allowlist,
  archive/container format, manifest version, file names, size/count limits,
  and native save intent;
- backend status/recovery commands, concurrency, restart budget, circuit,
  deadlines, task interruption, and descendant-cleanup behavior;
- settings repair/reset commands, preview, section preservation,
  previous-copy behavior, future-version safety, and stale-temp cleanup;
- supported targets, filesystems, engines, package formats, archive tools,
  runners, manual owners, and unavailable evidence;
- exact performance budgets, privacy corpus, deviations, fallbacks,
  amendments, and unresolved blockers.

Acceptance of this prompt does not accept predecessor implementation by
inference, activate this package, authorize P7-WP01, or approve telemetry,
remote support, customer diagnostics, enterprise export, product features,
release hardening, signing, updater, publication, or deployment.

## 5. Objective and measurable runnable outcome

Provide a local, privacy-preserving diagnostics and recovery surface that lets
the user understand safe application/backend state, inspect exactly what
would be exported, explicitly select an export destination, recover the
backend within accepted runtime authority, and perform targeted settings
repair without damaging unrelated valid data.

The future package must prove this exact order in one real installed or
packaged Tauri journey:

```text
accepted P3 structured status and P6-WP01 durable settings authority
→ bounded local diagnostic records retained under Rust policy
→ in-window diagnostics surface shows safe status and recent safe errors
→ sanitized export inventory is frozen
→ sanitized preview shown before export
→ user explicitly selects export destination through Rust-owned native intent
→ bounded local bundle and versioned manifest written
→ exported bundle inspected and verified against the allowlist
→ user triggers one bounded backend recovery action
→ runtime reports truthful readiness without replaying interrupted work
→ user previews and triggers one targeted settings reset or repair
→ unrelated valid settings remain unchanged across restart
```

Planning, type declarations, fabricated logs, browser-only storage, a raw log
viewer, a mock save dialog, an archive fixture presented as current output, a
restart button that bypasses P3 authority, or a whole-file settings rewrite
cannot satisfy the future outcome.

The package remains local and product-neutral. Diagnostics support review and
user self-recovery only. They do not become telemetry, automatic crash upload,
remote support, an enterprise support bundle, a customer workflow, a release
updater-repair system, or a general-purpose data export platform.

## 6. Explicit in-scope work

The activated Work Session may perform only the smallest coherent change set
needed for the following work.

### 6.1 Evidence-driven inventory and ownership

Inventory every Rust, Python, and frontend log producer and consumer; trace
and request IDs; safe errors; task and backend status; settings recovery;
native save/export intent; filesystem retention; permissions; capabilities;
test controls; and production exclusions.

Classify each component as retained, adapted, consolidated, removed,
test-only, or blocked. Record one authoritative owner for every mutable
diagnostic datum:

- Rust owns validation, retention, rotation, redaction enforcement, export
  selection, filesystem permissions, native save intent, settings repair,
  backend recovery authorization, and user-visible status snapshots.
- Python may emit only accepted bounded protocol-compliant structured records
  to stderr. It owns no retained log file, export destination, settings
  mutation, or restart policy.
- React owns presentation, trusted search/filter input, preview confirmation,
  and explicit user intent only. It receives no native path or unredacted
  backing record.

Use one finite export allowlist. Remove or reject duplicate log repositories,
browser persistence, alternate writers, raw exception channels, and implicit
collection paths.

### 6.2 Structured local logging boundary

Define one versioned bounded diagnostic record schema. Activation must fix:

- finite event code and component allowlists;
- allowed levels and stable safe result/status codes;
- bounded timestamp representation;
- optional bounded request, task, and trace correlations;
- bounded numeric durations, counters, queue depths, and lifecycle states;
- exact maximum encoded record, string, collection, and field counts;
- deterministic unknown-field, invalid-value, oversized-record, and
  truncation behavior;
- stable ordering where retained files or export digests require it.

Never record document content, user-entered text, settings values, native
paths, raw argv, URIs, environment values, secrets, credentials, tokens,
opaque-reference backing values, unrestricted Python tracebacks, source
paths, arbitrary payloads, or caller-supplied free-form metadata.

Rust must validate and bound Python records before display, retention, or
export. Invalid Python records fail safely and cannot poison stdout protocol,
block lifecycle drains, or expand the allowlist.

### 6.3 Bounded retention, rotation, and permissions

Activation must fix exact:

- maximum active log bytes;
- maximum rotated files and aggregate bytes;
- maximum records and queue capacity;
- optional maximum age and the clock/cleanup rule if age is used;
- rotation trigger and deterministic naming;
- restrictive user-only permissions where supported;
- same-directory temporary, flush/sync, replace, and cleanup behavior;
- behavior on disk full, permission denial, partial write, rename, rotation,
  deletion, and stale-temporary failure;
- startup cleanup and shutdown flush deadlines;
- memory-only fallback policy, if any, with explicit bounded limits.

Retention must be local and finite. No background daemon, scheduled upload,
cloud copy, hidden backup, unbounded archive, or automatic export is allowed.
Failures must not block protocol drains, settings writes, task cancellation,
backend shutdown, or application exit.

### 6.4 Recent safe errors and backend status

Expose only bounded typed safe summaries:

- current backend lifecycle and circuit state;
- accepted build/protocol/schema identity in safe bounded form;
- recent safe error code, component, event time, trace ID where useful, and
  one finite user-facing recovery hint;
- queue or retention saturation as a safe state;
- settings load/migration/recovered/unsupported/unavailable state;
- last user-triggered export, backend recovery, or settings repair result
  without native path or sensitive values.

Use bounded history, deterministic newest/oldest behavior, stable safe codes,
and explicit unavailable/unknown states. Do not expose raw log lines,
tracebacks, payloads, settings contents, file locations, arguments, or
backend internals.

### 6.5 Diagnostics and recovery UI with preview

Add one in-window diagnostics and recovery surface through the accepted P5
static contribution contract and existing shell. It may contain:

- safe application/backend/settings status;
- a bounded recent-safe-error list;
- trusted bounded filters over safe static fields;
- retention and privacy-policy summary;
- export preview listing exact allowlisted categories, files, record counts,
  byte estimates, omissions, and redaction verification state;
- explicit export action;
- bounded backend recovery action when accepted status permits it;
- targeted setting, section, and all-settings reset/repair actions only where
  accepted P6-WP01 commands permit them;
- clear success, cancelled, failed, unavailable, circuit-open, and blocked
  states.

Preview must precede export. Opening the surface must not start hidden
collection, export, repair, restart, upload, network access, or destructive
work. Destructive settings actions require an affected-section preview and
clear confirmation proportional to impact.

### 6.6 User-selected diagnostics export

Use a fixed-purpose Rust-owned native save/export intent with:

- one activation-approved file extension and container format;
- a trusted suggested base name containing no user or path data;
- a fixed picker purpose and filters;
- typed cancellation as a normal outcome;
- no raw path returned to React or Python;
- safe overwrite policy and bounded temporary output;
- explicit user action after a valid preview;
- a frozen sanitized snapshot so preview and export inventory agree;
- bounded file count, entry size, total uncompressed bytes, compressed bytes,
  compression ratio, name length, and generation time;
- safe deterministic entry names with no traversal, absolute paths,
  alternate separators, links, devices, or duplicate canonical names;
- cleanup of partial output after failure or cancellation;
- an opaque safe export result or artifact reference rather than a path.

The exporter may include only allowlisted sanitized diagnostic records,
versioned safe status, a versioned export manifest, and narrowly approved
non-sensitive build/package metadata. It must exclude prior exports, raw
settings files, previous/corrupt settings bytes, native paths, content,
arguments, screenshots, dumps, secrets, credentials, user data, arbitrary
environment details, caches, databases, and unrelated files.

No automatic, scheduled, background, silent, recursive, remote, or
support-service export is allowed.

### 6.7 Versioned export manifest and allowlist

Require a versioned manifest containing only safe metadata:

- manifest schema/version;
- application and backend safe build identities;
- target OS/architecture and package identity at the approved granularity;
- export creation time under the activated deterministic representation;
- diagnostic schema version and retention-policy version;
- each included entry name, role, byte length, and SHA-256;
- record counts, safe event-code counts, and bounded time range;
- explicit excluded categories;
- redaction-verification result;
- generation tool/version and exact package commit;
- limitations and unavailable evidence classifications.

The manifest and archive inventory must be deterministic for a frozen
snapshot except for explicitly activated time fields. Verify entry digests,
safe names, duplicates, total bounds, extraction path safety, and absence of
unlisted files. A manifest cannot grant authority to include a category that
is absent from the Rust allowlist.

### 6.8 Backend recovery within accepted runtime authority

Provide only finite user-triggered actions already permitted by P3 runtime
authority, such as:

- refresh/query status;
- request the accepted explicit restart when the runtime permits it;
- request the accepted circuit recovery or retry after policy checks.

Rust must enforce lifecycle state, circuit, restart budget, concurrency,
deadline, window authority, package identity, and process containment. Reject
duplicate or rapid concurrent recovery requests deterministically. One
accepted restart restores availability only; it does not retry, replay,
resume, or mark any interrupted task successful. Repeated failure remains
circuit-open or faulted under accepted policy.

Verify bounded acknowledgement, readiness or failure, affected-task
classification, sidecar generation identity, shutdown escalation, and zero
surviving descendants. Do not add a new restart loop, process manager,
sidecar selector, system-Python fallback, generic command, or background
health daemon.

### 6.9 Targeted settings repair and reset

Use only finite typed P6-WP01 commands and Rust sole-writer authority:

- validate/reload current settings status;
- reset one setting;
- reset one section;
- reset all settings;
- restore a valid previous copy only under accepted recovery policy;
- clear stale bounded temporary settings artifacts under Rust authority.

Activation must state which actions are reset, recovery, or cleanup rather
than calling all of them repair. Preview affected setting IDs or sections,
expected defaults, preservation boundary, restart/apply behavior, and
limitations before a destructive action. Preserve unrelated valid sections
and the authoritative Rust revision.

Never silently downgrade or overwrite an unsupported future-version document.
Never expose or edit raw JSON, corrupt bytes, file paths, arbitrary keys, or
whole-document payloads. Do not add a settings import/export format, backup
browser, corrupt-file viewer, repair scripting language, generic mutation
API, or second writer.

### 6.10 Recovery truth and no replay

Keep these outcomes independent and truthful:

- backend restart restores readiness only;
- interrupted or uncertain tasks remain `Interrupted` and are never replayed;
- settings reset or recovery changes only the activated settings scope;
- settings repair does not repair, retry, or replay tasks;
- diagnostics export reads a frozen sanitized snapshot and changes no runtime
  or settings state;
- export success does not prove backend or settings health;
- recovery acknowledgement does not prove eventual readiness;
- partial or failed export leaves no reported successful artifact;
- application restart restores no stale task or native reference.

Do not collapse `Cancelled`, `TimedOut`, `Interrupted`, `Failed`,
`Unavailable`, `CircuitOpen`, `Recovered`, `Reset`, and `Exported` into one
generic success/failure state.

### 6.11 Privacy and redaction verification

Activation must define deterministic synthetic privacy markers for every
prohibited category and representative encoded variants, including:

- native paths and file URLs;
- user-entered document text;
- settings values and corrupt bytes;
- raw argv, URI, working directory, and environment;
- credentials, tokens, secrets, and proxy variables;
- opaque-reference backing values;
- Python traceback and source locations;
- arbitrary request/response payloads.

Verify markers do not appear in UI, retained logs, preview, export files,
archive entry names, manifest, screenshots, CI artifacts, or external
deliverables. Inspect decoded text, JSON strings, archive inventory, and
approved encoded forms. Redaction failure must block retention or export
according to the activated safe policy; it must never silently pass or merely
hide leaked content in the UI.

Redaction must be schema/allowlist based. Avoid a claim that regular-expression
replacement alone can make arbitrary payloads safe.

### 6.12 Accessibility and responsive behavior

- Use logical diagnostics, status, export, backend recovery, and settings
  repair headings and regions.
- Provide accessible names/descriptions for filters, preview, export,
  cancellation, refresh, restart, retry, reset, repair, confirmations, and
  result links or references.
- Restore focus deterministically after preview, picker cancellation, export
  completion/failure, recovery completion/failure, confirmation dismissal,
  settings reset/repair, and panel or route transitions.
- Announce terminal results and safe status changes without reading high-rate
  log activity.
- Use non-color-only severity, recovery, preview, and destructive-action
  communication.
- Preserve forced colors, reduced motion/transparency, 200% scaling, text
  expansion, RTL, visible focus, minimum targets, keyboard operation, and the
  500-pixel compact shell.
- Keep browser automation, native focus/save-dialog, screen-reader, and manual
  evidence classifications separate.

### 6.13 CSP, capabilities, and production exclusion

Use finite named typed commands and least-privilege exact window permissions.
Preserve release CSP, safe text rendering, portal ownership, path secrecy,
native-intent ownership, and network denial.

Production source and packages must exclude:

- telemetry, crash-reporting, analytics, remote-upload, support-agent, or
  network SDKs and endpoints;
- raw log viewers, raw file readers, arbitrary export paths, generic archive
  or filesystem commands, generic invoke or repair tunnels;
- test fault injectors, privacy-marker injectors, evidence writers, drivers,
  debug recovery bypasses, and alternate writers;
- developer/system-Python fallback, caller-selected backend, shell commands,
  process arguments, and inherited credentials;
- release, signing, notarization, updater, store, deployment, and publication
  changes.

Tests alone do not prove production exclusion. Inspect source, capability and
permission files, bundled resources, binaries, archive inventories, and
installed package contents.

### 6.14 Cross-platform and packaged-runtime truth

Activation must refresh target OS/version/architecture, engine, package,
filesystem, permission model, archive/save-dialog tooling, runner, and manual
owner. Keep separate:

- schema and unit evidence;
- retention, rotation, permission, and failure-harness evidence;
- source-run Tauri behavior;
- package creation and package-content inspection;
- installed/native diagnostics UI;
- real native save dialog, cancellation, and export;
- archive inventory, extraction, digest, and privacy inspection;
- backend recovery, process containment, and descendant cleanup;
- settings repair, restart, and persistence evidence;
- WebView2, WKWebView, and WebKitGTK behavior;
- accessibility and manual evidence;
- CI and unavailable/blocked evidence.

One target cannot substitute for another. A unit test, browser mock, source
build, archive fixture, screenshot, package file, or previous P3/P6 result
cannot substitute for current installed/native export, backend recovery, or
settings repair evidence.

### 6.15 Performance and reliability measurements

Retain exact method, sample count, target, engine, filesystem, archive tool,
hardware/runner, activated budget, raw values, and classification for:

- record validation and enqueue;
- queue and retained-state high-water marks;
- rotation and retention cleanup;
- startup log-store validation;
- diagnostics route and recent-error rendering;
- filter/search latency;
- preview generation;
- export generation for small and maximum synthetic datasets;
- archive compression ratio and memory high-water mark;
- native picker cancellation;
- export cancellation and failed-temporary cleanup;
- backend recovery acknowledgement, shutdown, readiness, and cleanup;
- setting, section, and all-settings reset/recovery;
- application restart after settings recovery;
- host/sidecar memory and package-size delta;
- normal shutdown and zero-descendant observation.

Measure rather than invent universal service levels. Do not hide a miss by
changing the workload, target, filesystem, archive format, or classification.

### 6.16 Focused documentation and evidence

Document only the final local diagnostic ownership, record schema and codes,
allowlist, retention/rotation/permissions, recent-safe-error model,
preview/export, archive manifest, privacy exclusions, backend recovery,
settings repair, no-replay rules, target limitations, exact commands, and
review evidence. Do not create customer-support, enterprise, telemetry,
remote-operations, product, or release documentation.

## 7. Explicit exclusions and prohibited adjacent work

The activated package must not:

- activate, repair, reimplement, or rerun predecessors as substitute work;
- begin `GFD-P7-WP01` or any other package;
- create product/customer features, support workflows, accounts,
  collaboration, or customer-specific data;
- add telemetry, analytics, crash upload, automatic reporting, remote
  diagnostics, remote support, cloud sync, network endpoints, or background
  collection;
- add an enterprise support bundle, compliance export, database, data lake,
  unrestricted system inventory, process list, registry dump, environment
  dump, memory dump, or full settings export;
- export document/user content, native paths, raw settings, corrupt bytes,
  previous copies, raw logs, arguments, URIs, credentials, secrets, opaque
  references, source paths, arbitrary payloads, screenshots, caches, or prior
  exports;
- add scheduled/background/automatic export, upload, export without preview
  and explicit user action, recursive collection, or arbitrary destinations;
- add a raw log viewer, arbitrary file reader, generic archive builder,
  generic filesystem/save command, arbitrary export include list, or plugin
  exporter;
- add generic repair scripting, raw settings editing, settings import/export,
  backup browsing, corrupt-file viewing, arbitrary key mutation, or a second
  settings writer;
- bypass accepted backend lifecycle, circuit, restart budget, task terminal
  state, no-replay, process containment, or package identity;
- create a new background daemon, watchdog service, multiple backend workers,
  arbitrary subprocess launcher, or general process manager;
- redesign the shell, settings host, feature registry, task runtime,
  single-instance model, theme, title bar, or responsive behavior;
- create a public SDK, `packages/ui`, runtime plugin system, generator,
  marketplace, compatibility promise, or third-party extension point;
- weaken CSP, capabilities, permissions, bounds, redaction, safe rendering,
  path secrecy, single-writer ownership, or production exclusion;
- invent unsupported target, filesystem, permission, archive, accessibility,
  performance, support, or durability claims;
- perform opportunistic dependency upgrades, broad refactors, formatting
  sweeps, or unrelated cleanup;
- add signing, notarization, updater repair, release hardening, publication,
  deployment, store, or repository-setting work;
- author or modify another work-package prompt;
- open a PR, enable auto-merge, merge, force-push, rewrite history, or delete
  a branch.

A demonstrated predecessor defect, missing accepted privacy/export decision,
or required architecture change is a stop condition unless Chat Session
separately accepts and activation names one exact focused correction.

## 8. Allowed repository areas and expected changes

Future activation must replace these categories with an exact narrow path
allowlist from the accepted merged tree. Expected areas are no broader than:

```text
packages/app-contracts/schemas/              # finite diagnostics/status/export command schemas
packages/app-contracts/fixtures/             # valid, invalid, privacy, and bounded fixtures
accepted generated binding locations         # mechanically required outputs only
apps/desktop/src/diagnostics/                 # in-window diagnostics/recovery UI
apps/desktop/src/settings/                    # exact repair/reset contribution only
apps/desktop/src/backend/                     # typed status/recovery/export adapters
apps/desktop/src/state/ or accepted path      # one bounded diagnostics-facing state authority
apps/desktop/src-tauri/src/diagnostics/       # allowlist, preview, export, retention
apps/desktop/src-tauri/src/logging/           # accepted structured sink integration only
apps/desktop/src-tauri/src/settings/          # typed repair/reset integration only
apps/desktop/src-tauri/src/backend/           # accepted status/recovery integration only
apps/desktop/src-tauri/src/native/            # fixed-purpose export intent only
apps/desktop/src-tauri/src/commands/          # finite named diagnostics/recovery commands
apps/desktop/src-tauri/capabilities/          # least-privilege exact commands
apps/desktop/src-tauri/permissions/           # least-privilege exact commands
services/python-backend/                      # bounded accepted structured record fields only
scripts/verify/                               # privacy/export/retention/process checks
tests/e2e/ and tests/platform-smoke/          # packaged diagnostics/recovery journeys
.github/workflows/                            # minimal activated target evidence only
manifests, lockfiles, build configs           # mechanically required companions only
focused diagnostics/recovery documentation   # local policy, usage, evidence
```

Activation must remove nonexistent or unused entries, identify actual
accepted owners rather than duplicating them, and list every allowed and
protected path exactly.

Protected by default:

- architecture authority, Stage 1 files, prompt files, accepted decisions,
  predecessor reports, evidence, and snapshots;
- P2 shell/theme behavior and P5 contribution authority except the exact
  diagnostics contribution;
- P3 operation, task, logging, process, native-intent, path, and no-replay
  authority except exact typed integrations;
- P6-WP01 settings, migration, writer, atomicity, recovery, reset,
  single-instance, and startup authority except exact typed repair integration;
- unrelated product data, routes, features, commands, settings, and backend
  operations;
- public API/package boundaries and all release surfaces.

Every changed path must appear in the implementation report with purpose and
authority. A path outside the activated allowlist is a stop condition unless
it is a mechanically required generated or locked companion explicitly
authorized. Generated caches, build products, packages/installers, native
captures, logs, exports, settings files, temporary files, secrets, user
content, and unrelated artifacts must not be committed.

## 9. Ordered implementation procedure

After a separate activation, execute exactly in this order:

1. Read all authority and accepted predecessor evidence; issue the exact
   acknowledgement.
2. Verify repository, exact accepted merged `main`, fresh branch, permissions,
   clean tree, path allowlist, targets, and activation freshness.
3. Generate exactly one task-start RUN_ID after authority/preflight and before
   repository or deliverable writes; resolve four names and stop on collision.
4. Inventory logs, safe errors, task/backend status, retention, settings
   repair, native export, capabilities, packages, test controls, target
   tooling, and current ownership.
5. Run accepted P3 and P6-WP01 logging, lifecycle, process, settings,
   persistence, recovery, CSP, capability, package, and native baselines.
6. Freeze exact schemas, event codes, allowlists, redaction, retention,
   permissions, export format, archive bounds, repair/recovery commands,
   privacy corpus, target matrix, and evidence budgets.
7. Produce a minimal consolidation map with one final owner for every retained,
   adapted, removed, test-only, duplicate, unavailable, and blocked surface.
8. Implement bounded structured records and Rust validation for every accepted
   producer without broad free-form fields or protocol/log mixing.
9. Implement Rust-owned bounded retention, rotation, permissions, failure
   behavior, recent safe errors, and safe status snapshots.
10. Implement the in-window diagnostics/recovery UI with trusted filters,
    accessible status, and preview-before-export.
11. Implement fixed-purpose Rust-owned export with frozen sanitized snapshot,
    native save intent, versioned manifest, archive limits, inspection, and
    failed-temporary cleanup.
12. Integrate accepted backend recovery without replay, false task success,
    circuit bypass, duplicate process ownership, or descendant leakage.
13. Integrate typed settings reset/repair with affected-scope preview,
    future-version safety, authoritative revisions, and unrelated-section
    preservation.
14. Add deterministic privacy-marker, bounds, failure, race, accessibility,
    production-exclusion, archive, package, and native tests.
15. Run fast schema/TypeScript/Rust/Python checks continuously, followed by
    exact source, package, installed/native, archive, process, and manual
    journeys for each activated target.
16. Measure raw retention, preview, export, restart, repair, memory, package,
    and cleanup results and classify every miss or unavailable target.
17. Inspect final source, capabilities, permissions, packages, archives, UI,
    screenshots, logs, CI artifacts, and external deliverables for prohibited
    data and production-only exclusions.
18. Reconcile the final diff to the activated allowlist; remove duplicate
    stores, broad metadata, dead scaffolding, test leakage, unrelated
    refactors, and hidden stubs; rerun all checks.
19. Commit and push only the activated implementation branch; verify remote
    parity and leave `main` unchanged without opening a PR.
20. Build the source snapshot from the final commit; verify digest, CRC, path
    safety and one root; extract into a fresh empty directory; rerun complete
    activated verification.
21. Finalize exactly four deliverables, verify ordinary hashes and canonical
    manifest self-hash, return the section 18 handoff, and stop without
    entering P7-WP01.

Never solve a failed gate by adding telemetry, remote support, product data,
generic export/repair, another backend owner, release work, or a successor
package.

## 10. Cross-cutting constraints

### Ownership and authority

- Rust owns retained diagnostic state, validation, redaction enforcement,
  retention, rotation, permissions, preview inventory, export, native intent,
  backend recovery authorization, settings repair, and persistent writes.
- Python emits only bounded accepted structured records and owns no retained
  file, export, settings mutation, or restart policy.
- React owns presentation, bounded filters, confirmations, and explicit user
  intent only.
- Accepted P3 runtime/process/no-replay and P6-WP01 settings/single-writer
  authority remain canonical.
- Every mutable datum has one writer and one authoritative bounded snapshot.

### Bounds and deterministic behavior

- Activation fixes schemas, fields, codes, bounds, queues, files, rotation,
  cleanup, deadlines, archive rules, allowlists, and safe outcomes.
- Enforce byte/count/time bounds before allocation, parsing, retention,
  preview, compression, extraction, repair, or recovery admission.
- Reject unknown, invalid, oversized, stale, duplicate, ambiguous, and
  conflicting inputs deterministically.
- Use no last-write-wins, silent truncation presented as complete, hidden
  retry, silent repair, auto-inclusion, or unbounded fallback.

### Privacy and security

- Allowlist safe fields rather than trying to redact arbitrary payloads.
- Keep content, values, paths, argv, URIs, environment, secrets, credentials,
  references, raw errors, and arbitrary payloads out of all diagnostic layers.
- Preserve release CSP, network denial, safe text rendering, path secrecy,
  finite typed commands, and least-privilege capabilities.
- A redaction or allowlist verification failure blocks export.
- No diagnostic or repair metadata grants native/backend authorization.

### Recovery and no replay

- Backend restart restores availability only.
- First accepted task terminal remains authoritative.
- Interrupted or uncertain work is never replayed automatically.
- Settings repair changes only the previewed activated scope.
- Export changes no runtime or settings state.
- Secondary instances remain unable to write settings or own a backend.
- Normal and forced recovery preserve process containment and zero-descendant
  evidence.

### UI and accessibility

- Use accepted in-window shell hosts, one diagnostics-facing state authority,
  semantic tokens, portals, title-bar fallback, and responsive behavior.
- Keep high-rate records out of frontend state and announcements.
- Preserve keyboard access, deterministic focus, screen-reader status,
  non-color communication, forced colors, reduced effects, scaling, text
  expansion, RTL, and compact-shell behavior.
- Preview and confirmation text must state exact effect and excluded data.

### Reproducibility and evidence

- Use activated pins, locks, clean checkouts, deterministic synthetic fixtures,
  and exact commands.
- Build the snapshot from the reviewed commit and prove a fresh extraction
  rerun.
- Record command, exit status, tool, runner/hardware, OS/architecture,
  filesystem, engine, package, archive tool, commit, artifact, capture time,
  expected/actual result, measurement, limitation, and classification.
- One target or evidence class cannot substitute for another.
- Perform no opportunistic upgrades, broad refactors, formatting sweeps, or
  unrelated cleanup.

## 11. Proportional tests and exact evidence

Future activation must replace command families with exact current commands,
working directories, versions, targets, environment controls, expected
results, retained artifacts, and manual owners.

### 11.1 Schemas and cross-language parity

Verify:

- valid and invalid diagnostic records;
- finite event/component/level/status codes;
- missing, unknown, oversized, non-finite, malformed Unicode, and excessive
  field/collection inputs;
- Rust validation of Python stderr records before retention/display/export;
- frontend safe-status and recent-error response validation;
- generated binding and fixture parity where applicable;
- no free-form payload or arbitrary metadata field.

### 11.2 Retention, permissions, and failure handling

Cover:

- queue admission and saturation;
- active-file and aggregate-byte bounds;
- rotation boundary and deterministic retention order;
- restrictive permissions where supported;
- startup cleanup and bounded shutdown flush;
- disk full, permission denial, partial write, rename/replace/delete failure;
- stale temporary and corrupt retained-file behavior;
- memory-only fallback bounds if activated;
- no interference with protocol drain, task cancellation, settings write, or
  process shutdown.

### 11.3 Privacy and leakage

Inject every activated synthetic privacy marker and encoded variant. Verify:

- no marker in UI, logs, preview, export entries, manifest, archive names,
  screenshots, CI, or external deliverables;
- no raw path, argv, URI, settings value, content, secret, credential,
  environment value, traceback, reference, or payload;
- allowlisted safe codes and bounded correlations remain useful;
- invalid/redaction-failed records are rejected safely;
- redaction failure blocks export rather than silently leaking.

### 11.4 Preview and export

Verify:

- preview exactly matches the frozen export inventory;
- explicit action is required after preview;
- native picker cancellation is normal and path-free;
- successful bounded export;
- maximum synthetic export;
- deterministic safe entry names and no traversal, links, devices, duplicates,
  absolute paths, or alternate-separator escapes;
- manifest version, fields, entry lengths, hashes, counts, and exclusions;
- archive inventory and fresh safe extraction;
- failed/cancelled export removes temporary output;
- no prior export is recursively included;
- no arbitrary path or include list reaches React or Python.

### 11.5 Backend recovery and process truth

Cover:

- status refresh in ready, busy, faulted, restarting, and circuit-open states;
- accepted restart with bounded acknowledgement and readiness;
- restart during active work and truthful `Interrupted` outcome;
- repeated failure and circuit behavior;
- explicit recovery only when authorized;
- duplicate/rapid recovery rejection;
- package/build/schema mismatch rejection;
- shutdown escalation and zero descendants after close and forced recovery
  failure;
- no retry, replay, false task success, second backend owner, or infinite loop.

### 11.6 Settings repair

Cover:

- preview of affected setting IDs/sections and preservation boundary;
- reset one setting, one section, and all settings;
- accepted previous-valid-copy recovery;
- stale-temporary cleanup;
- invalid isolated section preserving unrelated valid sections;
- future-version document refusal without overwrite;
- stale revision and concurrent repair rejection;
- restart persistence and truthful recovered/reset states;
- no raw JSON, corrupt bytes, path, generic key mutation, import/export, or
  second writer.

### 11.7 UI and accessibility

Cover:

- safe statuses, bounded recent errors, filters, preview, export, backend
  recovery, settings repair, and terminal results;
- keyboard journey and logical headings/regions;
- focus restoration after preview, picker, export, recovery, confirmation,
  repair, error, and route transitions;
- screen-reader names, status, and bounded announcements;
- non-color severity and destructive-action clarity;
- forced colors, reduced effects, 200% scaling, text expansion, RTL, and
  500-pixel compact behavior;
- preview-before-export and confirmation before destructive settings actions.

### 11.8 Production, package, and platform inspection

Inspect:

- release CSP and least-privilege capabilities/permissions;
- no telemetry/crash SDK, network endpoint, support agent, raw log viewer,
  generic export/repair, debug bypass, evidence writer, privacy injector, or
  alternate writer;
- exact packaged files, permissions, resources, and sidecar identity;
- installed/native save dialog, archive, backend recovery, settings repair,
  restart, and process cleanup;
- separate target, engine, filesystem, archive, package, accessibility, CI,
  manual, and unavailable classifications.

Retain exact raw measurements from section 6.15. The review evidence index
must map every acceptance gate to exact source/tests, command and exit status,
CI run/job/artifact, native/manual evidence, measurement, limitation,
classification, and deliverable digest.

## 12. Measurable acceptance gates

All 26 gates are unsatisfied while this prompt remains provisional:

1. Exact accepted and merged predecessors through P6-WP01 were used from the
   exact clean activated `main`.
2. The final diff remains inside the activated allowlist with no P7,
   product/customer, telemetry, remote-support, release, prompt-pack, or
   unrelated work.
3. One versioned bounded diagnostic record schema and one Rust-owned local
   retention/export authority remain.
4. Python records are schema-validated and bounded by Rust before retention,
   display, preview, or export.
5. Record, queue, file, aggregate-byte, count, age where used, rotation,
   cleanup, and shutdown bounds are enforced deterministically.
6. Restrictive target permissions and activated temp, durability, replace,
   cleanup, and failure behavior are implemented and truthfully evidenced.
7. Diagnostics UI uses only sanitized typed data and accepted accessible shell
   hosts, with no raw log or sensitive backing-data exposure.
8. Preview exactly identifies the frozen allowlisted export content before
   explicit export and starts no hidden collection or mutation.
9. Rust-owned native export uses one fixed purpose/format, preserves path
   secrecy, treats cancellation normally, and returns only a safe opaque
   result.
10. Export manifest and archive enforce safe names, exact inventory, entry and
    aggregate bounds, digests, path-safe extraction, and no recursive prior
    exports.
11. Failed or cancelled export leaves no partial or leaked output and never
    reports success.
12. Deterministic privacy markers prove prohibited paths, values, content,
    argv, URIs, environment, secrets, tracebacks, references, and payloads do
    not reach UI, retention, preview, export, evidence, or deliverables.
13. A redaction/allowlist verification failure blocks retention or export
    according to the accepted safe policy.
14. Backend recovery respects accepted P3 status, restart budget, circuit,
    package identity, process containment, and window authority.
15. Recovery restores availability only, leaves interrupted work truthful,
    performs no retry/replay, and produces no false task success.
16. Settings repair/reset uses finite P6-WP01 commands, authoritative revisions,
    affected-scope preview, and preserves unrelated valid sections.
17. Unsupported future settings are not overwritten or downgraded, and raw
    settings/corrupt bytes/paths never enter diagnostics or repair UI.
18. Export, backend recovery, and settings repair remain independent
    operations with truthful cancelled, failed, blocked, recovered, reset, and
    completed states.
19. Keyboard, focus, screen-reader, non-color, forced-color, reduced-effect,
    scaling, text expansion, RTL, and compact-shell evidence passes
    proportionally.
20. CSP, capabilities, permissions, safe rendering, native-intent authority,
    single-writer ownership, and production exclusions remain least privilege.
21. Production source and packages exclude telemetry, crash upload, network,
    remote support, raw viewers, generic export/repair, test controls, debug
    bypasses, alternate writers, and developer/system-Python fallbacks.
22. Source, harness, package creation, installed/native, engine, filesystem,
    archive, process, accessibility, CI, manual, and unavailable evidence
    remains separate and truthful per target.
23. Raw retention, preview, export, restart, repair, memory, package, and
    cleanup measurements are retained without fabricated service-level claims.
24. Accepted P3 logging/task/process and P6-WP01 settings/single-instance
    behavior shows no relevant regression, including zero descendants and no
    replay.
25. Exactly four implementation deliverables use one task-start RUN_ID, have
    verified hashes, contain no sensitive data, and reproduce the reviewed
    commit through a safe fresh snapshot extraction and complete rerun.
26. The implementation branch matches its remote, `main` is unchanged, no PR
    or auto-merge exists, and the handoff stops at
    `READY FOR CHAT SESSION REVIEW` without starting P7-WP01.

If a gate is unmet, report `Blocked`, `Not run`, `Partial`, or the exact
accepted limitation as an evidence outcome. Do not claim implementation
success by inference.

## 13. Stop conditions and blocker reporting

Stop before writing if:

- activation is absent, provisional, stale, incomplete, or overbroad;
- repository, base, branch, clean-tree, path, permission, or target state
  differs;
- P6-WP01 or another required predecessor is unaccepted or unmerged;
- accepted P3 logging/task/process or P6-WP01 settings/recovery evidence is
  missing or inconsistent;
- logging, retention, privacy, export, native intent, repair, restart, target,
  permission, archive, or owner facts cannot be resolved safely;
- current source and accepted authority materially disagree;
- the allowlist cannot prevent sensitive-data retention or export;
- target permissions, durability, archive safety, native picker, settings
  preservation, or process containment cannot be proven truthfully;
- work would weaken P3 no-replay/circuit/process authority, P6-WP01
  single-writer/future-version authority, CSP, capabilities, or path secrecy;
- work requires telemetry, network, remote support, enterprise export,
  product/customer logic, database, generic repair/export, background agent,
  multiple workers, public SDK/plugins, updater/release, or another package;
- real packaged/native export, repair, or backend recovery cannot be proven
  where a pass would be claimed;
- privacy markers or sensitive data appear in source, UI, retention, logs,
  preview, export, archive, screenshots, CI, or deliverables;
- one target or evidence class would have to substitute for another;
- snapshot integrity or clean extraction rerun fails;
- unauthorized or unrelated files enter the diff;
- remote state changes, push is non-fast-forward, or history rewriting would
  be required.

When blocked:

1. Stop at the last clean non-destructive state.
2. Do not broaden scope, weaken a gate, or enter P7-WP01 as a workaround.
3. Record the exact failed fact, command/path, expected value, observed value,
   affected gates, changed files, tree state, and unchanged exclusions.
4. Distinguish repository defect, stale activation, missing authority or
   evidence, environment limitation, permission failure, platform limitation,
   filesystem/archive limitation, privacy failure, and design decision.
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
GFD-P6-WP01 provisional prompt: Implemented and accepted as Approved provisional
GFD-P6-WP01 activation/implementation: Not started
GFD-P6-WP02 package outcome: Not started
Diagnostic record schema and safe codes: Not started
Rust diagnostic validation authority: Not started
Bounded local retention and rotation: Not started
Retention permissions and failure handling: Not started
Recent safe errors and backend status: Not started
In-window diagnostics/recovery UI: Not started
Sanitized export preview: Not started
Rust-owned native export intent: Not started
Versioned export manifest and archive: Not started
Privacy marker and leakage verification: Not started
Backend status refresh and recovery: Not started
Circuit/restart/process authority integration: Not started
No-replay and truthful interruption: Not started
Settings repair/reset preview: Not started
Setting/section/all reset integration: Not started
Previous-copy recovery and stale-temp cleanup: Not started
Unrelated settings preservation: Not started
Accessibility and responsive behavior: Not started
Cross-platform packaged/native evidence: Not started
Performance and reliability measurements: Not started
Supporting infrastructure: Not started
Tests: Not started
Documentation for implementation: Not started
Generated code: Not started
Fixtures/mocks: Not started
Stubs/placeholders: Not started
Incomplete work: Not started
Blocked work: Blocked — P6-WP01 is not accepted and merged and P6-WP02 has no activated prompt
GFD-P7-WP01 and later work: Not started
Telemetry/crash upload/remote support: Not started
Product/customer features: Not started
Enterprise export or support bundle: Not started
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

1. `prime-shell-work-gfd-p6-wp02-<RUN_ID>-source-snapshot-r1.zip`
2. `prime-shell-work-gfd-p6-wp02-<RUN_ID>-diagnostics-repair-recovery-report-r1.md`
3. `prime-shell-work-gfd-p6-wp02-<RUN_ID>-review-evidence-index-r1.md`
4. `prime-shell-work-gfd-p6-wp02-<RUN_ID>-handoff-manifest-r1.md`

### Source snapshot

- Build it from the exact reviewed final commit.
- Use one top-level `prime-shell/` root and repository-relative paths.
- Include source, schemas, generated bindings, fixtures, tests, scripts,
  manifests, locks, workflows, and focused documentation needed for review.
- Exclude `.git`, environments, caches, build output, installers, native
  captures, logs, exports, settings files, temporary files, secrets, user
  content, and unrelated artifacts.
- Verify SHA-256, ZIP CRC, path safety, expected root, and exact inventory.
- Extract into a fresh empty directory and rerun complete activated
  verification.

### Diagnostics, repair, and recovery report

Record:

- activation, model, reasoning, repository, branch, base/final commit, parent,
  tree, and exact changed paths;
- every accepted predecessor commit, report, artifact, digest, evidence,
  deviation, fallback, limitation, amendment, and blocker;
- before/after logging, retention, status, export, settings repair, backend
  recovery, state, capability, package, and owner inventories;
- exact record schemas/codes/bounds, validation, queues, rotation, retention,
  permissions, failures, and safe recent-error behavior;
- export allowlist, preview/frozen snapshot, native intent, archive format and
  bounds, manifest, failure cleanup, and inspection;
- privacy marker corpus, redaction/allowlist method, raw results, failures,
  and production exclusions;
- backend recovery, circuit/restart/process/no-replay results and zero
  descendants;
- settings repair/reset previews, revisions, preservation, future-version,
  previous-copy, stale-temp, restart, and limitations;
- accessibility, CSP/capability, per-target native/package evidence, raw
  measurements, all 26 gate outcomes, and full status inventory;
- confirmation that P7-WP01, telemetry, remote support, product/customer,
  enterprise export, release, PR, and merge work did not start.

### Review evidence index

- Include one row for each of the 26 acceptance gates.
- Map every gate to exact source/tests, command and exit status, CI
  run/job/artifact, native/manual evidence, measurement, classification,
  limitation, and digest.
- Keep schema, logging, retention/permissions, privacy, preview/export,
  archive, backend recovery/process, settings repair, accessibility,
  CSP/security, packaged/native, CI/manual, performance, snapshot, and
  unavailable evidence separate.
- Record tool, runner/hardware, OS/version/architecture, filesystem, engine,
  package, archive tool, commit, capture time, expected/actual result, and
  limitation.
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
- Never include retained logs, export contents, settings values, corrupt
  bytes, user content, native paths, raw launch inputs, opaque-reference
  values, credentials, secrets, or other sensitive data.

## 17. Branch, draft-PR, review, merge, and deletion controls

This provisional prompt authorizes no implementation branch or PR. Future
activation must:

- start from the exact accepted merged `main` SHA supplied by Chat Session;
- use one fresh implementation branch named by Chat Session;
- never reuse the documentation branch or a predecessor implementation branch;
- keep changes limited to one coherent `GFD-P6-WP02` implementation;
- commit intentionally and push only after validation and an immediate remote
  race check;
- verify final remote commit, parent, tree, paths, hashes, and clean parity;
- leave `main` unchanged;
- stop at `READY FOR CHAT SESSION REVIEW`.

Do not open a draft or ready PR unless a later explicit Chat Session
instruction authorizes that separate action. Do not enable auto-merge. Prompt
acceptance is not activation, review acceptance does not authorize merge, and
P6-WP02 completion does not authorize P7-WP01 or release work.

Merge, squash, rebase, force-push, history rewriting, branch deletion,
telemetry, remote support, customer diagnostics, enterprise export, signing,
notarization, updater work, release hardening, publication, deployment, and
repository-setting changes require separate explicit authorization.

## 18. Completion response and return prompt

The future implementation response must lead with exactly one status:

- `READY FOR CHAT SESSION REVIEW`
- `BLOCKED`
- `NOT STARTED`

For `READY FOR CHAT SESSION REVIEW`, report concisely:

- model `GPT-5.6 Sol` and reasoning `Extra High`;
- repository, starting commit, branch, final commit, parent, and tree;
- exact changed paths and source snapshot;
- all four deliverable filenames and digests;
- all 26 acceptance-gate outcomes;
- schema/logging, retention/permissions, privacy, preview/export, archive,
  backend recovery/process, settings repair, accessibility, CSP/security,
  package/native, CI/manual, performance, snapshot, and unavailable evidence;
- raw measurements and full status inventory;
- confirmation that `main` is unchanged and no PR, merge, P7-WP01,
  telemetry, remote support, product/customer, enterprise export, signing,
  updater, or release work started.

For `BLOCKED`, report the exact stop condition, command or evidence,
repository state, changed files, unaffected scope, affected gates, and
smallest required Chat Session decision. Do not claim partial work as
implementation success.

End a successful future implementation response with:

```text
Chat Session: Review GFD-P6-WP02 on the exact implementation branch and commit
reported above. Read the diagnostics/repair/recovery report, review evidence
index, handoff manifest, and verified source snapshot. Return Accepted,
Focused correction required, or Blocked. Confirm bounded local structured
diagnostics and retention, restrictive permissions, safe recent errors,
preview-before-export, Rust-owned path-safe export and manifest, privacy
marker exclusion, backend restart/circuit/process/no-replay authority,
targeted settings repair preserving unrelated valid sections, future-version
safety, accessibility, production exclusions, target limitations, raw
measurements, status inventory, and artifact hashes. GFD-P7-WP01,
product/customer work, telemetry, remote support, enterprise export, PR
creation, merge, release, and branch deletion remain unauthorized.
```
