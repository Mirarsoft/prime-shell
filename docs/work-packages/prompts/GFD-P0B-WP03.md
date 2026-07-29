# GFD-P0B-WP03 — Cross-Platform Evidence and Spike Closure

## 1. Package identity and prompt status

**Package ID:** `GFD-P0B-WP03`
**Phase:** `Phase 0B`
**Title:** `Cross-Platform Evidence and Spike Closure`
**Task ID:** `GFD-P0B-WP03`
**Prompt ID:** `PRIME-SHELL-GFD-P0B-WP03-PROMPT`
**Prompt version:** `R1`
**Prompt lifecycle state:** `Provisional`
**Implementation status at authoring:** `Not started`
**Recommended implementation model:** `GPT-5.6 Sol`
**Reasoning/intelligence:** `Extra High`
**Authorization boundary:** Exactly one package, `GFD-P0B-WP03`
**Execution status:** `Execution is not authorized`

Extra High is the minimum suitable reasoning level because this package must
classify cross-platform build, package, packaged-runtime, native automation,
native-window, release-CSP, accessibility, process, and measurement evidence
without conflating evidence layers or expanding the spike.

This provisional prompt is complete for review but is non-executable. Chat
Session must first accept it as an approved provisional prompt and later issue
a separately activated revision after WP02 is accepted and merged.

### Activation metadata

```text
Activation ID: Not activated — Chat Session must refresh and supply this exact value.
Activated by: Not activated — Chat Session must refresh and supply this exact value.
Activation UTC: Not activated — Chat Session must refresh and supply this exact value.
Authoritative main SHA: Not activated — Chat Session must refresh and supply this exact value.
Required fresh implementation branch: Not activated — Chat Session must refresh and supply this exact value.
Accepted WP01 and WP02 heads/evidence: Not activated — Chat Session must supply exact commits, reports, artifacts, hashes, CI runs/jobs, runtime evidence, measurements, deviations, and blockers.
Accepted targeted amendments: Not activated — Chat Session must refresh and supply the exact list or None.
Predecessor deviations incorporated: Not activated — Chat Session must refresh and supply the exact list or None.
Current target matrix, runners, tools, and architectures: Not activated — Chat Session must verify and supply them.
Current native automation capability: Not activated — Chat Session must verify and supply exact supported targets and limitations.
Manual-test ownership and target access: Not activated — Chat Session must name exact owners/environments or mark them unavailable.
Signing/notarization credentials and scope: Not activated — Chat Session must record availability while preserving the production-signing deferral.
Unresolved blockers/assumptions: Not activated — Chat Session must refresh and supply the exact list or None.
Authorization boundary: Not activated — the future activation may authorize only GFD-P0B-WP03.
Authorization expires/invalidates when: Not activated — the future activation must invalidate on any base-SHA, accepted-predecessor, material runner/platform/tool capability, manual-owner, or authorization-boundary change.
```

## 2. Repository and exact starting state

**Repository:** `Mirarsoft/prime-shell`
**Authoritative base branch:** `main`
**Required starting commit:** Not activated — Chat Session must refresh and
supply the exact current accepted and merged `main` SHA after WP02 acceptance
and merge.
**Required implementation branch:** Not activated — Chat Session must verify
absence and supply one exact fresh branch name.

Authoring-time facts, for review only:

- accepted WP01 implementation head before squash merge:
  `bd502ffcd4c2d80bd3cd8817ebea4dfb18a76107`;
- accepted WP01 squash-merged `main`:
  `35d53bffec6e5b05aefdf8c7fed39a9bfdf288a2`;
- accepted WP01 Ubuntu evidence: Actions run `30285523445`, job
  `90042226699`, successful;
- prompt-pack documentation branch:
  `docs/gfd-work-package-prompt-pack-v1`;
- accepted Stage 1 foundation head:
  `cf2c2613d1facca1e61ba03eab88078a0fc1ffbf`;
- approved-provisional WP02 prompt documentation head:
  `4195d6588afa78448bb0a909e78ce973c0ab1b24`;
- WP02 implementation: `Not started`.

The documentation refs do not authorize implementation. WP03 activation and
implementation must start from the latest accepted and merged `main`, only
after accepted WP02 implementation is present there.

Required activation and Work Session preflight:

1. Verify exact access and write permission for `Mirarsoft/prime-shell`.
2. Verify the default branch is `main` and record its exact 40-character SHA.
3. Verify WP01 and WP02 are accepted and merged at that SHA.
4. Verify all required predecessor source, reports, evidence indexes,
   manifests, hashes, CI runs/jobs, runtime evidence, measurements, deviations,
   and blockers.
5. Verify no Phase 1 or later application package has begun unexpectedly.
6. Verify the exact fresh implementation branch does not already exist.
7. Use a clean fresh clone/worktree at the activated starting commit.
8. Verify every allowed and protected path against the current tree.
9. Stop if repository state differs from activation, is ambiguous, would
   overwrite unrelated work, or would require silently rebasing the package.

The merged repository is authoritative over snapshots, unmerged branches, chat
summaries, provisional prompts, and superseded handoffs.

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
    handoff manifest, source snapshot, artifact hashes, and CI/native evidence
    named by activation
