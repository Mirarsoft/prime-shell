# GFD-P2-WP02 — Responsive Application Shell

## 1. Package identity and prompt status

**Package ID:** `GFD-P2-WP02`
**Phase:** `Phase 2`
**Title:** `Responsive Application Shell`
**Task ID:** `GFD-P2-WP02`
**Prompt ID:** `PRIME-SHELL-GFD-P2-WP02-PROMPT`
**Prompt version:** `R1`
**Prompt lifecycle state:** `Provisional`
**Implementation status at authoring:** `Not started`
**Recommended implementation model:** `GPT-5.6 Sol`
**Reasoning/intelligence:** `High`
**Authorization boundary:** Exactly one package, `GFD-P2-WP02`
**Execution status:** `Execution is not authorized`

High is the minimum suitable reasoning level because this package crosses
responsive shell composition, accessible pane resizing, focus restoration,
platform title-bar behavior, narrow Rust-owned layout persistence,
release-CSP-safe Fluent/Griffel integration, and truthful native and
cross-engine evidence. The package is deliberately bounded to the reusable
shell, so Extra High is not required unless activation exposes a new native
window, persistence, security, or cross-platform architecture risk.

This provisional prompt is complete for review but is non-executable. Chat
Session must first accept it as an approved provisional prompt and later issue
a separately activated revision after `GFD-P2-WP01` is accepted and merged.

### Activation metadata

```text
Activation ID: Not activated — Chat Session must refresh and supply this exact value.
Activated by: Not activated — Chat Session must refresh and supply this exact value.
Activation UTC: Not activated — Chat Session must refresh and supply this exact value.
Authoritative main SHA: Not activated — Chat Session must refresh and supply this exact value.
Required fresh implementation branch: Not activated — Chat Session must refresh and supply this exact value.
Accepted WP01, WP02, WP03, P1-WP01, and P2-WP01 heads/evidence: Not activated — Chat Session must supply exact merged commits, reports, artifacts, hashes, CI runs/jobs, native/manual evidence, deviations, and blockers.
Final accepted Phase 0B closure report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Final accepted Phase 1 baseline report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Final accepted P2-WP01 theme-foundation report: Not activated — Chat Session must supply its exact filename, hash, classification, and accepted status.
Accepted targeted amendments: Not activated — Chat Session must refresh and supply the exact list or None.
Predecessor deviations incorporated: Not activated — Chat Session must refresh and supply the exact list or None.
Current Fluent, React, Griffel, Tauri, router, state, engine, capability, CSP, tool, and repository facts: Not activated — Chat Session must verify and supply them.
Current support matrix and native/manual evidence limitations: Not activated — Chat Session must refresh and supply the exact list or None.
Accepted title-bar result and per-platform fallback: Not activated — Chat Session must refresh and supply the exact result for Windows, macOS, and Linux.
Unresolved blockers/assumptions: Not activated — Chat Session must refresh and supply the exact list or None.
Authorization boundary: Not activated — the future activation may authorize only GFD-P2-WP02.
Authorization expires/invalidates when: Not activated — the future activation must invalidate on any base-SHA, accepted-predecessor, targeted-amendment, material tool/engine/platform/title-bar/capability/CSP, repository-layout, or authorization-boundary change.
```

## 2. Repository and exact starting state

**Repository:** `Mirarsoft/prime-shell`
**Authoritative base branch:** `main`
**Required starting commit:** Not activated — Chat Session must refresh and
supply the exact current accepted and merged `main` SHA after P2-WP01
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
- prompt-pack documentation branch:
  `docs/gfd-work-package-prompt-pack-v1`;
- WP02, WP03, Phase 1, and P2-WP01 activation and implementation:
  `Not started`;
- P2-WP02 implementation: `Not started`.

The documentation refs do not authorize implementation. P2-WP02 activation
and implementation must start from the latest accepted and merged `main`, only
after WP02, WP03, P1-WP01, and P2-WP01 are accepted and merged and their
required closure, baseline, and theme-foundation reports are accepted.

Required activation and Work Session preflight:

1. Verify exact access and write permission for `Mirarsoft/prime-shell`.
2. Verify the default branch is `main` and record its exact 40-character SHA.
3. Verify WP01, WP02, WP03, P1-WP01, and P2-WP01 are accepted and merged at
   that SHA.
4. Verify the final accepted Phase 0B closure, Phase 1 baseline, and P2-WP01
   theme-foundation reports, source snapshots, support matrix, platform
   evidence, measurements, deviations, blockers, and amendments.
5. Verify current Fluent, React, Griffel, Tauri, router, Zustand, CSP,
   capability, title-bar, engine, toolchain, dependency, lockfile, test, CI,
   and repository-path facts.
6. Verify the accepted title-bar choice and native fallback separately for
   Windows, macOS, and Linux.
7. Verify no P2-WP02, Phase 3, product feature, public UI package, or broad
   settings implementation already exists unexpectedly.
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
15. every separately accepted targeted architecture amendment named by
    activation
16. current application providers, routes, UI/layout state, shell or
    placeholder views, title-bar and window adapters, Rust settings/platform
    adapters, CSP/Griffel configuration, Tauri capabilities and permissions,
    shell/accessibility tests, package layout, toolchain/version files,
    dependency manifests and lockfiles, workflows, and accepted support
    claims
17. the exact activated revision of this execution prompt

In the first implementation progress message, state:

> `AGENTS.md, repository authority, accepted Phase 0A, accepted and merged
> WP01/WP02/WP03, P1-WP01, and P2-WP01 evidence, the final accepted Phase 0B
> closure report, the final accepted Phase 1 baseline report, the final
> accepted P2-WP01 theme-foundation report, and the activated GFD-P2-WP02
> prompt have been read and are active. Executing only GFD-P2-WP02 with
> GPT-5.6 Sol / High. GFD-P3-WP01 and later work remain unauthorized.`

