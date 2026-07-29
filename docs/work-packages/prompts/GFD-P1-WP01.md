# GFD-P1-WP01 — Post-Spike Architecture and Repository Baseline

## 1. Package identity and prompt status

**Package ID:** `GFD-P1-WP01`
**Phase:** `Phase 1`
**Title:** `Post-Spike Architecture and Repository Baseline`
**Task ID:** `GFD-P1-WP01`
**Prompt ID:** `PRIME-SHELL-GFD-P1-WP01-PROMPT`
**Prompt version:** `R1`
**Prompt lifecycle state:** `Provisional`
**Implementation status at authoring:** `Not started`
**Recommended implementation model:** `GPT-5.6 Sol`
**Reasoning/intelligence:** `High`
**Authorization boundary:** Exactly one package, `GFD-P1-WP01`
**Execution status:** `Execution is not authorized`

High is the minimum suitable reasoning level because this package reconciles
accepted cross-platform spike evidence with architecture, toolchain,
cross-language contract, CSP, Tauri capability, CI, security, contribution,
and release-guidance boundaries. The work is substantial, but its scope and
ownership are already bounded; Extra High is not required for the future
implementation unless activation discovers a new lifecycle, security, or
cross-platform risk.

This provisional prompt is complete for review but is non-executable. Chat
Session must first accept it as an approved provisional prompt and later issue
a separately activated revision after Phase 0B is accepted and merged.

### Activation metadata

```text
Activation ID: Not activated — Chat Session must refresh and supply this exact value.
Activated by: Not activated — Chat Session must refresh and supply this exact value.
Activation UTC: Not activated — Chat Session must refresh and supply this exact value.
Authoritative main SHA: Not activated — Chat Session must refresh and supply this exact value.
Required fresh implementation branch: Not activated — Chat Session must refresh and supply this exact value.
Accepted WP01, WP02, and WP03 heads/evidence: Not activated — Chat Session must supply exact merged commits, reports, artifacts, hashes, CI runs/jobs, runtime/manual evidence, measurements, deviations, and blockers.
Final accepted Phase 0B closure report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Accepted targeted amendments: Not activated — Chat Session must refresh and supply the exact list or None.
Predecessor deviations incorporated: Not activated — Chat Session must refresh and supply the exact list or None.
Current tools, lockfiles, runners, repository layout, and support matrix: Not activated — Chat Session must verify and supply them.
Current native/manual evidence limitations: Not activated — Chat Session must refresh and supply the exact list or None.
Unresolved blockers/assumptions: Not activated — Chat Session must refresh and supply the exact list or None.
Authorization boundary: Not activated — the future activation may authorize only GFD-P1-WP01.
Authorization expires/invalidates when: Not activated — the future activation must invalidate on any base-SHA, accepted-predecessor, targeted-amendment, material runner/tool/platform-capability, repository-layout, or authorization-boundary change.
```

## 2. Repository and exact starting state

**Repository:** `Mirarsoft/prime-shell`
**Authoritative base branch:** `main`
**Required starting commit:** Not activated — Chat Session must refresh and
supply the exact current accepted and merged `main` SHA after WP02 and WP03
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
- approved-provisional WP02 prompt documentation head:
  `4195d6588afa78448bb0a909e78ce973c0ab1b24`;
- approved-provisional WP03 prompt documentation head:
  `074bb2c8bd117e17477ac97ee859e5e89cfe4f82`;
- prompt-pack documentation branch:
  `docs/gfd-work-package-prompt-pack-v1`;
- WP02 and WP03 activation and implementation: `Not started`;
- Phase 1 implementation: `Not started`.

The documentation refs do not authorize implementation. Phase 1 activation
and implementation must start from the latest accepted and merged `main` only
after WP02 and WP03 are accepted and merged and the final Phase 0B closure
report is accepted.

Required activation and Work Session preflight:

1. Verify exact access and write permission for `Mirarsoft/prime-shell`.
2. Verify the default branch is `main` and record its exact 40-character SHA.
3. Verify WP01, WP02, and WP03 are accepted and merged at that SHA.
4. Verify the final accepted Phase 0B closure report, source snapshot,
   per-platform evidence matrix, measurements, deviations, blockers, and
   targeted amendment decisions.
5. Verify no Phase 1 or later implementation already exists unexpectedly.
6. Verify the exact fresh implementation branch does not already exist.
7. Use a clean fresh clone/worktree at the activated starting commit.
8. Verify every allowed and protected repository path against the current
   tree and remove unnecessary provisional paths during activation.
9. Stop if repository state differs from activation, is ambiguous, would
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
13. every separately accepted targeted architecture amendment named by
    activation
14. current toolchain/version files, dependency manifests and lockfiles,
    contract schemas/fixtures/representations, schema-hash paths, CSP,
    capabilities/permissions, package configuration, verification scripts,
    workflows, security/contribution/release guidance, and current support
    claims
15. the exact activated revision of this execution prompt

In the first implementation progress message, state:

