# GFD-P5-WP01 — Second Consumer and Proven Feature Contracts

## 1. Package identity and prompt status

**Package ID:** `GFD-P5-WP01`
**Phase:** `Phase 5`
**Title:** `Second Consumer and Proven Feature Contracts`
**Task ID:** `GFD-P5-WP01`
**Prompt ID:** `PRIME-SHELL-GFD-P5-WP01-PROMPT`
**Prompt version:** `R1`
**Prompt lifecycle state:** `Provisional`
**Implementation status at authoring:** `Not started`
**Recommended future implementation model:** `GPT-5.6 Sol`
**Recommended future implementation reasoning/intelligence:** `High`
**Authorization boundary:** Exactly one package, `GFD-P5-WP01`
**Execution status:** `Execution is not authorized`

High is the roadmap minimum for this substantial but bounded package because it
must use accepted P4 evidence to implement a second real consumer before
extracting shared contracts, distinguish current two-consumer needs from
speculation, preserve shell and settings ownership, validate identifiers and
shortcuts deterministically, and keep Rust authorization independent from
frontend metadata. A future activation may select a higher UI reasoning setting
without changing this roadmap minimum.

This provisional prompt is complete for review but non-executable. Chat Session
must first accept it as an Approved provisional prompt and later issue a
separately activated revision after all prerequisites are accepted and merged.
Acceptance is not activation, implementation authorization, PR authorization,
permission to begin `GFD-P6-WP01`, or permission to expose a public SDK,
package, or plugin system.

**Direct prerequisite:** Accepted and merged `GFD-P4-WP01`, including its
accepted reference-feature implementation, architecture-friction record,
route, command, and settings contribution evidence, and exact source and
review evidence.

**Direct dependent:** `GFD-P6-WP01`, which remains unauthorized.

### Activation metadata

```text
Activation ID: Not activated — Chat Session must refresh and supply this exact value.
Activated by: Not activated — Chat Session must refresh and supply this exact value.
Activation UTC: Not activated — Chat Session must refresh and supply this exact value.
Authoritative main SHA: Not activated — Chat Session must refresh and supply this exact value.
Required fresh implementation branch: Not activated — Chat Session must refresh and supply this exact value.
Accepted predecessors through P4-WP01: Not activated — Chat Session must supply exact merged commits, reports, artifacts, hashes, CI runs/jobs, native/manual evidence, deviations, fallbacks, and blockers.
Accepted P4-WP01 reference-feature report: Not activated — Chat Session must supply its exact filename, digest, classification, and accepted status.
Accepted P4-WP01 architecture-friction record: Not activated — Chat Session must supply its exact location, complete contents, evidence links, and accepted status.
Accepted P4-WP01 route, navigation, command, shortcut, setting, availability, state, and registration inventory: Not activated — Chat Session must refresh and supply exact facts.
Current feature-registration, route, navigation, command-palette, shortcut, settings, shell, and state paths: Not activated — Chat Session must verify and supply exact facts.
Current P3 operation registry and Rust authorization rules: Not activated — Chat Session must verify and supply exact facts.
Second-consumer behavior, IDs, bounds, shortcut, setting, paths, and exclusions: Not activated — Chat Session must fix exact values from accepted P4 evidence.
Feature, route, navigation, command, shortcut, setting, and required-operation normalization rules: Not activated — Chat Session must verify and supply exact rules.
Accepted targets, engines, tools, CI/native/manual capabilities, and owners: Not activated — Chat Session must refresh and supply exact facts.
Accepted targeted amendments: Not activated — Chat Session must refresh and supply the exact list or None.
Predecessor deviations incorporated: Not activated — Chat Session must refresh and supply the exact list or None.
Unresolved blockers and assumptions: Not activated — Chat Session must refresh and supply the exact list or None.
Authorization boundary: Not activated — a future activation may authorize only GFD-P5-WP01.
Authorization invalidates when: Not activated — a future activation must invalidate on any base-SHA, predecessor acceptance, evidence, friction, targeted amendment, contract, route, command, shortcut, settings, operation, platform, tool, repository-layout, or authorization-boundary change.
```

## 2. Repository and exact starting state

**Repository:** `Mirarsoft/prime-shell`
**Authoritative base branch:** `main`
**Required future starting commit:** Not activated — Chat Session must refresh
and supply the exact accepted and merged `main` SHA after `GFD-P4-WP01`
acceptance and merge.
**Required future implementation branch:** Not activated — Chat Session must
verify absence and supply one exact fresh branch name.

Authoring-time documentation facts, for review only:

- accepted WP01 squash-merged `main`:
  `35d53bffec6e5b05aefdf8c7fed39a9bfdf288a2`;
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
- P4-WP01 prompt SHA-256:
  `a1af627dd5181a0ab2d5c45790d9b7d90e63b164712d629ef55b5fcd8e7cad0d`;
- prompt-pack documentation branch:
  `docs/gfd-work-package-prompt-pack-v1`;
- predecessor activation and implementation through P4-WP01: `Not started`;
- P5-WP01 implementation: `Not started`.

These documentation refs do not authorize implementation. Future P5-WP01 work
must start only from the latest accepted and merged `main`, after every
predecessor through P4-WP01 and all required evidence are accepted.

Required activation and Work Session preflight:

1. Verify exact access and write permission for `Mirarsoft/prime-shell`.
2. Record the default branch and its exact 40-character SHA.
3. Verify WP01 and every package through P4-WP01 are accepted and merged at
   that state.
4. Verify all accepted reports, source snapshots, evidence indexes, manifests,
   hashes, CI jobs, native/manual artifacts, measurements, deviations,
   fallbacks, amendments, and blockers.
5. Read the complete accepted P4 architecture-friction record before selecting
   the second consumer or any shared field.
6. Inventory the current route, navigation, shell, command-palette, shortcut,
   settings, feature registration, state, operation availability, Rust
   authorization, tests, capabilities, CSP, packages, locks, and tools.