11. the accepted WP02 prompt, activation, review, implementation report,
    evidence index, handoff manifest, source snapshot, artifact hashes,
    CI/native evidence, measurements, deviations, and blockers named by
    activation
12. every accepted targeted architecture amendment named by activation
13. current contracts, frontend, Rust backend/lifecycle, Python sidecar,
    toolchains, lockfiles, native/runtime verification, package configuration,
    workflows, and current spike evidence/report paths
14. the exact activated revision of this execution prompt

In the first implementation progress message, state:

> `AGENTS.md, repository authority, accepted Phase 0A, accepted and merged WP01
> and WP02 predecessor evidence, and the activated GFD-P0B-WP03 prompt have
> been read and are active. Executing only GFD-P0B-WP03 with GPT-5.6 Sol /
> Extra High. Phase 1 and later work remain unauthorized.`

If a required source is unavailable, unreadable, stale, or materially
inconsistent, stop before writing. Do not conduct a broad architecture review.

## 4. Dependencies, predecessor outputs, and prerequisite evidence

**Direct package prerequisite:** Accepted and merged `GFD-P0B-WP02`.

**Direct dependent:** `GFD-P1-WP01`.

**Inherited predecessor:** Accepted and merged WP01/FIX01.

**Authoring-time accepted WP01 commits:**

- accepted WP01 implementation head before squash merge:
  `bd502ffcd4c2d80bd3cd8817ebea4dfb18a76107`;
- accepted merged WP01 `main`:
  `35d53bffec6e5b05aefdf8c7fed39a9bfdf288a2`.

**WP02 prerequisite state at authoring:** Its prompt is accepted as Approved
provisional, but activation and implementation are `Not started`. WP03 cannot
be activated while that remains true.

**Required predecessor outputs:**

- accepted WP01 source and evidence for the minimal native
  React/Fluent/Tauri host, Rust-authorized Unicode echo, packaged PyInstaller
  `onedir` sidecar, release CSP, backend readiness, safe error path, and
  zero-sidecar normal close;
- accepted WP02 source and evidence for bounded progress, cancellation,
  timeout, crash/hang handling, one bounded restart, circuit behavior, no
  replay, process-tree containment, and normal/forced-close zero descendants;
- exact accepted WP01 and WP02 commits, source snapshots, reports, evidence
  indexes, manifests, hashes, CI runs/jobs, runtime evidence, measurements,
  deviations, blockers, and current repository paths;
- current target toolchain, lockfile, package, native-test, workflow, and
  evidence-retention state.

**Required prerequisite evidence:**

- accepted real native WP01 Unicode/backend/CSP/safe-error/cleanup evidence;
- accepted real packaged-sidecar WP01 evidence;
- accepted WP02 real task lifecycle, cancel/timeout/crash/hang/restart/circuit,
  no-replay, log/queue/frame bounds, and process-tree evidence;
- successful predecessor CI evidence with exact run/job/commit identities;
- truthful predecessor limitations and manual/platform gaps.

**Accepted deviations:** None are inferred. Activation must list every accepted
WP01/WP02 deviation or state `None`.

**Known blockers carried forward:** Implementation remains `Blocked` until
WP02 is accepted and merged and Chat Session supplies an exact activated WP03
prompt. Activation must refresh all remaining blockers.

WP03 may consume and consolidate accepted WP02 lifecycle evidence. It must not
activate or implement WP02. It may correct WP01/WP02 behavior only when a
reproduced target-specific WP03 defect requires one narrowly scoped fix inside
the activated allowed areas.

## 5. Objective and measurable runnable outcome

**Objective:** Close Phase 0B by producing truthful, separately classified
cross-platform evidence for the accepted spike rather than adding product
functionality.

**Required runnable outcome:**

```text
accepted WP01 native baseline + accepted WP02 lifecycle/task implementation
→ target-specific build and package jobs
→ real native Tauri launch with bundled onedir sidecar
→ native WebdriverIO journey where supported
→ title-bar candidate validation or native fallback
→ release-CSP / Fluent / cross-engine checks
→ measurements and installed-artifact evidence
→ final Phase 0B pass/fail and deviation report
```

The package must establish truthful evidence for exactly this declared matrix:

| Platform | Architecture and engine | Package candidate |
|---|---|---|
| Windows 11 | x64 / WebView2 | NSIS candidate |
| Current and previous declared macOS major versions | arm64 / WKWebView | Unsigned or test-signed `.app` and DMG-layout candidate |
| Ubuntu 24.04 | x64 / WebKitGTK, with exercised Wayland/X11 behavior classified separately | `.deb` |

For each target, record source build, package creation, packaged launch,
bundled-sidecar launch without external Python, native automation, clean or
installed smoke where feasible, manual native UX, and
signing/notarization/update evidence as separate layers.

The package ends with one final Phase 0B spike report that maps every acceptance
item to WP01, WP02, or WP03 evidence; records raw measurements, deviations,
fallbacks, blockers, and manual gaps; proposes only evidence-backed targeted
`v0.2.1` amendments; and classifies Phase 0B as `READY`,
`READY WITH ASSUMPTIONS`, or `BLOCKED` for Chat Session review. It does not
begin or authorize Phase 1.

