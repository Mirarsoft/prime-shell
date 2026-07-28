# GFD-P3-WP01 — Productized Contracts and Native Intent Boundary

## 1. Package identity and prompt status

**Package ID:** `GFD-P3-WP01`
**Phase:** `Phase 3`
**Title:** `Productized Contracts and Native Intent Boundary`
**Task ID:** `GFD-P3-WP01`
**Prompt ID:** `PRIME-SHELL-GFD-P3-WP01-PROMPT`
**Prompt version:** `R1`
**Prompt lifecycle state:** `Provisional`
**Implementation status at authoring:** `Not started`
**Recommended implementation model:** `GPT-5.6 Sol`
**Reasoning/intelligence:** `Extra High`
**Authorization boundary:** Exactly one package, `GFD-P3-WP01`
**Execution status:** `Execution is not authorized`

Extra High is required because this package converts the spike-era boundary
into a productized, versioned, least-privilege contract across React,
TypeScript, Rust, Python, Tauri capabilities, native file intent, and
cross-platform evidence. Small inconsistencies could create a generic command
tunnel, leak native paths, bypass Rust authorization, or allow schema drift.

This provisional prompt is complete for review but is non-executable. Chat
Session must first accept it as an approved provisional prompt and later issue
a separately activated revision after `GFD-P2-WP02` is accepted and merged.

### Activation metadata

```text
Activation ID: Not activated — Chat Session must refresh and supply this exact value.
Activated by: Not activated — Chat Session must refresh and supply this exact value.
Activation UTC: Not activated — Chat Session must refresh and supply this exact value.
Authoritative main SHA: Not activated — Chat Session must refresh and supply this exact value.
Required fresh implementation branch: Not activated — Chat Session must refresh and supply this exact value.
Accepted WP01, WP02, WP03, P1-WP01, P2-WP01, and P2-WP02 heads/evidence: Not activated — Chat Session must supply exact merged commits, reports, artifacts, hashes, CI runs/jobs, native/manual evidence, deviations, and blockers.
Final accepted Phase 0B closure report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Final accepted Phase 1 baseline report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Final accepted P2-WP01 theme-foundation report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Final accepted P2-WP02 responsive-shell report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Accepted targeted amendments: Not activated — Chat Session must refresh and supply the exact list or None.
Predecessor deviations incorporated: Not activated — Chat Session must refresh and supply the exact list or None.
Current schema, generator, TypeScript, Rust, Python, Tauri, picker, capability, CSP, tool, engine, and repository facts: Not activated — Chat Session must verify and supply them.
Current support matrix and native/manual evidence limitations: Not activated — Chat Session must refresh and supply the exact list or None.
Accepted title-bar result and per-platform fallback: Not activated — Chat Session must refresh and supply the exact result for Windows, macOS, and Linux.
Unresolved blockers/assumptions: Not activated — Chat Session must refresh and supply the exact list or None.
Authorization boundary: Not activated — the future activation may authorize only GFD-P3-WP01.
Authorization expires/invalidates when: Not activated — the future activation must invalidate on any base-SHA, accepted-predecessor, targeted-amendment, material contract/tool/engine/platform/picker/capability/CSP, repository-layout, or authorization-boundary change.
```

## 2. Repository and exact starting state

**Repository:** `Mirarsoft/prime-shell`
**Authoritative base branch:** `main`
**Required starting commit:** Not activated — Chat Session must refresh and
supply the exact current accepted and merged `main` SHA after P2-WP02
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
- prompt-pack documentation branch:
  `docs/gfd-work-package-prompt-pack-v1`;
- WP02, WP03, Phase 1, P2-WP01, and P2-WP02 activation and
  implementation: `Not started`;
- P3-WP01 implementation: `Not started`.

The documentation refs do not authorize implementation. P3-WP01 activation
and implementation must start from the latest accepted and merged `main`, only
after WP02, WP03, P1-WP01, P2-WP01, and P2-WP02 are accepted and merged and
their required closure, baseline, theme-foundation, and responsive-shell
reports are accepted.

Required activation and Work Session preflight:

1. Verify exact access and write permission for `Mirarsoft/prime-shell`.
2. Verify the default branch is `main` and record its exact 40-character SHA.
3. Verify WP01, WP02, WP03, P1-WP01, P2-WP01, and P2-WP02 are accepted and
   merged at that SHA.
4. Verify the final accepted Phase 0B closure, Phase 1 baseline, P2-WP01
   theme-foundation, and P2-WP02 responsive-shell reports, source snapshots,
   support matrix, platform evidence, measurements, deviations, blockers,
   and amendments.
5. Verify current JSON Schema, fixture, generator, TypeScript, Rust, Python,
   Tauri capability, native-picker, CSP, engine, toolchain, dependency,
   lockfile, test, CI, and repository-path facts.
6. Verify the accepted title-bar choice and native fallback separately for
   Windows, macOS, and Linux.
7. Verify no productized contract/native-intent layer, P3-WP02 task runtime,
   product feature, public SDK, or generic operation tunnel already exists
   unexpectedly.
8. Verify the exact fresh implementation branch does not already exist.
9. Use a clean fresh clone/worktree at the activated starting commit.
10. Verify and narrow every allowed and protected repository path against the
    current tree.
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
11. the accepted WP02 prompt, activation, source, review, implementation
    report, evidence index, handoff manifest, source snapshot, hashes, CI and
    native evidence, measurements, deviations, and blockers named by
    activation
12. the accepted WP03 prompt, activation, source, review, spike-closure
    report, evidence index, handoff manifest, source snapshot, hashes,
    per-platform CI/runtime/native/manual evidence, measurements, deviations,
    fallbacks, and blockers named by activation
13. the accepted P1-WP01 prompt, activation, source, review, baseline report,
    evidence index, handoff manifest, source snapshot, hashes, CI/native/manual
    evidence, pins, locks, support claims, deviations, and blockers named by
    activation