7. Verify that no P5 second consumer, public feature SDK, runtime plugin system,
   broad settings repository, or successor work exists unexpectedly.
8. Verify the named fresh implementation branch is absent.
9. Use a clean fresh clone or worktree at the activated starting commit.
10. Verify and narrow every allowed and protected path against that exact tree.
11. Stop if repository state differs from activation, would overwrite unrelated
    work, or would require rebasing or rewriting history.

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
10. the accepted WP01 source, review, implementation report, evidence index,
    manifest, snapshot, hashes, CI jobs, and native evidence named by activation
11. accepted P0B-WP02 and P0B-WP03 prompts, activations, sources, reviews,
    reports, manifests, snapshots, measurements, platform evidence, deviations,
    fallbacks, and blockers named by activation
12. accepted P1-WP01 prompt, activation, source, review, baseline report,
    manifest, snapshot, pins, locks, support claims, deviations, and blockers
13. accepted P2-WP01 and P2-WP02 prompts, activations, source, reports,
    manifests, snapshots, theme, responsive, focus, layout, accessibility,
    native, fallback, deviation, and blocker evidence
14. accepted P3-WP01 and P3-WP02 prompts, activations, source, reports,
    manifests, snapshots, schemas, operations, native intent, references, task
    races, process/package evidence, measurements, deviations, and blockers
15. accepted P4-WP01 prompt, activation, source, review, reference-feature
    report, evidence index, manifest, snapshot, friction record, route,
    navigation, command, shortcut, setting, state, operation, native/package,
    performance, deviation, and blocker evidence
16. every separately accepted targeted architecture amendment named by
    activation
17. current feature, route, navigation, command, shortcut, settings, shell,
    state, operation, native/backend, Tauri capability, CSP, package, test,
    workflow, toolchain, manifest, lockfile, and support-claim surfaces
18. the exact activated revision of this prompt

The first implementation progress message must state:

> `AGENTS.md, repository authority, accepted Phase 0A, accepted and merged
> predecessors through GFD-P4-WP01, their accepted reports and artifacts, the
> accepted P4 reference feature and architecture-friction record, and the
> activated GFD-P5-WP01 prompt have been read and are active. Executing only
> GFD-P5-WP01 with GPT-5.6 Sol / High. GFD-P6-WP01, product/customer work,
> public SDK/plugin work, PR creation, and merge remain unauthorized.`

If a required source is missing, unreadable, stale, or inconsistent, stop
before writing. Do not substitute a broad architecture review or infer P4
friction that has not been accepted.

## 4. Dependencies, predecessor outputs, and prerequisite evidence

**Direct prerequisite:** Accepted and merged `GFD-P4-WP01`, including its real
document-analysis reference feature, complete architecture-friction record,
route/navigation/command/shortcut/settings contribution evidence, exact
source, report, evidence index, manifest, snapshot, hashes, native/package
evidence, measurements, deviations, and blockers.

**Inherited prerequisites:** Accepted and merged WP01, P0B-WP02, P0B-WP03,
P1-WP01, P2-WP01, P2-WP02, P3-WP01, and P3-WP02 source and evidence.

**Direct dependent:** `GFD-P6-WP01`, which remains unauthorized.

P5-WP01 cannot activate while any prerequisite exists only as a provisional
prompt, unmerged branch, unaccepted report, missing friction record, stale
evidence, or unresolved blocker affecting this scope.

Activation must supply exact:

- accepted merged commits and tree identities;
- implementation reports, evidence indexes, manifests, snapshots, byte lengths,
  and SHA-256 digests;
- CI run/job/artifact identities and per-target native/manual evidence;
- P4 reference-feature contribution and direct-code inventory;
- every P4 friction recommendation and its supporting use-site evidence;
- accepted shell regions, route/navigation types, command palette and shortcut
  representation, settings host and writer, focus, portals, theme, CSP, and
  title-bar contracts;
- P3 operation schemas, finite registry, typed adapters, Rust authorization,
  task/no-replay, path secrecy, and process cleanup contracts;
- current feature registration and conflict behavior;
- supported targets, engines, package formats, tools, runners, manual owners,
  and unavailable evidence;
- measurements, limitations, deviations, fallbacks, amendments, and blockers.

Acceptance of this prompt does not accept predecessor implementation by
inference, activate this package, authorize P6-WP01, or approve public or
third-party extension surfaces.

## 5. Objective and measurable runnable outcome

Prove reusable internal feature, command, route, navigation, and settings
contribution boundaries with a second tiny real static first-party consumer,
then extract only the extension points demonstrably used by both consumers
while Rust authorization remains independent.

The future package must prove this exact order:

```text
accepted P4 reference feature and friction record
→ one tiny second product-neutral feature implemented through direct accepted patterns
→ real evidence of duplicated contribution needs and conflicting identifiers
→ smallest internal FeatureDefinition/contribution contract extracted
→ P4 and P5 consumers migrated to the same contract
→ deterministic route/feature/navigation/command/setting/shortcut validation
→ both features register and run without shell source edits
→ Rust operation authorization remains independent of metadata
→ all one-consumer or broader surfaces remain private and unextracted
```

The baseline second consumer is one bounded local text utility route. It accepts
only manually entered or pasted bounded text and performs one deterministic
transform or inspection chosen at activation from a bounded enum. It provides
safe input and preview/result rendering, one navigation contribution, at least
one app-local command, one app-local shortcut where supported, and exactly one
small non-sensitive setting.

The second feature must be visibly real, locally runnable, static, first-party,
product-neutral, and substantially smaller than P4. It must not use a file
picker, native path, `DocumentRef`, Python, sidecar task, model, GPU, network,
durable content storage, product logic, or customer data. A backend operation
is prohibited unless the accepted P4 evidence proves that a second real
consumer needs one and activation names that narrow need.