If a required source is unavailable, unreadable, stale, or materially
inconsistent, stop before writing. Do not conduct a broad architecture review,
rerun predecessor packages as substitutes for accepted evidence, or improvise
new support, title-bar, persistence, or platform claims.

## 4. Dependencies, predecessor outputs, and prerequisite evidence

**Direct package prerequisite:** Accepted and merged `GFD-P2-WP01` theme,
tokens, and accessibility foundation.

**Direct dependent:** `GFD-P3-WP01`.

**Inherited predecessors:** Accepted and merged WP01/FIX01,
`GFD-P0B-WP02`, `GFD-P0B-WP03`, and `GFD-P1-WP01`.

**Authoring-time accepted WP01 commits:**

- accepted WP01 implementation head before squash merge:
  `bd502ffcd4c2d80bd3cd8817ebea4dfb18a76107`;
- accepted merged WP01 `main`:
  `35d53bffec6e5b05aefdf8c7fed39a9bfdf288a2`.

**Later prerequisite state at authoring:** P0B-WP02, P0B-WP03, P1-WP01, and
P2-WP01 prompts are accepted as Approved provisional, but their activation and
implementation are `Not started`. P2-WP02 cannot be activated while that
remains true.

**Required predecessor outputs:**

- accepted and merged WP01, WP02, WP03, P1-WP01, and P2-WP01 source;
- exact final merged predecessor commit and authoritative source snapshots;
- final accepted Phase 0B spike-closure, Phase 1 baseline, and P2-WP01
  theme-foundation reports;
- accepted platform title-bar/fallback, release-CSP, cross-engine,
  accessibility, support-matrix, toolchain, dependency, capability,
  permission, production-test-exclusion, and CI facts;
- exact Fluent, React, Griffel, Tauri, router, state, engine, material, theme,
  bootstrap, native-background, and current repository-path facts;
- accepted semantic token, accent, status, interaction, focus, density,
  reduced-motion/transparency, forced-colors, material fallback, portal,
  contrast, provenance, and raw-color contracts;
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
- accepted exact limitations and blockers that P2-WP02 must preserve rather
  than silently upgrade.

**Accepted deviations:** None are inferred. Activation must list every
accepted predecessor deviation or state `None`.

**Known blockers carried forward:** Implementation remains `Blocked` until
WP02, WP03, P1-WP01, and P2-WP01 are accepted and merged; the final Phase 0B
closure, Phase 1 baseline, and P2-WP01 theme-foundation reports are accepted;
and Chat Session supplies an exact activated P2-WP02 prompt.

Do not activate P2-WP02 while P2-WP01 is only an approved provisional prompt,
an unmerged implementation branch, an unaccepted theme-foundation report, or
evidence with an unresolved architecture-blocking failure.

## 5. Objective and measurable runnable outcome

Deliver one real, composable, keyboard-operable desktop application shell
around the accepted P2-WP01 theme foundation.

The activated package must produce a runnable Tauri path that:

- composes the platform-adaptive title bar, app rail, context sidebar, main
  workspace, optional inspector, optional bottom panel, status bar, toast
  region, dialog host, and in-window settings surface;
- uses real application routes and navigation state without product/domain
  behavior or a speculative feature SDK;
- follows the four accepted responsive bands and degrades coherently to a
  500 × 480 logical-pixel compact shell;
- keeps the main workspace usable and free from horizontal scrolling caused by
  surrounding panes;
- supports pointer and keyboard pane resizing, clamping, collapse, reopen, and
  deterministic focus restoration;
- applies the accepted platform title-bar result where it passed and the
  accepted native-decoration fallback everywhere else;
- persists only the bounded `ShellLayoutPreferencesV1` contract through one
  narrow typed Rust-owned bridge;
- consumes accepted theme, token, density, motion, transparency,
  forced-colors, material, focus, CSP/Griffel, and portal behavior without
  reimplementing or weakening the foundation;
- demonstrates correct shell landmarks, accessible names, keyboard order,
  200% scaling, RTL/text expansion, and focused screen-reader behavior;
- retains truthful real native and WebView2, WKWebView, and WebKitGTK evidence
  according to the accepted support matrix.

The runnable outcome is the reusable shell itself. Empty, illustrative shell
content may explain regions and states, but it must be clearly labeled and may
not be presented as a product feature, backend integration, domain workflow,
or Phase 3 implementation.

## 6. Explicit in-scope work

Implement only what is necessary for the runnable shell:

### Application composition and routing

- Add a small application bootstrap/composition layer under
  `apps/desktop/src/app/` when that matches the activated repository.
- Add real shell components under `apps/desktop/src/shell/` or the exact
  accepted current equivalent.
- Add the minimum React Router route structure for a home/workspace shell and
  an in-window settings route/dialog.
- Keep route IDs, navigation contributions, and settings sections static,
  local, and directly consumed.
- Add an error boundary and shell-level fallback only if not already supplied
  by the accepted foundation.

### Shell regions

- Platform-adaptive `TitleBarAdapter` and, only where accepted, window
  controls.
- Fixed 48 px icon-first application rail with accessible names and selected
  state.
- Context sidebar with docked, collapsible, and overlay behavior.
- Flexible main workspace that always receives layout priority.
- Optional inspector with docked and drawer/tab behavior.
- Optional bottom task/output panel with collapsed and expanded behavior.
- Status bar/task summary, toast region, and dialog host.
- In-window settings shell with navigation, grouped placeholder sections,
  safe immediate layout preference changes, and reset for the bounded shell
  fields.