14. the accepted P2-WP01 prompt, activation, source, review,
    theme-foundation report, evidence index, handoff manifest, source
    snapshot, hashes, CI/native/manual evidence, token provenance, contrast
    matrix, platform material and title-bar facts, deviations, and blockers
    named by activation
15. the accepted P2-WP02 prompt, activation, source, review,
    responsive-shell report, evidence index, handoff manifest, source
    snapshot, hashes, CI/native/manual evidence, responsive-band behavior,
    splitter and focus behavior, layout persistence schema, platform
    title-bar facts, deviations, and blockers named by activation
16. every separately accepted targeted architecture amendment named by
    activation
17. current shared schemas and fixtures, schema validation/generation tools,
    frontend contracts and backend invoke wrappers, Rust operation registry,
    backend client and authorization path, Python protocol dispatcher, Tauri
    commands/capabilities/permissions, native file-dialog adapters, persistence
    adapters, CSP, package layout, toolchain/version files, dependency
    manifests and lockfiles, tests, workflows, and accepted support claims
18. the exact activated revision of this execution prompt

In the first implementation progress message, state:

> `AGENTS.md, repository authority, accepted Phase 0A, accepted and merged
> WP01/WP02/WP03, P1-WP01, P2-WP01, and P2-WP02 evidence, the final accepted
> Phase 0B closure report, the final accepted Phase 1 baseline report, the
> final accepted P2-WP01 theme-foundation report, the final accepted P2-WP02
> responsive-shell report, and the activated GFD-P3-WP01 prompt have been read
> and are active. Executing only GFD-P3-WP01 with GPT-5.6 Sol / Extra High.
> GFD-P3-WP02 and later work remain unauthorized.`

If a required source is unavailable, unreadable, stale, or materially
inconsistent, stop before writing. Do not conduct a broad architecture review,
rerun predecessor packages as substitutes for accepted evidence, or improvise
new product, platform, picker, persistence, or support claims.

## 4. Dependencies, predecessor outputs, and prerequisite evidence

**Direct package prerequisite:** Accepted and merged `GFD-P2-WP02`
responsive application shell.

**Inherited predecessors:** Accepted and merged WP01/FIX01,
`GFD-P0B-WP02`, `GFD-P0B-WP03`, `GFD-P1-WP01`, and `GFD-P2-WP01`.

**Authoring-time accepted WP01 commits:**

- accepted WP01 implementation head before squash merge:
  `bd502ffcd4c2d80bd3cd8817ebea4dfb18a76107`;
- accepted merged WP01 `main`:
  `35d53bffec6e5b05aefdf8c7fed39a9bfdf288a2`.

**Later prerequisite state at authoring:** P0B-WP02, P0B-WP03, P1-WP01,
P2-WP01, and P2-WP02 prompts are accepted as Approved provisional, but their
activation and implementation are `Not started`. P3-WP01 cannot be activated
while that remains true.

**Required predecessor outputs:**

- accepted and merged WP01, WP02, WP03, P1-WP01, P2-WP01, and P2-WP02 source;
- exact final merged predecessor commit and authoritative source snapshots;
- final accepted Phase 0B spike-closure, Phase 1 baseline, P2-WP01
  theme-foundation, and P2-WP02 responsive-shell reports;
- accepted platform title-bar/fallback, release-CSP, cross-engine,
  accessibility, support-matrix, toolchain, dependency, capability,
  permission, production-test-exclusion, and CI facts;
- exact schema, fixture, envelope, error, handshake, operation-registry,
  TypeScript invoke, Rust backend-client, Python dispatcher, native-picker,
  storage, engine, and current repository-path facts;
- accepted provider/bootstrap/native-background, semantic-theme, responsive
  shell, splitter, focus-restoration, route, portal, material, layout
  persistence, and title-bar contracts;
- accepted architecture/ADR amendments, deviations, fallbacks, unresolved
  blockers, and manual evidence limitations;
- exact predecessor reports, evidence indexes, manifests, source snapshots,
  hashes, CI run/job/artifact identities, tools, runners, and support claims;
- every separately accepted targeted architecture amendment or an explicit
  accepted `None`.

**Required prerequisite evidence:**

- accepted real native WP01 Unicode/backend/CSP/safe-error/cleanup evidence;
- accepted WP02 lifecycle, cancellation, timeout, crash/hang, restart,
  circuit, no-replay, bounds, and zero-descendant evidence;
- accepted WP03 per-target build/package/runtime/native/manual
  classifications, native WebdriverIO evidence where supported, production
  test-driver exclusion, title-bar/fallback result, cross-engine results, and
  final closure classification;
- accepted P1-WP01 pins, locks, contract-drift checks, CSP/capability
  baseline, support-matrix CI, contribution/security/release guidance, and
  clean-checkout reproducibility;
- accepted P2-WP01 provider/bootstrap/native-background behavior, semantic
  contracts, deterministic token snapshots/provenance/contrast, forced
  colors, material fallback, portals, release-CSP-safe Griffel behavior,
  accessibility, real native startup, and cross-engine results;
- accepted P2-WP02 real routed shell regions, four responsive bands,
  pointer/keyboard splitters, deterministic focus restoration, bounded
  Rust-owned layout preferences, title-bar fallback behavior, accessibility,
  native runtime, and cross-engine results;
- accepted exact limitations and blockers that P3-WP01 must preserve rather
  than silently upgrade.

**Accepted deviations:** None are inferred. Activation must list every
accepted predecessor deviation or state `None`.

**Known blockers carried forward:** Implementation remains `Blocked` until
WP02, WP03, P1-WP01, P2-WP01, and P2-WP02 are accepted and merged; the final
Phase 0B closure, Phase 1 baseline, P2-WP01 theme-foundation, and P2-WP02
responsive-shell reports are accepted; current repository and contract facts
are refreshed; and Chat Session supplies a valid activation block.

## 5. Objective and measurable runnable outcome