Activation may substitute an equally small product-neutral feature only when
the accepted P4 friction record and current tree prove it tests the same
contribution surfaces more directly. Activation must name the substitution and
its exact bounds without broadening scope.

Planning, types alone, a fake route, hard-coded success, a fixture presented as
user data, or conflict tests without a functioning second feature do not
satisfy the runnable outcome.

## 6. Explicit in-scope work

The activated Work Session may perform only the smallest coherent change set
needed for the following work.

### 6.1 Evidence-driven second-consumer selection

- Read the accepted P4 friction table before choosing behavior or fields.
- Identify P4 route, navigation, command, shortcut, settings, availability,
  state, and registration needs that remain direct one-off code.
- Select a tiny consumer that naturally needs a meaningful subset of the same
  contribution surfaces.
- Freeze exact behavior, IDs, paths, input/result bounds, transform or
  inspection semantics, shortcut, setting, and exclusions in activation.
- Reject a feature selected merely to justify a predesigned API.
- Stop if a credible consumer requires product, backend, persistence, or shell
  expansion.

### 6.2 Tiny real local text utility

- Add one integrated internal route with clear empty, input, result, and reset
  states.
- Use one bounded text area or accepted equivalent for manual input and paste.
- Use one fixed bounded deterministic transform or inspection mode.
- Expose an explicit run/apply command and safe deterministic preview/result.
- Add one navigation contribution.
- Add at least one app-local command and one app-local shortcut when the
  accepted command system supports shortcuts.
- Add exactly one small non-sensitive setting that changes only a bounded
  default or presentation choice.
- Preserve keyboard operation, accessible names, focus order, forced colors,
  reduced motion, 200% scaling, text expansion, RTL, and the 500-pixel compact
  shell state.

Prefer simple case, whitespace, line, or text inspection behavior with explicit
Unicode and normalization semantics. Avoid locale-ambiguous or lossy behavior
unless the rule and preview are fixed and tested.

The utility must not use native file intent, `DocumentRef`, Python, the
sidecar, network, hidden background work, durable storage, or clipboard APIs
beyond ordinary user paste.

### 6.3 Consumer-before-extraction evidence

Implement the second feature first through the accepted direct/static patterns,
using only enough local duplication to observe real commonality. Before
extraction, prove:

- the feature executes as real code in the accepted shell;
- P4 and P5 have exact route, navigation, command, shortcut, and settings
  contribution inventories;
- every proposed shared field is used by both consumers or is a registry-level
  invariant needed to combine them safely;
- P4-only fields may remain optional or direct;
- no interface is designed for hypothetical third parties or future consumers.

Then extract and migrate both consumers in the same package. Do not publish a
contract first and fit the consumers into it afterward.

### 6.4 Minimal internal feature definition

Treat this architecture concept only as an upper bound:

```ts
interface FeatureDefinition {
  id: string;
  routes: readonly AppRouteDefinition[];
  navigation?: readonly NavigationContribution[];
  commands?: readonly CommandContribution[];
  settings?: readonly SettingsContribution[];
  requiredOperations?: readonly BackendOperationName[];
}
```

Activation and accepted P4 evidence determine the exact minimal internal
fields, names, ownership, and locations. Require:

- static first-party definitions;
- readonly, typed, bounded trusted metadata;
- one internal owner for registration and validation;
- no arbitrary executable payload, module URL, package name, script, native
  path, process, capability grant, or dynamic import in metadata;
- no lifecycle hooks, migrations, installer, dependency injection container,
  service locator, plugin manifest, or marketplace;
- no public export, compatibility promise, or cross-application package.

Prefer an app-internal location. `packages/app-contracts` remains the owner of
accepted cross-language schemas and is not automatically the owner of frontend
feature metadata.

### 6.5 Static registry and shell integration

- Create one authoritative static registry or accepted equivalent importing
  both first-party features at build time.
- Route both features through the same registration path.
- Render contributions through accepted shell hosts without feature-specific
  branches in shared shell components.
- Preserve navigation, route, command, settings, state, focus, portal, CSP,
  title-bar, and theme behavior.
- Allow removal only through source/build changes, never untrusted runtime
  loading.

A narrow registry composition file may statically list features. It is not a
runtime plugin system.

### 6.6 Route and navigation contributions

- Use unique bounded feature IDs, route IDs, canonical paths, navigation IDs,
  and existing ordering/group keys.
- Normalize before comparing conflicts.
- Reject duplicates, invalid targets, reserved identifiers, and malformed
  metadata deterministically.
- Validate in focused tests and at registry construction.
- Fail closed with stable safe codes and both owning feature IDs.
- Keep labels and icons trusted and static.
- Preserve accepted deep-link and back-navigation behavior.

Do not redesign the router or navigation rail. Extract only the surfaces used
by both consumers.

### 6.7 Commands and app-local shortcuts

Each proven command contribution must have:

- a unique stable command ID;
- trusted static title, description, and category metadata;
- typed enablement or availability from current app state;
- a trusted static handler or action reference resolved in source;
- optional app-local shortcut metadata in the accepted representation;
- deterministic disabled and unavailable behavior;
- accessible command-palette and keyboard behavior.

Shortcut handling must remain app-local, normalize modifiers and keys by
accepted platform rules, detect exact and platform-equivalent conflicts,
respect reserved shell/native shortcuts, reject rather than override conflicts,
report unsupported platform behavior truthfully, and avoid intercepting normal
text-editing shortcuts inside editable controls.

Do not create OS-global hotkeys, macros, user-defined command languages,
arbitrary key binding executors, shell launchers, or plugin command APIs.

### 6.8 Settings contribution

Extract only the metadata and host integration proven by the P4 setting and the
P5 utility's one setting. Each contribution must have:

- a unique stable ID;
- trusted static label, description, section/group, and ordering metadata;
- a bounded boolean or small enum/integer domain;
- documented default, reset, and apply semantics;
- one accepted owner and writer;
- accessible rendering through the current in-window settings surface;
- deterministic duplicate-ID and incompatible-schema validation.

