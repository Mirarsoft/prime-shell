# GFD-P2-WP01 — Theme, Tokens, and Accessibility Foundation

## 1. Package identity and prompt status

**Package ID:** `GFD-P2-WP01`
**Phase:** `Phase 2`
**Title:** `Theme, Tokens, and Accessibility Foundation`
**Task ID:** `GFD-P2-WP01`
**Prompt ID:** `PRIME-SHELL-GFD-P2-WP01-PROMPT`
**Prompt version:** `R1`
**Prompt lifecycle state:** `Provisional`
**Implementation status at authoring:** `Not started`
**Recommended implementation model:** `GPT-5.6 Sol`
**Reasoning/intelligence:** `High`
**Authorization boundary:** Exactly one package, `GFD-P2-WP01`
**Execution status:** `Execution is not authorized`

High is the minimum suitable reasoning level because this package crosses
Fluent theme architecture, semantic-token ownership, accent and contrast
generation, forced-colors behavior, theme-before-paint native coordination,
platform material fallback, release-CSP/Griffel behavior, cross-engine
rendering, and proportional accessibility evidence. The scope and ownership
are bounded, so Extra High is not required unless activation discovers a new
security, lifecycle, or cross-platform architecture risk.

This provisional prompt is complete for review but is non-executable. Chat
Session must first accept it as an approved provisional prompt and later issue
a separately activated revision after Phase 1 is accepted and merged.

### Activation metadata

```text
Activation ID: Not activated — Chat Session must refresh and supply this exact value.
Activated by: Not activated — Chat Session must refresh and supply this exact value.
Activation UTC: Not activated — Chat Session must refresh and supply this exact value.
Authoritative main SHA: Not activated — Chat Session must refresh and supply this exact value.
Required fresh implementation branch: Not activated — Chat Session must refresh and supply this exact value.
Accepted WP01, WP02, WP03, and P1-WP01 heads/evidence: Not activated — Chat Session must supply exact merged commits, reports, artifacts, hashes, CI runs/jobs, runtime/manual evidence, deviations, and blockers.
Final accepted Phase 0B closure report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Final accepted Phase 1 baseline report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Accepted targeted amendments: Not activated — Chat Session must refresh and supply the exact list or None.
Predecessor deviations incorporated: Not activated — Chat Session must refresh and supply the exact list or None.
Current Fluent, React, Griffel, Tauri, engine, capability, CSP, tool, and repository facts: Not activated — Chat Session must verify and supply them.
Current support matrix and native/manual evidence limitations: Not activated — Chat Session must refresh and supply the exact list or None.
Unresolved blockers/assumptions: Not activated — Chat Session must refresh and supply the exact list or None.
Authorization boundary: Not activated — the future activation may authorize only GFD-P2-WP01.
Authorization expires/invalidates when: Not activated — the future activation must invalidate on any base-SHA, accepted-predecessor, targeted-amendment, material tool/engine/platform/capability/CSP, repository-layout, or authorization-boundary change.
```

## 2. Repository and exact starting state

**Repository:** `Mirarsoft/prime-shell`
**Authoritative base branch:** `main`
**Required starting commit:** Not activated — Chat Session must refresh and
supply the exact current accepted and merged `main` SHA after WP02, WP03, and
P1-WP01 acceptance and merge.
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
- approved-provisional P1-WP01 prompt documentation head:
  `511f80f60f2bf367a725d96742deba6cb79f6551`;
- prompt-pack documentation branch:
  `docs/gfd-work-package-prompt-pack-v1`;
- WP02, WP03, and Phase 1 activation and implementation: `Not started`;
- Phase 2 implementation: `Not started`.

The documentation refs do not authorize implementation. Phase 2 activation
and implementation must start from the latest accepted and merged `main`, only
after WP02, WP03, and P1-WP01 are accepted and merged and their required
closure/baseline reports are accepted.

Required activation and Work Session preflight:

1. Verify exact access and write permission for `Mirarsoft/prime-shell`.
2. Verify the default branch is `main` and record its exact 40-character SHA.
3. Verify WP01, WP02, WP03, and P1-WP01 are accepted and merged at that SHA.
4. Verify the final accepted Phase 0B closure and Phase 1 baseline reports,
   source snapshots, support matrix, platform evidence, measurements,
   deviations, blockers, and targeted amendment decisions.
5. Verify current Fluent, React, Griffel, Tauri, CSP, capability, engine,
   toolchain, dependency, lockfile, test, CI, and repository-path facts.
6. Verify no Phase 2 or later implementation already exists unexpectedly.
7. Verify the exact fresh implementation branch does not already exist.
8. Use a clean fresh clone/worktree at the activated starting commit.
9. Verify and narrow every allowed and protected repository path against the
   current tree.
10. Stop if repository state differs from activation, is ambiguous, would
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
14. every separately accepted targeted architecture amendment named by
    activation
15. current Fluent providers, theme/bootstrap code, native-window background
    and material adapters, CSP/Griffel configuration, Tauri capabilities and
    permissions, accessibility tests, package layout, toolchain/version files,
    dependency manifests and lockfiles, workflows, and accepted support claims
16. the exact activated revision of this execution prompt

In the first implementation progress message, state:

> `AGENTS.md, repository authority, accepted Phase 0A, accepted and merged
> WP01/WP02/WP03 and P1-WP01 evidence, the final accepted Phase 0B closure
> report, the final accepted Phase 1 baseline report, and the activated
> GFD-P2-WP01 prompt have been read and are active. Executing only
> GFD-P2-WP01 with GPT-5.6 Sol / High. GFD-P2-WP02 and later work remain
> unauthorized.`

If a required source is unavailable, unreadable, stale, or materially
inconsistent, stop before writing. Do not conduct a broad architecture review,
rerun predecessor packages as substitutes for accepted evidence, or improvise
new support claims.

## 4. Dependencies, predecessor outputs, and prerequisite evidence

**Direct package prerequisite:** Accepted and merged `GFD-P1-WP01` baseline.

**Direct dependent:** `GFD-P2-WP02`.

**Inherited predecessors:** Accepted and merged WP01/FIX01,
`GFD-P0B-WP02`, and `GFD-P0B-WP03`.

**Authoring-time accepted WP01 commits:**

- accepted WP01 implementation head before squash merge:
  `bd502ffcd4c2d80bd3cd8817ebea4dfb18a76107`;
- accepted merged WP01 `main`:
  `35d53bffec6e5b05aefdf8c7fed39a9bfdf288a2`.

**WP02, WP03, and P1-WP01 prerequisite state at authoring:** Their prompts are
accepted as Approved provisional, but activation and implementation are
`Not started`. P2-WP01 cannot be activated while that remains true.

**Required predecessor outputs:**

- accepted and merged WP01, WP02, WP03, and P1-WP01 source;
- exact final merged predecessor commit and authoritative source snapshots;
- final accepted Phase 0B spike-closure report and Phase 1 baseline report;
- accepted platform title-bar/fallback, release-CSP, cross-engine,
  accessibility, support-matrix, toolchain, dependency, capability,
  permission, production-test-exclusion, and CI facts;
- exact Fluent, React, Griffel, Tauri, engine, platform-material, theme,
  bootstrap, native-background, and current repository-path facts;
- accepted architecture and ADR amendments, deviations, fallbacks, unresolved
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
- accepted exact limitations and blockers that Phase 2 must preserve rather
  than silently upgrade.

**Accepted deviations:** None are inferred. Activation must list every
accepted predecessor deviation or state `None`.

**Known blockers carried forward:** Implementation remains `Blocked` until
WP02, WP03, and P1-WP01 are accepted and merged; the final Phase 0B closure and
Phase 1 baseline reports are accepted; and Chat Session supplies an exact
activated P2-WP01 prompt.

Do not activate P2-WP01 while P1-WP01 is only an approved provisional prompt,
an unmerged implementation branch, an unaccepted baseline report, or evidence
with an unresolved architecture-blocking failure.

## 5. Objective and measurable runnable outcome

**Objective:** Establish the real Fluent theme, semantic-token, accent,
material-fallback, and focused accessibility foundation before composing the
full application shell.

**Required runnable outcome:**

```text
accepted and merged Phase 1 baseline
→ pinned Fluent/theme/platform capability facts
→ theme resolved before first visible frame
→ Fluent provider plus semantic/accent/status/focus token authority
→ system/light/dark/forced-colors and motion/transparency behavior
→ deterministic material capability fallback
→ release-CSP-safe portals and Griffel styling
→ focused accessible component and cross-engine evidence
```

A future activated implementation must provide one small real native Tauri
path that visibly demonstrates correct system, light, dark, and forced-colors
theme behavior; semantic, accent, status, and focus roles; compact and
comfortable density; deterministic material fallback; reduced motion and
transparency; themed portals; and accessible Fluent controls. Theme and native
background must be correct before the first visible frame, and supported
theme/capability changes must propagate without restart.

The package establishes foundations consumed immediately by the desktop
application. It does not compose the full multi-pane shell, add product
feature UI, or create a public UI library.

## 6. Explicit in-scope work

The package authorizes only:

1. Verify and record accepted Fluent, React, Griffel, Tauri, engine, CSP,
   capability, platform-material, theme/bootstrap, tool, support-matrix, and
   accessibility facts at activation.
2. Define typed theme lifecycle, user-selection, semantic-token, accent,
   status, focus, density, material-capability, forced-colors, reduced-motion,
   and reduced-transparency contracts owned by the accepted layers.
3. Add `packages/design-tokens/` only when the real desktop application and
   required tests consume it immediately in this package.
4. Implement pinned light/dark Fluent base themes plus application semantic
   roles, complete accent/status/focus roles, deterministic snapshots,
   provenance, contrast matrices, and raw-color policy checks.
5. Implement system, light, dark, platform forced-colors/high-contrast,
   reduced-motion, reduced-transparency, compact/comfortable density, default
   accent, accepted Windows system accent, and validated custom accent seed
   behavior.
6. Implement the smallest accepted native capability boundary for effective
   theme, system accent, material support, reduced transparency, and matching
   native window background.
7. Implement theme-before-paint through native theme resolution or a safe
   fallback, a matching native window background, and a tiny local HTML
   bootstrap before React/Fluent hydration.
8. Integrate one Fluent provider path with release-CSP-safe Griffel behavior
   and active-theme propagation to dialogs, menus, popovers, tooltips, and
   other portals.
9. Implement deterministic solid semantic fallback for unsupported materials,
   accessibility modes, remote sessions, policy/battery constraints, or
   capability failure.