Implement the first productized application boundary: versioned,
least-privilege operation contracts and native file intent that remain
consistent across React/TypeScript, Rust, and Python without introducing a
generic command tunnel or a product feature.

The future package is complete only when a clean checkout can prove all of the
following:

- JSON Schema draft 2020-12 is the canonical wire-contract source, with
  versioned envelopes, operations, safe errors, native intents, and opaque
  references;
- shared valid and invalid fixtures exercise every supported operation and
  boundary in TypeScript, Rust, and Python;
- typed TypeScript, Rust, and Python bindings are generated or mechanically
  derived from canonical schemas through pinned, deterministic commands;
- committed generated outputs are reproducible and a drift check fails when
  schemas, fixtures, registries, or generated types disagree;
- Rust owns a compile-time operation registry and authorizes every operation
  before any request can reach Python;
- Python dispatches only the already-authorized, explicitly enumerated
  operation and never accepts an unrestricted method, command, executable, or
  path;
- Rust owns native open/save intent, invokes the native picker, and converts
  selected resources into bounded opaque `DocumentRef` or `ArtifactRef`
  values;
- React receives only typed safe metadata and opaque references, never native
  filesystem paths, unrestricted operation names, picker configuration, or
  backend transport details;
- read/write semantics preserve single-writer ownership, bounded content,
  atomic replacement where required, explicit cancellation, and safe
  rendering;
- malformed, oversized, unauthorized, stale-version, unknown-operation,
  unknown-reference, cancelled-picker, permission, I/O, and backend failures
  produce stable bounded safe errors without secrets or native paths;
- unit, fixture, integration, release-CSP, cross-engine, and focused real
  native evidence prove the boundary at every layer on the accepted support
  matrix;
- no P3-WP02 task runtime, product workflow, feature-specific behavior,
  plugin system, public SDK, or broad storage abstraction is implemented.

## 6. Explicit in-scope work

The activated Work Session may perform only the smallest coherent change set
needed for these items:

### 6.1 Canonical versioned contracts

- Replace spike-only contract authority with a documented productized
  versioning convention that preserves exact request/result correlation,
  trace IDs, bounded frames, handshake compatibility, and stable errors.
- Keep JSON Schema draft 2020-12 as the canonical language-neutral wire
  authority.
- Define strict schemas with explicit required fields,
  `additionalProperties: false`, bounded strings/arrays/objects, finite
  operation enums, and safe version negotiation.
- Define only the minimal operations needed to prove productized contract and
  native-intent behavior. Names, request/result payloads, and ownership must be
  fixed in the accepted activation before implementation.
- Define `DocumentRef` and `ArtifactRef` as opaque, bounded,
  non-path-bearing identifiers with safe display metadata. They must not be
  forgeable into arbitrary filesystem access by React or Python.
- Preserve stable safe error categories and add only narrowly required codes
  for authorization, version, reference, picker, permission, I/O, validation,
  timeout, cancellation, backend, and resource bounds.

### 6.2 Shared fixtures and typed bindings

- Add representative valid and invalid fixtures for every new envelope,
  operation, safe error, reference, cancellation, and boundary case.
- Make the same fixtures run against TypeScript, Rust, and Python validators.
- Add pinned deterministic generation or derivation for TypeScript, Rust, and
  Python types without introducing an unreviewed runtime dependency.
- Commit generated output only when the repository convention requires it,
  identify it clearly, and prove clean regeneration.
- Add a fast drift guard that detects schema, fixture, generated-type,
  operation-registry, dispatcher, and frontend-wrapper disagreement.

### 6.3 Rust-owned operation authorization

- Replace stringly or duplicated operation authorization with one
  compile-time Rust registry that enumerates every supported operation,
  request/result contract, version, limits, required capability, and handler
  destination.
- Parse and validate the request in Rust, resolve the operation through the
  registry, authorize it, enforce version and size limits, and only then
  serialize the exact accepted request to Python.
- Reject unknown, disabled, malformed, stale-version, or unauthorized
  operations before Python receives them.
- Keep the existing bounded sidecar lifecycle, no-shell launch, minimal
  environment, containment, handshake, logging, cancellation, timeout,
  circuit, and no-replay controls accepted from predecessors.
- Keep Tauri commands and capabilities named and narrow. A generic invoke,
  command name, method name, script, argument vector, raw file operation, or
  arbitrary payload tunnel is prohibited.

### 6.4 Python typed dispatch boundary

- Derive Python request/result models or validators from the canonical
  contract source.
- Dispatch through an explicit finite mapping that cannot execute arbitrary
  commands, import arbitrary handlers, or accept an unregistered operation.
- Treat Rust authorization as mandatory but still validate the typed payload
  and resource bounds defensively in Python.
- Keep stdout protocol-only, stderr structured and bounded, and all returned
  errors stable, safe, and path-free.
- Preserve Unicode exactly where the operation contract permits text.

### 6.5 Native intent and opaque references

- Add the minimal Rust-owned native open/save intent adapters required by the
  accepted activation.
- Invoke native pickers only from named least-privilege Tauri commands with
  fixed intent, bounded filters, explicit cancellation, and deterministic
  ownership.
- Canonicalize and authorize selected paths in Rust, then store any path only
  inside a bounded Rust-owned reference registry.
- Return only opaque `DocumentRef` or `ArtifactRef` values plus approved safe
  metadata such as bounded display name, media category, size, and dirty or
  save state when applicable.
- Make references scoped, expirable or invalidatable, non-guessable, and
  rejected after expiry, revocation, wrong intent, wrong type, or host
  restart according to the accepted design.
- Keep React and Python unable to supply or recover an arbitrary native path.
  If Python needs content, Rust supplies only the bounded authorized bytes or
  a purpose-specific stream defined by the operation registry.
- Define one writer for mutable document/artifact state. Use atomic write and
  replacement semantics where the accepted operation requires persistence.

### 6.6 Safe UI integration

- Add only the smallest routed-shell integration needed to demonstrate open,
  cancel, safe metadata, bounded content/result rendering, and save intent.