Preserve session-only truth when Phase 6 durable settings work is absent. Do
not add migrations, corruption recovery, settings search architecture,
single-instance behavior, sync, secrets, or a separate settings window.

### 6.9 Required operations and Rust independence

Retain `requiredOperations` or its accepted equivalent only if P4 uses it for
availability, documentation, or CI checks. Require:

- exact finite names from accepted P3 schemas;
- rejection of unknown operation names;
- availability without leaking backend internals;
- no operation registration or capability grant from frontend metadata;
- no generic invoke, process command, path, protocol payload, or dynamic
  operation dispatch;
- independent Rust authority for risk, timeout, cancellability, idempotency,
  window, payload/reference, and lifecycle policy;
- tests proving forged or modified metadata cannot authorize operations.

The P5 utility must not add a backend operation merely to consume this optional
field.

### 6.10 Deterministic conflict validation

One canonical pass over all statically registered features must detect:

- duplicate feature IDs;
- duplicate route IDs and canonical paths;
- duplicate navigation IDs and invalid targets;
- duplicate command IDs;
- conflicting normalized app-local shortcuts;
- duplicate setting contribution IDs;
- incompatible setting definitions sharing a key;
- unknown required operations;
- invalid cross-references;
- reserved shell IDs, paths, and shortcuts;
- empty, malformed, or oversized trusted metadata.

Results must be independent of import order. Use stable safe conflict codes
with both owners. Never auto-resolve by priority, suffix, registration order,
last-write-wins, silent replacement, or hidden disabling.

### 6.11 Extraction and ownership matrix

Record for every proposed contract element:

- P4 use site and evidence;
- P5 use site and evidence;
- duplicated need or registry invariant;
- final internal owner and API;
- why the field is needed now;
- why omitted fields remain private;
- migration impact inside the current app;
- why a second application would be needed before public extraction.

An element lacking two real use sites or one necessary registry invariant must
remain direct/private or be removed.

### 6.12 State, accessibility, theme, and CSP

- Keep P5 input/form state local.
- Preserve accepted app navigation, command, settings, and route owners.
- Add no second global store or registry authority.
- Never persist user-entered text in browser or native storage.
- Add no high-frequency task state.
- Prove keyboard-only use, logical headings and landmarks, accessible names,
  deterministic focus, palette and shortcut discoverability, non-color state,
  visible focus, forced colors, reduced motion/transparency, 200% scaling, text
  expansion, RTL, and 500-pixel compact behavior.
- Preserve semantic tokens, portal roots, release CSP, native background, and
  title-bar fallback.
- Exclude raw HTML, evaluation, remote content, arbitrary URLs, inline scripts,
  and CSP weakening.

Invalid registries and conflict fixtures are test-only and must not ship as
production features.

### 6.13 Security, privacy, and runtime truth

- Treat definitions as trusted static presentation and availability metadata,
  never native authorization.
- Keep Rust authoritative for native/backend operations.
- Keep native paths, secrets, credentials, environment dumps, arbitrary
  payloads, source paths, raw exceptions, and user text out of metadata, logs,
  UI diagnostics, screenshots, CI, and external deliverables.
- Bound input and results in memory and clear them on reset, route/window
  close, or host exit.
- Add no cache, analytics, telemetry, remote diagnostics, crash upload,
  network, cloud sync, database, generic process/filesystem/shell tunnel, or
  capability tunnel.
- Keep test features, fault injectors, drivers, evidence writers, debug
  commands, and bypasses out of production.

### 6.14 Cross-platform and performance evidence

Refresh accepted targets, engines, package identities, runners, and manual
owners. Keep source/type, browser/component, real Tauri source runtime,
packaged/installed runtime, engine, palette/shortcut, accessibility,
production-exclusion, CI/manual, and unavailable evidence separate.

Measure and retain method, sample count, target, engine, hardware/runner,
activated budget, raw values, and classification for:

- registry construction and validation;
- route, navigation, command, and setting resolution;
- palette open/filter/execute latency;
- shortcut dispatch and conflict-check cost;
- P5 route-ready and transform/inspection latency;
- P4 route and command regression before and after extraction;
- startup, memory, bundle, and package-size delta;
- invalid-registry fail-closed behavior;
- close and cleanup where P4/P3 paths run.

Record misses truthfully without changing workload or hiding fixtures.

### 6.15 Focused friction and documentation

For every accepted P4 friction recommendation, classify it as confirmed
reusable, not reproduced and retained private, contradicted by current
evidence, or blocked pending a second application or later package. Record the
exact extraction or non-extraction decision.

Record new P5 friction for route, navigation, command, shortcut, setting,
validation, availability, shell integration, state, accessibility, testing,
CSP, and packaging. Update only focused implementation documentation needed to
run and review the feature and contracts. Do not rewrite architecture authority
or document unimplemented public APIs.

## 7. Explicit exclusions and prohibited adjacent work

The activated package must not:

- implement or activate any predecessor as part of P5;
- begin `GFD-P6-WP01` or later work;
- create product or customer features, branding, workflows, or data;
- create `packages/ui`, a public `module-sdk`, public SDK, public feature
  package, plugin API, runtime plugin system, dynamic module registry,
  marketplace, generator, scaffold CLI, or compatibility promise;
- build a universal component library or cross-application contract;
- redesign the shell, router, navigation rail, command palette, settings host,
  state ownership, P3 operation registry, task runtime, or Rust authorization;
- add generic lifecycle hooks, installers, migrations, dependency injection,
  service location, user scripting, macros, or arbitrary key binding;
- add native file intent, Python/sidecar work, models, GPU, network, telemetry,
  analytics, database, durable tasks, content persistence, cloud sync,
  diagnostics, crash upload, multiple workers, or background daemons for P5;
- implement Phase 6 settings persistence, migrations, recovery,
  single-instance coordination, search, secrets, or a separate settings window;