> `AGENTS.md, repository authority, accepted Phase 0A, accepted and merged
> WP01/WP02/WP03 evidence, the final accepted Phase 0B closure report, and the
> activated GFD-P1-WP01 prompt have been read and are active. Executing only
> GFD-P1-WP01 with GPT-5.6 Sol / High. Phase 2 and later work remain
> unauthorized.`

If a required source is unavailable, unreadable, stale, or materially
inconsistent, stop before writing. Do not conduct a broad architecture review
or rerun Phase 0B as a substitute for accepted predecessor evidence.

## 4. Dependencies, predecessor outputs, and prerequisite evidence

**Direct package prerequisite:** Accepted and merged `GFD-P0B-WP03` spike
closure.

**Direct dependent:** `GFD-P2-WP01`.

**Inherited predecessors:** Accepted and merged WP01/FIX01 and
`GFD-P0B-WP02`.

**Authoring-time accepted WP01 commits:**

- accepted WP01 implementation head before squash merge:
  `bd502ffcd4c2d80bd3cd8817ebea4dfb18a76107`;
- accepted merged WP01 `main`:
  `35d53bffec6e5b05aefdf8c7fed39a9bfdf288a2`.

**WP02 and WP03 prerequisite state at authoring:** Their prompts are accepted
as Approved provisional, but activation and implementation are `Not started`.
Phase 1 cannot be activated while that remains true.

**Required predecessor outputs:**

- accepted and merged WP01, WP02, and WP03 source;
- exact final merged Phase 0B commit and authoritative source snapshot;
- final accepted Phase 0B spike-closure report;
- per-platform source-build, package, packaged-runtime, bundled-sidecar,
  native-E2E, installed/clean smoke, native UX/manual, and
  signing/notarization/update evidence matrix;
- raw startup, handshake, echo, task, cancellation, crash/restart, circuit,
  process, package-size, memory, CSP, E2E, platform, and title-bar
  measurements plus normalized results;
- accepted title-bar promotion/fallback decisions, release-CSP and
  cross-engine results, accessibility results, deviations, unresolved
  blockers, and manual evidence gaps;
- exact predecessor reports, evidence indexes, manifests, source snapshots,
  hashes, CI run/job/artifact identities, tools, runners, and support claims;
- every separately accepted targeted `v0.2.1` architecture amendment or an
  explicit accepted `None`.

**Required prerequisite evidence:**

- accepted real native WP01 Unicode/backend/CSP/safe-error/cleanup evidence;
- accepted WP02 lifecycle, cancellation, timeout, crash/hang, restart,
  circuit, no-replay, bounds, and zero-descendant evidence;
- accepted WP03 per-target build/package/runtime/native/manual
  classifications, native WebdriverIO evidence where supported, production
  test-driver exclusion, title-bar/fallback result, and final closure
  classification;
- accepted exact limitations and blockers that Phase 1 must preserve rather
  than silently upgrade.

**Accepted deviations:** None are inferred. Activation must list every
accepted predecessor deviation or state `None`.

**Known blockers carried forward:** Implementation remains `Blocked` until
WP02 and WP03 are accepted and merged, the final Phase 0B closure report is
accepted, and Chat Session supplies an exact activated Phase 1 prompt.

Do not activate Phase 1 while WP03 is only an approved provisional prompt, an
unmerged implementation branch, an unaccepted closure report, or evidence with
an unresolved architecture-blocking failure.

## 5. Objective and measurable runnable outcome

**Objective:** Reconcile accepted Phase 0B evidence with architecture and
repository policy, then establish the smallest maintainable, reproducible
baseline from which Phase 2 can begin safely.

**Required runnable outcome:**

```text
accepted and merged Phase 0B closure evidence
→ accepted targeted architecture and ADR reconciliation
→ pinned and locked toolchains and dependencies
→ executable contract-drift, CSP, and capability checks
→ minimal CI matching the accepted support matrix
→ concise contribution, security, and release guidance
→ clean checkout reproduces every declared baseline gate
```

A fresh authorized checkout must be able to install through pinned toolchains
and frozen locks, detect lock and contract drift, run schema/fixture and
cross-language representation checks, lint/type/test/build the accepted
application, build the accepted sidecar and declared smoke targets, validate
production CSP and capability boundaries, and distinguish supported,
deviation, unverified, and blocked platform claims.

The package must implement executable repository checks and CI gates;
documentation alone is insufficient. It must not add product behavior, expand
the shell, productize the spike, or duplicate Phase 0B evidence collection.

## 6. Explicit in-scope work

The package authorizes only:

1. Compare the final accepted Phase 0B evidence, deviations, fallbacks,
   blockers, and targeted amendment decisions with the frozen architecture,
   decision log, accepted ADRs, and current repository.
2. Apply only separately accepted targeted architecture amendments and add or
   update only narrow ADRs required to record measured repository decisions.
3. Record accepted platform fallbacks, support limitations, deferred
   decisions, and superseded Phase 0A assumptions without broad redesign.