- Keep UI states explicit: idle, picking, ready, running, cancelling,
  succeeded, failed, stale-reference, and unavailable only as needed by the
  accepted contracts.
- Render untrusted content as text or through an explicitly accepted safe
  renderer. No raw HTML, scriptable URL, arbitrary file URL, or webview
  navigation is allowed.
- Preserve semantic theme, responsive shell, keyboard/focus, forced-colors,
  reduced-motion/transparency, CSP, portal, and title-bar contracts.

### 6.7 Focused documentation and evidence

- Update only the contract/native-intent guidance needed to explain ownership,
  schema versioning, regeneration, operation authorization, opaque references,
  picker cancellation, safe rendering, and known limits.
- Add proportional CI and local commands for contract drift, validators,
  unit/integration tests, release build, and focused native evidence.
- Record exact tool, runner, OS/architecture, engine, commit, command, result,
  and limitation for every evidence claim.

## 7. Explicit exclusions and prohibited adjacent work

Do not:

- implement, activate, repair, or rerun WP02, WP03, P1-WP01, P2-WP01, or
  P2-WP02 as substitute work;
- implement `GFD-P3-WP02`, its bounded task runtime, task queue, scheduling,
  concurrency policy, progress model, cancellation tree, recovery, replay, or
  orchestration;
- implement a product feature, business workflow, editor, import/export
  experience, recent-files experience, template system, search, settings
  center, or domain-specific operation;
- expose a generic frontend-to-Rust invoke, Rust-to-Python method tunnel,
  arbitrary operation name, executable, shell, argument vector, filesystem
  path, URI, SQL, dynamic import, plugin entry point, or unrestricted payload;
- create a public SDK, reusable external protocol, plugin marketplace,
  extension system, network API, remote service, or compatibility promise
  beyond this repository;
- pass native paths, picker options, raw file handles, unrestricted URLs, or
  backend process details to React or Python;
- move picker ownership, path authorization, operation authorization, or
  persistence ownership out of Rust;
- weaken CSP, capabilities, permissions, sandboxing, schema strictness,
  frame/output bounds, logging hygiene, environment minimization, or safe
  error behavior;
- add broad filesystem, shell, process, dialog, URL, clipboard, database, or
  network permissions;
- add a second writer, silently overwrite an existing target, follow unsafe
  symlinks, accept path traversal, or persist unbounded content;
- render untrusted HTML, SVG, Markdown HTML, scriptable media, or file URLs;
- broaden platform or engine support claims beyond accepted evidence;
- redesign the architecture, navigation, theme, responsive shell, title bar,
  sidecar lifecycle, CI estate, or repository layout except for one narrowly
  documented blocker approved by Chat Session;
- perform opportunistic dependency upgrades, formatting sweeps, refactors, or
  cleanup unrelated to this package;
- author or modify another work-package prompt;
- open a PR, enable auto-merge, merge, delete branches, release, publish, or
  deploy.

## 8. Allowed repository areas and expected changes

Activation must replace this provisional list with an exact path inventory
from the accepted merged tree. Expected allowed areas are:

- `packages/app-contracts/schemas/**`
- `packages/app-contracts/fixtures/**`
- narrowly scoped generated or derived contract bindings under an existing or
  explicitly accepted shared package
- `apps/desktop/src/contracts.ts`
- the smallest named frontend backend/native-intent wrappers and focused UI
  integration under `apps/desktop/src/**`
- Rust operation registry, backend client, protocol, native-intent adapter,
  opaque-reference registry, and named command wiring under
  `apps/desktop/src-tauri/src/**`
- narrowly scoped Tauri capability and permission files under
  `apps/desktop/src-tauri/capabilities/**` and
  `apps/desktop/src-tauri/permissions/**`
- Python protocol models, validators, and explicit dispatcher under
  `services/python-backend/**`
- contract generation, drift, fixture, and native verification scripts under
  `scripts/**`
- proportional tests colocated with those areas
- only the existing dependency manifests, lockfiles, CI workflow, and focused
  repository guidance proven necessary by the accepted design.

Protected by default:

- architecture authority and accepted decision records;
- accepted predecessor reports, evidence, and snapshots;
- unrelated routes, shell regions, theme tokens, title-bar behavior, and
  layout preferences;
- packaging, updater, signing, release, deployment, product feature, and
  public API surfaces;
- all work-package prompts.

Every changed path must be listed in the implementation report with its
purpose. Stop for approval if implementation needs a path outside the
activated allowlist. Generated caches, build products, local environments,
secrets, native evidence captures, and temporary files must not be committed.

## 9. Ordered implementation procedure

After activation, execute exactly this order:

1. Complete the full reading order and issue the required acknowledgement.
2. Run the exact activation preflight, record repository/access/base/branch
   facts, and stop on any mismatch.
3. Inventory the current schema, fixture, frontend invoke, Rust registry,
   backend client, Python dispatcher, Tauri capability, picker, persistence,
   CSP, toolchain, dependency, lockfile, test, and CI surfaces.
4. Reconcile that inventory only against accepted authority and predecessor
   evidence; document exact retained contracts, blockers, and narrowly
   required replacements.
5. Write the minimal contract/native-intent design record or implementation
   plan: canonical source, version policy, finite operations, safe errors,
   ownership, reference lifecycle, limits, generation flow, and evidence
   matrix.
6. Define strict JSON Schema 2020-12 contracts and shared valid/invalid
   fixtures for the accepted operations, envelopes, errors, and opaque
   references.
7. Implement pinned deterministic generation or mechanical derivation for
   TypeScript, Rust, and Python bindings, then add clean-regeneration and
   drift checks.
8. Implement the compile-time Rust operation registry with operation,
   version, payload/result, limit, capability, and destination metadata.
9. Move request parsing, schema/version validation, registry lookup,
   authorization, and bounds enforcement ahead of all Python dispatch.