- weaken CSP, permissions, capabilities, path secrecy, safe rendering,
  no-replay, or production exclusions;
- add signing, notarization, updater, release, publication, deployment, or
  store work;
- open a PR, merge, force-push, rebase accepted history, or delete a branch
  without separate authorization.

If any excluded work is required, stop and return `Blocked` with the smallest
required decision.

## 8. Allowed repository areas and expected changes

Future activation must replace these categories with exact current paths and a
narrow allowlist. Expected areas are:

- the app-internal P5 feature route, views, local state, and tests;
- accepted P4 feature definition and contribution use sites needed for
  migration;
- the smallest app-internal feature, route, navigation, command, shortcut, and
  settings contribution types;
- one authoritative static registry and deterministic validators;
- existing shell host integration only where a generic contribution seam must
  replace proven P4/P5 duplication;
- focused styles and accessibility tests;
- accepted P3 operation-availability types/tests only if P4 already requires
  them and migration is necessary;
- focused native/package inspection, performance, and verification scripts;
- focused package implementation documentation.

Protected unless activation explicitly proves necessity:

- architecture authority;
- prompt-pack documentation;
- cross-language operation schemas and generated bindings;
- Rust authorization, native-intent, task, and process internals;
- Python sidecar behavior;
- capabilities, permissions, CSP, packaging, and workflows;
- workspace/package boundaries;
- lockfiles and toolchain pins.

Adding a path category requires an activation correction. Unrelated formatting,
dependency churn, generated output without provenance, broad refactors, and
dead scaffolding are prohibited.

The final diff must be one coherent P5 implementation and must not contain
P6, public SDK/plugin, product/customer, release, or unrelated work.

## 9. Ordered implementation procedure

After a separate activation, execute exactly in this order:

1. Read all authority and activated predecessor evidence; issue the exact
   acknowledgement.
2. Verify exact repository, accepted merged `main`, fresh branch, clean tree,
   permissions, paths, and activation freshness.
3. Generate exactly one task-start run ID after authority and preflight and
   before any repository or deliverable write; resolve all four implementation
   artifact names and stop on collision.
4. Inventory accepted P4 feature source, friction table, route/navigation,
   commands/shortcuts, setting contribution, operation availability, shell
   integration, state, tests, packaging, and current feature-definition code.
5. Run focused P4/P3/P2 regressions before editing and record any demonstrated
   blocker.
6. Select and freeze the exact tiny second consumer from accepted P4 evidence,
   with exact IDs, bounds, behavior, shortcut, setting, paths, and exclusions.
7. Implement the second feature through current direct/static accepted patterns
   before extracting a shared contract.
8. Prove the second feature is real, accessible, bounded, and locally runnable
   in the accepted native shell.
9. Capture exact duplicated contribution needs and a two-consumer use-site
   matrix.
10. Design the smallest app-internal feature/route/navigation/command/settings
    contract and one authoritative static registry.
11. Add deterministic feature/route/navigation/command/shortcut/setting/
    required-operation conflict validation with stable safe outcomes.
12. Migrate P4 and P5 consumers to the shared internal contract without adding
    feature-specific shell branches.
13. Prove Rust authorization remains independent from feature and command
    metadata, including forged and invalid metadata tests.
14. Remove one-consumer abstractions, unused fields, duplicate registries,
    speculative adapters, and broad exports.
15. Complete keyboard, command-palette, shortcut, focus, screen-reader,
    forced-colors, reduced-motion, scaling, RTL, and compact-shell behavior.
16. Add proportional type, unit, component, integration, conflict, native,
    accessibility, CSP, package-inspection, performance, and regression checks.
17. Run fast checks continuously, then exact source, native, package, and
    manual journeys for both features on the accepted evidence matrix.
18. Reconcile every P4 friction recommendation and record P5 extraction
    decisions and raw measurements.
19. Inspect final source and production artifacts for dynamic loading, public
    SDK or package exports, generic commands, broad permissions, test features,
    source paths, user text, network, persistence, P6 work, product claims, and
    release creep.
20. Reconcile the final diff to the activated allowlist; remove dead
    scaffolding, formatting churn, unrelated refactors, and hidden stubs; rerun
    all checks.
21. Commit and push only the activated implementation branch; verify exact
    remote parity and leave `main` unchanged without opening a PR.
22. Build the required source snapshot from the final commit, verify digest,
    CRC and path safety and one root, extract into a fresh empty directory, and
    rerun the complete activated verification.
23. Finalize exactly four implementation deliverables, verify ordinary hashes
    and canonical manifest self-hash, return the section 18 handoff, and stop
    without entering P6-WP01.

Never create the broad SDK first and use the second feature as a post-hoc
sample. Never enter Phase 6 to solve a settings or persistence limitation.

## 10. Cross-cutting constraints

### Scope and evidence

- Accepted merged source and evidence outrank provisional assumptions.
- Every extracted element needs two real consumers or one necessary registry
  invariant.
- The second consumer must exist and run before extraction.
- A test fixture, type declaration, or browser mock is not a consumer.
- Keep one-consumer and future-facing surfaces private and direct.

### Ownership and trust

- Use one static registry and one validator authority.
- Preserve existing route, navigation, command, settings, shell, and state
  owners.
- Treat frontend metadata as trusted static presentation and availability,
  never native authorization.
- Keep Rust independently authoritative for all native and backend actions.
- Do not create generic invocation, path, process, capability, or payload
  tunnels.

### Bounds and deterministic behavior

- Activation fixes exact IDs, paths, normalizers, bounds, semantics, reserved
  values, and safe conflict codes.
- Reject invalid metadata deterministically and independently of import order.
- Use no last-write-wins, auto-renaming, hidden disabling, or priority-based
  conflict resolution.
- Bound input, results, metadata, registry size, labels, and retained state.

### Security and privacy

- Keep user text in bounded memory and clear it deterministically.
- Never persist, log, capture, or export user text.
- Keep native paths, secrets, source paths, raw exceptions, credentials, and
  environment data out of UI and evidence.