### Responsive and resizing behavior

- Implement the exact responsive bands and dimensions in section 10.
- Use one deterministic source for band selection and pane constraints.
- Clamp persisted and live pane sizes after startup, window resize, scale/DPI
  change, display transition, and band transition.
- Support pointer dragging and keyboard resizing for docked splitters.
- Provide non-drag controls for users who cannot use pointer dragging.
- Restore focus to the invoking control after overlay, drawer, dialog, or
  panel closure.
- Avoid layout animation when reduced motion applies.

### State and narrow persistence

- Use local component state for hover, transient drag, open popovers, and form
  drafts.
- Use the accepted Zustand pattern for navigation and bounded live layout
  state when available.
- Define one versioned `ShellLayoutPreferencesV1` contract containing only
  approved pane visibility, sizes, collapse state, and the minimum accepted
  navigation/settings-shell preference.
- Add one narrow typed React-to-Rust read/write/reset bridge for this contract.
- Keep Rust as the sole durable writer, validate and clamp at the native
  boundary, debounce frequent layout writes, use accepted atomic/previous-copy
  behavior available at activation, and recover invalid shell fields without
  discarding unrelated valid settings.
- Keep comprehensive settings schemas, migrations, single-instance behavior,
  and diagnostics/recovery for their later packages.

### Focused verification and documentation

- Add proportional shell component, routing, state, persistence-contract,
  accessibility, release-CSP, native, and cross-engine checks.
- Add stories or a compact shell gallery only for complex reusable shell
  states that materially improve review.
- Update concise repository documentation only where necessary to run,
  inspect, and verify the implemented shell.
- Preserve exact evidence classifications and platform limitations.

## 7. Explicit exclusions and prohibited adjacent work

Do not:

- activate or implement WP02, WP03, Phase 1, or P2-WP01 as part of this
  package;
- start `GFD-P3-WP01`, `GFD-P3-WP02`, Phase 3, or any later package;
- add product/domain behavior, document import, analysis, search, real task
  output, backend product operations, product entities, or reference-feature
  UX;
- productize or redesign the Rust/Python task runtime, operation registry,
  file-intent boundary, backend protocol, lifecycle, or recovery behavior;
- create a public `packages/ui`, universal component library, module SDK,
  runtime plugin system, feature generator, or speculative contribution API;
- extract generic wrappers around standard Fluent controls without a concrete
  project convention enforced by the wrapper;
- implement broad settings ownership, comprehensive migration infrastructure,
  single-instance behavior, multiple workspaces/windows, a separate settings
  window, secrets, diagnostics, repair, telemetry, sync, or a product
  database;
- reopen P2-WP01 token, contrast, theme, material, portal, forced-color,
  density, motion, or transparency decisions without an accepted targeted
  amendment;
- re-litigate title-bar strategy or promote custom chrome beyond the exact
  accepted per-platform evidence;
- add a custom Linux title bar, fake native controls, browser-only title-bar
  claims, or visual uniformity that breaks native behavior;
- weaken release CSP, add remote content, enable broad IPC, add generic
  filesystem/process/window access, or retain test-driver permissions in
  production;
- add Storybook broadly, snapshot every component, or inflate tests with
  low-value variants;
- add unsupported platform, architecture, package-format, signing,
  notarization, updater, publication, release, store, or enterprise work;
- open or merge a pull request, enable auto-merge, delete branches, or change
  repository settings unless the activated revision explicitly and separately
  authorizes a draft PR;
- modify authority documents except through a separately accepted targeted
  amendment named by activation;
- hide incomplete behavior behind mocks, screenshots, placeholder returns, or
  documentation-only claims.

Static neutral shell labels and empty-region demonstrations are permitted only
to prove layout, navigation, focus, accessibility, and state behavior. They
must not claim product functionality.

## 8. Allowed repository areas and expected changes

Activation must replace these review-time areas with exact current paths and
must narrow them further where possible.

Expected primary ownership:

- `apps/desktop/src/app/`
- `apps/desktop/src/shell/`
- the current application entry and root composition files
- the current route and UI/layout state files
- focused shell tests, fixtures, stories, or gallery files beside owned code
- one narrow typed frontend layout-preference adapter
- one narrow Rust shell-layout settings module and command registration
- exact Tauri capabilities/permissions required by those named commands
- concise shell-specific documentation

Conditionally allowed only when demonstrated necessary:

- `apps/desktop/package.json`
- root `package.json`
- `pnpm-lock.yaml`
- current TypeScript/Vite/test configuration
- current Tauri `Cargo.toml` and `Cargo.lock`
- native window/title-bar adapter files
- one focused shell/native evidence workflow
- the already accepted design-token package when the shell consumes it
- shared schemas only for the narrow shell-layout preference contract if the
  accepted repository places cross-boundary contracts there

Protected unless separately authorized:

- `docs/authority/`
- accepted Phase 0A, Phase 0B, Phase 1, and P2-WP01 evidence
- Python backend product or lifecycle code
- backend operation registry and broad Rust command surface
- unrelated package/workspace configuration
- release, signing, updater, installer, store, publication, and diagnostics
  paths
- public package, module SDK, feature generator, product feature, and
  comprehensive settings paths
- any Phase 3 or later implementation path

Before writing, record the exact proposed file inventory and map each file to
an in-scope requirement. At completion, compare the final diff to that
inventory and explain every variance. An allowed area is not permission for
broad cleanup.

## 9. Ordered implementation procedure

For a future activated execution:

1. Complete section 2 preflight and stop on any mismatch.
2. Read all section 3 sources in order and record their exact identities.
3. Build a prerequisite ledger covering accepted commits, artifacts, hashes,
   reports, CI/native/manual evidence, deviations, fallbacks, amendments, and
   blockers.
4. Inspect the current application/provider/route/state/native-window/settings
   paths and map the smallest real shell integration.
5. Record the accepted platform title-bar decision and fallback for each
   supported target before designing title-bar code.
6. Define and test the shell region, route, state, focus, responsive-band,
   dimension, clamping, and `ShellLayoutPreferencesV1` contracts.
7. Implement the app composition, real routes, rail, sidebar, workspace,
   inspector, bottom panel, status/toast/dialog hosts, and settings shell.
8. Implement responsive docking, overlay/drawer transitions, pointer and
   keyboard splitters, non-drag alternatives, clamping, and focus restoration.
9. Integrate only the accepted title-bar/native-decoration behavior.
10. Implement the narrow typed Rust-owned layout read/write/reset bridge and
    exact least-privilege capability entries.
11. Integrate the accepted P2-WP01 provider, semantic tokens, density, motion,
    transparency, forced-colors, material fallback, focus, portal, and
    release-CSP/Griffel paths without duplication.
12. Add proportional component, route, reducer/contract, native, accessibility,
    cross-engine, and regression tests.
13. Run fast static/unit checks, then source build, package/native shell path,
    target evidence, manual checks, and accepted regressions in the activated
    order.
14. Inspect production artifacts and permissions for test-driver, dangerous
    IPC, remote content, duplicate renderer, and CSP exclusions.
15. Validate 500 × 480, 1097 × 617, 1280 × 720, each responsive threshold,
    scaling, RTL/text expansion, forced colors, reduced motion/transparency,
    title-bar fallback, and persisted-value clamping.
16. Update only the concise shell documentation required by the package.
17. Reconcile the final diff to scope and remove accidental or unused work.
18. Commit one coherent package change, push only the activated branch, and
    create or update a draft PR only if activation explicitly authorizes it.
19. Generate one run ID, create the four section 16 deliverables, verify their
    hashes and snapshot integrity, extract into a fresh empty directory, and
    rerun the complete activated verification.
20. Return the section 18 handoff and stop. Do not begin GFD-P3-WP01.

Never use a later package as the workaround for a failed shell gate.

## 10. Cross-cutting constraints

### Shell composition contract

The implementation must preserve this responsibility graph:

```text
AppRoot
├── AppProviders
│   ├── accepted Fluent/theme provider
│   ├── RouterProvider
│   ├── ErrorBoundary
│   └── accepted state providers
└── DesktopWindow
    ├── TitleBarAdapter
    ├── AppShell
    │   ├── NavigationRail
    │   ├── ContextSidebar
    │   ├── WorkspaceHost
    │   ├── InspectorHost
    │   └── BottomPanelHost
    ├── StatusBar
    ├── ToastRegion
    └── DialogHost
```

Names may follow the activated repository, but ownership and behavior must
remain clear. Standard Fluent controls are consumed directly unless a wrapper
enforces a real cross-shell convention.

### Required dimensions and responsive bands

| Region | Required default | Constraint |
|---|---:|---|
| Title bar | 32 px Windows baseline | Adapter may vary by accepted platform result |
| App rail | 48 px | Fixed, icon-first |
| Context sidebar | 280 px | Clamp to 220–400 px when docked |
| Inspector | 340 px | Clamp to 280–480 px when docked |
| Bottom panel | 30% window height | Minimum 160 px; maximum 50% |
| Main workspace | Flexible | Always receives layout priority |
| Minimum window | 500 × 480 logical px | Compact shell |

| Logical width | Sidebar | Inspector | Bottom panel |
|---|---|---|---|
| ≥ 1440 | Docked | Docked | User-controlled |
| 1200–1439 | Docked | Docked with narrower default | User-controlled |
| 840–1199 | Docked or collapsible | Drawer/tab | Reduced default |
| 500–839 | Overlay | Drawer/tab | Collapsed by default |

Rules:

- The app rail remains visible where accepted space and title-bar behavior
  permit.
- The workspace does not horizontally scroll because adjacent panes consume
  excessive width.
- Responsive state derives from logical application width, not a browser-only
  device guess.
- Threshold behavior is deterministic at both sides of every boundary.
- Live and persisted values clamp after window, display, DPI/scale, and band
  changes.
- An overlay or drawer traps focus only while appropriate, closes
  predictably, and restores focus to its invoking control.
- At 500 × 480, primary navigation, workspace content, settings escape, and
  essential status remain usable.
- Validate 1280 × 720 and 1097 × 617 logical degradation scenarios in addition
  to the threshold matrix.

### Splitter and focus contract

- Use semantic separators with correct orientation and current/minimum/maximum
  values.
- Pointer dragging and keyboard resizing update the same bounded state.
- Provide documented Arrow-key increments, modified larger increments, and
  explicit collapse/reset actions.
- Do not require fine pointer movement; expose non-drag alternatives.
- Keep visible, unobscured focus throughout resizing.
- Announce material size/state changes proportionately without flooding a
  screen reader.
- Restore focus after collapse, overlay/drawer closure, dialog closure, route
  change, and title-bar fallback transition where applicable.
- Do not animate state changes when reduced motion applies.

### Title-bar and native-window contract

- Use the accepted `TitleBarAdapter` result with native fallback.
- Windows custom chrome is permitted only if accepted evidence covers Snap
  Layouts, `Win+Z`, resize, system menu, keyboard, accessibility, active and
  inactive state, mixed DPI, restore, and required native hit testing.