10. Implement typed Python validation and finite dispatch while preserving
    the accepted protocol, process, logging, lifecycle, and resource controls.
11. Implement the narrow Rust-owned native picker intents and bounded opaque
    `DocumentRef`/`ArtifactRef` registry.
12. Implement bounded Rust-owned read/write mediation, reference
    authorization, single-writer rules, and atomic persistence where required.
13. Implement the smallest typed frontend wrappers and safe routed-shell UI
    integration without native paths or unrestricted operation strings.
14. Add unit and shared-fixture tests for every layer, including unknown,
    unauthorized, malformed, stale-version, oversized, stale-reference,
    cancelled-picker, I/O, and safe-error paths.
15. Add integration tests proving Rust rejects unauthorized operations before
    Python, references cannot escape their intent, paths never cross the
    frontend/backend boundary, and generated contracts do not drift.
16. Run formatting, linting, typechecking, contract generation/drift,
    TypeScript, Rust, Python, build, release-CSP, packaging, and focused
    native test commands required by the accepted support matrix.
17. Capture focused real native evidence for picker open/cancel, opaque
    reference handling, bounded safe rendering, save intent, failure paths,
    and cleanup on each support target where accepted tooling permits.
18. Review the complete diff for least privilege, secret/path leakage,
    production test-only surface exclusion, unrelated changes, generated
    files, and prohibited P3-WP02 or product-feature work.
19. Create the required source snapshot and three Markdown handoff artifacts
    with one UTC run ID, hashes, evidence mapping, limitations, and truthful
    status inventory.
20. Commit and push only the activated implementation branch, verify remote
    parity, and stop for Chat Session review without opening a PR.

Do not reorder authorization after Python dispatch, picker ownership outside
Rust, or evidence generation before the final validated source state.

## 10. Cross-cutting constraints

### Architecture and ownership

- React owns presentation and user intent only.
- Rust owns trust boundaries, Tauri command authorization, operation
  authorization, native picker intent, native paths, opaque-reference
  lifecycle, sidecar supervision, bounded I/O mediation, and persistent writes.
- Python owns only typed, authorized operation computation.
- JSON Schema 2020-12 is canonical for the language-neutral wire shape;
  generated language types cannot silently redefine it.
- Every mutable datum has one writer. Readers receive snapshots, typed
  results, or bounded content.

### Least privilege and security

- Use named Tauri commands and explicit capabilities only.
- No generic invoke, operation, shell, process, path, URL, or payload tunnel.
- Rust must reject unauthorized work before Python receives a request.
- Native paths never enter React state, DOM, logs, errors, screenshots,
  artifacts, or Python requests.
- Opaque references must be bounded, intent-scoped, host-scoped, and
  invalidatable; a caller cannot choose or derive a native path from one.
- File filters and picker modes are fixed by the named intent, not supplied
  freely by React.
- Preserve release CSP and prove production bundles exclude test drivers,
  runtime evidence writers, debug permissions, and test-only commands.
- Logs and errors remain structured, bounded, path-free, secret-free, and
  user-safe.

### Contract and compatibility discipline

- Versioning rules must distinguish compatible additive evolution from
  breaking changes and reject unsupported versions deterministically.
- Strict schemas and finite operation enums are required.
- Generators and validators must be pinned and lockfile-consistent.
- Shared fixtures must fail if any language accepts a shape another rejects.
- Handshake `schemaHash`, supported-operation claims, Rust registry, Python
  dispatcher, and frontend wrappers must agree exactly.
- Do not promise public or cross-version compatibility beyond accepted
  in-repository needs.

### Bounds, cancellation, and failure

- Bound frame bytes, content bytes, metadata, reference count, reference
  lifetime, concurrent requests, logs, stderr, timeout, cancellation, and
  shutdown.
- Picker cancellation is an expected typed outcome, not a crash or leaked
  partially created reference.
- Unknown or stale references, rejected permissions, I/O failures, sidecar
  faults, and invalid responses must fail safely without replaying
  side-effecting work.
- Preserve accepted crash/hang/circuit/no-replay/zero-descendant behavior.
- Atomic replacement must avoid partial target corruption and must not
  overwrite without an explicitly accepted save intent.

### UI, accessibility, and rendering

- Preserve the accepted semantic-theme and responsive-shell contracts.
- Every picker or operation state must have accessible name, status, keyboard
  flow, focus restoration, and non-color-only feedback.
- Forced colors, reduced motion/transparency, zoom, narrow width, and
  cross-engine behavior must remain usable.
- Render untrusted content as text or through the one accepted bounded safe
  renderer; never inject raw markup or navigate to file content.

### Reproducibility and evidence

- Use the activated pins, locks, support matrix, and clean-checkout commands.
- Keep generated outputs deterministic across supported targets or classify
  target-specific differences precisely.
- Separate schema/fixture, generated-binding, TypeScript, Rust, Python,
  integration, packaged-runtime, browser-engine, and real-native evidence.
- A mocked picker, jsdom test, schema parse, or process harness cannot be
  presented as real native UI evidence.
- Record exact command, exit status, runner, OS/architecture, engine, tool,
  commit, artifact, hash, and limitation for every claim.

## 11. Proportional tests and exact evidence

Activation must resolve exact commands from the accepted tree. At minimum,
collect these distinct evidence layers:

1. **Schema and fixture evidence**
   - strict draft 2020-12 compilation;
   - all valid fixtures accepted and all invalid fixtures rejected;
   - version, unknown-field, unknown-operation, malformed, oversized,
     reference, cancellation, and safe-error cases.
2. **Generation and drift evidence**
   - pinned generator versions and lockfiles;
   - deterministic TypeScript, Rust, and Python regeneration;
   - clean-tree reproduction;
   - an intentional schema or registry mismatch causes the drift check to
     fail.
3. **TypeScript and frontend evidence**
   - typecheck, lint, unit tests, and production build;
   - typed named wrappers expose no raw path or unrestricted operation;
   - safe rendering and accessible state/focus behavior;
   - release CSP and cross-engine checks.