- Preserve CSP, capabilities, safe rendering, production exclusions, and path
  secrecy.
- Add no network, telemetry, diagnostics, database, or cloud behavior.

### User experience and accessibility

- Preserve accepted Fluent semantics, tokens, shell regions, focus, portals,
  title bar, scaling, RTL, compact behavior, and reduced effects.
- Commands, shortcuts, settings, routes, and validation errors must be
  discoverable and operable without a pointer.
- Do not intercept ordinary editing shortcuts or hide unavailable behavior.

### Platform and evidence truth

- Keep source, browser, native source-run, packaged/installed, engine,
  shortcut, accessibility, CI, manual, performance, and unavailable evidence
  separate.
- A successful target cannot substitute for an unavailable target.
- Record exact tools, versions, commands, exit codes, owners, times, and raw
  measurements.
- Do not invent universal service-level claims.

## 11. Proportional tests and exact evidence

Future activation must replace command families with exact current commands,
working directories, tool versions, targets, expected results, retained
artifacts, and manual owners.

### Contract and registration

Verify:

- exact internal definitions and contribution types;
- one static registry and both real feature definitions;
- readonly bounded trusted metadata;
- duplicate and malformed feature IDs;
- duplicate route IDs and canonical paths;
- invalid navigation targets;
- duplicate command IDs;
- exact and platform-equivalent shortcut conflicts;
- duplicate or incompatible setting contributions;
- unknown required operations;
- invalid cross-references and reserved values;
- deterministic import-order-independent outcomes;
- no silent override or renaming.

### Second feature

Verify:

- real empty, input, result, and reset journey;
- exact input and result bounds;
- deterministic transform or inspection rules;
- safe text rendering and clearing;
- no persistence of entered text;
- route and navigation behavior;
- palette command and shortcut where supported;
- one setting with default, reset, and apply semantics;
- keyboard, focus, screen reader, forced colors, scaling, text expansion, RTL,
  and compact shell;
- source, native, and package execution rather than fixture-only proof.

### Two-consumer extraction

Map every extracted element to P4 and P5 source/use evidence, duplication or a
registry invariant, final internal owner, tests covering both consumers,
intentionally omitted fields, and the reason no public package or compatibility
promise is justified.

### Command, shortcut, settings, and authorization

Verify:

- normalized command and shortcut uniqueness;
- editable-control and reserved-shortcut behavior;
- deterministic disabled/unavailable commands;
- accessible palette labels and focus;
- P4 backend commands still use only the central typed adapter;
- forged metadata cannot authorize Rust operations;
- unknown operations fail safely;
- no generic invoke, process, path, or payload tunnel;
- both settings use one minimal contract, bounded domains, one writer, and
  truthful session-only behavior;
- no Phase 6 settings work.

### Regression, security, and production exclusion

Verify both features register without feature-specific shell edits. Re-run
accepted P4 document-analysis, P3 authorization/task/path/process, and P2
shell/theme/focus/portal/title-bar/CSP checks proportionally. Inspect source,
capabilities, and production packages for:

- no runtime plugin or dynamic feature loading;
- no public module SDK, UI package, generator, marketplace, or universal API;
- no arbitrary handler, script, module, path, or payload metadata;
- no test feature, conflict injector, evidence writer, debug command, or
  driver in production;
- no user text, native path, secret, source path, raw exception, or environment
  dump in evidence;
- no network, telemetry, database, broad persistence, diagnostics, signing,
  updater, or release work.

### Native, accessibility, and performance

On each claimed target, separately prove:

1. native launch with both features statically registered;
2. P4 reference feature remains runnable;
3. P5 route and navigation journey;
4. command-palette discovery and execution;
5. app-local shortcut dispatch and conflict behavior;
6. settings contribution rendering and behavior;
7. keyboard, focus, scaling, RTL, and compact behavior;
8. invalid-registry behavior in test-only execution;
9. production packages exclude invalid and test features;
10. clean close and relevant P3 zero-descendant behavior.

Retain raw registry, startup, route, command, shortcut, settings, P5 feature,
P4 regression, memory, bundle/package, invalid-registry, and cleanup
measurements. A browser mock, component snapshot, package file, documentation
statement, or previous P4 result cannot substitute for current native evidence.

The review evidence index must map every acceptance gate to exact source and
tests, command and exit status, CI run/job/artifact, native/manual evidence,
measurement, limitation, classification, and deliverable digest.

## 12. Measurable acceptance gates

All 25 gates are unsatisfied while this prompt remains provisional:

1. Activation names exact accepted and merged predecessors through P4-WP01,
   including the accepted P4 friction record, and starts from exact clean
   `main`.
2. The final diff stays within the activated allowlist and contains no P6,
   product/customer, broad persistence, diagnostics, release, prompt-pack, or
   unrelated work.
3. One tiny second product-neutral static first-party feature is real, bounded,
   accessible, locally runnable, and not a mock or hard-coded demonstration.
4. The second feature is implemented and proven before common contract
   extraction begins.
5. P4 and P5 both register through one authoritative static internal feature
   registry without feature-specific shell branches.
6. Every extracted contract field has two real consumer use sites or is a
   necessary registry invariant; one-consumer and speculative fields remain
   direct or are removed.
7. Feature, route, navigation, command, setting, and shortcut identifiers use
   exact bounded trusted metadata and deterministic canonical forms.
8. Duplicate feature IDs, route IDs and paths, navigation IDs, command IDs,
   setting IDs, and conflicting shortcuts fail deterministically with stable
   safe outcomes.
9. Registry validation is independent of import order and uses no silent
   override, last-write-wins, auto-renaming, or hidden disabling.
10. Both features' route and navigation contributions work through accepted
    shell hosts without router or navigation redesign.
11. Both features expose real command contributions; palette, enablement,
    focus, and app-local shortcut behavior are accessible and deterministic
    where supported.