- macOS retains native traffic lights and accepted movement, full-screen,
  focus, theme-background, and safe-area behavior.
- Linux uses native decorations unless an accepted targeted amendment supplies
  a named GNOME/KDE and Wayland/X11 result matrix.
- Buttons and editable controls are never draggable regions.
- Custom minimize, maximize/restore, and close controls require accessible
  names, correct state, keyboard reachability, and native behavior.
- If a promotion gate fails, use native decorations with a styled in-app
  header and report the exact fallback. Do not repair appearance by weakening
  native behavior.

### State and `ShellLayoutPreferencesV1`

The exact activated schema may use repository naming, but version 1 is bounded
to shell layout and must be semantically equivalent to:

```ts
interface ShellLayoutPreferencesV1 {
  schemaVersion: 1;
  sidebarWidth: number;
  sidebarCollapsed: boolean;
  inspectorWidth: number;
  inspectorOpen: boolean;
  bottomPanelHeightRatio: number;
  bottomPanelOpen: boolean;
  activeNavigationId?: string;
}
```

Constraints:

- Persist user intent, not computed responsive placement, effective
  capabilities, active focus, drag state, overlay state, window dimensions,
  or runtime platform facts.
- Zustand may own current navigation and layout UI state; Rust is the sole
  durable writer.
- React uses named typed commands rather than generic file access.
- Rust validates schema version, finite numbers, known IDs, and bounds.
- Invalid individual fields recover to safe defaults without discarding
  unrelated accepted settings.
- Writes are debounced and use the accepted atomic/previous-valid-copy
  mechanism available at activation.
- Do not introduce comprehensive settings migrations or claim Phase 6
  durability beyond the bounded bridge proven here.

### Theme, CSP, security, and trust boundaries

- Consume the accepted P2-WP01 semantic tokens and provider; do not fork a
  shell-specific palette or raw-color system.
- Preserve correct system/light/dark, forced-colors, density, reduced-motion,
  reduced-transparency, material fallback, focus, and portal behavior.
- Fluent/Griffel styling must work under the accepted release CSP without
  broad unsafe weakening or duplicate renderers.
- The React webview remains least trusted. Rust owns native commands, settings
  writes, platform policy, and capability decisions.
- Add only exact named layout/title-bar commands and capability permissions.
- No generic filesystem, process, shell, arbitrary window, backend tunnel, or
  remote-network API.
- Production artifacts exclude WebdriverIO/test-driver commands,
  capabilities, ports, listeners, fixtures, and secrets.
- Render any illustrative text as untrusted text, never executable markup.

### Accessibility and platform truth

- Use correct landmarks, headings, navigation names, selected/current state,
  accessible icon labels, dialog/drawer semantics, and status behavior.
- Preserve keyboard access and focus at 200% zoom/text scaling.
- Validate RTL and 30–50% text expansion without truncating required actions
  or breaking pane constraints.
- Use forced-color system behavior rather than a simulated palette.
- Keep screen-reader, title-bar, mixed-DPI, and native UX checks manual where
  automation cannot prove them.
- Visual equivalence across WebView2, WKWebView, and WebKitGTK is required;
  identical pixels are not.
- Do not upgrade unavailable or blocked platform evidence into a pass.

The package may claim evidence only within the accepted three-target matrix:

| Platform | Architecture and engine | Shell-specific authority |
|---|---|---|
| Windows 11 | x64 / WebView2 | Accepted custom-title-bar result or native-decoration fallback |
| Current and previous declared macOS major versions | arm64 / WKWebView | Native traffic lights and accepted overlay/safe-area result |
| Ubuntu 24.04 | x64 / WebKitGTK | Native decorations; Wayland/X11 behavior classified separately |

Activation must refresh the exact OS versions, runner images, engine versions,
native WebdriverIO capability, and manual owners. A target build or screenshot
does not prove native title-bar, screen-reader, DPI/display, installed-runtime,
or manual behavior. Another architecture, Linux distribution, engine, or
package format is outside this package unless a separately accepted amendment
and activation explicitly replace the matrix.

## 11. Proportional tests and exact evidence

Activation must replace command families with exact current commands, target
versions, environment identities, expected results, and artifact locations.
Do not claim a command was run unless its actual output is retained.

### Required static and unit checks

- frozen dependency install and lockfile integrity;
- formatting, lint, and TypeScript checks;
- Rust formatting, clippy, and focused unit/integration tests;
- route and navigation conflict checks for the actual small static route set;
- responsive-band threshold and dimension-clamping tests;
- layout reducer/state and `ShellLayoutPreferencesV1` validation tests;
- keyboard splitter, non-drag control, collapse, reopen, and focus-restoration
  component tests;
- landmark, heading, name, selected/current, separator-value, dialog/drawer,
  status, and live-region checks;
- accepted P2-WP01 token, contrast, provider, portal, forced-color,
  reduced-motion/transparency, and release-CSP regression checks;
- production capability and test-driver exclusion checks.

### Required real runnable evidence

- source frontend build and Tauri release build;
- real native Tauri launch into the shell on the declared primary environment;
- the accepted native WebdriverIO shell journey on each target where the
  accepted environment supports it, with production test capability absent;
- real navigation between workspace and in-window settings;
- every responsive band, 500 × 480 compact state, 1097 × 617, and
  1280 × 720;
- pointer and keyboard resizing, clamp behavior, collapse/reopen, and focus
  restoration;
- bounded layout save, restart/read, reset, invalid-field recovery, and
  resize/DPI reclamping through the real Rust command path;
- accepted custom title-bar behavior or the exact native-decoration fallback;
- release-CSP-safe Fluent/Griffel styling and themed portals in the packaged
  shell;