4. **Rust boundary evidence**
   - format, clippy, unit, integration, and locked build;
   - registry completeness and compile-time enumeration;
   - authorization and bounds occur before Python dispatch;
   - picker ownership, reference scoping, stale/forged reference rejection,
     atomic write behavior, and path-free safe errors.
5. **Python boundary evidence**
   - typed validation and fixture parity;
   - finite dispatch only;
   - unknown/unauthorized/malformed/oversized inputs rejected;
   - stdout protocol-only, structured bounded stderr, Unicode fidelity, and
     path-free errors.
6. **Packaged and real native evidence**
   - accepted per-target package/build/runtime checks;
   - real native picker open and cancellation;
   - opaque-reference open/read or save behavior within the accepted minimal
     operation set;
   - bounded safe content rendering, safe failure, cleanup, and zero leaked
     descendant processes;
   - exact classification of automated native, native manual, browser-engine,
     harness-only, blocked, or not run evidence.
7. **Security and negative evidence**
   - no native path reaches React, DOM, logs, errors, artifacts, or Python;
   - no generic operation/command/payload tunnel;
   - no broad Tauri permission;
   - production bundles exclude test-only commands, evidence writers, and
     drivers;
   - malicious filenames, traversal, symlink, stale reference, wrong-intent,
     oversized content, and forged identifiers fail safely.

Use established repository commands where they remain current. If activation
authorizes new commands, name them explicitly in the implementation report.
Any unsupported target or unavailable native runner must be classified
truthfully; substitute evidence must not be silently upgraded.

The review evidence index must map every acceptance gate to:

- exact source path and relevant test;
- exact command and exit status;
- exact CI run, job, and artifact when applicable;
- exact native/manual evidence file and platform;
- exact limitation, deviation, blocker, or `None`;
- exact SHA-256 digest of every external deliverable.

## 12. Measurable acceptance gates

All gates are future implementation requirements. None is satisfied by this
provisional document.

1. Activation names an exact accepted merged `main` SHA containing WP01,
   WP02, WP03, P1-WP01, P2-WP01, and P2-WP02 plus accepted closure/baseline
   reports, and the Work Session starts cleanly from it.
2. The diff stays within the activated path allowlist and contains no
   P3-WP02, product feature, unrelated refactor, authority rewrite, or
   prompt-pack change.
3. Canonical JSON Schema draft 2020-12 contracts define strict versioned
   envelopes, operations, results, safe errors, and opaque references with
   explicit bounds and no unexpected fields.
4. Shared valid and invalid fixtures cover every supported operation and
   required boundary case, and TypeScript, Rust, and Python agree on every
   result.
5. Pinned deterministic generation or mechanical derivation produces typed
   TypeScript, Rust, and Python bindings from canonical authority.
6. Clean regeneration is byte-stable, and contract drift fails CI when
   schemas, fixtures, generated types, handshake claims, Rust registry,
   Python dispatcher, or frontend wrappers disagree.
7. Rust has one compile-time finite operation registry describing every
   operation, version, limits, required capability, and destination.
8. Rust parses, validates, resolves, authorizes, and bounds every request
   before Python can observe it; negative evidence proves rejected requests
   never reach Python.
9. Python uses typed defensive validation and an explicit finite dispatcher;
   it cannot execute arbitrary commands, dynamic handlers, paths, or
   unregistered operations.
10. Tauri exposes only named least-privilege commands and capabilities; no
    generic invoke, broad dialog/filesystem/process/shell permission, or raw
    transport bridge exists.
11. Native picker ownership remains in Rust with fixed intent and bounded
    filters; cancellation is typed and creates no reference or partial state.
12. `DocumentRef` and `ArtifactRef` are opaque, bounded, non-path-bearing,
    intent-scoped, host-scoped, non-guessable, and rejected when forged,
    stale, revoked, wrong-type, or wrong-intent.
13. Native paths do not cross into React, DOM, errors, logs, screenshots,
    external artifacts, or Python; automated negative checks and diff review
    support the claim.
14. Rust mediates bounded authorized content to Python and preserves
    single-writer ownership, atomic replacement where required, explicit
    overwrite intent, and safe cleanup.
15. Stable bounded path-free errors cover malformed, oversized, unsupported
    version, unknown or unauthorized operation, picker cancellation,
    permission, I/O, stale reference, timeout, cancellation, backend, and
    resource exhaustion outcomes.
16. Existing accepted sidecar handshake, schema-hash, containment, no-shell,
    minimal-environment, lifecycle, cancellation, timeout, circuit,
    no-replay, structured logging, and zero-descendant guarantees remain
    green.
17. The minimal routed-shell UI uses typed wrappers only, exposes no native
    path or unrestricted operation, renders untrusted content safely, and
    presents accessible deterministic states with focus restoration.
18. Accepted theme, forced-colors, reduced-motion/transparency, zoom,
    responsive bands, splitter, title-bar fallback, portal, and keyboard
    contracts remain green in focused tests.
19. Release CSP remains least privilege and production packages exclude
    native evidence writers, test drivers, debug commands, fixtures not
    required at runtime, and test-only permissions.
20. TypeScript, Rust, Python, integration, contract, build, package,
    cross-engine, and accepted support-matrix CI commands pass from a clean
    checkout with locked dependencies.
21. Focused real native evidence proves picker open/cancel, opaque-reference
    behavior, bounded safe rendering, save intent where authorized, safe
    failure, and cleanup on every claimed target; limitations are classified
    exactly.
22. The source snapshot, report, evidence index, and manifest use one run ID,
    have verified SHA-256 digests, contain no secret or native path leakage,
    and reproduce the reviewed commit.
23. The implementation branch contains only the coherent package commit(s),
    matches the remote, has no open PR or auto-merge, and is returned as
    `READY FOR CHAT SESSION REVIEW` without merge or deletion.