10. Add only the smallest focused Fluent component surface needed to exercise
    theme, accent, status, focus, density, portals, material fallback, and
    accessibility behavior, such as focused buttons, links, fields,
    menu/dialog/tooltip portals, and status messaging.
11. Use only the minimum accepted preference mechanism needed to demonstrate
    theme mode, accent mode, density, material preference, or motion
    preference; do not create comprehensive settings ownership.
12. Add proportional token, provider/component, native startup, CSP/portal,
    accessibility, visual, cross-engine, and real native Tauri evidence for
    the declared support matrix.
13. Preserve accepted WP01/WP02/WP03/P1 behavior and prove no relevant
    regression or unsupported claim.
14. Produce the required authoritative source snapshot and three Markdown
    handoff artifacts after implementation validation.

This section authorizes one theme/accessibility-foundation package only. It
grants no P2-WP02, product, publication, merge, release, signing, updater, or
repository-setting authority.

## 7. Explicit exclusions and prohibited adjacent work

Do not:

- activate or implement WP02, WP03, or P1-WP01 as part of this package;
- begin `GFD-P2-WP02` shell composition, app rail, context sidebar, workspace,
  inspector, bottom panel, responsive shell states, splitters, route
  architecture, settings shell, broad layout state, or layout persistence;
- implement product feature UI, document analysis, file import, product
  routes, account, database, or runtime network service;
- create public `packages/ui`, component SDK, module SDK, plugins, template
  extraction, generic wrappers, or a public design system;
- add broad Storybook, design-site, visual-fixture, or component-gallery work;
- create custom Linux chrome or relitigate accepted title-bar decisions and
  native fallbacks;
- add comprehensive settings storage, migrations, a settings window, generic
  preference framework, database ownership, cloud sync, diagnostics, repair,
  or recovery;
- persist resolved system accent, effective theme, material capability,
  forced-colors state, or reduced-transparency state as user choices;
- expand backend, task runtime, protocol, sidecar, lifecycle, operation,
  worker, durability, cancellation, restart, or process-containment behavior;
- introduce multiple workers, durable tasks, automatic replay, GPU/model
  orchestration, runtime plugins, telemetry, crash upload, updater, or
  enterprise deployment;
- add fake Mica/Acrylic materials, screenshot blur, continuous custom blur, or
  unsupported translucency;
- redistribute proprietary fonts without verified licensing;
- proliferate arbitrary raw colors or use one ambiguous accent-foreground
  role;
- weaken production CSP with broad `unsafe-inline`, enable remote content or
  dangerous IPC, or ship duplicate Griffel runtimes/test capabilities;
- add an unsupported platform, architecture, Linux distribution, package
  format, engine, signing, notarization, publication, or release claim;
- treat unit, story, browser-mock, screenshot, build, package, or unavailable
  evidence as real native startup, runtime, manual, or cross-engine proof;
- broadly refactor predecessor code or silently change frozen architecture,
  trust, capability, title-bar, support, accessibility, data-ownership, or
  release boundaries;
- implement a predecessor correction without first stopping for Chat Session
  review, unless activation names one exact accepted defect correction;
- create, open, merge, publish, or delete PRs/branches without the exact
  authorization required in section 17;
- begin Phase 3 or any dependent/later package.

If full shell composition, broad settings, a public package, a material
pipeline, a new platform claim, an architecture amendment, or a predecessor
correction appears necessary, stop with evidence and request a focused
correction or targeted amendment.

Stories or visual fixtures may be added only when they directly prove complex
reusable theme behavior and are consumed by required tests. Browser-only
stories cannot replace real native Tauri evidence.

## 8. Allowed repository areas and expected changes

Activation must verify current paths, substitute accepted Phase 1 locations,
and remove unnecessary entries. Subject to that refresh, only these
responsibility areas may change:

```text
packages/design-tokens/                     # only when immediately consumed by app and tests
apps/desktop/src/theme/                     # or the activation-verified equivalent
apps/desktop/src/providers/                 # focused provider/bootstrap wiring only
apps/desktop/src/main.*                     # theme-before-React bootstrap only
apps/desktop/index.html                     # tiny matching launch background only
apps/desktop/src/App.*                      # smallest focused theme demonstration only
apps/desktop/src/*.css                      # semantic/forced-colors/bootstrap styles only
apps/desktop/src-tauri/src/                 # narrow theme/accent/material/native-background adapter only
apps/desktop/src-tauri/tauri.conf.json      # exact background/CSP capability baseline only
apps/desktop/src-tauri/capabilities/        # only a least-privilege theme capability when required
apps/desktop/src-tauri/permissions/         # only a least-privilege theme capability when required
apps/desktop/package.json                   # exact theme/test wiring only
package.json                                # exact script wiring only
pnpm-lock.yaml                              # mechanically resulting lock only
tests/                                      # focused theme/accessibility/native evidence only
scripts/verify/                             # focused token/CSP/theme checks only
.github/workflows/                          # minimal retained P2-WP01 evidence only when activated
```

Expected changes are typed theme/token contracts, immediately consumed
semantic/accent/material assets, provider/bootstrap/native-background wiring,
the smallest focused demonstration, and proportional token, CSP, portal,
accessibility, visual, native, and cross-engine verification.