## 6. Explicit in-scope work

The package authorizes only:

1. Verify and retain the exact target, architecture, engine, runner, package
   tool, artifact-retention, native-automation, manual-owner, and credential
   facts supplied by activation.
2. Add one small real native Tauri journey using WebdriverIO with
   `@wdio/tauri-service` or the activation-verified accepted equivalent.
3. Exercise only accepted functionality in that journey: visible shell,
   backend readiness, exact Unicode echo, one focused accepted WP02 lifecycle
   path, one safe error/recovery state, and orderly close/cleanup where the
   driver can observe it.
4. Verify the embedded WebDriver/testing capability is absent from production
   release artifacts.
5. Build the target-specific PyInstaller `onedir` sidecar and application
   candidate on Windows x64, macOS arm64, and Ubuntu x64 using
   activation-verified pinned tools.
6. Create and inspect exactly the Windows NSIS candidate, macOS `.app` and
   DMG-layout candidate, and Ubuntu `.deb`.
7. Launch the packaged application and bundled sidecar without external Python
   on every target where runtime evidence is claimed.
8. Run installed-artifact or clean-runner smoke where actually feasible and
   classify unavailable installation evidence separately.
9. Validate Windows custom-title-bar promotion gates or activate and report the
   accepted native-decoration fallback.
10. Validate macOS native traffic-light/overlay behavior and Ubuntu native
    decorations where target/manual evidence is available; retain missing
    manual evidence as unverified or blocked.
11. Verify release CSP preserves Fluent and Griffel rendering without broad
    unsafe weakening and retain WebView2, WKWebView, and WebKitGTK shell
    evidence with engine-specific deviations.
12. Perform proportional keyboard, focus, controlled-announcement, forced
    colors/high-contrast, scaling, reduced-motion/transparency, RTL/text
    expansion, and native accessibility checks appropriate to the spike.
13. Collect, normalize, retain, and classify the required timing, task,
    restart, process, package-size, memory, CSP, E2E, platform, and title-bar
    measurements.
14. Regress accepted WP01 and WP02 behavior on relevant targets, including
    Unicode, backend readiness, lifecycle/cancel/crash/restart/no-replay, and
    zero-descendant cleanup.
15. Add only focused target-specific fixes demonstrated necessary by WP03
    evidence, documenting each defect, changed path, evidence, and why it does
    not add functionality.
16. Produce the final repository Phase 0B spike report and the required source
    snapshot plus three external Markdown handoff artifacts.

This section authorizes one evidence-and-closure package only. It grants no
WP02 activation/implementation, Phase 1, product, publication, merge, release,
or repository-setting authority.

## 7. Explicit exclusions and prohibited adjacent work

Do not:

- activate or implement WP02 as part of WP03;
- add lifecycle/task behavior beyond accepted WP02 except a focused,
  demonstrated portability correction;
- implement a product feature, document analysis, file import, database,
  account, or runtime network service;
- expand into the full Phase 2 design system, broad application shell,
  settings, diagnostics, persistence, or recovery;
- claim production signing/notarization, public release, publishing, store
  distribution, or updater enablement;
- add AppImage, another Linux distribution, another package format, x86 macOS,
  Windows arm64, universal binaries, or another architecture;
- add runtime plugins, a module SDK, `packages/ui`, template extraction,
  generators, or speculative frameworks;
- introduce multiple Python workers, a worker pool, per-task processes,
  durable task resume, automatic replay, GPU/model/scientific runtime, or a
  product database;
- add telemetry, crash upload, enterprise deployment, remote diagnostics, or
  release-governance infrastructure;
- treat CI build success as packaged runtime, native UX, clean installation,
  signing, notarization, or manual evidence;
- treat screenshots as title-bar/native-window interaction authority;
- silently pass unavailable Windows, macOS, Wayland/X11, accessibility, or
  manual checks;
- ship an embedded WebDriver/testing plugin in production artifacts;
- weaken CSP with broad `unsafe-inline` or enable dangerous remote-domain IPC;
- broadly refactor WP01/WP02 or redesign frozen architecture;
- amend architecture beyond targeted `v0.2.1` proposals supported by measured
  evidence and separately approved before implementation;
- begin `GFD-P1-WP01` or any dependent/later package;
- create, open, merge, publish, or delete PRs/branches without the exact
  authorization required in section 17.

If a new package format, architecture, product path, worker model, production
credential claim, or broad predecessor correction appears necessary, stop with
evidence and request a focused correction or targeted amendment.

## 8. Allowed repository areas and expected changes

Activation must verify current paths and narrow this list where possible.
Subject to that refresh, only these responsibility areas may change:

```text
tests/e2e/
tests/platform-smoke/
scripts/verify/                         # focused native/platform/measurement checks
scripts/package/                        # current target packaging support only when required
.github/workflows/                      # target-specific WP03 build/package/native evidence only
wdio.*                                  # exact native-E2E configuration only
package.json                            # exact WebdriverIO/script wiring only
pnpm-lock.yaml                          # mechanically resulting locked resolution only
apps/desktop/package.json               # exact native-E2E/script wiring only
apps/desktop/src-tauri/tauri.conf.json  # focused package/test-driver configuration only
apps/desktop/src-tauri/capabilities/    # least-privilege test/production separation only
apps/desktop/src-tauri/permissions/     # least-privilege test/production separation only
apps/desktop/src-tauri/Cargo.toml       # only a direct native-test/package requirement
apps/desktop/src-tauri/Cargo.lock       # mechanically resulting locked resolution only
docs/spike/                             # or the activation-verified spike-report location
```