If any gate is not met, report `Blocked`, `Not run`, `Partial`, or the exact
accepted limitation. Do not mark the package implemented by inference.

## 13. Stop conditions and blocker reporting

Stop before writing if:

- the activation block is absent, provisional, stale, incomplete, or
  authorizes more than `GFD-P3-WP01`;
- the exact base SHA or fresh branch differs from activation;
- a required predecessor is not accepted and merged;
- a required closure/baseline report, source snapshot, hash, CI/native
  evidence, deviation, amendment, or support claim is missing or inconsistent;
- repository authority, accepted evidence, and current source materially
  disagree;
- the target branch already exists or unrelated work is present;
- an allowed path is absent, renamed, protected, or needs broader scope;
- the design requires a generic tunnel, native path outside Rust, Python-side
  authorization, second writer, broad permission, CSP weakening, public API,
  P3-WP02 runtime, product feature, or architecture redesign;
- canonical schemas cannot generate or mechanically derive consistent typed
  bindings under pinned tools;
- native picker/reference ownership or safe atomic persistence cannot be
  implemented within the accepted platform APIs and support matrix;
- a secret, credential, personal native path, signing material, or sensitive
  user content could enter source, logs, evidence, or artifacts;
- required cross-platform/native evidence is unavailable and Chat Session has
  not accepted a precise limitation;
- remote state changes, push is non-fast-forward, or review would require
  opening/merging a PR.

When blocked:

1. stop at the last clean, non-destructive state;
2. do not broaden scope or weaken a gate;
3. record the exact failed fact, command, path, expected value, observed
   value, affected gates, and whether the tree/branch changed;
4. distinguish repository defect, stale activation, missing authority,
   missing evidence, environment limitation, permission failure, or design
   decision;
5. return one smallest concrete Chat Session decision or refreshed activation
   needed to continue.

## 14. Required functional and status inventory

The final implementation report must state every item below independently:

```text
WP01 packaged Unicode spike: Accepted and merged, with exact commit/evidence.
GFD-P0B-WP02 lifecycle hardening: Accepted and merged, with exact commit/evidence.
GFD-P0B-WP03 packaging and validation closure: Accepted and merged, with exact commit/evidence.
GFD-P1-WP01 technical baseline: Accepted and merged, with exact commit/evidence.
GFD-P2-WP01 theme and accessibility foundation: Accepted and merged, with exact commit/evidence.
GFD-P2-WP02 responsive application shell: Accepted and merged, with exact commit/evidence.
Canonical versioned JSON Schema contracts: Implemented / Partially implemented / Blocked / Not started.
Shared cross-language valid and invalid fixtures: Implemented / Partially implemented / Blocked / Not started.
Deterministic TypeScript binding generation or derivation: Implemented / Partially implemented / Blocked / Not started.
Deterministic Rust binding generation or derivation: Implemented / Partially implemented / Blocked / Not started.
Deterministic Python binding generation or derivation: Implemented / Partially implemented / Blocked / Not started.
Contract and registry drift guard: Implemented / Partially implemented / Blocked / Not started.
Compile-time Rust operation registry: Implemented / Partially implemented / Blocked / Not started.
Rust authorization before Python dispatch: Implemented / Partially implemented / Blocked / Not started.
Typed finite Python dispatcher: Implemented / Partially implemented / Blocked / Not started.
Stable bounded path-free safe errors: Implemented / Partially implemented / Blocked / Not started.
Rust-owned native picker intent: Implemented / Partially implemented / Blocked / Not started.
Opaque DocumentRef boundary: Implemented / Partially implemented / Blocked / Not started.
Opaque ArtifactRef boundary: Implemented / Partially implemented / Blocked / Not started.
Bounded Rust-owned reference lifecycle: Implemented / Partially implemented / Blocked / Not started.
Single-writer and atomic persistence behavior: Implemented / Partially implemented / Blocked / Not started.
Typed frontend intent wrappers: Implemented / Partially implemented / Blocked / Not started.
Safe bounded UI rendering and accessible states: Implemented / Partially implemented / Blocked / Not started.
Release CSP and production test-surface exclusion: Passed / Failed / Blocked / Not run.
TypeScript tests: Passed / Failed / Blocked / Not run.
Rust tests: Passed / Failed / Blocked / Not run.
Python tests: Passed / Failed / Blocked / Not run.
Shared contract/fixture tests: Passed / Failed / Blocked / Not run.
Cross-engine tests: Passed / Failed / Blocked / Not run.
Per-target build/package/runtime evidence: Passed / Failed / Partial / Blocked / Not run.
Focused real native picker/reference evidence: Passed / Failed / Partial / Blocked / Not run.
Native path leakage negative evidence: Passed / Failed / Blocked / Not run.
Generic tunnel negative evidence: Passed / Failed / Blocked / Not run.
Known deviations: Exact list or None.
Known blockers: Exact list or None.
GFD-P3-WP02 bounded task runtime: Not started.
Product features: Not started.
Public SDK or plugin system: Not started.
PR creation: Not started.
Merge: Not started.
Branch deletion: Not started.
```

Do not compress independent rows into a general claim. `Partially implemented`
for an implementation row, `Partial` for an evidence row, `Blocked`, and
`Not run` require a concise reason and evidence reference.

## 15. One `RUN_ID` and collision-resistant external naming

At the start of the future implementation task, after the mandatory authority
reads and preflight but before any repository or deliverable write, create
exactly one UTC run ID in basic ISO-8601 form:

```text
YYYYMMDDTHHMMSSZ
```

Resolve all four target filenames immediately from that one run ID and reuse
it unchanged for every implementation deliverable. Never generate or
substitute a second run ID, timestamp, branch name, mutable label, local time,
or random suffix in the same task. Before any write, verify that none of the
four resolved target names exists in the destination. A collision is a stop
condition: report `Blocked` without generating another run ID.

The run ID identifies the evidence set, not the implementation version. The
manifest must record the exact reviewed commit, tree, parent, branch, source
snapshot root, and each deliverable digest.