- forced colors, reduced motion/transparency, density, 200% scaling, RTL, and
  30–50% text expansion;
- focused screen-reader and platform-native title-bar checks;
- WebView2, WKWebView, and WebKitGTK shell evidence according to the accepted
  support matrix;
- accepted WP01/WP02/WP03/P1/P2-WP01 regression checks relevant to touched
  paths;
- source-snapshot CRC/path/root validation, fresh empty extraction, and
  complete activated rerun.

### Required focused scenarios

At minimum, cover:

1. first launch with default layout;
2. reopen with valid persisted layout;
3. invalid, stale, out-of-range, non-finite, and partially corrupt layout
   fields;
4. transition at 1440/1439, 1200/1199, 840/839, and the 500 minimum;
5. 500 × 480, 1097 × 617, and 1280 × 720 logical sizes;
6. pointer resizing to both bounds;
7. keyboard resizing with normal and larger increments;
8. non-drag collapse/reset controls;
9. sidebar overlay and inspector drawer open/close with focus restoration;
10. bottom panel open/collapse and focus restoration;
11. route change between workspace and settings;
12. 200% zoom/text scaling and 30–50% text expansion;
13. RTL navigation, splitters, overlays, and title-bar regions;
14. forced colors and reduced motion/transparency;
15. accepted title-bar path and forced native fallback;
16. platform/system theme change while shell and portals are open;
17. window resize, DPI/scale change, and multi-monitor transition;
18. release CSP and production test-driver/capability exclusion.

### Required evidence layers

For every check, record command or method, expected behavior, actual result,
OS/version/architecture/engine/tool identity, duration where relevant,
retained artifact or log, and blocker.

Keep these evidence classes separate:

1. shell route, composition, state, and responsive-contract evidence;
2. splitter, focus, keyboard, screen-reader, scaling, RTL, text-expansion, and
   forced-color accessibility evidence;
3. layout preference schema, frontend adapter, Rust validation, write,
   recovery, and restart evidence;
4. native title-bar, system-menu, window, DPI/display, and fallback evidence;
5. P2-WP01 provider, token, portal, material, and theme regression evidence;
6. release-CSP, Griffel-renderer, capability, and production-exclusion
   evidence;
7. source build and package-creation evidence;
8. real native Tauri and packaged-application runtime evidence;
9. WebView2, WKWebView, and WebKitGTK cross-engine/platform evidence;
10. CI workflow/run/job/artifact evidence;
11. manual native/accessibility evidence;
12. signing/notarization/update evidence;
13. documentation accuracy evidence;
14. unavailable or blocked evidence.

A component test, browser story, screenshot, source build, package,
documentation statement, prior result, or unavailable environment cannot be
relabeled as current real native startup/runtime, title-bar, cross-engine,
screen-reader, layout-restart, native UX, installation, signing, or manual
evidence.

### Required measurements

Retain raw values, method, sample count, environment, target, and truthful
pass/deviation/unverified/blocked classification for:

- first correctly themed visible shell frame;
- route-to-stable shell and settings navigation;
- pointer and keyboard resize response;
- responsive-band transition stability;
- layout preference write debounce and restart restore;
- shell behavior at 500 × 480, 1097 × 617, and 1280 × 720;
- native window resize and DPI/display transition;
- packaged startup and memory/package-size deltas relative to the accepted
  predecessor baseline.

Architecture timing and size values are measured goals, not fabricated service
levels. Hardware-sensitive misses remain raw evidence and receive an explicit
acceptance, focused-correction, or provisional-budget classification.

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

1. Exact accepted and merged WP01, WP02, WP03, P1-WP01, and P2-WP01
   predecessors plus accepted closure, baseline, theme-foundation, deviations,
   blockers, amendments, support facts, and repository paths were used.
2. The real Tauri application composes title bar, rail, sidebar, workspace,
   inspector, bottom panel, status, toast, dialog, and in-window settings
   responsibilities without product/domain behavior or hidden stubs.
3. Real routes and static navigation operate with conflict-free IDs, correct
   selected/current state, escape paths, and deterministic focus.
4. Required dimensions and all four responsive bands behave at both sides of
   every threshold; 500 × 480, 1097 × 617, and 1280 × 720 remain usable.
5. The workspace keeps priority and does not horizontally scroll because
   adjacent panes exceed their allowed space.
6. Sidebar, inspector, and bottom-panel docked, collapsed, overlay, drawer,
   and default states match the matrix and restore focus predictably.
7. Pointer and keyboard splitters share one bounded model, expose semantic
   values and non-drag alternatives, clamp correctly, and retain unobscured
   focus.
8. `ShellLayoutPreferencesV1` contains only approved shell fields; Zustand
   owns bounded UI state and Rust is the sole durable writer through named
   typed commands.
9. Valid layout preferences persist across a real restart; invalid or partial
   fields recover safely; frequent writes debounce; window/DPI/band changes
   reclamp without discarding unrelated settings.
10. The accepted Windows, macOS, and Linux title-bar result is used exactly;
    custom behavior passes its native gates or falls back to native
    decorations without false promotion.
11. The accepted P2-WP01 provider, semantic tokens, density, motion,
    transparency, forced-colors, material, focus, and portal contracts are
    consumed without a forked palette, raw-color leakage, or theme regression.
12. Release CSP permits the real Fluent/Griffel shell and portals without
    broad unsafe weakening, duplicate renderers, remote content, dangerous
    IPC, or production test capability.
13. Landmarks, headings, names, icon labels, selected/current state,
    separators, drawers/dialogs, status, keyboard order, focus restoration,
    and live announcements pass focused accessibility checks.