The token package may own token contracts, light/dark semantics, accent and
material types, forced-colors support, snapshots, provenance, contrast
matrices, and raw-color policy checks. It must not own shell components,
product features, a public API, or an unused extraction.

The following must remain unchanged:

- full shell, route, settings-shell, broad persistence, product feature, and
  Phase 3+ areas;
- accepted predecessor contracts, lifecycle, sidecar, process, package, and
  evidence behavior except exact theme integration points authorized here;
- product database, file-intent, diagnostics, recovery, updater, signing,
  telemetry, SDK, UI-package, template, and release areas;
- `docs/authority/`, `docs/phase-0a/`, and prompt-pack files unless activation
  names one separately accepted targeted amendment and exact path;
- unrelated dependencies, workflows, scripts, tests, documentation, and
  repository settings.

A necessary path outside the activated allowed list is a stop condition unless
it is a mechanically generated or locked companion explicitly anticipated by
activation.

## 9. Ordered implementation procedure

1. Complete authority, repository, activation, clean-tree, and predecessor
   preflight at the exact activated `main` SHA.
2. Record current theme/bootstrap/CSP/native-background behavior, repository
   paths, pinned Fluent/React/Griffel/Tauri versions, material capabilities,
   support matrix, evidence limits, and the exact initial status inventory.
3. Run the accepted predecessor contract, frontend, Rust, Python, native,
   CSP/capability, package, and support-matrix baselines relevant to files this
   package may change. Stop on an unexplained predecessor regression.
4. Define the typed theme lifecycle, user selections, semantic/accent/status/
   focus/material contracts, allowed raw-color categories, and permitted
   foreground/background contrast matrix before styling components.
5. Create `packages/design-tokens/` only if the real desktop application and
   tests consume it immediately; otherwise keep the narrow contract in the
   activation-verified application-owned location.
6. Implement light/dark semantics, complete accent/status/focus roles,
   deterministic custom-accent generation, token snapshots, provenance,
   contrast checks, and raw-color policy enforcement.
7. Implement theme-before-paint, matching native and tiny HTML/bootstrap
   backgrounds, and no-flash provider hydration.
8. Add system theme, accepted system accent, native background, and material
   capability resolution through the smallest accepted native boundary.
9. Integrate one Fluent provider path with release-CSP-safe Griffel rendering
   and portal inheritance without duplicate runtime copies.
10. Implement forced-colors, reduced-motion, reduced-transparency,
    compact/comfortable density, and deterministic solid material fallbacks.
11. Add the smallest focused demonstration and proportional keyboard, focus,
    accessible-name, status, scaling, RTL/text-expansion, screen-reader,
    visual, portal, and native startup evidence.
12. Verify cross-engine behavior only where accepted environments actually
    support it; keep unavailable manual/engine evidence unverified or blocked.
13. Prove no P2-WP02 shell, product UI, broad persistence, public UI package,
    unsupported platform, or later-phase work entered the diff.
14. Run all exact activated checks from a clean tree and classify source,
    token, provider, native, CSP, accessibility, platform, CI, manual, and
    unavailable evidence separately.
15. Commit only the coherent P2-WP01 implementation, create a complete source
    snapshot of the final commit, verify CRC/path safety/one root/hash, extract
    it into a fresh empty directory, and rerun every command required by
    activation.
16. Verify the exact diff, status inventory, evidence classifications,
    deliverables, hashes, branch/PR state, and P2-WP02 exclusion; then return
    the handoff without entering P2-WP02.

Prefer direct Fluent controls and semantic tokens over wrapper proliferation.
Stop rather than widening the package to make an unsupported claim pass.

## 10. Cross-cutting constraints

### Contract

- Theme mode, effective theme, accent mode, density, motion preference,
  material preference, platform capability, forced-colors state, and reduced
  transparency must use explicit bounded types.
- Theme layers remain pinned Fluent `webLightTheme` or `webDarkTheme`,
  application semantic tokens, then the platform capability/material adapter.
- Fluent tokens remain authoritative inside Fluent controls; features consume
  Fluent or semantic tokens.
- The semantic contract must cover shell/window and stable surfaces;
  elevated/card, panel, workspace, divider, border, overlay, code/terminal
  roles needed by the demonstration; primary, secondary, tertiary, disabled,
  link, and on-accent foreground roles; hover, pressed, selected, disabled,
  and focus roles; danger, success, and warning foreground/subtle-background
  roles; and distinct accent background, hover, pressed, on-accent, link,
  subtle, focus-inner, and focus-outer roles.
- Do not use one ambiguous accent-foreground role.
- Status meaning includes text or icon, never color alone.
- Focus remains visible on neutral, accent, status, image, and translucent
  surfaces.
- Dark-theme elevation uses deliberate neutral surfaces rather than default
  pure black.
- Typography and dimensions must not depend on redistribution of proprietary
  fonts; use platform fonts or a verified licensed fallback.
- Fluent System Icons must follow a documented regular/filled selected-state
  convention.
- Token snapshots and provenance identify the pinned Fluent source,
  generated/custom values, reviewed exemptions, and intentional divergence.
- Stale snapshots, generated ramps, provenance, or contract representations
  fail deterministically.

### Security and trust

- React remains least trusted; Rust remains native policy, capability, window,
  theme/accent/material-resolution, and process authority; Python remains
  trusted first-party native code, not a sandbox.