Activation must refresh the exact final repository spike-report path before
implementation.

A reproduced target-specific WP03 defect may authorize a narrow correction in
the current application, Rust, Python, frontend, contract, packaging, or
workflow path only when activation explicitly retains this correction rule,
the defect is documented before the change, and the correction adds no new
functionality.

Expected changes are native-E2E and platform-smoke coverage, target build and
package evidence, release-CSP/test-driver separation checks, measurement
collection, minimal package support where missing, and one final Phase 0B
spike report.

The following must remain unchanged:

- `AGENTS.md`;
- `docs/authority/` and `docs/phase-0a/`, except a separately approved
  evidence-backed targeted amendment;
- `docs/work-packages/`, including WP02 and this prompt;
- product features, databases, accounts, network services, settings,
  diagnostics, SDK, UI-package, updater, telemetry, enterprise, and Phase 1+
  areas;
- WP02 lifecycle scope except a focused reproduced portability correction;
- unrelated dependencies, workflows, scripts, tests, and release files.

A necessary path outside the activated allowed list is a stop condition unless
it is a mechanically generated/locked companion or a documented focused
defect correction explicitly permitted by activation.

## 9. Ordered implementation procedure

1. Complete the authority, repository, activation, clean-tree, and predecessor
   preflight at the exact activated `main` SHA.
2. Record current paths, pinned tool versions, target matrix, runner images,
   package tools, native-automation capability, target/manual owners,
   credentials, accepted deviations, blockers, and the exact initial status
   inventory before writing.
3. Verify accepted WP01 and WP02 source snapshots, hashes, CI/native evidence,
   measurements, and real behavior relevant to WP03. Stop on predecessor
   absence or regression.
4. Define a per-target evidence matrix before implementation, with separate
   cells for build, package, packaged runtime, bundled sidecar, native E2E,
   installed/clean smoke, native UX/manual, signing/notarization/update, and
   blocker status.
5. Verify the current WebdriverIO/Tauri-service compatibility and add the
   smallest real native journey with production test-capability separation.
6. Add only the target-specific build/package/native evidence jobs and focused
   verification scripts required by the activated matrix.
7. Build the target sidecar and application package on its target OS and
   architecture; retain exact commands, versions, logs, hashes, sizes, and
   artifact layouts.
8. Launch packaged applications with their bundled sidecars, prove no external
   Python, exercise accepted Unicode/backend/WP02 behavior, and record process
   cleanup on every target where runtime evidence is claimed.
9. Run the native WebdriverIO journey on every declared automated target the
   activated environment actually supports. Keep unsupported targets
   unverified or blocked.
10. Exercise release CSP/Fluent/Griffel, engine-specific shell behavior,
    accessibility smoke, and production test-driver exclusion.
11. Execute Windows title-bar promotion checks; promote only when every
    critical native gate passes, otherwise use and report native decorations.
    Retain truthful macOS traffic-light and Ubuntu Wayland/X11 native-decoration
    results according to available ownership.
12. Run installed-artifact or clean-runner smoke where feasible. Do not convert
    package creation into installation or native UX evidence.
13. Collect the activated measurement set with raw values, environment,
    method, sample count, and classification.
14. Reproduce and document any target-specific defect before applying one
    narrow correction; rerun the affected predecessor and WP03 checks.
15. Converge target results into the final Phase 0B spike report, mapping every
    requirement to WP01, WP02, or WP03 evidence, deviations, blockers, manual
    gaps, fallbacks, and any targeted amendment proposal.
16. Run the full activated proportional baseline from a clean tree and confirm
    no production test capability, unsupported claim, product work, or later
    package is present.
17. Commit only the coherent WP03 implementation, create a complete source
    snapshot of the final commit, verify CRC/path safety/one root/hash, extract
    it into a fresh empty directory, and rerun every command required by
    activation.
18. Verify the exact diff, status inventory, evidence classifications,
    descendants, deliverables, hashes, branch/PR state, and final Phase 0B
    classification; then return the handoff without beginning Phase 1.

Prefer direct target evidence and a native fallback over abstractions or
unsupported visual uniformity. Stop rather than widening the support matrix.

## 10. Cross-cutting constraints

### Contract

- JSON Schema 2020-12 plus shared valid/invalid fixtures remains authoritative.
- WP03 adds no new product or test operation. It consumes the accepted WP01 and
  WP02 operation set.
- Every operation remains schema-bound and explicitly authorized in Rust.
- Unknown operations are rejected in Rust before Python receives them.
- Preserve accepted handshake, normal-frame, structured-log, pending-request,
  event-queue, and UI-progress bounds.
- Preserve UTF-8 JSON Lines, CRLF acceptance, protocol-only stdout,
  structured-log-only stderr, continuous pipe draining, terminal retention,
  and deterministic malformed/oversized/exhaustion behavior.