4. Verify and minimally pin or govern Node, pnpm, Rust, Python, Tauri,
   frontend, schema, test, lint, package, native-E2E, target package, and
   accepted CI runner/tool versions.
5. Preserve and verify complete pnpm, Cargo, Python-build, and other accepted
   lock mechanisms, frozen installation commands, lock consistency, version
   ownership, and a proportionate upgrade/rollback procedure.
6. Add the smallest executable drift protection that keeps JSON Schema and
   fixtures authoritative and detects divergence in schema hashes,
   generated/embedded representations, Rust/Python/frontend boundary types,
   and supported-operation declarations.
7. Reconcile and verify the production release CSP, network-denied posture,
   least-privilege Tauri capabilities/permissions, production test-driver
   exclusion, and Rust-owned sidecar/operation/process authority.
8. Establish minimal deterministic CI gates for repository-independent
   checks, contracts, frontend, Python, Rust, accepted target smoke
   builds/packages, and only achievable native/runtime checks.
9. Add concise contribution setup and verification guidance, security
   reporting and trust-boundary guidance, supported/blocked platform claims,
   and release-candidate evidence guidance with Phase 7 signing/updater work
   explicitly deferred.
10. Add or update narrow verification scripts and command wiring only where
    required to make the declared baseline executable from a clean checkout.
11. Run the complete baseline from a clean checkout and retain source/config,
    build, package, runtime/native, CI, manual/unavailable, and documentation
    evidence separately.
12. Produce the required authoritative source snapshot and three Markdown
    handoff artifacts after implementation validation.

This section authorizes one baseline-hardening package only. It grants no
Phase 2, product, publication, merge, release, signing, updater, or
repository-setting authority.

## 7. Explicit exclusions and prohibited adjacent work

Do not:

- activate or implement WP02 or WP03 as part of Phase 1;
- rerun Phase 0B as a substitute for reconciling accepted evidence;
- convert a WP03 amendment proposal into authority without separate Chat
  Session acceptance;
- broadly redesign architecture or silently change trust ownership, runtime
  plugin policy, worker count, durability, data ownership, platform scope,
  package scope, accessibility baseline, or release boundaries;
- begin `GFD-P2-WP01` theme, tokens, semantic palette, Fluent component
  foundation, forced-colors implementation, or material-capability work;
- begin `GFD-P2-WP02` shell, navigation, routing, pane, layout, settings-shell,
  title-bar expansion, or layout-persistence work;
- add a product feature, document analysis, file import, account, database,
  settings system, diagnostics, persistence, repair, recovery, or runtime
  network service;
- productize or expand the WP02 lifecycle/task runtime or add a product
  operation;
- add workers, worker pools, per-task processes, durable tasks, pause/resume,
  replay, GPU/model/scientific orchestration, runtime plugins, a module SDK,
  `packages/ui`, or template extraction;
- complete production signing, notarization, updater enablement, publishing,
  store distribution, release-secret handling, or enterprise deployment;
- add a platform, architecture, Linux distribution, package format, or support
  claim beyond accepted Phase 0B evidence;
- add broad dependency churn, an automated dependency-update service,
  monorepo framework, version catalog, build-system replacement, CI framework,
  benchmark platform, release automation, or deployment pipeline;
- create speculative governance, compliance, ownership, contributor-portal,
  CODEOWNERS, release-management, or enterprise policy;
- treat documentation, fixtures, source/config checks, builds, packages,
  screenshots, browser mocks, or unavailable evidence as real
  runtime/native/manual proof;
- broadly refactor accepted WP01/WP02/WP03 code or evidence harnesses;
- implement a predecessor correction without first stopping for Chat Session
  review, unless activation explicitly authorizes one exact baseline defect
  correction supported by accepted evidence;
- create, open, merge, publish, or delete PRs/branches without the exact
  authorization required in section 17;
- begin `GFD-P2-WP01` or any later package.

If a new product path, platform, package type, worker model, release system,
architecture boundary, or broad repository framework appears necessary, stop
with evidence and request a focused correction or targeted amendment.

## 8. Allowed repository areas and expected changes

Activation must verify current paths, substitute exact accepted Phase 0B
locations, and remove unnecessary entries. Subject to that refresh, only these
responsibility areas may change:

```text
.node-version
package.json
pnpm-workspace.yaml
pnpm-lock.yaml
.npmrc                                      # only if an accepted package-manager baseline requires it
rust-toolchain.toml
rustfmt.toml                                # only when already justified by accepted evidence
apps/desktop/package.json                   # script/dependency pin wiring only
apps/desktop/src-tauri/Cargo.toml
apps/desktop/src-tauri/Cargo.lock
services/python-backend/requirements-build.in
services/python-backend/requirements-build.lock
services/python-backend/pyproject.toml       # only if activation verifies this as current Python authority
packages/app-contracts/                     # drift checks/metadata only; no new product contract
scripts/verify/                             # narrow baseline/drift verification only
apps/desktop/src-tauri/build.rs             # schema-hash drift wiring only
apps/desktop/src-tauri/tauri.conf.json      # accepted CSP/package baseline only
apps/desktop/src-tauri/capabilities/        # least-privilege baseline only
apps/desktop/src-tauri/permissions/         # least-privilege baseline only
.github/workflows/                          # minimal accepted support-matrix gates only
docs/authority/                             # only separately accepted targeted amendments
docs/adrs/                                  # or the activation-verified current ADR location
SECURITY.md
CONTRIBUTING.md
README.md                                   # exact setup/verification/support claims only
docs/release/                               # concise baseline guidance only
```

