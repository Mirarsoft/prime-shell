# GFD-P4-WP01 — Local Document-Analysis Reference Feature

## 1. Package identity and prompt status

**Package ID:** `GFD-P4-WP01`
**Phase:** `Phase 4`
**Title:** `Local Document-Analysis Reference Feature`
**Task ID:** `GFD-P4-WP01`
**Prompt ID:** `PRIME-SHELL-GFD-P4-WP01-PROMPT`
**Prompt version:** `R1`
**Prompt lifecycle state:** `Provisional`
**Implementation status at authoring:** `Not started`
**Recommended implementation model:** `GPT-5.6 Sol`
**Reasoning/intelligence:** `High`
**Authorization boundary:** Exactly one package, `GFD-P4-WP01`
**Execution status:** `Execution is not authorized`

High is the roadmap minimum because this package must prove one real vertical
slice across the accepted shell, Rust-owned native intent, opaque references,
typed cross-language operation contracts, the bounded sidecar task runtime,
safe rendering, settings contribution, accessibility, packaged-native
evidence, and architecture-friction recording. A small inconsistency could
leak a native path or document content, duplicate state authority, weaken
operation authorization, replay uncertain work, or extract a public contract
before a second consumer exists.

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
Accepted WP01, P0B-WP02, P0B-WP03, P1-WP01, P2-WP01, P2-WP02, P3-WP01, and P3-WP02 heads/evidence: Not activated — Chat Session must supply exact merged commits, reports, artifacts, hashes, CI runs/jobs, native/manual evidence, deviations, fallbacks, and blockers.
Final accepted Phase 0B closure report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Final accepted Phase 1 baseline report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Final accepted P2-WP01 theme-foundation report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Final accepted P2-WP02 responsive-shell report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Final accepted P3-WP01 contracts/native-intent report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Final accepted P3-WP02 task-runtime/sidecar report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Accepted targeted amendments: Not activated — Chat Session must refresh and supply the exact list or None.
Predecessor deviations incorporated: Not activated — Chat Session must refresh and supply the exact list or None.
Document bound, encoding policy, picker filters, operation ID/schema, tokenizer, result bounds, search semantics, setting, and performance budgets: Not activated — Chat Session must verify and supply them.
Current feature/route/navigation/settings surfaces, schema registry, native intent, task runtime, support matrix, toolchain, packaging, capabilities, CSP, and manual owners: Not activated — Chat Session must verify and supply them.
Unresolved blockers/assumptions: Not activated — Chat Session must refresh and supply the exact list or None.
Authorization boundary: Not activated — the future activation may authorize only GFD-P4-WP01.
Authorization expires/invalidates when: Not activated — the future activation must invalidate on any base-SHA, predecessor acceptance, targeted amendment, material contract/tool/engine/platform/picker/task/settings/packaging/capability/CSP, repository-layout, or authorization-boundary change.
```

## 2. Repository and exact starting state

**Repository:** `Mirarsoft/prime-shell`
**Authoritative base branch:** `main`
**Required starting commit:** Not activated — Chat Session must refresh and
supply the exact current accepted and merged `main` SHA after P3-WP02
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
- approved-provisional P3-WP02 prompt documentation head:
  `1157616413408b3b2fb596f52493ed3026188afc`;
- P3-WP02 prompt SHA-256:
  `79b0361817b3710e08888aa47020e4e0cc9f278b3424c4947278d6e03d7792ab`;
- prompt-pack documentation branch:
  `docs/gfd-work-package-prompt-pack-v1`;
- P0B-WP02, P0B-WP03, Phase 1, Phase 2, and Phase 3 activation and
  implementation: `Not started`;
- P4-WP01 implementation: `Not started`.

The documentation refs do not authorize implementation. P4-WP01 activation
and implementation must start from the latest accepted and merged `main`, only
after P0B-WP02, P0B-WP03, P1-WP01, P2-WP01, P2-WP02, P3-WP01, and P3-WP02
are accepted and merged and all required reports are accepted.

Required activation and Work Session preflight:

1. Verify exact access and write permission for `Mirarsoft/prime-shell`.
2. Verify the default branch is `main` and record its exact 40-character SHA.
3. Verify WP01 and every package through P3-WP02 are accepted and merged at
   that SHA.
4. Verify all accepted predecessor reports, source snapshots, evidence
   indexes, manifests, hashes, CI jobs/artifacts, native/manual evidence,
   measurements, deviations, fallbacks, amendments, and blockers.
5. Verify the accepted P2 shell/theme contracts, P3-WP01 schemas, finite
   operation registry, native intent, reference and safe-error rules, and
   P3-WP02 task/runtime contracts.
6. Verify current route/navigation/settings integration, frontend state,
   Tauri commands/channels, Rust native reference and content mediation,
   Python operation layout, sidecar packaging, tests, capabilities, CSP,
   toolchains, locks, support targets, and manual owners.
7. Verify that no real document-analysis reference feature, public module
   contract, comprehensive settings framework, or P5-WP01 consumer exists
   unexpectedly.
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
    report, evidence index, handoff manifest, snapshot, hashes, lifecycle
    measurements, deviations, and blockers named by activation
12. the accepted P0B-WP03 prompt, activation, source, review, closure report,
    evidence index, handoff manifest, snapshot, per-platform evidence,
    fallbacks, deviations, and blockers named by activation
13. the accepted P1-WP01 prompt, activation, source, review, baseline report,
    evidence index, handoff manifest, snapshot, pins, locks, support claims,
    deviations, and blockers named by activation
14. the accepted P2-WP01 prompt, activation, source, review,
    theme-foundation report, evidence index, manifest, snapshot, native
    evidence, token provenance, contrast matrix, material/title-bar facts,
    deviations, and blockers named by activation
15. the accepted P2-WP02 prompt, activation, source, review,
    responsive-shell report, evidence index, manifest, snapshot, native
    evidence, responsive bands, splitter/focus behavior, layout schema,
    title-bar facts, deviations, and blockers named by activation
16. the accepted P3-WP01 prompt, activation, source, review,
    contracts/native-intent report, evidence index, manifest, snapshot,
    schemas, operation identities, reference rules, capabilities, native
    evidence, deviations, and blockers named by activation
17. the accepted P3-WP02 prompt, activation, source, review,
    task-runtime/sidecar report, evidence index, manifest, snapshot, task and
    race policy, package identities, process evidence, measurements,
    deviations, and blockers named by activation
18. every separately accepted targeted architecture amendment named by
    activation
19. current feature/route/navigation/settings contribution surfaces, schema
    and fixture packages, generated bindings, frontend adapters/state, Rust
    operation/native-reference/content/task/process modules, Python dispatcher
    and task control, Tauri commands/channels/capabilities/permissions, CSP,
    sidecar build/resources, tests, workflows, toolchain/version files,
    manifests, lockfiles, and support claims
20. the exact activated revision of this execution prompt

In the first implementation progress message, state:

> `AGENTS.md, repository authority, accepted Phase 0A, accepted and merged
> WP01/P0B-WP02/P0B-WP03, P1-WP01, P2-WP01, P2-WP02, P3-WP01, and P3-WP02
> evidence, the final accepted Phase 0B closure, Phase 1 baseline, P2-WP01
> theme-foundation, P2-WP02 responsive-shell, P3-WP01
> contracts/native-intent, and P3-WP02 task-runtime/sidecar reports, and the
> activated GFD-P4-WP01 prompt have been read and are active. Executing only
> GFD-P4-WP01 with GPT-5.6 Sol / High. GFD-P5-WP01, product/customer work, PR
> creation, and merge remain unauthorized.`

If a required source is unavailable, unreadable, stale, or materially
inconsistent, stop before writing. Do not conduct a broad architecture review,
rerun predecessor packages as substitutes for accepted evidence, or improvise
new product, persistence, platform, picker, task, packaging, or support claims.

## 4. Dependencies, predecessor outputs, and prerequisite evidence

**Direct prerequisites:**

- accepted and merged `GFD-P2-WP02` responsive application shell;
- accepted and merged `GFD-P3-WP02` productized task runtime and sidecar
  operations.

**Inherited prerequisites:** accepted and merged WP01, P0B-WP02, P0B-WP03,
P1-WP01, P2-WP01, and P3-WP01 source and evidence.

**Direct dependent:** `GFD-P5-WP01`, which remains unauthorized and must prove
a second real consumer before any shared feature/module contract extraction.

P4-WP01 cannot activate while any prerequisite exists only as an approved
provisional prompt, an unmerged implementation branch, an unaccepted report,
or stale evidence.

Activation must supply exact:

- accepted merged commits and tree identities;
- implementation reports, evidence indexes, manifests, source snapshots, and
  SHA-256 digests;
- CI run/job/artifact identities and per-target native/manual evidence;
- shell region, responsive-band, focus, portal, theme, forced-colors,
  title-bar, and settings contribution contracts;
- native text-open intent, picker filters, opaque `DocumentRef`, bounded
  content mediation, safe errors, reference cleanup, and path-secrecy rules;
- analysis operation ID, schemas, version, risk, idempotency, cancellability,
  timeout, limits, registry metadata, fixtures, and generated bindings;
- task/request/trace/event identities, task states, race rules, progress,
  cancellation, timeout, restart/circuit, no-replay, and snapshot policy;
- document encoding/byte bounds, search semantics/result bounds,
  tokenizer/normalization rules, analysis result bounds, and the one setting;
- sidecar build and resource identity, target matrix, package formats,
  engines, toolchain/dependency pins and locks, manual owners, and unavailable
  evidence;
- measurements, deviations, fallbacks, amendments, and unresolved blockers.

Activation must preserve P3-WP01 and P3-WP02 as authority. It may add exactly
the finite document-analysis operation and feature integration needed for this
package through their accepted extension mechanisms; it may not create a
second operation registry, task runtime, native-intent path, state authority,
or writer.

Acceptance of this prompt does not accept predecessor implementation by
inference, activate this package, authorize P5-WP01, or approve product or
customer behavior.

## 5. Objective and measurable runnable outcome

Prove the accepted architecture through one realistic, product-neutral local
document-analysis feature that uses only the accepted shell, native intent,
typed operation, task-runtime, safe-rendering, and settings-contribution
boundaries.

The future package is complete only when one real packaged Tauri path proves:

```text
empty reference-feature route
→ native open intent for a bounded local UTF-8 text document
→ Rust-owned DocumentRef and safe display metadata
→ selected document item in the context sidebar
→ safe bounded text/status in the workspace
→ safe metadata in the inspector
→ local text search with visible result navigation
→ one typed product-neutral Python analysis through P3-WP01/P3-WP02
→ ordered progress and explicit cancel/timeout/interrupted/backend-unavailable states
→ bounded result/output in the bottom panel
→ one small internal settings contribution
→ explicit retry/re-open recovery without automatic replay
```

The route must be visibly real and locally runnable. Planning, disconnected
interfaces, mock user data, hard-coded results, fixtures presented as user
data, deliberate delay presented as analysis, or a browser-only demo cannot
satisfy the outcome.

The analysis must deterministically compute bounded text statistics: line
count, Unicode-aware character/code-point count under the activated rule,
word/token count under one documented tokenizer, and a bounded normalized
top-term/frequency summary with deterministic ordering. Activation must fix
the exact operation ID, request/result/progress/error schemas, tokenization and
normalization rules, limits, setting, and budgets against the accepted P3
registry.

The feature uses no network service, model download, model inference, GPU,
external API, product database, customer-specific logic, telemetry, cloud
sync, or remote content. Its purpose is architecture proof, and it must record
friction before any shared feature/module contract is considered.

## 6. Explicit in-scope work

The activated Work Session may perform only the smallest coherent change set
needed for the following items.

### 6.1 Smallest real vertical slice

- Add one integrated internal route inside the accepted shell.
- Represent real empty, picking/loading, ready, searching, analysing,
  cancelling, succeeded, cancelled, timed-out, interrupted, failed,
  backend-unavailable, circuit-open, and stale-reference states only where
  accepted contracts make them applicable.
- Support one bounded local UTF-8 text-document type and one selected document
  at a time.
- Add a bounded session item list only when required to prove context-sidebar
  selection.
- Add one deterministic Python analysis, one direct local search mode, and one
  small feature setting.
- Keep all labels and semantics product-neutral.

### 6.2 Static internal feature integration

- Use existing accepted route, navigation, shell, and settings integration
  points.
- If the activated tree has no internal feature definition, add only the
  smallest private static definition aligned with the accepted
  `FeatureDefinition` concept.
- Use static first-party imports and unique internal feature, route,
  navigation, command, operation, and setting identifiers.
- Reuse existing startup/test conflict validation where present; add only the
  narrow validation required by this real contribution.
- Use `requiredOperations` or its accepted equivalent only for availability
  and documentation. Rust authorization remains independent and authoritative.
- Keep all integration internal. Do not create a public SDK, public feature
  package, generator, migration framework, dynamic registry, plugin system,
  or `packages/ui`.

### 6.3 Rust-owned native file intent and opaque reference

- Consume the accepted P3-WP01 Rust-owned native text-open intent.
- Expose one named least-privilege text-document open command with fixed
  activation-approved picker mode and file filters.
- Treat picker cancellation as a normal typed outcome with deterministic
  focus recovery and no partial reference.
- Keep canonicalization, authorization, bounded read mediation, and native
  path ownership in Rust.
- Return an opaque, non-path-bearing, host-scoped `DocumentRef` plus only
  bounded safe metadata such as display name, media category, byte count,
  encoding status, and approved selection/modified state.
- Reject binary, disallowed encoding, oversized, stale, forged, wrong-intent,
  permission, and I/O cases with stable bounded path-free errors.
- Keep React and Python unable to choose, receive, reconstruct, or log a
  native path, URI, raw handle, picker configuration, or arbitrary file
  operation.
- Clean references deterministically when documents are closed, replaced,
  revoked, or the host exits.

No recent-files list, directory import, drag-and-drop expansion, watcher,
auto-reload, multi-file batch, save/edit workflow, or durable document library
belongs in this package.

### 6.4 Bounded document model and safe rendering

- Rust owns native resource authorization and bounded content mediation.
- Python owns only the accepted deterministic analysis computation.
- React owns presentation, selected item interaction, search query, and other
  transient UI intent.
- Preserve user-visible Unicode exactly. Normalize only where the activated
  search or analysis contract explicitly requires it.
- Bound text bytes/characters, safe metadata, session items, search query and
  matches, analysis result arrays/strings, snapshots, and retained task state.
- Render imported content as text nodes or through one accepted safe text
  renderer.
- Render search highlights by splitting escaped text into safe text segments,
  never by injecting markup.
- Prohibit raw HTML, scriptable Markdown, embedded SVG, `file:` navigation,
  arbitrary URL activation, executable content, and untrusted markup.
- Keep durable domain storage absent.

### 6.5 Accepted shell-region behavior

- **Context sidebar:** show the empty/import action, bounded document/item list,
  selected state, safe display name/status, and keyboard selection.
- **Workspace:** show selected bounded text, loading/status/error surfaces,
  search input/results, visible current-match position, and safe content.
- **Inspector:** show safe metadata and bounded analysis summary/details, never
  a path, reference backing value, or backend internal.
- **Bottom panel:** show current task progress, cancel action, terminal
  result/safe error, and explicit retry or re-open controls.
- **Status, toast, and dialog layers:** use only accepted shell authority and
  avoid duplicate notification state.
- At the 500-pixel compact shell state, preserve the primary journey through
  accepted sidebar, inspector, and bottom-panel fallbacks without horizontal
  overflow.

### 6.6 Direct bounded local search

- Search only the already authorized loaded bounded text.
- Activation must fix query-length and match-count limits, case and Unicode
  behavior, substring or similarly simple deterministic semantics, and
  previous/next navigation.
- Show explicit no-result state and visible current/total result position.
- Provide keyboard access and screen-reader labels.
- Keep highlighting text-safe and independent of raw HTML.
- Prefer direct frontend search when that is the simplest accepted design; do
  not create a backend operation merely for architectural appearance.
- Add no regex execution from untrusted input, fuzzy-search framework,
  semantic model, index database, search service, network request, or
  cross-document global search.

### 6.7 One typed real Python analysis

- Consume the accepted P3-WP01 finite registry and P3-WP02 task runtime.
- Add one exact activation-approved, long-running, cancellable, idempotent,
  product-neutral analysis operation through existing schema, fixture,
  generation, registry, and drift mechanisms.
- Compute real deterministic line, character/code-point, word/token, and
  bounded normalized top-term/frequency statistics.
- Fix exact content mediation, tokenizer, normalization, stable tie ordering,
  result bounds, progress checkpoints, timeout, and safe errors at activation.
- Give Python only typed bounded content or an accepted purpose-specific
  Rust-mediated stream, never a native path, unrestricted reference value, or
  caller-chosen handler.
- Keep stdout protocol-only and stderr structured, bounded, trace-correlated,
  path-free, content-free, and secret-free.
- Prohibit hard-coded production results, sleep-only work, mock-only handlers,
  dynamic dispatch, arbitrary commands, network/model/GPU/API/database use,
  and external services.
- Keep test-only slow/fault controls absent from production commands,
  capabilities, and packages.

### 6.8 Progress, cancellation, timeout, and recovery

- Reuse the accepted P3-WP02 runtime unchanged.
- Keep request/task/trace identifiers, event sequence, task snapshots,
  terminal precedence, timeout, cancellation, restart/circuit, and process
  lifecycle Rust-authoritative.
- Deliver ordered and bounded progress, coalescing only according to the
  accepted runtime, and preserve exactly one valid terminal result.
- Check cooperative cancellation at activated bounded analysis checkpoints.
- Distinguish cancellation acknowledgement from completion and distinguish
  `Cancelled`, `TimedOut`, `Interrupted`, operation failure, stale reference,
  backend unavailable, and circuit open.
- Treat crash, hang, or forced sidecar termination as `Interrupted` under the
  accepted rules.
- Never automatically replay uncertain work or imply rollback.
- Permit explicit retry only as a new idempotent task after current operation,
  version, circuit, reference, and authorization checks pass.
- After host restart, restore neither task nor native reference silently.
  Require the user to re-open the document.

### 6.9 Frontend state ownership

- Use local component state for transient search input, selection interaction,
  and form drafts.
- Use one bounded Zustand authority for live task/UI state only where the
  accepted shell/runtime requires it.
- Use TanStack Query only for appropriate fetchable snapshots or entities.
- Keep Rust authoritative for native references, backend status, lifecycle,
  task snapshots, and terminal state.
- Do not mirror the same state across query, Zustand, and local authorities.
- Do not put high-frequency progress into TanStack Query or raw content into
  browser persistence.
- Bound session items, matches, snapshots, and history, and clean
  subscriptions/state on route or window closure.

### 6.10 Exactly one small settings contribution

- Add exactly one non-sensitive bounded product-neutral setting to the
  existing in-window settings surface.
- Activation must select one boolean or small bounded enum/integer, such as
  analysis term-detail count or default search-case behavior.
- Give it one stable internal contribution ID, accessible label/description,
  documented default, reset behavior, and safe immediate or explicit-apply
  semantics.
- Store no native path, reference backing value, document content, secret, or
  backend credential.
- Create no separate settings window, broad settings schema, migration
  framework, corruption recovery, sync, or Phase 6 diagnostics.
- Persist the setting only if activation proves an already accepted narrow
  Rust-owned settings path can hold the field without expanding scope.
  Otherwise classify it truthfully as session-only and durable persistence as
  `Not started`.

### 6.11 Accessibility and responsive behavior

- Use logical landmarks and headings for feature regions.
- Provide accessible names for import, search, result navigation, analyse,
  cancel, retry, re-open, settings, and panel controls.
- Restore focus deterministically after picker cancellation, successful
  import, search navigation, task terminal state, error dismissal, and
  panel/drawer/dialog closure.
- Use non-color-only selected, progress, result, and error communication.
- Throttle progress announcements and prioritize terminal announcements.
- Prove keyboard use, screen-reader labeling/status, forced colors, reduced
  motion/transparency, 200% scaling, text expansion, RTL, visible focus,
  minimum target sizes, and the 500-pixel compact state.
- Use no focus trap except a valid modal dialog.
- Keep browser tests, screenshots, and snapshots distinct from real native
  focus or screen-reader evidence.

### 6.12 Theme, CSP, portals, and production exclusion

- Preserve P2-WP01/P2-WP02 semantic tokens, theme-before-paint behavior,
  native background, forced colors, title-bar fallback, responsive shell, and
  portal hosts.
- Use no raw product colors outside accepted tokens.
- Add no inline script, eval, remote content, CSP weakening, arbitrary resource
  URL, or imported-content navigation.
- Render overlays and dialogs through accepted portal roots.
- Prove production source, commands, capabilities, and packages exclude test
  drivers, evidence writers, fault controls, developer-sidecar fallbacks, and
  debug commands.

### 6.13 Data ownership, privacy, and cleanup

- Keep the native reference registry and authorized reads Rust-owned.
- Keep document contents, native paths, reference backing values, secrets,
  credentials, raw exceptions, and sensitive values out of UI diagnostics,
  DOM metadata, logs, errors, screenshots, CI artifacts, and external
  deliverables.
- Keep content and derived state bounded in memory and clean it when a document
  is closed, replaced, revoked, or the host exits.
- Preserve exactly one writer for the accepted setting.
- Add no hidden content cache, durable task, journal, checkpoint, automatic
  replay, telemetry, remote diagnostics, cloud sync, crash upload, or network.
- Use only synthetic, non-sensitive repository test documents.

### 6.14 Cross-platform and packaged-runtime truth

Activation must refresh accepted targets, OS/architecture/engine versions,
package identities, runners, and manual owners. Keep separate:

- source and browser checks;
- Rust/Python integration;
- sidecar build;
- host package creation;
- installed/packaged native launch;
- real native picker behavior;
- WebView2, WKWebView, and WebKitGTK behavior;
- keyboard, screen-reader, and manual accessibility;
- process cleanup;
- unavailable or blocked evidence.

The accepted journey must separately prove native picker open/cancel, safe
document load, search, real analysis, progress, cancellation, terminal result
or error, backend-unavailable recovery, setting behavior, and clean close
using the exact bundled sidecar on every claimed target where tooling permits.
One target cannot substitute for another. Add no platform, architecture,
format, store, signing, notarization, updater, or production-release claim.

### 6.15 Performance and reliability measurements

Retain raw values, method, sample count, target, engine, hardware/runner,
activated budget, and classification for:

- picker-to-ready latency excluding user think time;
- bounded read/validation and first safe content render;
- local-search latency and result-navigation responsiveness;
- task acceptance and progress-update behavior;
- analysis completion on one small and one upper-bound synthetic document;
- cancellation acknowledgement and cooperative stop or escalation;
- frontend responsiveness during analysis;
- host and sidecar memory deltas;
- retained document/task/search state high-water marks;
- feature package-size delta;
- close/cleanup duration and zero descendants.

Measure rather than invent universal service levels. Classify a miss
truthfully and do not hide it by changing the workload after measurement.

### 6.16 Architecture-friction record

The implementation report must contain a focused table for every shell,
native-intent, operation, task, settings, route/navigation, state, testing,
accessibility, or packaging integration issue. For each item record:

- current accepted boundary used;
- exact friction observed;
- direct local solution used;
- whether the need is plausibly reusable;
- why one real consumer is insufficient for extraction;
- recommendation for P5-WP01: retain direct code, test with a second consumer,
  or propose a narrow contract only after evidence.

Do not extract a public/shared contract in P4 merely because the feature
exposed friction. P5-WP01 must add a second real consumer before any justified
extraction.

### 6.17 Focused documentation and evidence

- Update only focused implementation documentation needed to run and review
  the reference feature.
- Record exact commands, expected and actual behavior, platform/tool identity,
  evidence class, limitation, and friction.
- Keep contract, frontend, Rust, Python, task/runtime, accessibility,
  CSP/security, packaged/native, CI/manual, performance, friction, and
  unavailable evidence separate.
- Do not rewrite architecture authority or create product/customer
  documentation.

## 7. Explicit exclusions and prohibited adjacent work

Do not:

- activate, repair, reimplement, or rerun predecessors as substitute work;
- weaken or duplicate P3-WP01 schema, registry, authorization, native-intent,
  opaque-reference, safe-error, capability, or single-writer authority;
- weaken or duplicate P3-WP02 task states, terminal precedence, progress,
  cancellation, timeout, restart/circuit, process containment, or no-replay
  authority;
- implement `GFD-P5-WP01`, add a second real consumer, or extract a public
  feature/module contract;
- add customer/domain semantics, product workflow, accounts, collaboration,
  cloud services, remote APIs, model inference, GPU work, or external data;
- create `packages/ui`, a public SDK, generator, dynamic feature/operation
  registry, plugin system, marketplace, extension point, or compatibility
  promise;
- add a database, durable document library, recent-files store, directory
  import, watcher, auto-reload, multi-file batch, editor/save flow, export
  workflow, search index, fuzzy/semantic search, or cross-document search;
- add broad settings persistence, schema migration, corruption recovery,
  single-instance handling, diagnostics export, telemetry, crash upload,
  network, sync, or Phase 6 work;
- add durable tasks, journals, checkpoints, replay, hidden resume, multiple
  Python workers, worker pools, arbitrary subprocesses, or a generic service;
- expose paths, raw handles, unrestricted references, picker options, file
  URLs, arbitrary commands, operations, methods, payloads, environment,
  process, filesystem, or network access;
- render raw HTML, scriptable Markdown, embedded SVG, executable content, or
  arbitrary URL/file navigation;
- weaken CSP, capabilities, permissions, bounds, safe errors, redaction,
  production exclusion, process containment, or cleanup;
- broaden platform, engine, package, accessibility, performance, or release
  claims beyond accepted evidence;
- redesign shell, theme, title bar, responsive behavior, P3 runtime, or
  repository architecture;
- perform opportunistic dependency upgrades, formatting sweeps, broad
  refactors, or unrelated cleanup;
- author or modify another work-package prompt;
- open a PR, enable auto-merge, merge, force-push, rewrite history, delete a
  branch, sign, notarize, release, publish, or deploy.

A demonstrated predecessor defect or required architecture change is a stop
condition unless Chat Session separately accepts and activation names one
exact focused correction.

## 8. Allowed repository areas and expected changes

Activation must replace this provisional inventory with exact paths from the
accepted merged tree. Expected allowed areas are no broader than:

```text
packages/app-contracts/schemas/             # exact open/reference/analysis contracts
packages/app-contracts/fixtures/            # bounded valid/invalid shared fixtures
accepted generated binding locations        # mechanically required outputs only
apps/desktop/src/routes/ or accepted path    # one internal feature route
apps/desktop/src/navigation/ or shell path   # one static navigation contribution
apps/desktop/src/features/ or accepted path  # private feature definition if required
apps/desktop/src/backend/                    # typed native/task adapters
apps/desktop/src/state/ or accepted path     # one bounded feature/task authority
apps/desktop/src/settings/ or accepted path  # exactly one internal setting contribution
apps/desktop/src-tauri/src/commands/         # exact named feature commands
apps/desktop/src-tauri/src/native/           # accepted text-open intent integration
apps/desktop/src-tauri/src/references/       # bounded DocumentRef mediation
apps/desktop/src-tauri/src/backend/          # finite operation integration only
apps/desktop/src-tauri/src/tasks/            # accepted runtime integration only
apps/desktop/src-tauri/capabilities/         # least-privilege exact commands
apps/desktop/src-tauri/permissions/          # least-privilege exact commands
services/python-backend/                     # one finite deterministic analysis handler
scripts/verify/ and focused build scripts    # bounded feature/native checks only
tests/e2e/ and tests/platform-smoke/         # focused accepted journey evidence
.github/workflows/                           # minimal activated target evidence
manifests, lockfiles, build configs          # mechanically required changes only
focused feature documentation                # operation, limits, usage, evidence
```

Activation must remove nonexistent or unused entries, identify actual accepted
integration points, and list every allowed and protected path exactly.

Protected by default:

- architecture authority, Stage 1 files, prompt files, accepted decisions,
  predecessor reports, evidence, and snapshots;
- accepted P2 shell/theme ownership except exact feature contribution points;
- accepted P3 operation/native-intent/task/runtime authority except the one
  finite contribution through established extension points;
- unrelated routes, features, settings, state, persistence, diagnostics, and
  data ownership;
- release, signing, notarization, updater, publication, deployment, repository
  settings, and public API surfaces.

Every changed path must be listed in the implementation report with purpose
and authority. A path outside the activated allowlist is a stop condition
unless it is a mechanically required generated or locked companion explicitly
authorized. Generated caches, build outputs, installers, local environments,
native captures, secrets, user content, and temporary files must not be
committed.

## 9. Ordered implementation procedure

After activation, execute exactly this order:

1. Read all authority and activated predecessor evidence; issue the exact
   acknowledgement.
2. Verify exact repository, accepted merged `main`, fresh branch, clean tree,
   permissions, paths, and activation freshness.
3. Generate exactly one task-start run ID after authority/preflight and before
   any repository or deliverable write; resolve all four implementation
   artifact names and stop on collision.
4. Inventory current shell feature/route/settings surfaces, native text-open
   intent, `DocumentRef` handling, content mediation, operation registry,
   P3-WP02 task runtime, frontend state, Python handler layout, tests, CSP,
   packaging, and support evidence.
5. Run focused predecessor regression commands before editing and record any
   demonstrated blocker.
6. Write a minimal implementation plan fixing exact document bounds, encoding,
   operation ID/schema, analysis semantics, search semantics, feature setting,
   routes/contributions, state ownership, evidence matrix, and excluded work.
7. Implement the smallest real feature route and empty/import shell
   composition.
8. Wire the accepted Rust-owned native open intent and safe `DocumentRef`
   metadata/content mediation.
9. Implement bounded safe workspace rendering, sidebar selection, inspector
   metadata, and bottom-panel task/result surfaces.
10. Implement direct bounded local search with safe highlighting and keyboard
    result navigation.
11. Add the exact typed analysis schema/operation only through the accepted
    P3-WP01 registry and drift mechanisms.
12. Implement the real deterministic Python analysis using bounded authorized
    content and cooperative cancellation/progress checkpoints.
13. Integrate the accepted P3-WP02 task channel, snapshots, terminal states,
    cancellation, timeout, interrupted, backend-unavailable, circuit, and
    explicit retry behavior without replay.
14. Add exactly one small internal settings contribution, using narrow
    existing persistence only when activation proves it is already accepted.
15. Complete accessibility, responsive 500-pixel, 200% scaling, forced-colors,
    reduced-motion/transparency, focus restoration, and safe announcement
    behavior.
16. Add proportional schema, TypeScript, Rust, Python, integration, E2E,
    accessibility, CSP, packaging, native, failure, and performance checks.
17. Run continuous fast checks, then exact packaged/native journeys and
    accepted per-target/manual evidence.
18. Record raw measurements and architecture friction before considering any
    cleanup or abstraction.
19. Inspect final source and production artifacts for paths/content/secrets,
    raw markup, generic commands, broad permissions, test/fault surfaces,
    network use, durable state, product claims, and P5 extraction.
20. Remove mocks, stubs, dead scaffolding, duplicate state, unused
    abstractions, and unrelated changes; rerun all activated checks.
21. Commit and push only the activated implementation branch; verify exact
    remote parity and leave `main` unchanged without opening a PR.
22. Build the required source snapshot from the final commit, verify digest,
    CRC/path safety and one root, extract into a fresh empty directory, and
    rerun the complete activated verification.
23. Finalize exactly four deliverables, hashes, 24 acceptance-gate mappings,
    status inventory, limitations, and friction record; stop for Chat Session
    review.

Do not use P5-WP01 extraction, Phase 6 settings/persistence or diagnostics, a
database, network service, or release work as a workaround for a failed
feature gate.

## 10. Cross-cutting constraints

### Architecture and ownership

- React owns presentation and transient feature intent only.
- Rust owns named native intent, native paths, `DocumentRef` lifecycle,
  content authorization/mediation, operation authorization, task IDs,
  terminal decisions, sidecar/process lifecycle, capabilities, and persistent
  writes.
- Python owns only the typed authorized deterministic analysis.
- Accepted P3-WP01 schemas and registry metadata remain canonical.
- Accepted P3-WP02 task/runtime state remains authoritative.
- Every mutable datum has one writer; readers receive typed snapshots, bounded
  content, safe metadata, or results.
- The first consumer remains direct internal code. Record friction; do not
  extract a public/shared contract before P5-WP01 proves a second consumer.

### Bounds, content, and Unicode

- Enforce actual byte/count/time bounds before allocation, admission, search,
  analysis, or retention.
- Preserve user-visible Unicode exactly.
- Define encoding rejection and normalization rules explicitly.
- Bound document content, metadata, session items, queries, matches, results,
  progress, snapshots, history, logs, and temporary state.
- Use deterministic ordering and tie-breaking.
- Do not silently truncate source content and present it as complete.

### Least privilege, privacy, and safe rendering

- Use finite named typed Tauri commands and operation identifiers.
- Keep native paths and reference backing values inside Rust.
- Keep document content out of logs, errors, screenshots, artifacts,
  analytics, and diagnostics.
- Use no generic invoke, operation, process, shell, filesystem, path, URL,
  environment, log, or payload tunnel.
- Render imported content and highlights only as safe text.
- Keep network denied and release CSP least privilege.
- Prove production exclusion of test, evidence, fault, and debug surfaces.

### Task and recovery semantics

- Rust mints and owns request/task/trace identities, sequences, snapshots, and
  terminal state.
- Progress is ordered/bounded and terminal events are never lost or
  overwritten.
- Cancellation acknowledgement is not terminal completion or rollback.
- `Cancelled`, `TimedOut`, and `Interrupted` remain distinct.
- Restart restores availability only and never replays uncertain work.
- Explicit retry creates a new authorized idempotent task.
- Host restart invalidates native references and never resumes tasks.

### State and settings

- Use one state authority for each datum and no mirrored caches.
- Keep high-frequency progress out of query caches.
- Keep raw document content out of browser persistence.
- Add exactly one bounded setting with one writer.
- Use durable setting persistence only through an already accepted narrow
  Rust-owned path; otherwise retain session scope truthfully.

### UI and accessibility

- Use accepted shell regions and responsive fallbacks.
- Preserve semantic themes, forced colors, native background, title-bar
  fallback, portal roots, reduced motion/transparency, and visible focus.
- Provide complete keyboard access, deterministic focus restoration, bounded
  announcements, 200% scaling, text expansion, RTL, and 500-pixel behavior.
- Keep automated, browser, native, screen-reader, and manual evidence
  classifications distinct.

### Reproducibility and evidence

- Use activated pins, lockfiles, clean checkouts, deterministic fixtures, and
  exact commands.
- Build the snapshot from the reviewed commit and prove a clean extraction
  rerun.
- Record exact command, status, runner/hardware, OS/architecture, engine,
  tool, commit, artifact, capture time, expected/actual result, measurement,
  and limitation.
- One platform or evidence class cannot substitute for another.
- Perform no opportunistic upgrades, broad refactors, formatting sweeps, or
  unrelated cleanup.

## 11. Proportional tests and exact evidence

Activation must resolve exact commands, working directories, tool versions,
targets, environment controls, expected results, retained artifacts, and
manual owners from the accepted merged tree.

### 11.1 Contract and fixture evidence

- exact document-open intent and opaque-reference schemas;
- scoped analysis request/result/progress/error schemas;
- bounded valid and invalid cross-language fixtures;
- generated binding and drift agreement across TypeScript, Rust, and Python;
- unknown, unauthorized, stale-version, oversized, stale-reference, forged,
  wrong-intent, binary, and invalid-encoding rejection.

### 11.2 Frontend and shell evidence

- lint, typecheck, unit/component tests, and production build;
- real route, navigation, shell-region, and setting contribution integration;
- sidebar, workspace, inspector, and bottom-panel states;
- bounded local search, safe highlighting, no-result state, and result
  navigation;
- one task state authority, navigation/remount retention, and subscription
  cleanup;
- empty, loading, success, cancellation, timeout, error, interrupted,
  backend-unavailable, circuit-open, and stale-reference behavior;
- no raw path, HTML, scriptable URL, duplicated state, or product-specific UI.

### 11.3 Rust and native-intent evidence

- locked format, lint, tests, and build;
- named least-privilege picker command with fixed mode and filters;
- picker cancellation and permission/I/O failures;
- opaque-reference creation and stale/forged/wrong-intent rejection;
- bounded exact-Unicode content mediation;
- path-free safe metadata, errors, and logs;
- reference cleanup and no arbitrary filesystem access.

### 11.4 Python analysis evidence

- real deterministic outputs for representative Unicode text;
- exact tokenizer and normalization behavior;
- deterministic result ordering and bounds;
- progress checkpoints and cooperative cancellation;
- malformed, oversized, unauthorized, and unsupported input rejection;
- no path, network, model, GPU, dynamic handler, shell, database, or external
  service;
- accepted stdout/stderr and trace/log rules.

### 11.5 Task-runtime and failure evidence

- ordered/coalesced progress and one terminal result;
- cancellation before, during, near completion, and after terminal;
- timeout, cooperative stop, and escalation classification;
- deliberate backend unavailable, crash, hang, restart, and circuit states
  through accepted test-only fault paths;
- no automatic replay and explicit idempotent retry only;
- route navigation/remount without terminal-state loss;
- close/cleanup with zero descendants.

### 11.6 Accessibility and responsive evidence

- complete keyboard journey and logical landmarks;
- focus restoration after picker, search, task, errors, panels, and dialogs;
- screen-reader names, status, progress, and terminal announcements;
- forced colors and non-color-only states;
- reduced motion/transparency;
- 200% scaling, text expansion, RTL, and 500-pixel compact shell;
- pointer and touch target usability where claimed;
- separate browser automation and native/manual evidence.

### 11.7 CSP, security, and production-exclusion evidence

- release CSP, Griffel, and portal behavior;
- safe text rendering with no raw markup or file URL;
- no path, content, secret, raw exception, or reference backing value in
  source, UI, logs, errors, screenshots, or deliverables;
- no broad Tauri filesystem, process, shell, dialog, or network permission;
- production exclusion of test drivers, fault controls, evidence writers,
  developer sidecars, and debug commands;
- no telemetry, remote content, database, durable task, public SDK, or plugin
  surface.

### 11.8 Packaged and real native journey

On every claimed accepted target where tooling permits, separately prove:

1. packaged/installed native app launch using the exact bundled sidecar;
2. empty feature route;
3. native picker cancellation;
4. native picker selection of a valid bounded synthetic UTF-8 document;
5. safe sidebar/workspace/inspector rendering with no path;
6. direct local search and previous/next result navigation;
7. real Python analysis with ordered progress;
8. cancellation and explicit terminal classification;
9. successful bounded result rendering in the bottom panel;
10. backend-unavailable or accepted fault/recovery state;
11. exactly one setting contribution behavior;
12. keyboard, focus, scaling, and compact-shell behavior;
13. clean close and zero descendants.

A browser mock, component test, process harness, source build, package file,
screenshot, or predecessor result is not current real packaged/native journey
evidence.

### 11.9 Performance and architecture-friction evidence

- Retain raw import, render, search, navigation, analysis, progress,
  cancellation, memory, package, and cleanup measurements.
- Map every friction item to exact source paths, accepted boundaries, local
  solution, reuse hypothesis, and P5 recommendation.
- Keep performance misses and architecture friction separate.
- Keep tests proportional to core behavior and critical boundaries; test
  count alone is not a gate.

The review evidence index must map each acceptance gate to exact source/tests,
command and exit status, CI run/job/artifact, native/manual evidence,
measurement, limitation, and deliverable digest.

## 12. Measurable acceptance gates

All gates are future implementation requirements. None is satisfied by this
provisional document.

1. Activation names exact accepted merged WP01, P0B-WP02, P0B-WP03, P1-WP01,
   P2-WP01, P2-WP02, P3-WP01, and P3-WP02 source/evidence and starts from the
   exact clean `main` SHA.
2. The diff stays inside the activated allowlist and contains no P5-WP01,
   product/customer logic, broad persistence, diagnostics, release,
   prompt-pack, or unrelated work.
3. One real internal reference-feature route integrates with the accepted
   shell and contains no hidden mock, stub, fixture-as-user-data, or hard-coded
   success path.
4. Native text-file open/cancel uses the accepted Rust-owned named intent,
   returns an opaque `DocumentRef`, and exposes no native path or arbitrary
   picker/filesystem control.
5. Valid bounded UTF-8 content loads with exact Unicode fidelity; binary,
   invalid, oversized, stale, forged, wrong-intent, permission, and I/O cases
   fail safely with path-free errors.
6. Sidebar, workspace, inspector, and bottom panel present coherent bounded
   empty, loading, ready, task, result, and error states through accepted shell
   regions.
7. Imported content and search highlights render safely as text with no raw
   HTML, scriptable URL, file navigation, or untrusted markup execution.
8. Local search has bounded query/results, deterministic documented semantics,
   accessible previous/next navigation, visible result position, and explicit
   no-result behavior.
9. One exact P3-authorized, typed, bounded, idempotent, cancellable Python
   analysis performs real deterministic text computation without path,
   network, model, GPU, shell, dynamic dispatch, or external service access.
10. TypeScript, Rust, and Python agree on the analysis operation, fixtures,
    limits, errors, generated bindings, registry metadata, and schema hash
    with no drift.
11. Rust/P3-WP02 remains authoritative for task IDs, trace IDs, progress
    order, snapshots, terminal precedence, timeout, cancellation,
    restart/circuit, and no replay.
12. Real progress is ordered and bounded; cancellation, timeout, interrupted,
    and successful terminal states are distinct, accessible, and never falsely
    claim rollback.
13. Backend unavailable, circuit open, stale reference, crash/hang
    interruption, and explicit idempotent retry recover safely without
    automatic replay or hidden task resume.
14. One central typed frontend adapter/state authority survives
    navigation/remount, cleans subscriptions, bounds retained state, and does
    not duplicate query, Zustand, and local authorities.
15. Exactly one small internal setting works with documented default/reset and
    no secret, path, or content; persistence occurs only through an already
    accepted narrow Rust path or is truthfully session-only.
16. Keyboard, focus restoration, screen-reader status, non-color feedback,
    forced colors, reduced motion/transparency, 200% scaling, text expansion,
    RTL, and 500-pixel compact behavior pass proportional evidence.
17. P2 semantic theme, title-bar fallback, portals, release CSP, and
    production test-surface exclusion remain green.
18. No document content, native path, reference backing value, secret, raw
    exception, or sensitive data appears in logs, errors, DOM diagnostics,
    screenshots, CI artifacts, or external deliverables.
19. No database, recent-files store, watcher, save/editor workflow, durable
    task, journal, checkpoint, multiple worker, plugin, public SDK, telemetry,
    network, diagnostics, signing, updater, or product feature entered the
    package.
20. Per-target source, sidecar, package, installed/native, engine, picker,
    accessibility, process, CI, manual, and unavailable evidence is separate
    and truthful; one target does not substitute for another.
21. Raw import, render, search, analysis, progress, cancellation, memory,
    package, and cleanup measurements are retained against
    activation-approved budgets without fabricated service-level claims.
22. A focused architecture-friction record maps every observed integration
    issue and defers extraction until P5's second real consumer.
23. Exactly four implementation deliverables use one task-start run ID, have
    verified hashes, contain no sensitive data, and reproduce the reviewed
    final commit through a safe fresh snapshot extraction and complete rerun.
24. The implementation branch matches its remote, `main` is unchanged, no PR
    or auto-merge exists, and the handoff stops at
    `READY FOR CHAT SESSION REVIEW` without starting P5-WP01.

If a gate is not met, report `Blocked`, `Not run`, `Partial`, or the exact
accepted limitation as an evidence outcome. Do not mark implementation
successful by inference.

## 13. Stop conditions and blocker reporting

Stop before writing if:

- the activation block is absent, provisional, stale, incomplete, or
  authorizes more than `GFD-P4-WP01`;
- repository, base, branch, clean-tree, path, or permission state differs;
- any required predecessor is unaccepted or unmerged;
- any required predecessor report, snapshot, hash, CI/native/manual evidence,
  deviation, amendment, limitation, or support claim is missing or
  inconsistent;
- the operation ID/schema, document bound/encoding, file-intent policy,
  task-runtime contract, search semantics, setting, support target, manual
  owner, or state ownership cannot be resolved safely;
- current source and accepted P3 authority materially disagree;
- the design would weaken P3 authorization, path secrecy, no-replay,
  terminal-state, process containment, or single-writer rules;
- the work requires a public feature/module SDK, `packages/ui`, a second
  consumer, database, durable document/task state, comprehensive
  settings/persistence, diagnostics, multiple workers, network, signing,
  updater, release, or product/customer behavior;
- bounded text, metadata, search, result, task, log, or state cannot be
  enforced;
- imported content cannot be rendered safely without raw markup or unsafe
  navigation;
- a real packaged native import-analysis journey cannot be proven;
- only mock, browser, harness, source-build, package-file, screenshot, or
  predecessor evidence is available for a claimed native pass;
- a native path, content, reference backing value, secret, credential, raw
  exception, or sensitive data could enter source, UI, logs, errors, evidence,
  or deliverables;
- required CI/tooling/runner/manual evidence is blocked by policy, permission,
  quota, billing, architecture, engine, package tool, or unavailable target;
- one target or evidence class would have to substitute for another;
- source-snapshot integrity or fresh-extraction rerun fails;
- unauthorized or unrelated files enter the diff;
- remote state changes, push is non-fast-forward, or history rewriting would
  be required.

When blocked:

1. stop at the last clean, non-destructive state;
2. do not broaden scope, weaken a gate, or enter P5-WP01 as a workaround;
3. record the exact failed fact, command/path, expected value, observed value,
   affected gates, changed files, tree state, and unchanged exclusions;
4. distinguish repository defect, stale activation, missing
   authority/evidence, environment limitation, permission failure, platform
   limitation, or design decision;
5. report `Blocked` or `Partially implemented` using approved implementation
   vocabulary; use `Partial` only for an explicitly labeled evidence outcome;
6. return the smallest safe Chat Session decision, focused correction, or
   refreshed activation required to continue.

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
GFD-P4-WP01 package outcome: Not started
Internal reference-feature route: Not started
Native text open/cancel intent integration: Not started
Opaque DocumentRef and safe metadata: Not started
Bounded UTF-8 content mediation: Not started
Sidebar document/item state: Not started
Workspace content/status/search: Not started
Inspector metadata/analysis view: Not started
Bottom-panel task/output view: Not started
Safe text rendering/highlighting: Not started
Local bounded search: Not started
Typed Python document analysis: Not started
Progress/cancellation/timeout/recovery: Not started
Backend unavailable/circuit/interrupted states: Not started
One internal settings contribution: Not started
Accessibility and responsive behavior: Not started
Theme/CSP/portal preservation: Not started
Cross-platform packaged/native evidence: Not started
Performance and reliability measurements: Not started
Architecture-friction record: Not started
Supporting infrastructure: Not started
Tests: Not started
Documentation for implementation: Not started
Generated code: Not started
Fixtures/mocks: Not started
Stubs/placeholders: Not started
Incomplete work: Not started
Blocked work: Blocked — required predecessor implementations are not accepted and merged and P4-WP01 has no activated prompt
GFD-P5-WP01 and later work: Not started
Product/customer features: Not started
Public SDK, plugin system, or packages/ui: Not started
Comprehensive settings/persistence/diagnostics: Not started
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

1. `prime-shell-work-gfd-p4-wp01-<RUN_ID>-source-snapshot-r1.zip`
2. `prime-shell-work-gfd-p4-wp01-<RUN_ID>-document-analysis-reference-feature-report-r1.md`
3. `prime-shell-work-gfd-p4-wp01-<RUN_ID>-review-evidence-index-r1.md`
4. `prime-shell-work-gfd-p4-wp01-<RUN_ID>-handoff-manifest-r1.md`

### Source snapshot

- Build from the exact reviewed final commit, not an uncommitted worktree.
- Use one top-level `prime-shell/` root and repository-relative paths.
- Exclude `.git`, environments, caches, build output, installers, native
  captures, secrets, user content, and unrelated artifacts.
- Include source, schemas, generated bindings, fixtures, tests, scripts,
  manifests, locks, workflows, and focused documentation required for review.
- Verify SHA-256, ZIP CRC, safe paths, one expected root, and exact inventory.
- Extract into a fresh empty directory and rerun the complete activated
  verification.

### Document-analysis reference-feature report

Record:

- activation/model/reasoning, repository, branch, base/final commit, parent,
  tree, and exact changed paths;
- every accepted predecessor commit, report, artifact, hash, evidence,
  deviation, fallback, amendment, and blocker;
- before/after route, contribution, shell-region, state, native-intent,
  reference, operation, task, setting, and ownership inventories;
- exact document type/bounds/encoding, picker intent/filters, reference
  lifecycle, content mediation, and safe errors;
- search semantics, match bounds, safe highlighting, and navigation;
- analysis operation/schema/limits, tokenizer/normalization, deterministic
  outputs, progress/cancellation/timeout, and result bounds;
- frontend state authority, accessibility, responsive behavior, theme, CSP,
  portals, privacy, cleanup, and production exclusions;
- exact commands, raw performance/reliability measurements, native/package
  evidence, limitations, failures, and full independent status inventory;
- the architecture-friction table and explicit confirmation that extraction
  is deferred to a separately activated P5-WP01 with a second consumer;
- confirmation that product/customer, broad persistence, diagnostics, release,
  PR, and merge work did not start.

### Review evidence index

- Include one row for each of the 24 acceptance gates.
- Map every gate to exact source/tests, command and exit status, CI
  run/job/artifact, native/manual artifact, measurement, classification,
  limitation, and digest.
- Keep contract, frontend, Rust, Python, task/runtime, accessibility,
  CSP/security, packaged/native, CI/manual, performance, friction, and
  unavailable evidence separate.
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
- Recompute every digest after finalization and verify no deliverable changed.
- Never include native paths, document content, reference backing values,
  credentials, secrets, or sensitive user data.

## 17. Branch, draft-PR, review, merge, and deletion controls

This provisional prompt authorizes no implementation branch or PR. A future
activation must:

- start from the exact accepted merged `main` SHA supplied by Chat Session;
- create exactly one fresh implementation branch named by Chat Session;
- never reuse the documentation branch, a predecessor branch, or a dirty
  worktree;
- keep changes limited to one coherent `GFD-P4-WP01` implementation;
- commit intentionally with a concise package-scoped message;
- push only after local validation and an immediate remote-race recheck;
- verify remote commit, parent, tree, paths, hashes, and clean local parity;
- leave `main` unchanged;
- stop at `READY FOR CHAT SESSION REVIEW`.

Do not open a draft or ready PR unless a later explicit Chat Session
instruction authorizes that separate action. Do not enable auto-merge. Prompt
acceptance is not activation. Review acceptance does not authorize merge.
P4-WP01 completion does not authorize P5-WP01, product/customer work, or
public/shared feature-contract extraction.

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

- model `GPT-5.6 Sol` and reasoning level `High`;
- repository, starting commit, branch, final commit, parent, and tree;
- exact changed-path inventory and source snapshot;
- all four deliverable filenames and digests;
- contract, frontend, Rust, Python, task/runtime, accessibility, CSP/security,
  package/native, CI/manual, performance, friction, and unavailable evidence
  classifications with exact limitations;
- all 24 acceptance-gate outcomes;
- raw performance and reliability measurements;
- the full independent functional/status inventory;
- confirmation that `main` is unchanged and no PR, merge, branch deletion,
  P5-WP01, product/customer feature, public extraction, broad persistence,
  diagnostics, or release work was started.

For `BLOCKED`, report the exact stop condition, command/evidence, repository
state, changed files, unaffected scope, affected gates, and smallest required
Chat Session decision. Do not claim partial work as implementation success.

End a successful response with this return prompt:

```text
Chat Session: Review GFD-P4-WP01 on the exact implementation branch and commit
reported above. Read the document-analysis reference-feature report, review
evidence index, handoff manifest, and verified source snapshot. Return
Accepted, Focused correction required, or Blocked. Confirm every acceptance
gate, real packaged native import-analysis journey, shell-region behavior,
Rust-owned native intent and path secrecy, bounded safe rendering/search,
typed deterministic Python analysis, task progress/cancellation/recovery,
settings contribution, state ownership, accessibility, target claim,
limitation, measurement, architecture-friction record, and artifact hash.
GFD-P5-WP01, product/customer features, public contract extraction, PR
creation, merge, release, and branch deletion remain unauthorized.
```