- Activation must refresh exact schemas, generated types, fixtures, commands,
  hashes, and any accepted WP02 deviations.
- A demonstrated engine/package defect may correct the contract only with a
  targeted accepted amendment when it changes an external boundary.

### Security and trust

- React remains least trusted; Rust remains native policy, operation, task,
  and process-lifecycle authority; Python remains trusted first-party native
  code, not a sandbox.
- Do not expose generic shell, process, arbitrary filesystem, secret, native
  path, or unrestricted operation access to React.
- Rust launches only the exact verified bundled sidecar without a shell,
  constructs a minimal environment, and validates build/schema/target identity.
- Production artifacts must exclude WebDriver/testing capabilities and
  test-only permissions/plugins.
- Keep capabilities least privilege and separate test configuration from
  production configuration.
- Preserve local-only assets, network-denied runtime, and dangerous
  remote-domain IPC denial.
- Release CSP must preserve Fluent/Griffel without broad `unsafe-inline`.
- Logs and evidence must exclude raw payloads, user content, credentials,
  authorization headers, environment dumps, signing material, secrets, and
  unrestricted full paths.

### Lifecycle and concurrency

- Inherit the exact accepted WP02 lifecycle, task states, queue limits,
  progress limits, cancellation/terminal precedence, timeout semantics,
  crash/hang behavior, restart budget, circuit behavior, no-replay rule, and
  process-tree containment.
- WP03 must not redefine or productize the WP02 lifecycle.
- A target-specific defect correction must retain exactly one active long task
  and must not add workers, durable tasks, or replay.
- Native E2E and platform-smoke harnesses must close the host and observe
  sidecar cleanup where supported.
- Normal and forced host close must leave zero sidecar descendants on every
  target where process evidence is claimed.
- Missing process inspection capability remains unverified or blocked rather
  than inferred from test-process exit.

### Accessibility

- Native journey controls and evidence UI must remain keyboard operable with
  visible focus and accessible names.
- Backend and terminal-task state changes use controlled announcements; do not
  announce every progress tick.
- Verify proportional forced-colors/high-contrast, reduced-motion,
  reduced-transparency, 200% scaling/text zoom, RTL/text expansion, and focus
  behavior where the target environment supports them.
- Windows title-bar controls require names, keyboard reachability,
  active/inactive state, scaling, forced-colors, and fallback evidence.
- macOS traffic lights and Ubuntu native decorations retain native behavior;
  do not replace them for pixel equivalence.
- Cross-engine visual equivalence is required, not identical pixels.
- Unavailable screen-reader or manual native checks remain unverified or
  blocked and are named in the final report.

### Platform and packaging

- Exact matrix: Windows 11 x64/WebView2/NSIS candidate; macOS
  arm64/WKWebView/unsigned or test-signed `.app` and DMG-layout candidate;
  Ubuntu 24.04 x64/WebKitGTK/`.deb`.
- Build the PyInstaller `onedir` sidecar on each target OS/architecture.
- Native application and bundled sidecar architectures must match.
- Keep source build, package creation, packaged launch, bundled-sidecar launch,
  native E2E, installed/clean smoke, manual UX, and
  signing/notarization/update evidence separate.
- No external Python is allowed for a claimed packaged runtime.
- Windows custom title bar is only a candidate. Promote it only after native
  behavior passes; native decorations are the accepted successful fallback.
- macOS retains native traffic lights with overlay/transparent treatment only
  when movement, full screen, focus, theme background, and safe area are
  correct.
- Ubuntu uses native decorations; custom Linux chrome is prohibited.
- Production macOS signing/notarization, public distribution, publishing,
  stores, and updater enablement remain deferred and cannot be WP03 pass claims.

### Data ownership and privacy

- WP03 introduces no product database, durable task store, settings store,
  file import, network state, or new durable-data writer.
- In-memory task and lifecycle ownership remains as accepted in WP02.
- Raw measurement files and evidence contain only allowlisted operational
  fields and sanitized target/tool context.
- Evidence retention must not include credentials, signing material, secrets,
  user document contents, raw protocol payloads, environment dumps, or
  unrestricted full paths.
- Final reports distinguish raw observation, inference, deviation, unverified,
  and blocked evidence.
- Source snapshots contain repository source and sanitized evidence only.

## 11. Proportional tests and exact evidence

Activation must verify every command against current scripts, accepted WP02
outputs, runner images, target package tools, and pinned versions. It must
replace stale commands before changing the lifecycle state to `Activated`.

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

### Exact focused commands required at activation

This provisional revision does not authorize execution. Before activation,
Chat Session must inspect the accepted repository and supply exact runnable
commands, target working directories, environment controls, and expected
artifacts for:

- native WebdriverIO configuration validation and one real Tauri journey;
- target-specific sidecar builds;
- Windows x64 NSIS build, package inspection, launch, and smoke checks;
- macOS arm64 `.app` and DMG-layout build, package inspection, launch, and
  smoke checks;
- Ubuntu 24.04 x64 `.deb` build, package inspection, install/remove, launch,
  and smoke checks;