14. The shell remains usable at 200% scaling, with RTL and 30–50% text
    expansion, forced colors, reduced motion/transparency, non-pointer input,
    and target-size constraints.
15. Real native Tauri evidence exists for the declared primary environment,
    while WebView2, WKWebView, WebKitGTK, title-bar, screen-reader, native
    manual, and unavailable evidence remain separate and truthful.
16. Accepted WP01/WP02/WP03/P1/P2-WP01 behavior shows no relevant regression,
    including backend trust, lifecycle, CSP, capability, theme-before-paint,
    token, portal, and platform-baseline behavior.
17. Focused tests protect shell contracts and critical failure/accessibility
    paths without broad Storybook, speculative abstractions, or inflated
    low-value suites.
18. The final diff contains no Phase 3, P3-WP01, product feature, public UI or
    module SDK, broad settings/migrations, diagnostics/recovery, unsupported
    title-bar/platform, release/publication, or unrelated work.
19. Exact commands, environments, raw measurements, actual results,
    limitations, blockers, changed files, native/manual classifications, and
    accepted fallbacks are retained without documentation-only completion
    claims.
20. The final source snapshot represents the final commit, has valid SHA-256
    and CRC, contains one safe expected root, extracts into a fresh empty
    directory, and passes the complete activated rerun with a clean final tree.

Hardware-sensitive timing misses do not become silent passes. Retain raw values
and classify whether a miss invalidates the package, requires a focused
correction, or only adjusts a provisional budget.

## 13. Stop conditions and blocker reporting

Stop without improvising when:

- the repository, default branch, activated base SHA, predecessor state,
  accepted closure/baseline/theme-foundation report, or required fresh branch
  differs from activation;
- P2-WP01 is not accepted and merged;
- a required authority, predecessor source, report, evidence index, manifest,
  snapshot, hash, CI run/job/artifact, measurement, deviation, fallback,
  blocker, targeted amendment, support fact, title-bar result, or repository
  path is missing, stale, unaccepted, or materially conflicting;
- accepted predecessor behavior or evidence regresses before P2-WP02 changes;
- a frozen trust, CSP, capability, platform, accessibility, data-ownership,
  title-bar, token, settings, privacy, or release boundary would need to
  change;
- the accepted theme/provider/portal/CSP foundation cannot support the shell
  without redesign;
- real custom title-bar behavior fails an accepted native gate and native
  fallback cannot be used;
- responsive behavior, keyboard splitters, non-drag alternatives, focus
  restoration, 500 × 480 usability, or layout clamping cannot be made
  deterministic;
- Rust single-writer ownership, a narrow layout contract, atomic accepted
  persistence, or invalid-field recovery cannot be preserved;
- the package requires product behavior, broad settings/migrations,
  diagnostics/recovery, a public UI package, feature SDK, backend/lifecycle
  redesign, another package, or an unapproved path/dependency/platform/
  credential;
- exact current paths, versions, commands, engine capabilities, support
  claims, evidence classifications, or manual owners cannot be determined
  safely;
- required real native, cross-engine, title-bar, accessibility, restart, or
  manual evidence is unavailable and would have to be silently passed;
- the only support for a claimed gate is mock-only, story-only, browser-only,
  screenshot-only, source/build/package-only, documentation-only,
  unavailable, fabricated, or successor-dependent evidence;
- a workflow is blocked by permissions, policy, quota, billing, runner,
  architecture, engine, package tool, credential, or artifact-retention
  limits;
- completing the work would require P3-WP01, broad predecessor redesign,
  production release work, unsupported platform claims, or an unapproved
  architecture amendment;
- source-snapshot integrity or fresh-extraction rerun fails;
- the final diff includes an unauthorized path or unrelated existing changes
  cannot be preserved safely.