Expected changes are accepted evidence-backed architecture/ADR reconciliation,
minimal pin/lock and command ownership, contract-drift and schema-hash checks,
CSP/capability/test-driver-exclusion checks, minimal CI, and concise
contribution/security/release guidance.

The following must remain unchanged:

- application feature behavior and UI composition;
- accepted WP02 lifecycle behavior and WP03 evidence harnesses except exact
  baseline wiring or one activation-authorized defect correction justified by
  accepted evidence;
- new product contracts, operations, file intent, settings, diagnostics,
  persistence, recovery, updater, signing, telemetry, SDK, UI-package,
  template, and Phase 2+ areas;
- `docs/phase-0a/` unless activation names one separately accepted targeted
  amendment and exact path;
- prompt-pack files, including this prompt;
- unrelated dependencies, workflows, scripts, tests, documentation, and
  repository settings.

A necessary path outside the activated allowed list is a stop condition unless
it is a mechanically generated/locked companion explicitly anticipated by
activation.

## 9. Ordered implementation procedure

1. Complete the authority, repository, activation, clean-tree, and predecessor
   preflight at the exact activated `main` SHA.
2. Record current repository paths, version authorities, manifests, lockfiles,
   commands, runner images, support claims, accepted Phase 0B evidence,
   deviations, amendments, blockers, and the exact initial status inventory.
3. Run the accepted Phase 0B contract, frontend, Python, Rust, package,
   native/runtime, and process baselines relevant to files that Phase 1 may
   change. Stop on an unexplained predecessor regression.
4. Build an evidence-to-decision map before editing: every proposed authority,
   ADR, support, toolchain, CI, CSP, capability, and guidance change must name
   its accepted evidence and unchanged boundaries.
5. Apply only separately accepted targeted authority amendments and the
   smallest narrow ADR updates required by measured evidence.
6. Reconcile toolchain and dependency pins with the exercised Phase 0B
   versions; preserve existing lock mechanisms and add only missing
   deterministic frozen/locked checks.
7. Extend the current contract verifier rather than creating a broad
   generation framework. Detect schema, fixture, embedded hash,
   representation, and supported-operation drift deterministically.
8. Add focused production CSP, capability/permission, sidecar authority, and
   production test-driver-exclusion verification without granting a new
   product permission.
9. Establish the minimum CI gates matching the accepted support matrix,
   separating fast checks, contracts, frontend, Python, Rust, target smoke
   builds/packages, achievable native/runtime checks, and unavailable
   manual/native evidence.
10. Add concise `CONTRIBUTING.md`, `SECURITY.md`, README, and release guidance
    only as required, using exact commands, trust rules, support limitations,
    credential prohibitions, and deferred Phase 7 work.
11. Run every focused and baseline command continuously, inspect the exact
    diff, and remove redundant tooling, policy, documentation, and
    unsupported claims.
12. Prove the declared baseline from a clean checkout using frozen installs,
    locked Rust resolution, exact contract/CSP/capability checks, frontend,
    Python, Rust, accepted smoke targets, and only the native/runtime gates
    actually retained by activation.
13. Commit only the coherent Phase 1 baseline implementation, create a
    complete source snapshot of the final commit, verify CRC/path safety/one
    root/hash, extract it into a fresh empty directory, and rerun every command
    required by activation.
14. Verify the exact diff, status inventory, evidence classifications,
    deliverables, hashes, branch/PR state, and Phase 2 exclusion; then return
    the handoff without entering Phase 2.

Prefer existing files, direct checks, and concise guidance over a new
framework. Stop rather than broadening scope to make an unsupported claim pass.

## 10. Cross-cutting constraints

### Contract

- JSON Schema 2020-12 plus shared valid/invalid fixtures remains authoritative.
- Phase 1 adds no product or spike operation and changes no accepted public
  contract unless a separately accepted targeted amendment explicitly requires
  it.
- Every accepted operation remains schema-bound and explicitly authorized in
  Rust before Python receives it.
- Contract checks must cover actual schema and fixture paths plus every
  generated or embedded representation retained by the accepted repository.
- The schema bundle hash calculated by verification scripts, embedded by Rust
  build wiring, stored in sidecar build metadata, and checked at handshake
  must use one deterministic definition and fail on stale output.
- Supported-operation declarations in JSON Schema, Rust, Python, frontend
  representations, sidecar metadata, and tests must not silently diverge.