12. App-local shortcuts do not become OS-global hotkeys, silently override
    reserved or text-editing behavior, or vary without truthful platform
    evidence.
13. Both feature settings use the smallest proven internal contribution
    contract with bounded values, unique IDs, defaults, reset and apply
    semantics, and one writer.
14. P5 adds no Phase 6 settings repository, migrations, corruption recovery,
    single-instance behavior, secrets, sync, diagnostics, or separate settings
    window.
15. `requiredOperations` or equivalent remains availability or documentation
    metadata only; unknown names fail safely and Rust authorization remains
    independent and authoritative.
16. Forged or modified frontend feature or command metadata cannot authorize,
    register, or bypass a native or backend operation.
17. P4 document-analysis behavior, path secrecy, task and no-replay semantics,
    safe rendering, process containment, and accepted evidence show no relevant
    regression.
18. Both feature journeys and contribution hosts pass proportional keyboard,
    focus, screen-reader, forced-colors, reduced-motion, 200% scaling, text
    expansion, RTL, and 500-pixel compact evidence.
19. Semantic tokens, portals, release CSP, native background, title-bar
    fallback, and production test-surface exclusion remain green.
20. No user text, native path, secret, credential, raw exception, source path,
    or sensitive value appears in logs, UI diagnostics, screenshots, CI, or
    external deliverables.
21. No runtime plugin, dynamic import registry, public `module-sdk`,
    `packages/ui`, generator, compatibility promise, false feature security
    boundary, database, network, telemetry, multiple worker, durable task, or
    product behavior entered the package.
22. Source, browser, native, package, engine, shortcut, accessibility, CI,
    manual, and unavailable evidence remain separate and truthful across every
    claimed target.
23. Raw registry, startup, route, command, shortcut, setting, second-feature,
    P4 regression, memory, and package measurements and extraction decisions
    are retained without fabricated service-level claims.
24. Exactly four implementation deliverables use one task-start run ID, have
    verified hashes, contain no sensitive data, and reproduce the reviewed
    commit through a safe fresh snapshot extraction and complete rerun.
25. The implementation branch matches its remote, `main` is unchanged, no PR
    or auto-merge exists, and the handoff stops at
    `READY FOR CHAT SESSION REVIEW` without starting P6-WP01.

If a gate is unmet, report `Blocked`, `Not run`, `Partial`, or the exact
accepted limitation as an evidence outcome. Do not claim success by inference.

## 13. Stop conditions and blocker reporting

Stop before writing if:

- activation is provisional, stale, incomplete, or overbroad;
- repository, base, branch, clean-tree, path, or permission state differs;
- P4-WP01 or its friction and evidence are not accepted and merged;
- a credible tiny consumer cannot be selected from accepted evidence;
- a proposed field lacks two consumers or a necessary registry invariant;
- shell, router, settings, P3 authority, or Rust authorization would need
  redesign or weakening;
- deterministic conflict or shortcut normalization cannot be defined;
- work requires dynamic loading, public SDK or package, `packages/ui`,
  generator, runtime plugins, feature migrations, database, network,
  diagnostics, durable work, multiple workers, product behavior, P6, or
  release work;
- real native, package, shortcut, or accessibility evidence is unavailable
  where a pass would be claimed;
- user text, secrets, source paths, or native paths could enter source,
  evidence, or artifacts;
- one target would need to substitute for unavailable CI, runner, engine, or
  manual evidence;
- snapshot integrity or fresh rerun fails;
- unauthorized or unrelated files enter the diff;
- remote state changes, push is non-fast-forward, or history rewriting would
  be required.

When blocked:

1. Stop at the last clean non-destructive state.
2. Do not broaden scope, weaken a gate, or enter P6 as a workaround.
3. Record the failed fact, command or path, expected value, observed value,
   affected gates, changed files, tree state, and unchanged exclusions.
4. Distinguish repository defect, stale activation, missing evidence,
   environment limitation, permission failure, platform limitation, and design
   decision.
5. Use `Blocked` or `Partially implemented` for implementation status. Use
   `Partial` only for an explicitly labeled evidence outcome.
6. Return the smallest safe Chat Session decision or focused correction needed.

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
GFD-P5-WP01 package outcome: Not started
Accepted P4 friction reconciliation: Not started
Second tiny real feature: Not started
Second-feature route/navigation contribution: Not started
Second-feature command contribution: Not started
Second-feature app-local shortcut: Not started
Second-feature setting contribution: Not started
Second-feature accessibility/responsive behavior: Not started
Static first-party feature registry: Not started
Internal FeatureDefinition contract: Not started
Route contribution contract: Not started
Navigation contribution contract: Not started
Command contribution contract: Not started
Settings contribution contract: Not started
Required-operations availability metadata: Not started
Rust authorization independence: Not started
Feature/route/navigation conflict validation: Not started
Command/shortcut conflict validation: Not started
Setting conflict validation: Not started
Two-consumer extraction matrix: Not started
P4 consumer migration/regression: Not started
Production plugin/test-surface exclusion: Not started
Cross-platform native/package evidence: Not started
Performance and reliability measurements: Not started
Supporting infrastructure: Not started
Tests: Not started
Documentation for implementation: Not started
Generated code: Not started
Fixtures/mocks: Not started
Stubs/placeholders: Not started
Incomplete work: Not started
Blocked work: Blocked — P4-WP01 is not accepted and merged and P5-WP01 has no activated prompt
GFD-P6-WP01 and later work: Not started
Product/customer features: Not started
Public module SDK, plugin system, or packages/ui: Not started
Comprehensive settings/persistence/diagnostics: Not started
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

Evidence classifications may use `Passed`, `Failed`, `Partial`, `Blocked`, or
`Not run` only when clearly labeled as evidence. Keep functionality,
infrastructure, tests, documentation, generated code, fixtures, stubs,
incomplete work, and blocked work separate.

## 15. One RUN_ID and collision-resistant external naming