- Do not expose generic shell, process, arbitrary filesystem, secret, native
  path, unrestricted operation, or generic platform-capability access to
  React.
- Theme/accent/material commands and permissions must be explicit, typed,
  bounded, least privilege, and limited to the authorized window.
- Rust or the accepted native authority resolves selected/effective theme or a
  safe fallback before showing the window and applies a matching native
  background.
- Startup behavior is local-only, uses no remote content, and remains
  compatible with production CSP.
- Release CSP preserves accepted local assets, network denial, dangerous
  remote-domain IPC denial, and production test-driver exclusion.
- Fluent/Griffel style injection must use the accepted nonce/hash mechanism;
  no broad `unsafe-inline` or duplicate renderer copy is permitted.
- Source, logs, reports, CI, and snapshots must not contain credentials,
  signing material, tokens, user content, raw payloads, environment dumps, or
  unrestricted full paths.

### Lifecycle and concurrency

- Inherit the exact accepted WP02 lifecycle, task states, queue/frame/log
  limits, progress limit, cancellation/terminal precedence, timeout,
  crash/hang, restart, circuit, no-replay, and process-tree behavior.
- This package must not redefine or productize the task lifecycle, add an
  operation unrelated to theme capability, or introduce a worker.
- System theme, accepted accent, forced-colors, reduced-transparency, and
  material-capability changes update provider, portals, native background, and
  focused demonstration state without restart.
- Theme changes must have deterministic subscription ownership and cleanup;
  they must not duplicate listeners or produce stale portal/native state.
- Theme switching must not restart, replay, or interrupt accepted backend work.
- Any predecessor lifecycle regression is a blocker, not theme scope.

### Accessibility

- Target WCAG 2.2 AA for applicable webview content while preserving accepted
  native/manual classifications.
- The focused demonstration must support keyboard operation, logical focus
  order, visible and unobscured focus, accessible names for icon-only
  controls, and appropriate headings/landmarks.
- Controlled screen-reader state and label behavior must avoid noisy
  announcements.
- Platform forced-colors/high-contrast behavior uses system colors and
  accepted Fluent behavior, not a simulated fixed palette.
- Verify 200% text scaling/zoom, reduced motion, reduced transparency, status
  meaning without color alone, target size/non-pointer alternatives where
  relevant, RTL, and text expansion.
- Focus-inner and focus-outer roles must be contrast-verified separately on
  neutral, accent, status, image, and translucent surfaces.
- Materials and decorative effects yield to forced colors and reduced
  transparency.
- Do not broaden this focused foundation into the full shell accessibility
  program owned by P2-WP02.

### Platform and packaging

- Activation must use exactly the final accepted support matrix; authoring
  facts do not silently become support authority.
- Treat Mica, Mica Alt, Acrylic, translucency, and similar effects as
  capabilities, not guaranteed visuals.
- Mica is limited to accepted long-lived Windows shell/title surfaces;
  Acrylic is limited to accepted transient surfaces and never adjacent
  permanent panes.
- Unsupported platforms, accessibility modes, remote sessions, reduced
  transparency, policy/battery limits, or failures resolve to solid semantic
  fallback.
- macOS/Linux translucency is permitted only where accepted evidence shows
  accessible and performant behavior.
- Window material preference remains separate from transient-surface
  treatment.
- Do not implement fake material, screenshot, or continuous custom-blur
  pipelines.
- Keep WebView2, WKWebView, and WebKitGTK source build, native runtime,
  cross-engine, package, manual, and unavailable evidence separate.
- Cross-engine visual equivalence is required, not pixel identity.
- Build/package/signing/update claims remain inherited facts unless activation
  authorizes a narrow regression check; this package is not release work.

### Data ownership and privacy

- Persist only the minimum user selections that accepted Phase 1 ownership
  permits for the focused demonstration.
- Resolved system accent, effective theme, material capability, forced-colors
  state, and reduced-transparency state are runtime facts and are not blindly
  persisted.
- Do not add comprehensive settings schemas, migrations, a settings window,
  layout persistence, database ownership, cloud sync, diagnostics, or a
  generic settings framework.
- Existing single-writer durable-data ownership remains unchanged.
- Token snapshots and provenance contain design metadata only, not user data,
  host secrets, unrestricted paths, or environment dumps.
- Evidence contains only allowlisted operational, visual, tool, engine, and
  platform context.

## 11. Proportional tests and exact evidence

Activation must inspect the accepted Phase 1 repository, verify every command,
working directory, version authority, support claim, tool, and retained output,
and replace stale commands before changing the lifecycle state to `Activated`.

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
runnable commands, working directories, environment controls, tools, target
engines, retained outputs, and manual owners for:

- theme/semantic/accent/material contract and type checks;
- deterministic token snapshots, provenance, generated-ramp drift, and
  raw-color policy checks;
- explicit foreground/background contrast matrix and named exemption checks;
- invalid custom-accent rejection or deterministic adjustment;
- light, dark, system, forced-colors, reduced-motion, reduced-transparency,
  compact/comfortable density, and material-fallback behavior;
- theme-before-paint, matching native/bootstrap backgrounds, first-visible
  frame observation, and theme-switch stability measurements;
- Fluent provider, Griffel renderer, release CSP, and portal inheritance;
- production duplicate-renderer, remote-content, unsafe-CSP, dangerous-IPC,
  test-driver, and test-permission exclusion;