- Stale generated output, embedded hashes, fixtures, or boundary types must
  fail deterministically from a clean checkout.
- Prefer extending the current verifier. Do not introduce a broad IDL or
  code-generation framework without accepted evidence.

### Security and trust

- React remains least trusted; Rust remains native policy, operation, file,
  task, sidecar, and process-lifecycle authority; Python remains trusted
  first-party native code, not a sandbox.
- Do not expose generic shell, process, arbitrary filesystem, secret, native
  path, or unrestricted operation access to React.
- Preserve exact bundled-sidecar resource containment, no-shell launch,
  minimal environment, build/schema/target validation, operation registry,
  bounded frames/queues/logs, and safe redacted errors.
- Production release CSP must preserve accepted Fluent/Griffel behavior
  without broad `unsafe-inline`, arbitrary remote content, or dangerous
  remote-domain IPC.
- Production Tauri capabilities and permissions remain least privilege.
  Test-only WebDriver/runtime-probe permissions must be isolated and absent
  from production artifacts according to accepted Phase 0B evidence.
- Phase 1 adds no new product permission, generic native tunnel, remote
  service, release secret, or publication authority.
- Source, logs, reports, CI, and snapshots must not contain credentials,
  signing material, tokens, user content, raw payloads, environment dumps, or
  unrestricted full paths.

### Lifecycle and concurrency

- Inherit the exact accepted WP02 lifecycle, task states, queue/frame/log
  limits, progress limit, cancellation and terminal precedence, timeout
  semantics, crash/hang behavior, restart budget, circuit behavior, no-replay
  rule, and process-tree containment.
- Phase 1 must not redefine, productize, widen, or reimplement the accepted
  lifecycle.
- Baseline and CI wiring may exercise accepted lifecycle checks but must not
  duplicate the Phase 0B evidence program.
- No worker, worker pool, per-task process, durable task, pause/resume,
  automatic replay, or new product operation is authorized.
- Any required lifecycle correction is a stop condition unless activation
  names the exact accepted defect and allowed correction.

### Accessibility

- Phase 1 adds no shell or feature UI. Preserve accepted keyboard, focus,
  controlled announcement, forced-colors/high-contrast, scaling,
  reduced-motion/transparency, RTL/text-expansion, and native-window results.
- Tooling or CI may retain only proportional accessibility checks supported by
  accepted Phase 0B evidence.
- Documentation must classify unavailable screen-reader, native title-bar, or
  manual checks truthfully rather than treating automation or screenshots as
  authority.
- A baseline change that regresses accepted accessibility or native fallback
  behavior is a blocker.

### Platform and packaging

- The activated support matrix must equal the final accepted Phase 0B matrix;
  no provisional authoring-time matrix is silently upgraded to support
  authority.
- Build the sidecar and native packages only on the target OS/architecture
  required by accepted evidence.
- Keep source checks, build, package creation, packaged runtime, bundled
  sidecar, native E2E, installed/clean smoke, manual/native UX, and
  signing/notarization/update evidence separate.
- CI may automate only achievable accepted gates. Missing manual/hardware
  evidence remains unverified or blocked outside automatic pass claims.
- Phase 1 may establish smoke-build/package gates but may not add production
  signing, notarization, updater, publishing, store, extra package format,
  architecture, distribution, or deployment work.

### Data ownership and privacy

- Phase 1 introduces no product database, durable task store, settings store,
  diagnostics export, file import, network state, or new durable-data writer.
- Preserve accepted single-writer ownership and local/network-denied defaults.
- Build and verification outputs contain only allowlisted operational evidence
  and sanitized tool/target context.
- Contribution, security, and release guidance must prohibit secrets,
  credentials, signing material, raw user content, environment dumps, and
  unrestricted paths in commits, logs, artifacts, and reports.
- Source snapshots contain repository source and sanitized evidence only.

## 11. Proportional tests and exact evidence

Activation must inspect the accepted Phase 0B repository, verify every command,
working directory, version authority, and retained output, and replace stale
commands before changing the lifecycle state to `Activated`.

### Current baseline commands, subject to activation refresh

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

### Exact focused commands required at activation

This provisional revision does not authorize execution. Before activation,
Chat Session must inspect the final accepted repository and supply exact
runnable commands, working directories, environment controls, tools, and
retained outputs for:

- Node, pnpm, Rust, Python, Tauri, frontend, schema, test, package,
  native-E2E, target package-tool, and runner version verification;
- frozen pnpm installation and pnpm manifest/lockfile consistency;
- hash-verified frozen Python build-environment installation and lock
  consistency on every retained target;
- Cargo locked resolution, manifest/lock consistency, format, lint, and test;
- schema, fixture, generated/embedded representation, schema-hash,
  supported-operation, and stale-output drift checks;
- production CSP and Tauri capability/permission validation;
- production WebDriver, runtime-probe, test permission, and test plugin
  exclusion;
- exact fast repository-independent CI gates;
- accepted frontend, Python, Rust, and contract CI gates;
- accepted target smoke builds/packages matching the final WP03 support
  matrix;