Return `Blocked` or `Partially implemented` using the approved labels. Name the
exact failed gate, observed evidence, unaffected scope, files changed,
unchanged exclusions, and smallest safe next action. Do not enter P3-WP01 or
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
GFD-P2-WP01 provisional prompt: Implemented and accepted as Approved provisional
GFD-P2-WP01 activation/implementation: Not started
GFD-P2-WP02 package outcome: Not started
Application composition and routes: Not started
App rail and context sidebar: Not started
Workspace and inspector: Not started
Bottom panel, status, toast, and dialog hosts: Not started
Responsive bands and compact shell: Not started
Pointer/keyboard splitters and focus restoration: Not started
Selected title-bar adapter/native fallback: Not started
In-window settings shell: Not started
ShellLayoutPreferencesV1 and Rust bridge: Not started
P2-WP01 theme/CSP/portal integration: Not started
Focused accessibility/native/cross-engine evidence: Not started
Supporting infrastructure: Not started
Tests: Not started
Documentation for implementation: Not started
Generated code: Not started
Fixtures/mocks: Not started
Stubs/placeholders: Not started
Incomplete work: Not started
Blocked work: Blocked — P2-WP01 is not accepted and merged and P2-WP02 has no activated prompt
GFD-P3-WP01 and later work: Not started
```

At implementation completion, report every applicable line above with exactly
one of: `Implemented`, `Partially implemented`, `Stub`, `Mock-only`,
`Not started`, or `Blocked`. Add evidence references and separately report:

- accepted predecessor, Phase 0B closure, Phase 1 baseline, and P2-WP01
  theme-foundation identities;
- application composition, real routes, region ownership, and navigation;
- every responsive band, required dimension, compact state, and clamping path;
- pointer/keyboard/non-drag splitter behavior and focus restoration;
- selected title-bar behavior and native fallback per platform;
- bounded frontend state, `ShellLayoutPreferencesV1`, Rust single-writer
  commands, validation, persistence, reset, and recovery;
- provider/token/density/motion/transparency/forced-color/material/focus/
  portal integration;
- release CSP, Griffel, capability, and production-test exclusion;
- focused component, keyboard, focus, screen-reader, scaling, RTL/text
  expansion, native startup, and restart evidence;
- WebView2, WKWebView, WebKitGTK, CI, manual, and unavailable evidence
  separately;
- source snapshot and fresh-extraction rerun;
- incomplete or blocked work;
- P3-WP01 and later work as `Not started`.

Documentation or tests may be `Implemented` while native, engine, title-bar,
screen-reader, persistence, or manual gates are `Partially implemented` or
`Blocked`; never conflate them.

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
prime-shell-work-gfd-p2-wp02-<RUN_ID>-source-snapshot-r1.zip
prime-shell-work-gfd-p2-wp02-<RUN_ID>-shell-implementation-report-r1.md
prime-shell-work-gfd-p2-wp02-<RUN_ID>-review-evidence-index-r1.md
prime-shell-work-gfd-p2-wp02-<RUN_ID>-handoff-manifest-r1.md
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

The shell-implementation report must:

- map accepted predecessor, support, deviation, fallback, blocker, and
  targeted-amendment facts to implemented repository decisions;
- identify application composition, routes, region ownership, UI/layout
  state, title-bar adapters, and narrow Rust settings ownership;
- list every responsive band, dimension, clamping rule, splitter input,
  collapse/overlay/drawer state, and focus-restoration result;
- identify `ShellLayoutPreferencesV1`, validation, debounce, persistence,
  restart, reset, invalid-field recovery, and unrelated-setting preservation;
- classify P2-WP01 theme/provider/token/portal integration,
  release-CSP/Griffel/capability exclusions, component/accessibility, real
  native, cross-engine, CI, manual, signing/update, and unavailable evidence
  separately;
- state every raw measurement, method, environment, sample count, deviation,
  limitation, fallback, and item deferred to P3-WP01 or later.

The review evidence index must map every package requirement and acceptance
gate to a repository file/section, command/result, retained artifact, and
unresolved item without duplicating the shell-implementation report.

The handoff manifest must record:

- activation ID, prompt ID/version, model/reasoning, repository, branch, exact
  base and final commit;
- accepted WP01/WP02/WP03/P1/P2-WP01 refs, closure/baseline/theme-foundation
  reports, source snapshots, artifacts, hashes, CI/native/manual evidence,
  measurements, deviations, fallbacks, amendments, limitations, and blockers;
- tools, runners, engines, target matrix, Fluent/React/Griffel/Tauri/router/
  state versions, changed files, and responsible execution paths;
- exact commands and actual results;
- shell/route/state, responsive/splitter/focus, layout persistence, title-bar,
  theme/provider/portal, CSP/capability, build/package, runtime, cross-engine,
  CI, manual, signing/update, documentation, and unavailable evidence
  separately;
- SHA-256 of the source snapshot, prompt-required reports, and required
  repository artifacts;
- a reproducible canonical self-hash convention if the manifest contains its
  own digest;
- truthful functional/status inventory, limitations, files intentionally not
  created, blockers, P3-WP01 exclusion, and exact next controlled action.

Expose the four artifacts individually to Chat Session. Do not create another
package prompt or present an incremental overlay as the authoritative source.

## 17. Branch, draft-PR, review, merge, and deletion controls

This provisional prompt authorizes no implementation branch or pull request.

When Chat Session activates it:

1. Use only the exact fresh implementation branch named in the activation
   record, created from the exact activated `main` SHA.
2. Do not reuse the documentation branch or any predecessor implementation
   branch.
3. Keep one coherent P2-WP02 shell change and preserve unrelated user work.
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
    merge authorization; P2-WP02 completion is not P3-WP01 authorization.

Report exact local/remote branch, commit, PR, draft, auto-merge, merge, and
deletion state in the handoff.

## 18. Completion response and return prompt

Keep progress and completion messages concise. Lead with the truthful package
result and include:

- model and reasoning level;
- repository, base, branch, final commit, clean-tree, PR, and merge state;
- accepted predecessor, Phase 0B closure, Phase 1 baseline, P2-WP01
  theme-foundation, and targeted amendment identities;
- application composition, routes, shell regions, responsive states,
  splitter/focus, title-bar/fallback, settings shell, and layout persistence
  outcomes;
- P2-WP01 theme/provider/token/portal, release-CSP/Griffel/capability,
  component/accessibility, native, cross-engine, CI, manual/unavailable,
  signing/update, and documentation evidence separately;
- exact commands, raw measurements, actual results, deviations, limitations,
  and blockers;
- full status inventory;
- four external artifact links and hashes;
- confirmation that P3-WP01 and later work were not started.

At completion, define:

```text
FINAL_SHA=the exact 40-character final implementation commit
```

Then end with this short prompt, substituting the defined `FINAL_SHA` and the
section 15 run ID with their exact values:

```text
Chat Session: Review GFD-P2-WP02 on the activated implementation branch at FINAL_SHA.
Read the four prime-shell-work-gfd-p2-wp02-RUN_ID deliverables. Verify the
real shell routes and regions, four responsive bands and 500 px compact state,
pointer/keyboard splitters, focus restoration, accepted title-bar fallback,
bounded Rust-owned layout persistence, P2-WP01 theme/CSP/portal integration,
focused accessibility/native/cross-engine evidence, and snapshot rerun, then
return Accepted, Focused correction required, or Blocked. GFD-P3-WP01 was not
started, and this handoff does not authorize merge or later implementation.
```

Do not include authorization for P3-WP01 or any later package. Do not continue
after returning the handoff.