## 16. Required deliverables, hashes, and evidence index

Produce exactly these four future implementation deliverables:

1. `prime-shell-work-gfd-p3-wp01-<RUN_ID>-source-snapshot-r1.zip`
2. `prime-shell-work-gfd-p3-wp01-<RUN_ID>-contracts-native-intent-report-r1.md`
3. `prime-shell-work-gfd-p3-wp01-<RUN_ID>-review-evidence-index-r1.md`
4. `prime-shell-work-gfd-p3-wp01-<RUN_ID>-handoff-manifest-r1.md`

### Source snapshot

- Build from the exact reviewed commit, not an uncommitted worktree.
- Use one top-level `prime-shell/` directory and repository-relative paths.
- Exclude `.git`, local environments, dependency caches, build products,
  native packages, evidence captures, secrets, and unrelated artifacts.
- Include the contract sources, generated bindings committed by the package,
  source, tests, scripts, manifests, locks, workflow, and focused guidance
  needed to reproduce review.
- Verify archive integrity, safe paths, expected root, exact inventory, and
  SHA-256 digest.

### Contracts and native-intent report

Record:

- activation identity, model/reasoning, repository, branch, starting commit,
  final commit, parent, tree, and exact changed paths;
- accepted predecessor commits, reports, artifacts, hashes, evidence,
  deviations, blockers, amendments, and support claims;
- before/after contract inventory and every ownership decision;
- canonical schema/version strategy, finite operation set, limits, safe error
  taxonomy, and compatibility policy;
- generator/derivation tools, pins, outputs, and drift behavior;
- Rust registry and pre-Python authorization flow;
- Python typed dispatch and defensive validation;
- native picker intents, opaque-reference lifecycle, path containment,
  content mediation, single-writer, and atomic write behavior;
- frontend wrappers, UI states, safe rendering, accessibility, theme, shell,
  CSP, capability, and production-exclusion results;
- exact commands, CI/native evidence, measurements, failures, limitations,
  deviations, and the full status inventory.

### Review evidence index

- Include a row for each of the 23 acceptance gates.
- Map each gate to exact source/tests, command and exit status, CI run/job and
  artifact, native/manual evidence, classification, and limitation.
- Separate schema, generation, TypeScript, Rust, Python, integration,
  packaged-runtime, browser-engine, and real-native evidence.
- Record evidence tool, runner, OS/architecture, engine, commit, and capture
  time.
- Use `None`, `Not run`, `Blocked`, or `Partial` explicitly; do not leave
  ambiguous blanks.

### Handoff manifest and hashing

- List all four deliverables with exact filename, role, byte length, and
  SHA-256 digest.
- Record source snapshot root, reviewed commit, tree, parent, branch, run ID,
  and generation commands.
- Record ordinary SHA-256 for the other three files and the archive.
- If the manifest records its own digest, designate exactly one self-digest
  value. To compute it, copy the final manifest bytes, replace only that
  self-digest value with the literal `<SELF_SHA256>`, and compute SHA-256 over
  those canonical bytes. Record the resulting lowercase hexadecimal digest in
  the designated value, then reproduce the hash by repeating only that
  replacement; do not alter the field label, spacing, line endings, or any
  other byte.
- After finalization, recompute every digest and verify no deliverable changed.
- Never include credentials, secrets, native paths, or sensitive user content.

## 17. Branch, draft-PR, review, merge, and deletion controls

This prompt does not authorize implementation. A future activation must:

- start from the exact accepted merged `main` SHA supplied by Chat Session;
- create exactly one fresh implementation branch whose name is supplied by
  activation;
- never reuse the documentation branch, a predecessor branch, or a dirty
  worktree;
- keep changes limited to one coherent `GFD-P3-WP01` package;
- commit intentionally with a concise package-scoped message;
- push only after local validation and an immediate remote-race recheck;
- verify the remote branch commit, parent, tree, changed paths, hashes, and
  clean local parity;
- leave `main` unchanged;
- stop at `READY FOR CHAT SESSION REVIEW`.

Do not open a draft or ready PR during implementation unless a later explicit
Chat Session instruction authorizes that separate action. Do not enable
auto-merge, merge, squash, rebase, force-push, rewrite accepted history,
delete a branch, publish, release, or deploy.

If a PR is later authorized, re-read the accepted review checklist and
activation procedure, verify the exact head and base, open only the authorized
draft PR, and stop. Review acceptance, merge authorization, merge execution,
post-merge verification, and branch deletion are separate future decisions.

## 18. Completion response and return prompt

The future implementation response must lead with one exact status:

- `READY FOR CHAT SESSION REVIEW`
- `BLOCKED`
- `NOT STARTED`

For `READY FOR CHAT SESSION REVIEW`, report concisely:

- model and reasoning level;
- repository, starting commit, branch, final commit, parent, and tree;
- exact changed-path inventory and source snapshot;
- all four deliverable filenames and SHA-256 digests;
- test/CI/native evidence classifications and exact limitations;
- all 23 acceptance-gate outcomes;
- the full independent functional/status inventory;
- confirmation that `main` is unchanged and no PR, merge, branch deletion,
  P3-WP02, or product feature was started.

For `BLOCKED`, report the exact stop condition, evidence, repository state,
affected gates, and smallest required Chat Session decision. Do not claim
partial work as implementation success.

End a successful response with this return prompt:

```text
Chat Session: Review GFD-P3-WP01 on the exact implementation branch and commit
reported above. Read the contracts/native-intent report, review evidence index,
handoff manifest, and verified source snapshot. Return Accepted, Focused
correction required, or Blocked. Confirm every acceptance gate, contract
version and drift rule, Rust-before-Python authorization path, native-intent
ownership, opaque-reference and path-secrecy boundary, single-writer behavior,
safe rendering, support claim, limitation, and artifact hash. GFD-P3-WP02,
product features, PR creation, merge, release, and branch deletion remain
unauthorized.
```