- any runtime/native check retained as a baseline gate by accepted evidence;
- contribution, security, release, command, link, and support-matrix
  documentation validation;
- clean-tree and clean-checkout reproducibility;
- source-snapshot CRC/path/root validation, fresh empty extraction, and
  complete activated rerun.

Until those exact focused commands and accepted Phase 0B inputs are supplied in
an activated revision, this prompt remains non-executable.

### Required evidence layers

For every check, record command or method, expected behavior, actual result,
OS/version/architecture/engine/tool identity, duration where relevant,
retained artifact or log, and blocker.

Keep these evidence classes separate:

1. source, configuration, schema, fixture, and drift-check evidence;
2. frozen/locked dependency-installation evidence;
3. frontend, Python, Rust, and cross-language test/build evidence;
4. target source-build and package-creation evidence;
5. packaged-application, bundled-sidecar, and retained runtime/native smoke
   evidence;
6. CI workflow/run/job/artifact evidence;
7. manual/native/accessibility evidence;
8. production signing/notarization/update evidence;
9. documentation accuracy evidence;
10. unavailable or blocked evidence.

A configuration check, unit test, source build, package, screenshot, browser
mock, documentation statement, or prior result cannot be relabeled as current
runtime, native UX, installation, signing, or manual evidence.

### Required CI evidence

Activation must supply exact workflow paths/names, runner labels and
architectures, triggers, least-privilege permissions, commands, caches if any,
artifact-retention behavior, target package tools, and required logs/artifacts.
Retain final run IDs, job IDs, commit SHAs, conclusions, artifact names,
hashes, sizes, and unavailable evidence.

CI must match, never exceed, the accepted Phase 0B support matrix. Do not add
release publishing, signing, deployment, broad benchmark, matrix expansion, or
duplicate spike evidence collection.

## 12. Measurable acceptance gates

The package passes only when retained evidence proves:

1. Exact accepted and merged WP01, WP02, and WP03 predecessors and the final
   accepted Phase 0B closure report, source snapshot, deviations, blockers,
   and per-platform evidence were used.
2. Every authority or ADR change is targeted, separately accepted where
   required, mapped to exact Phase 0B evidence, and identifies the frozen
   boundaries that remain unchanged.
3. No frozen architecture, trust, lifecycle, platform, package,
   accessibility, data-ownership, privacy, or release boundary changed
   silently.
4. Node, pnpm, Rust, Python, Tauri, frontend, contract, packaging, test,
   native-E2E, target package-tool, and runner versions are pinned or
   explicitly governed through accepted repository mechanisms and match
   exercised evidence.
5. Frozen/locked dependency installation, manifest/lock consistency, Python
   hash verification, and Cargo locked resolution work from a clean checkout.
6. Contract/schema drift checks detect stale fixtures, generated or embedded
   representations, schema hashes, boundary types, and supported-operation
   declarations without introducing a new product contract or broad
   generation framework.
7. Production CSP and Tauri capabilities remain least privilege, preserve
   accepted Fluent/Griffel and network-denied behavior, retain Rust sidecar
   authority, and exclude production test-driver/test permissions.
8. Minimal CI commands are exact, deterministic, least privilege, runnable,
   and match the accepted support matrix without unsupported platform,
   runtime, native, installation, signing, or manual claims.
9. Frontend, Rust, Python, contract, drift, CSP/capability, and declared
   smoke-build/package gates pass or are truthfully classified `Blocked` with
   exact cause and unaffected scope.
10. Contribution, security, and release guidance states exact setup and
    verification commands, authority/scope discipline, contract-update rules,
    trust boundaries, credential prohibitions, support limitations,
    release-candidate evidence expectations, Phase 7 deferrals, and no
    publication authority without speculative governance.
11. Accepted Phase 0B functionality, evidence, measurements, fallbacks,
    limitations, and unavailable/manual classifications are not weakened,
    duplicated, falsely upgraded, or presented as rerun when they were not
    exercised.
12. The final diff contains no application feature, Phase 2 implementation,
    lifecycle expansion, product contract, persistence, diagnostics,
    telemetry, SDK, UI package, production signing/updater, release
    publication, unsupported platform, or unrelated work.
13. The source snapshot represents the final commit, has valid SHA-256 and
    CRC, contains one safe expected root, extracts into a fresh empty
    directory, and passes the complete activated rerun with a clean final tree.
14. `GFD-P2-WP01` and every later package remain `Not started`, with no
    unauthorized PR merge, publication, release, branch deletion, or
    repository-setting change.

## 13. Stop conditions and blocker reporting

Stop without improvising when:

- the repository, default branch, activated base SHA, predecessor state,
  accepted closure report, or required fresh branch differs from activation;
- WP02 or WP03 is not accepted and merged;
- a required authority, predecessor source, report, evidence index, manifest,
  snapshot, hash, CI run/job/artifact, measurement, deviation, fallback,
  blocker, targeted amendment decision, or repository path is missing, stale,
  unaccepted, or materially conflicting;