- production WebDriver/testing-capability exclusion;
- release-CSP/Fluent/Griffel rendering on WebView2, WKWebView, and WebKitGTK;
- title-bar/native-window checks and fallback evidence;
- cross-engine visual and proportional accessibility smoke;
- installed-artifact or clean-runner smoke where feasible;
- measurement collection, sample counts, normalization, and retained raw data;
- accepted WP01 and WP02 regression checks, including process cleanup;
- final spike-report schema/content validation;
- source-snapshot CRC/path/root validation, fresh empty extraction, and full
  activated rerun.

Until these exact commands are supplied in an activated revision, this prompt
remains non-executable.

### Required evidence layers

For every check, record command or method, expected behavior, actual result,
OS/version/architecture/engine/tool identity, duration where relevant,
retained artifact or log, and blocker.

Keep these evidence classes separate:

1. source, schema, unit, and integration evidence;
2. source build evidence;
3. package-creation evidence;
4. packaged-application and bundled-sidecar runtime evidence;
5. native automated WebdriverIO evidence;
6. installed-artifact or clean-environment smoke evidence;
7. manual native-window/accessibility evidence;
8. production signing/notarization/update evidence;
9. unavailable evidence.

Browser-only mocked IPC, renderer-only journeys, source builds, package
creation, screenshots, or an unexercised artifact cannot satisfy a native
runtime, interaction, installation, signing, or manual gate.

### Required native WebdriverIO evidence

- Use WebdriverIO with `@wdio/tauri-service` or the activation-verified accepted
  equivalent.
- Drive the real Tauri application.
- Exercise visible shell, backend readiness, exact Unicode echo, one focused
  accepted WP02 lifecycle path, one safe error/recovery state, and orderly
  close/cleanup where supported.
- Retain target identity, configuration, exact command, report/log, and actual
  result.
- Prove the embedded WebDriver/testing capability is absent from production
  artifacts.

### Required title-bar and native-window evidence

Windows custom-title-bar promotion must address, where actual test ownership
exists:

- drag and non-drag regions;
- minimize, maximize/restore, and close;
- double-click maximize/restore;
- system menu and keyboard behavior;
- Snap Layouts, `Win+Z`, and maximize-button hit testing;
- resize/restore and mixed-DPI movement;
- active/inactive state;
- accessibility, scaling, forced colors, and RTL/text expansion;
- native fallback when any critical gate fails.

macOS evidence must address native traffic lights, movement, full screen,
focus, safe area, and theme background. Ubuntu evidence must classify native
decorations under actually exercised Wayland and X11 sessions. Missing
manual/native evidence remains unverified or blocked.

### Required measurements and retained evidence

Activation must define exact methods, tools, sampling, environment, raw output,
and classification for:

- themed window visible time;
- shell interactive time;
- cold packaged-sidecar handshake time;
- Unicode echo round-trip;
- UI-observed progress event rate;
- cancellation acknowledgement;
- cooperative stop time;
- crash detection time;
- bounded restart result and time;
- circuit behavior;
- orphan/descendant count after normal and forced close;
- package size per target;
- app idle memory;
- sidecar idle and active memory;
- release-CSP rendering result;
- native WebdriverIO result;
- per-platform build/package/launch result;
- title-bar promotion/fallback and known deviation.

Architecture budgets are initial targets, not fabricated service-level
results. Preserve raw values and classify each result as pass, deviation,
unverified, or blocked.

### Required CI evidence

Activation must supply exact workflow paths/names, runner labels and
architectures, triggers, permissions, commands, package tools,
artifact-retention behavior, and required logs/artifacts for the declared
matrix. Retain final run IDs, job IDs, commit SHAs, conclusions, logs, artifact
names, hashes, sizes, and unavailable evidence.

Target jobs may run in parallel within WP03, but their evidence must converge
before Phase 0B closure. No target may pass solely because another target ran.

## 12. Measurable acceptance gates

The package passes only when retained evidence proves:

1. Exact accepted and merged WP01 and WP02 predecessors were used.
2. The declared target/package matrix was refreshed and no unsupported target,
   architecture, distribution, store, or package format was added.
3. Windows x64, macOS arm64, and Ubuntu 24.04 x64 build/package results are
   recorded separately and truthfully.
4. Packaged applications launch with the bundled `onedir` sidecar and no
   external Python on every target where packaged runtime evidence is claimed.
5. One real native WebdriverIO Tauri journey passes on every declared automated
   target supported by the activated environment; unsupported/unavailable
   targets are not falsely passed.
6. Production artifacts exclude the embedded WebDriver/testing capability.
7. Release CSP permits Fluent/Griffel rendering without broad unsafe weakening
   or dangerous remote-domain IPC.
8. WebView2, WKWebView, and WebKitGTK shell evidence is retained with
   engine-specific deviations and truthful unavailable classifications.
9. Windows custom title bar passes all promotion gates or the
   native-decoration fallback is active and reported as the accepted result.
10. macOS native traffic-light and Linux native-decoration strategies remain
    reliable or are truthfully blocked/unverified where evidence is unavailable.
11. Accepted WP01 Unicode/backend/CSP/safe-error behavior and accepted WP02
    lifecycle/cancel/crash/restart/no-replay/zero-descendant behavior show no
    relevant regression.