- focused component keyboard, focus, accessible name, status, screen-reader,
  200% scaling, RTL, text expansion, target size, and non-pointer behavior;
- real native Tauri theme demonstration on the declared primary environment;
- truthful WebView2, WKWebView, and WebKitGTK cross-engine evidence according
  to accepted environment availability;
- accepted WP01/WP02/WP03/P1 regression checks relevant to touched paths;
- source-snapshot CRC/path/root validation, fresh empty extraction, and
  complete activated rerun.

Until those exact commands and accepted predecessor inputs are supplied in an
activated revision, this prompt remains non-executable.

### Required evidence layers

For every check, record command or method, expected behavior, actual result,
OS/version/architecture/engine/tool identity, duration where relevant,
retained artifact or log, and blocker.

Keep these evidence classes separate:

1. token contract, type, snapshot, provenance, ramp, contrast, and policy
   evidence;
2. frontend provider, component, portal, visual, and accessibility evidence;
3. native theme-before-paint, background, material, capability, and runtime
   evidence;
4. release-CSP, Griffel-renderer, portal, and production-exclusion evidence;
5. source build and package-creation evidence;
6. real native Tauri and packaged-application runtime evidence;
7. WebView2, WKWebView, and WebKitGTK cross-engine/platform evidence;
8. CI workflow/run/job/artifact evidence;
9. manual native/accessibility evidence;
10. signing/notarization/update evidence;
11. documentation accuracy evidence;
12. unavailable or blocked evidence.

A unit test, browser story, screenshot, source build, package, documentation
statement, prior result, or unavailable environment cannot be relabeled as
current real native startup/runtime, cross-engine, screen-reader, native UX,
installation, signing, or manual evidence.

### Theme-before-paint and measurement evidence

Retain real native observation and timing/visual evidence for:

- process start to first correctly themed visible frame;
- native window and tiny HTML/bootstrap background match;
- absence of observable light-to-dark or dark-to-light launch flash;
- React/Fluent hydration and portal theme correctness;
- system/light/dark switch to stable paint without restart;
- native background and material state after theme/capability changes.

Architecture timing targets such as themed-window visibility and
theme-switch stability are initial measured goals, not fabricated service
levels. Record raw values, method, sample count, environment, target, and
truthful pass/deviation/unverified/blocked classification.

### Contrast and accessibility evidence

- Use an explicit permitted foreground/background matrix.
- Target WCAG 2.2 AA where applicable.
- Keep disabled-control and decorative-stroke exemptions named and narrow.
- Verify on-accent, link, subtle-accent, focus-inner, and focus-outer roles
  separately.
- Verify keyboard order, unobscured focus, names, status meaning, scaling,
  forced colors, reduced motion/transparency, RTL/text expansion, and
  proportionate screen-reader behavior on the focused demonstration.
- Keep manual/platform limitations explicit.

### Required CI evidence

Activation must supply exact workflow paths/names, runner labels and
architectures, triggers, least-privilege permissions, commands,
artifact-retention behavior, target engines, and required logs/artifacts.
Retain final run IDs, job IDs, commit SHAs, conclusions, artifact names,
hashes, sizes, and unavailable evidence.

CI must match, never exceed, the accepted support matrix. Target jobs may run
in parallel inside this package, but their evidence must converge before
acceptance. No target may pass solely because another engine or platform ran.

## 12. Measurable acceptance gates

The package passes only when retained evidence proves:

1. Exact accepted and merged WP01, WP02, WP03, and P1-WP01 predecessors plus
   the accepted Phase 0B closure and Phase 1 baseline reports, deviations,
   blockers, amendments, support facts, and repository paths were used.
2. Theme lifecycle state and persisted user selections are typed, bounded,
   minimally owned, and separated from resolved runtime capability facts.
3. System, light, and dark modes render correctly and switch without restart.
4. Forced-colors/high-contrast uses platform-compatible system behavior and
   does not rely on a simulated fixed palette.
5. Native and tiny HTML/bootstrap backgrounds match the effective theme before
   the first visible frame, with retained real native evidence showing no
   observable launch flash.
6. Semantic, accent, status, interaction, and focus roles are complete,
   explicit, unambiguous, and consumed by the real desktop application.
7. Token snapshots, generated ramps, and provenance are deterministic; stale
   or generated drift fails.
8. Every approved foreground/background contrast pair passes, with named
   exemptions narrow and visible.
9. Invalid custom accent seeds are rejected or adjusted deterministically,
   and on-accent/link/subtle/focus roles are verified separately.
10. Arbitrary raw colors fail outside the reviewed allowed categories.
11. Reduced motion, reduced transparency, and compact/comfortable density
    behave correctly without hiding state or meaning.
12. Material capability falls back to solid semantic colors under unsupported,
    accessibility, remote-session, policy/battery, or error conditions.
13. Dialogs, menus, popovers, tooltips, and other portals inherit active theme
    and semantic tokens across changes without restart.
14. Release CSP permits Fluent/Griffel without broad unsafe weakening,
    duplicate renderer copies, remote content, dangerous IPC, or production
    test capability.
15. Focused keyboard, focus, accessible-name, status, 200% scaling, RTL/text
    expansion, target-size/non-pointer, and proportionate screen-reader checks
    pass or are truthfully classified with exact manual limitations.