- accepted Phase 0B behavior or evidence regresses before Phase 1 changes;
- a frozen trust, contract, lifecycle, platform, package, accessibility,
  data-ownership, privacy, or release boundary would need to change;
- an authority or ADR change lacks exact accepted evidence or required
  separate Chat Session amendment acceptance;
- the package requires an unapproved path, dependency, platform, package type,
  credential, worker, durable state, product feature, release system, or
  adjacent package;
- exact current repository paths, pin owners, locks, commands, tools, runner
  capabilities, support matrix, or evidence classifications cannot be
  determined safely;
- frozen/locked installation or clean-checkout reproducibility cannot be
  achieved;
- contract drift, schema-hash, CSP, capability, or production-test-exclusion
  checks cannot be made deterministic;
- minimal CI cannot represent the accepted matrix truthfully and with least
  privilege;
- a required smoke build/package or retained runtime/native gate cannot be
  executed in the authorized environment;
- the only support for a claimed gate is documentation-only, fixture-only,
  mock-only, browser-only, source/build/package-only, screenshot-only,
  unavailable, fabricated, or successor-dependent evidence;
- a workflow is blocked by permissions, policy, quota, billing, runner,
  architecture, package tool, credential, or artifact-retention limits;
- unavailable manual/hardware evidence would have to be silently passed;
- completing the work would require Phase 2, broad Phase 0B redesign,
  production release work, unsupported platform claims, speculative
  governance, or an unapproved architecture amendment;
- source-snapshot integrity or fresh-extraction rerun fails;
- the final diff includes an unauthorized path or unrelated existing changes
  cannot be preserved safely.

Return `Blocked` or `Partially implemented` using the approved labels. Name the
exact failed gate, observed evidence, unaffected scope, files changed,
unchanged exclusions, and smallest safe next action. Do not enter Phase 2 or
expand Phase 1 as a workaround.

## 14. Required functional and status inventory

Before activation, the truthful inventory is:

```text
Stage 1 foundation: Implemented and accepted
GFD-P0B-WP02 provisional prompt: Implemented and accepted as Approved provisional
GFD-P0B-WP02 activation/implementation: Not started
GFD-P0B-WP03 provisional prompt: Implemented and accepted as Approved provisional
GFD-P0B-WP03 activation/implementation: Not started
GFD-P1-WP01 package outcome: Not started
Architecture/ADR reconciliation: Not started
Toolchain/dependency baseline: Not started
Contract/schema drift protection: Not started
CSP/capability baseline: Not started
Minimal CI baseline: Not started
Contribution/security/release guidance: Not started
Supporting infrastructure: Not started
Tests: Not started
Documentation for implementation: Not started
Generated code: Not started
Fixtures/mocks: Not started
Stubs/placeholders: Not started
Incomplete work: Not started
Blocked work: Blocked — WP02/WP03 are not accepted and merged, the final Phase 0B closure report is not accepted, and Phase 1 has no activated prompt
GFD-P2-WP01: Not started
```

At implementation completion, report every applicable line above with exactly
one of: `Implemented`, `Partially implemented`, `Stub`, `Mock-only`,
`Not started`, or `Blocked`. Add evidence references and separately report:

- accepted predecessor and final Phase 0B evidence identity;
- each targeted architecture/ADR decision and unchanged boundary;
- each version authority, direct pin, lockfile, frozen/locked command, upgrade
  rule, and rollback expectation;
- contract, fixture, representation, schema-hash, and supported-operation
  drift checks;
- production CSP, capabilities/permissions, Rust sidecar authority, and
  production test-driver exclusion;
- fast, contract, frontend, Python, Rust, target smoke, native/runtime,
  manual/unavailable, and CI evidence separately;
- contribution, security, support, and release-guidance accuracy;
- source snapshot and fresh-extraction rerun;
- incomplete or blocked work;
- Phase 2 and later work as `Not started`.

Documentation or tests may be `Implemented` while a smoke target or CI gate is
`Partially implemented` or `Blocked`; never conflate them.

## 15. One `RUN_ID` and collision-resistant external naming

At the start of a future activated implementation, generate exactly one UTC
identifier:

```text
RUN_ID=YYYYMMDDTHHMMSSZ
```

Use that value unchanged for every external artifact and retained handoff name.
Do not reuse a prior run ID or generate separate IDs for CI targets, reruns,
corrections, snapshots, or the manifest within the same execution.

Repository filenames remain conventional and do not receive timestamps unless
their existing repository contract requires versioning.

## 16. Required deliverables, hashes, and evidence index

The future activated implementation must produce exactly these external
deliverables:

```text
prime-shell-work-gfd-p1-wp01-<RUN_ID>-source-snapshot-r1.zip
prime-shell-work-gfd-p1-wp01-<RUN_ID>-baseline-report-r1.md
prime-shell-work-gfd-p1-wp01-<RUN_ID>-review-evidence-index-r1.md
prime-shell-work-gfd-p1-wp01-<RUN_ID>-handoff-manifest-r1.md
```