12. Required startup, handshake, task, crash/restart, circuit, memory,
    package-size, process, CSP, E2E, platform, and title-bar measurements are
    retained with raw values, methods, environment, and truthful classification.
13. Build, package, packaged runtime, native UX, installed/clean smoke, manual,
    signing/notarization, and update evidence are never conflated.
14. The final Phase 0B spike report maps every requirement to retained WP01,
    WP02, or WP03 evidence, deviations, blockers, manual gaps, fallbacks, and
    any narrowly proposed architecture amendment.
15. The source snapshot represents the final commit, has valid SHA-256 and CRC,
    contains one safe expected root, extracts into a fresh empty directory, and
    passes the activated required rerun.
16. Phase 1 and every later implementation remain `Not started`, with no
    unauthorized product, release, platform, architecture, PR-merge, or
    publication work.

Missing evidence may make the final Phase 0B result
`READY WITH ASSUMPTIONS` or `BLOCKED` according to authority, but it may never
be silently converted into a passed evidence layer.

## 13. Stop conditions and blocker reporting

Stop without improvising when:

- the repository, default branch, activated base SHA, predecessor state, or
  required fresh branch differs from activation;
- WP02 is not accepted and merged;
- a required authority, accepted predecessor artifact, hash, CI run/job,
  repository path, measurement, deviation, or blocker is missing, stale, or
  materially conflicting;
- accepted WP01 or WP02 behavior regresses before WP03 changes;
- a frozen trust, protocol, lifecycle, platform, package, accessibility,
  data-ownership, or architecture boundary would need to change;
- the package requires an unapproved path, dependency, package format,
  architecture, distribution, store, credential, worker, durable state,
  product feature, or adjacent package;
- exact current repository paths or commands cannot be determined safely;
- the real packaged application, bundled sidecar, native automation, or
  required target evidence cannot be executed;
- production artifacts cannot exclude the WebDriver/testing capability;
- release CSP requires broad unsafe weakening;
- Windows custom chrome fails a promotion gate and native fallback is
  unavailable;
- zero descendants cannot be proven on a target where process evidence is
  claimed;
- the only support for a claimed gate is mock-only, browser-only, build-only,
  package-only, screenshot-only, unavailable, fabricated, or
  successor-dependent evidence;
- a workflow is blocked by permissions, policy, quota, billing, runner,
  architecture, package tool, credential, or artifact-retention limits;
- unavailable manual/hardware evidence would have to be silently passed;
- completing the work would require Phase 1, broad WP01/WP02 redesign,
  production release work, unsupported platform claims, or an unapproved
  architecture amendment;
- source-snapshot integrity or fresh-extraction rerun fails;
- the final diff includes an unauthorized path or unrelated existing changes
  cannot be preserved safely.

Return `Blocked` or `Partially implemented` using the approved labels. Name the
exact failed gate, observed evidence, unaffected scope, files changed,
unchanged exclusions, and smallest safe next action. Do not enter later work
as a workaround.

## 14. Required functional and status inventory

Before activation, the truthful inventory is:

```text
Stage 1 foundation: Implemented and accepted
GFD-P0B-WP02 provisional prompt: Implemented and accepted as Approved provisional
GFD-P0B-WP02 activation: Not started
GFD-P0B-WP02 application implementation: Not started
GFD-P0B-WP03 package outcome: Not started
Real cross-platform evidence/closure functionality: Not started
Supporting infrastructure: Not started
Tests: Not started
Documentation for implementation: Not started
Generated code: Not started
Fixtures/mocks: Not started
Stubs/placeholders: Not started
Incomplete work: Not started
Blocked work: Blocked — WP02 is not accepted and merged and WP03 has no activated prompt
GFD-P1-WP01: Not started
```

At implementation completion, report every applicable line with exactly one
of: `Implemented`, `Partially implemented`, `Stub`, `Mock-only`,
`Not started`, or `Blocked`. Add evidence references and separately report:

- accepted WP01 and WP02 regression status;
- Windows build/package/runtime/native-E2E/native-UX/manual evidence;
- macOS build/package/runtime/native-E2E/native-UX/manual evidence;
- Ubuntu build/package/runtime/native-E2E/Wayland/X11/manual evidence;
- bundled-sidecar/no-external-Python evidence;
- production test-driver exclusion;
- release-CSP/Fluent/Griffel and cross-engine evidence;
- title-bar candidate/fallback result;
- accessibility evidence;
- installed/clean-environment smoke;
- measurements and raw data;
- final Phase 0B spike report;
- source snapshot and fresh-extraction rerun;
- CI evidence;
- signing/notarization/update evidence as `Not started`, `Blocked`, or the
  narrowly truthful non-production feasibility status authorized at
  activation;
- Phase 1 and later work as `Not started`.

Documentation or tests may be `Implemented` while a platform runtime or manual
gate is `Partially implemented` or `Blocked`; never conflate them.

## 15. One `RUN_ID` and collision-resistant external naming

At the start of a future activated implementation, generate exactly one UTC
identifier:

```text
RUN_ID=YYYYMMDDTHHMMSSZ
```

Use that value unchanged for every external artifact and retained handoff name.
Do not reuse a prior run ID or generate separate IDs for target jobs, reruns,
corrections, snapshots, measurements, or the manifest within the same
execution.