16. Real native Tauri evidence exists for the declared primary environment,
    while WebView2, WKWebView, WebKitGTK, native/manual, and unavailable
    evidence remain separate and truthful.
17. Accepted WP01/WP02/WP03/P1 behavior shows no relevant regression.
18. The final diff contains no P2-WP02 shell, product UI, broad persistence,
    public UI package, Phase 3, unsupported platform, PR-merge, release,
    publication, or unrelated work.
19. The final source snapshot represents the final commit, has valid SHA-256
    and CRC, contains one safe expected root, extracts into a fresh empty
    directory, and passes the complete activated rerun with a clean final tree.

Hardware-sensitive timing misses do not become silent passes. Retain raw values
and classify whether a miss invalidates the package, requires a focused
correction, or only adjusts a provisional budget.

## 13. Stop conditions and blocker reporting

Stop without improvising when:

- the repository, default branch, activated base SHA, predecessor state,
  accepted closure/baseline report, or required fresh branch differs from
  activation;
- P1-WP01 is not accepted and merged;
- a required authority, predecessor source, report, evidence index, manifest,
  snapshot, hash, CI run/job/artifact, measurement, deviation, fallback,
  blocker, targeted amendment, support fact, or repository path is missing,
  stale, unaccepted, or materially conflicting;
- accepted predecessor behavior or evidence regresses before P2-WP01 changes;
- a frozen trust, CSP, capability, platform, accessibility, data-ownership,
  title-bar, package, privacy, or release boundary would need to change;
- theme-before-paint or release-CSP/Griffel behavior cannot be proven through
  a real native path;
- the package requires the full shell, broad persistence/settings, a public UI
  package, product behavior, backend/lifecycle redesign, another package, or
  an unapproved path/dependency/platform/credential;
- `packages/design-tokens/` would be unused, speculative, public, or without
  immediate application and test consumers;
- a raw-color, custom-accent, material, forced-colors, contrast, portal, or
  theme-lifecycle rule cannot be made deterministic;
- exact current paths, versions, commands, engine capabilities, support
  claims, evidence classifications, or manual owners cannot be determined
  safely;
- required real native, cross-engine, accessibility, or manual evidence is
  unavailable and would have to be silently passed;
- the only support for a claimed gate is mock-only, story-only, browser-only,
  screenshot-only, source/build/package-only, documentation-only,
  unavailable, fabricated, or successor-dependent evidence;
- a workflow is blocked by permissions, policy, quota, billing, runner,
  architecture, engine, package tool, credential, or artifact-retention
  limits;
- completing the work would require P2-WP02, broad predecessor redesign,
  production release work, unsupported platform claims, or an unapproved
  architecture amendment;
- source-snapshot integrity or fresh-extraction rerun fails;
- the final diff includes an unauthorized path or unrelated existing changes
  cannot be preserved safely.

Return `Blocked` or `Partially implemented` using the approved labels. Name the
exact failed gate, observed evidence, unaffected scope, files changed,
unchanged exclusions, and smallest safe next action. Do not enter P2-WP02 or
another package as a workaround.

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
GFD-P2-WP01 package outcome: Not started
Theme/provider/bootstrap foundation: Not started
Semantic/accent/status/focus tokens: Not started
Token snapshots/provenance/drift checks: Not started
Forced-colors and contrast foundation: Not started
Material capability/fallback adapter: Not started
Reduced-motion/transparency and density: Not started
Release-CSP/Griffel/portal evidence: Not started
Focused accessibility/native evidence: Not started
Supporting infrastructure: Not started
Tests: Not started
Documentation for implementation: Not started
Generated code: Not started
Fixtures/mocks: Not started
Stubs/placeholders: Not started
Incomplete work: Not started
Blocked work: Blocked — P1-WP01 is not accepted and merged and P2-WP01 has no activated prompt
GFD-P2-WP02: Not started
```

At implementation completion, report every applicable line above with exactly
one of: `Implemented`, `Partially implemented`, `Stub`, `Mock-only`,
`Not started`, or `Blocked`. Add evidence references and separately report:

- accepted predecessor, Phase 0B closure, and Phase 1 baseline identities;
- typed theme lifecycle and selection ownership;
- provider/bootstrap/native-background path;
- semantic, accent, status, interaction, and focus contracts;
- token snapshots, provenance, ramp generation, contrast, and raw-color
  policy;
- forced colors, density, reduced motion/transparency, and material fallback;
- release CSP, Griffel, portal, duplicate-renderer, and production-exclusion
  evidence;
- focused component, keyboard, focus, screen-reader, scaling, RTL/text
  expansion, and native startup evidence;
- WebView2, WKWebView, WebKitGTK, CI, manual, and unavailable evidence
  separately;
- source snapshot and fresh-extraction rerun;
- incomplete or blocked work;
- P2-WP02 and later work as `Not started`.

Documentation or tests may be `Implemented` while a native, engine, or manual
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
prime-shell-work-gfd-p2-wp01-<RUN_ID>-source-snapshot-r1.zip
prime-shell-work-gfd-p2-wp01-<RUN_ID>-theme-foundation-report-r1.md
prime-shell-work-gfd-p2-wp01-<RUN_ID>-review-evidence-index-r1.md
prime-shell-work-gfd-p2-wp01-<RUN_ID>-handoff-manifest-r1.md
```