`RUN_ID` is the one runtime token defined in section 15.

The source snapshot must:

- represent the exact final implementation commit;
- contain one expected repository root;
- exclude repository metadata, caches, build output, credentials, signing
  material, and unrelated local files;
- pass ZIP CRC and path-safety checks;
- have an ordinary SHA-256;
- extract into a fresh empty directory;
- pass the exact activated required rerun.

The baseline report must:

- map every accepted Phase 0B deviation, fallback, blocker, limitation, and
  targeted amendment decision to the resulting repository decision;
- identify every authority/ADR change and unchanged frozen boundary;
- list exact version authorities, pins, locks, upgrade/rollback rules,
  executable drift checks, CSP/capability checks, CI gates, and responsible
  repository paths;
- classify source, build, package, runtime/native, CI, manual/unavailable,
  signing/update, and documentation evidence separately;
- state supported, deviation, unverified, blocked, and deferred claims
  truthfully;
- identify what remains deferred to Phase 2 or later.

The review evidence index must map every package requirement and acceptance
gate to a repository file/section, command/result, retained artifact, and
unresolved item without duplicating the baseline report.

The handoff manifest must record:

- activation ID, prompt ID/version, model/reasoning, repository, branch, exact
  base and final commit;
- accepted WP01/WP02/WP03 refs, closure report, source snapshots, artifacts,
  hashes, CI/native/manual evidence, measurements, deviations, fallbacks,
  amendments, limitations, and blockers;
- tools, runners, target matrix, version authorities, locks, changed files,
  and responsible checks;
- exact commands and actual results;
- source/config, locked install, frontend, Python, Rust, contract, build,
  package, runtime/native, CI, manual, documentation, signing/update, and
  unavailable evidence separately;
- SHA-256 of the source snapshot, prompt-required reports, and required
  repository artifacts;
- a reproducible canonical self-hash convention if the manifest contains its
  own digest;
- truthful functional/status inventory, limitations, files intentionally not
  created, blockers, and exact next controlled action.

Expose the four artifacts individually to Chat Session. Do not create another
package prompt or present an incremental overlay as the authoritative source.

## 17. Branch, draft-PR, review, merge, and deletion controls

This provisional prompt authorizes no implementation branch or pull request.

When Chat Session activates it:

1. Use only the exact fresh implementation branch named in the activation
   record, created from the exact activated `main` SHA.
2. Do not reuse the documentation branch or a Phase 0B implementation branch.
3. Keep one coherent Phase 1 baseline change and preserve unrelated user work.
4. Commit intentionally with a package-scoped message and push only the
   activated branch.
5. Open or update one draft PR only if the activated prompt explicitly
   authorizes it and supplies the exact base/head controls.
6. Keep any authorized PR draft until Chat Session accepts the exact head.
7. Do not enable or use auto-merge.
8. Do not merge, publish, release, sign, notarize, enable updates, change
   repository settings, or delete a branch without separate explicit user
   authorization after Chat Session acceptance.
9. After an explicitly authorized merge, verify `main` contains the accepted
   result before deleting only the exact merged implementation branch.
10. Prompt acceptance is not activation; implementation acceptance is not
    merge authorization; Phase 1 completion is not Phase 2 authorization.

Report exact local/remote branch, commit, PR, draft, auto-merge, merge, and
deletion state in the handoff.

## 18. Completion response and return prompt

Keep progress and completion messages concise. Lead with the truthful package
result and include:

- model and reasoning level;
- repository, base, branch, final commit, clean-tree, PR, and merge state;
- accepted Phase 0B evidence and targeted amendment identities;
- architecture/ADR reconciliation and unchanged boundaries;
- pins, locks, contract drift, CSP/capability, CI, and guidance outcomes;
- exact commands and actual results;
- source/config, locked-install, frontend, Python, Rust, contract, target
  smoke, runtime/native, CI, manual/unavailable, signing/update, and
  documentation evidence separately;
- full status inventory, limitations, and blockers;
- four external artifact links and hashes;
- confirmation that Phase 2 and later work were not started.

At completion, define:

```text
FINAL_SHA=the exact 40-character final implementation commit
```

Then end with this short prompt, substituting the defined `FINAL_SHA` and the
section 15 `RUN_ID` with their exact values:

```text
Chat Session: Review GFD-P1-WP01 on the activated implementation branch at FINAL_SHA.
Read the four prime-shell-work-gfd-p1-wp01-RUN_ID deliverables. Verify targeted
architecture reconciliation, exact pins and frozen locks, contract/schema drift
protection, least-privilege CSP/capabilities, minimal support-matrix CI,
guidance accuracy, clean-checkout and snapshot reruns, then return Accepted,
Focused correction required, or Blocked. Phase 2 was not started, and this
handoff does not authorize merge or later implementation.
```

Do not include authorization for Phase 2 or any later package. Do not continue
after returning the handoff.