Repository filenames remain conventional and do not receive timestamps unless
their existing repository contract requires versioning.

## 16. Required deliverables, hashes, and evidence index

The future activated implementation must produce exactly these external
deliverables:

```text
prime-shell-work-gfd-p0b-wp03-<RUN_ID>-source-snapshot-r1.zip
prime-shell-work-gfd-p0b-wp03-<RUN_ID>-spike-closure-report-r1.md
prime-shell-work-gfd-p0b-wp03-<RUN_ID>-review-evidence-index-r1.md
prime-shell-work-gfd-p0b-wp03-<RUN_ID>-handoff-manifest-r1.md
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

The spike-closure report must contain:

- every Phase 0B acceptance item mapped to WP01, WP02, or WP03 evidence;
- separate per-target build, package, packaged-runtime, bundled-sidecar,
  native-E2E, installed/clean smoke, manual/native UX, and
  signing/notarization/update classifications;
- raw measurements, methods, environment, budgets, and deviations;
- title-bar promotion/fallback decisions;
- release-CSP/Fluent/Griffel and cross-engine/accessibility results;
- unresolved blockers and manual evidence gaps;
- only evidence-backed targeted `v0.2.1` amendment proposals;
- final `READY`, `READY WITH ASSUMPTIONS`, or `BLOCKED` classification for
  Phase 1 review without authorizing Phase 1.

The review evidence index must map every package requirement and acceptance
gate to a repository file/section, command/result, retained artifact, and
unresolved item without duplicating the closure report.

The handoff manifest must record:

- activation ID, prompt ID/version, model/reasoning, repository, branch, exact
  base and final commit;
- accepted WP01/WP02 refs, artifacts, hashes, deviations, amendments, CI and
  native/manual evidence;
- target matrix, runner images, tool/package versions, manual owners, and
  credential limitations;
- changed files and real execution paths;
- exact commands and actual results;
- source, build, package, packaged runtime, native E2E, installed/clean,
  manual, signing/notarization/update, and unavailable evidence separately;
- raw measurement artifacts and normalized results;
- SHA-256 of the source snapshot, prompt-required reports, and required
  repository artifacts;
- a reproducible canonical self-hash convention if the manifest contains its
  own digest;
- truthful status inventory, limitations, files intentionally not created,
  blockers, final Phase 0B classification, and exact next controlled action.

Expose the four artifacts individually to Chat Session. Do not create another
package prompt or present an incremental overlay as authoritative source.

## 17. Branch, draft-PR, review, merge, and deletion controls

This provisional prompt authorizes no implementation branch or pull request.

When Chat Session activates it:

1. Use only the exact fresh implementation branch named in the activation
   record, created from the exact activated `main` SHA.
2. Do not reuse the documentation branch or a WP01/WP02 implementation branch.
3. Keep one coherent WP03 evidence/closure change and preserve unrelated user
   work.
4. Commit intentionally with a package-scoped message and push only the
   activated branch.
5. Open or update one draft PR only if the activated prompt explicitly
   authorizes it and supplies exact base/head controls.
6. Keep any authorized PR draft until Chat Session accepts the exact head.
7. Do not enable or use auto-merge.
8. Do not merge, publish, release, change repository settings, or delete a
   branch without separate explicit user authorization after Chat Session
   acceptance.
9. After an explicitly authorized merge, verify `main` contains the accepted
   result before deleting only the exact merged implementation branch.
10. Prompt acceptance is not activation; implementation acceptance is not
    merge authorization; WP03 completion is not Phase 1 authorization.

Report exact local/remote branch, commit, PR, draft, auto-merge, merge, and
deletion state in the handoff.

## 18. Completion response and return prompt

Keep progress and completion messages concise. Lead with the truthful package
result and include:

- model and reasoning level;
- repository, base, branch, final commit, clean-tree, PR, and merge state;
- real evidence/closure functionality and responsible files;
- exact commands and actual results;
- per-target build, package, packaged runtime, bundled sidecar, native E2E,
  installed/clean, native UX/manual, and unavailable evidence;
- release-CSP/cross-engine/accessibility and title-bar/fallback results;
- measurements, deviations, blockers, and final Phase 0B classification;
- full status inventory;
- four external artifact links and hashes;
- confirmation that Phase 1 and later work were not started.

At completion, define:

```text
FINAL_SHA=the exact 40-character final implementation commit
```

Then end with this short prompt, substituting the defined `FINAL_SHA` and the
section 15 `RUN_ID` with their exact values:

```text
Chat Session: Review GFD-P0B-WP03 on the activated implementation branch at FINAL_SHA.
Read the four prime-shell-work-gfd-p0b-wp03-RUN_ID deliverables. Verify truthful
per-target build/package/runtime/native/manual classification, native
WebdriverIO, release CSP, title-bar promotion/fallback, retained measurements,
zero-descendant regressions, final spike closure, and snapshot rerun, then
return Accepted, Focused correction required, or Blocked. Phase 1 was not
started, and this handoff does not authorize merge or later implementation.
```

Do not include authorization for Phase 1 or any later package. Do not continue
after returning the handoff.