The section 15 run ID is the one runtime token used in these four names.

The source snapshot must:

- represent the exact final implementation commit;
- contain one expected repository root;
- exclude repository metadata, caches, build output, credentials, signing
  material, and unrelated local files;
- pass ZIP CRC and path-safety checks;
- have an ordinary SHA-256;
- extract into a fresh empty directory;
- pass the exact activated required rerun.

The theme-foundation report must:

- map accepted predecessor, support, deviation, fallback, blocker, and
  targeted-amendment facts to the implemented repository decisions;
- identify typed theme, selection, token, accent, status, focus, material,
  forced-colors, motion/transparency, and density ownership;
- identify provider/bootstrap/native-background, theme-before-paint, and
  no-flash evidence;
- list token snapshots, provenance, custom-ramp behavior, contrast matrix,
  exemptions, and raw-color policy;
- classify release-CSP/Griffel/portal, component/accessibility, real native,
  cross-engine, CI, manual, signing/update, and unavailable evidence
  separately;
- state every raw measurement, method, environment, sample count, deviation,
  limitation, and item deferred to P2-WP02 or later.

The review evidence index must map every package requirement and acceptance
gate to a repository file/section, command/result, retained artifact, and
unresolved item without duplicating the theme-foundation report.

The handoff manifest must record:

- activation ID, prompt ID/version, model/reasoning, repository, branch, exact
  base and final commit;
- accepted WP01/WP02/WP03/P1 refs, closure/baseline reports, source snapshots,
  artifacts, hashes, CI/native/manual evidence, measurements, deviations,
  fallbacks, amendments, limitations, and blockers;
- tools, runners, engines, target matrix, Fluent/React/Griffel/Tauri versions,
  changed files, and responsible execution paths;
- exact commands and actual results;
- token, provider/component, native startup, CSP/portal, build/package,
  runtime, cross-engine, CI, manual, signing/update, documentation, and
  unavailable evidence separately;
- SHA-256 of the source snapshot, prompt-required reports, and required
  repository artifacts;
- a reproducible canonical self-hash convention if the manifest contains its
  own digest;
- truthful functional/status inventory, limitations, files intentionally not
  created, blockers, P2-WP02 exclusion, and exact next controlled action.

Expose the four artifacts individually to Chat Session. Do not create another
package prompt or present an incremental overlay as the authoritative source.

## 17. Branch, draft-PR, review, merge, and deletion controls

This provisional prompt authorizes no implementation branch or pull request.

When Chat Session activates it:

1. Use only the exact fresh implementation branch named in the activation
   record, created from the exact activated `main` SHA.
2. Do not reuse the documentation branch or any predecessor implementation
   branch.
3. Keep one coherent P2-WP01 theme/accessibility change and preserve unrelated
   user work.
4. Commit intentionally with a package-scoped message and push only the
   activated branch.
5. Open or update one draft PR only if the activated prompt explicitly
   authorizes it and supplies exact base/head controls.
6. Keep any authorized PR draft until Chat Session accepts the exact head.
7. Do not enable or use auto-merge.
8. Do not merge, publish, release, sign, notarize, enable updates, change
   repository settings, or delete a branch without separate explicit user
   authorization after Chat Session acceptance.
9. After an explicitly authorized merge, verify `main` contains the accepted
   result before deleting only the exact merged implementation branch.
10. Prompt acceptance is not activation; implementation acceptance is not
    merge authorization; P2-WP01 completion is not P2-WP02 authorization.

Report exact local/remote branch, commit, PR, draft, auto-merge, merge, and
deletion state in the handoff.

## 18. Completion response and return prompt

Keep progress and completion messages concise. Lead with the truthful package
result and include:

- model and reasoning level;
- repository, base, branch, final commit, clean-tree, PR, and merge state;
- accepted predecessor, Phase 0B closure, Phase 1 baseline, and targeted
  amendment identities;
- theme/provider/bootstrap/native-background and theme-before-paint outcomes;
- semantic/accent/status/focus tokens, snapshots, provenance, contrast,
  material fallback, forced colors, motion/transparency, and density outcomes;
- release-CSP/Griffel/portal, component/accessibility, native, cross-engine,
  CI, manual/unavailable, signing/update, and documentation evidence
  separately;
- exact commands, raw measurements, actual results, deviations, limitations,
  and blockers;
- full status inventory;
- four external artifact links and hashes;
- confirmation that P2-WP02 and later work were not started.

At completion, define:

```text
FINAL_SHA=the exact 40-character final implementation commit
```

Then end with this short prompt, substituting the defined `FINAL_SHA` and the
section 15 run ID with their exact values:

```text
Chat Session: Review GFD-P2-WP01 on the activated implementation branch at FINAL_SHA.
Read the four prime-shell-work-gfd-p2-wp01-RUN_ID deliverables. Verify
theme-before-paint, semantic/accent/status/focus tokens, deterministic
snapshots/provenance/contrast, forced colors, material fallback,
reduced-motion/transparency, release-CSP-safe Griffel and portals, focused
accessibility/native/cross-engine evidence, and snapshot rerun, then return
Accepted, Focused correction required, or Blocked. GFD-P2-WP02 was not
started, and this handoff does not authorize merge or later implementation.
```

Do not include authorization for P2-WP02 or any later package. Do not continue
after returning the handoff.