At the start of the future implementation task, after mandatory authority reads
and preflight but before any repository or deliverable write, generate exactly
one UTC run ID in basic ISO-8601 form:

```text
YYYYMMDDTHHMMSSZ
```

Resolve all four target filenames immediately from that one value and reuse it
unchanged for every implementation deliverable. Never generate or substitute a
second run ID, timestamp, branch label, local time, or random suffix in the same
task.

Before any write, verify that none of the four resolved names exists in the
destination. A collision is a stop condition: report `Blocked` without
generating another run ID.

The run ID identifies the evidence set, not the implementation version. The
manifest must record the exact reviewed commit, tree, parent, branch, snapshot
root, and every deliverable digest.

## 16. Required deliverables, hashes, and evidence index

Produce exactly these four future implementation deliverables:

1. `prime-shell-work-gfd-p5-wp01-<RUN_ID>-source-snapshot-r1.zip`
2. `prime-shell-work-gfd-p5-wp01-<RUN_ID>-second-consumer-feature-contracts-report-r1.md`
3. `prime-shell-work-gfd-p5-wp01-<RUN_ID>-review-evidence-index-r1.md`
4. `prime-shell-work-gfd-p5-wp01-<RUN_ID>-handoff-manifest-r1.md`

### Source snapshot

- Build it from the exact reviewed final commit.
- Use one top-level `prime-shell/` root and repository-relative paths.
- Include source, types, contracts, fixtures, tests, scripts, manifests, locks,
  workflows, and focused documentation required for review.
- Exclude `.git`, environments, caches, build output, installers, native
  captures, secrets, user content, and unrelated files.
- Verify SHA-256, ZIP CRC, path safety, expected root, and exact inventory.
- Extract into a fresh empty directory and rerun the complete activated
  verification.

### Second-consumer feature-contracts report

Record:

- activation, model, reasoning, repository, branch, base/final commit, parent,
  tree, and exact paths;
- every accepted predecessor commit, report, artifact, digest, evidence,
  deviation, fallback, amendment, and blocker;
- accepted P4 friction and its complete reconciliation;
- second-consumer selection, bounds, IDs, behavior, shortcut, setting,
  exclusions, and real journey;
- before and after contribution inventories and the two-consumer extraction
  matrix;
- registry ownership, validators, conflict codes, command, shortcut, settings,
  and required-operation semantics;
- proof that Rust authorization remains independent;
- accessibility, security, native/package, performance, cleanup, regression,
  limitation, and unavailable evidence;
- raw measurements and the full independent status inventory;
- confirmation that P6, product/customer, public SDK/plugin, broad settings,
  diagnostics, release, PR, and merge work did not start.

### Review evidence index

- Include one row for each of the 25 acceptance gates.
- Map each gate to exact source and tests, command and exit status, CI
  run/job/artifact, native/manual evidence, measurement, classification,
  limitation, and digest.
- Keep contract, second-feature, P4 regression, registration/conflicts,
  command/shortcut, settings, Rust authorization, accessibility, CSP/security,
  native/package, CI/manual, performance, extraction, and unavailable evidence
  separate.
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
  literal `<SELF_SHA256>`, hash those canonical bytes with SHA-256, and record
  the resulting lowercase hexadecimal digest in the designated value.
- Reproduce the self-digest by repeating only that replacement without
  changing the field label, spacing, line endings, or any other byte.
- Recompute all digests after finalization.
- Never include user-entered text, native paths, source paths, credentials,
  secrets, or sensitive data.

## 17. Branch, draft-PR, review, merge, and deletion controls

This provisional prompt authorizes no implementation branch or PR. Future
activation must:

- start from the exact accepted merged `main` SHA supplied by Chat Session;
- use one fresh implementation branch named by Chat Session;
- never reuse this documentation branch or a predecessor branch;
- keep changes limited to one coherent `GFD-P5-WP01` implementation;
- commit intentionally and push only after validation and a remote-race check;
- verify final remote commit, parent, tree, paths, hashes, and clean parity;
- leave `main` unchanged;
- stop at `READY FOR CHAT SESSION REVIEW`.

Do not open a draft or ready PR unless a later explicit instruction authorizes
that separate action. Do not enable auto-merge. Prompt acceptance is not
activation, review acceptance does not authorize merge, and P5 completion does
not authorize P6-WP01 or public extraction.

Merge, squash, rebase, force-push, history rewriting, branch deletion, release,
publication, deployment, signing, notarization, updater work, and repository
settings changes require separate explicit authorization.

No public SDK, `packages/ui`, runtime plugin system, generator, or broader
contract extraction may start without separately accepted evidence and
authorization.

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
- all 25 gate outcomes;
- contract, second-feature, P4 regression, registration/conflict,
  command/shortcut, settings, Rust authorization, accessibility, CSP/security,
  native/package, CI/manual, performance, extraction, and unavailable evidence;
- raw measurements and full status inventory;
- confirmation that `main` is unchanged and no PR, merge, P6, product/customer,
  public SDK/plugin, broad settings, diagnostics, or release work started.

For `BLOCKED`, report the exact stop condition, command or evidence, repository
state, changed files, unaffected scope, affected gates, and smallest required
Chat Session decision. Do not claim partial work as implementation success.

End a successful future implementation response with:

```text
Chat Session: Review GFD-P5-WP01 on the exact implementation branch and commit
reported above. Read the second-consumer feature-contracts report, review
evidence index, handoff manifest, and verified source snapshot. Return
Accepted, Focused correction required, or Blocked. Confirm the real second
consumer preceded extraction; both P4 and P5 use the smallest static internal
feature, route, navigation, command, shortcut, and settings contributions;
conflicts fail deterministically; Rust authorization remains independent; all
accessibility, security, native/package, regression, performance, extraction,
status, and artifact-hash evidence is truthful. GFD-P6-WP01, product/customer
features, public SDK/plugin work, PR creation, merge, release, and branch
deletion remain unauthorized.
```
