# Generic Fluent Desktop App Architecture

**Document status:** Consolidated architecture baseline  
**Version:** 0.2  
**Date:** 2026-07-27  
**Target:** Reusable cross-platform desktop application foundation  
**Primary visual direction:** Microsoft Fluent 2 and Windows 11 light/dark design language  
**Implementation model:** React + TypeScript frontend, Tauri 2/Rust native host, optional packaged Python sidecar  
**Review inputs:** `generic-fluent-desktop-app-architecture-v0.1.1.md`, `cursor-opus5-architecture-review.md`, and `work-session-architecture-review.md`

---

## 1. Purpose

Create a reusable desktop application foundation that can support future products without rebuilding the application shell, theming, settings, native integration, Python communication, diagnostics, testing, packaging, and release infrastructure.

The visual structure is based on the attached references, while their original titles and functionality are intentionally ignored.

The reusable shell consists of:

- A platform-adaptive title bar
- A narrow global navigation rail
- A contextual sidebar
- A flexible main workspace
- An optional right inspector
- An optional bottom task/output panel
- A status and notification layer
- An in-window settings surface
- Light, dark, system, forced-colors, and reduced-motion behavior

The architecture favors a practical small-team implementation. Abstractions are extracted only after a working reference feature proves that they are necessary.

---

## 2. Architecture principles

1. **Desktop-first and cross-platform.** Windows, macOS, and Linux use the same product structure but may use different native window behavior.
2. **Platform-adaptive, not pixel-identical.** Preserve native reliability and accessibility before enforcing visual uniformity.
3. **Least privilege at the webview boundary.** React never receives generic shell, process, arbitrary filesystem, or unrestricted native access.
4. **Rust is the native policy authority.** Rust validates frontend-originated operations, owns process lifecycle, file intent, durable settings, secrets, window behavior, and release integration.
5. **Python is trusted native code, not a sandbox.** The packaged sidecar runs with the current user’s privileges and must be governed through dependency control, a minimized environment, scoped inputs, and explicit operation contracts.
6. **No external Python installation.** Supported releases include the Python runtime and dependencies required by the product.
7. **Bounded communication.** IPC frames, queues, logs, task events, and large payloads have explicit limits and failure behavior.
8. **No automatic replay after failure.** A sidecar restart restores service availability; it does not replay uncertain business operations.
9. **Static first-party features in version 1.** Runtime third-party plugins require a different threat model and are excluded.
10. **Evidence before extraction.** The cross-platform technical spike and reference feature precede the public feature/module SDK.
11. **Offline-capable and network-denied by default.** Products may enable named HTTPS endpoints through reviewed native services; privileged webviews do not load arbitrary remote content.
12. **Accessibility is structural.** Keyboard, focus, screen-reader, forced-colors, scaling, and non-pointer alternatives are shell requirements rather than late polish.

---

## 3. Trust and system context

```text
┌──────────────────────────────── Desktop application ────────────────────────────────┐
│                                                                                    │
│  ┌──────────── React / TypeScript webview ────────────┐                            │
│  │ UI, routes, feature views, commands, task display  │                            │
│  │ Least-trusted application layer                    │                            │
│  └─────────────────────────┬──────────────────────────┘                            │
│                            │ typed Tauri commands and channels                      │
│  ┌─────────────────────────▼──────────────────────────┐                            │
│  │ Tauri 2 / Rust host                               │                            │
│  │ Native policy authority                           │                            │
│  │ Operation registry, files, settings, secrets,     │                            │
│  │ windows, Python lifecycle, logs, updater           │                            │
│  └─────────────────────────┬──────────────────────────┘                            │
│                            │ bounded versioned JSON Lines over stdio                │
│  ┌─────────────────────────▼──────────────────────────┐                            │
│  │ Packaged Python sidecar                           │                            │
│  │ Trusted first-party native worker                 │                            │
│  │ Domain computation and product integrations        │                            │
│  └────────────────────────────────────────────────────┘                            │
└────────────────────────────────────────────────────────────────────────────────────┘
```

### 3.1 Boundary rules

- The React webview is not allowed to start arbitrary executables.
- Rust rejects unknown backend operations before Python receives them.
- Tauri capabilities are applied per window/webview and command/plugin scope.
- Feature metadata inside one shared webview is not a security boundary.
- Python receives the minimum practical environment and task-scoped inputs.
- Secrets remain in Rust unless one specific backend operation requires a value.
- Remote HTML, scripts, and arbitrary SVG markup are not rendered in privileged webviews.
- Rust and Python are both trusted release artifacts and require locked, reviewed dependencies.

---

## 4. Selected stack

| Layer | Selection | Responsibility |
|---|---|---|
| UI runtime | React + TypeScript | Component-based desktop UI |
| Frontend build | Vite | Development and production bundling |
| Design system | Fluent UI React v9 | Fluent controls and accessibility foundation |
| Styling | Fluent tokens + app semantic tokens + Griffel/CSS | Themes, surfaces, layout, interaction states |
| Desktop host | Tauri 2 | Native application host and packaging |
| Native policy layer | Rust | Commands, windows, files, settings, secrets, process lifecycle |
| Python integration | Packaged sidecar | Domain computation and product-specific services |
| Initial Python bundle candidate | PyInstaller `onedir` | Inspectable and testable per-platform bundle |
| UI/layout state | Zustand | UI preferences, layout, navigation, bounded task reducer |
| Fetchable backend state | TanStack Query | Snapshots, entities, searches, invalidation |
| Routing | React Router | Application and settings routes |
| Contract authority | JSON Schema 2020-12 + shared fixtures | Cross-language boundary contract |
| Frontend validation | Generated TypeScript/Zod or equivalent | Runtime validation at the webview boundary |
| Unit/component tests | Vitest + React Testing Library + axe | React behavior and accessibility |
| Rust tests | `cargo test` + integration fixtures | Native policy, protocol, process lifecycle |
| Python tests | pytest | Protocol and domain behavior |
| Native desktop E2E | WebdriverIO + `@wdio/tauri-service` | Packaged/dev Tauri flows on all target platforms |
| Optional browser journeys | Playwright or WebdriverIO browser mode | Fast UI tests with mocked Tauri IPC |
| Component development | Storybook, selectively | Complex reusable shell components |
| CI/CD | GitHub Actions | Per-platform build, test, package, signing, and release |

### 4.1 Version policy

- Pin Node, pnpm, Rust, Python, Tauri, Fluent UI, and packaging tool versions in-repository.
- Commit all lockfiles.
- Upgrade dependencies through dedicated reviewed changes.
- Regenerate contract and token snapshots in CI; fail if generated output differs.
- Build the Python bundle on the target operating system. PyInstaller is not treated as a cross-compiler.

### 4.2 Contingency choices

- Electron is reconsidered only if a hard requirement needs identical Chromium behavior or Node-only APIs that cannot be cleanly brokered through Tauri.
- PyInstaller `onefile` remains an optional measured alternative, not the initial production default.
- A different contract IDL is adopted only if JSON Schema plus fixtures proves inadequate during the spike.
- A separate Python worker pool is introduced only when measured workloads require concurrent or uninterruptible jobs.

---

## 5. Initial platform and distribution baseline

The final public support matrix is approved in Phase 0A. The initial validation baseline is intentionally small:

| Platform | Initial validation target | CPU | Window system | Initial package |
|---|---|---|---|---|
| Windows | Windows 11 | x64 | Desktop Window Manager/WebView2 | NSIS installer |
| macOS | Current and previous major version | arm64 | WKWebView | Signed/notarized `.app` in DMG |
| Linux | Ubuntu 24.04 LTS | x64 | Wayland and X11/WebKitGTK | One of AppImage or `.deb`, selected in spike |

Additional operating systems, distributions, package types, and CPU architectures require explicit product need, CI capacity, and test ownership.

### 5.1 Support principles

- “Latest CI runner” does not define the user support contract.
- Every supported target must have a clean-install smoke test.
- Native package and sidecar architectures must match.
- WebView2 installation strategy and Linux runtime dependencies are documented in the release guide.
- Store distribution is out of scope for version 1 unless required by the first product; store sandboxing and entitlement rules can change the sidecar model.

---

## 6. UI shell architecture

### 6.1 Shell regions

```text
┌──────────────────────── Platform-adaptive title bar ────────────────────────────┐
├───────┬──────────────────────┬─────────────────────────────┬───────────────────┤
│ App   │ Context sidebar      │ Main workspace              │ Inspector         │
│ rail  │                      │                             │ optional/drawer   │
│       │                      ├─────────────────────────────┤                   │
│       │                      │ Bottom task/output panel    │                   │
│       │                      │ optional/collapsible        │                   │
├───────┴──────────────────────┴─────────────────────────────┴───────────────────┤
│ Optional status/task summary                                                    │
└────────────────────────────────────────────────────────────────────────────────┘
```

### 6.2 Initial dimensions

| Region | Default | Behavior |
|---|---:|---|
| Title bar | 32 px Windows baseline | Platform adapter may vary |
| App rail | 48 px | Fixed, icon-first |
| Context sidebar | 280 px | Resizable, 220–400 px when docked |
| Inspector | 340 px | Resizable, 280–480 px when docked |
| Bottom panel | 30% window height | Minimum 160 px; maximum 50% |
| Main workspace | Flexible | Always receives priority |
| Minimum window | 500 × 480 logical px | Compact shell; supports smaller snap zones |

### 6.3 Responsive states

| Logical width | Sidebar | Inspector | Bottom panel |
|---|---|---|---|
| ≥ 1440 | Docked | Docked | User-controlled |
| 1200–1439 | Docked | Docked with narrower default | User-controlled |
| 840–1199 | Docked or collapsible | Drawer/tab | Reduced default |
| 500–839 | Overlay | Drawer/tab | Collapsed by default |

Rules:

- The app rail remains visible where space permits.
- The main workspace never horizontally scrolls because surrounding panels are too wide.
- Splitters support pointer and keyboard resizing.
- Collapsing or closing a panel restores focus predictably.
- Panel sizes are clamped after display, DPI, or window-size changes.
- Validation includes 1280×720 logical and 1097×617 logical degradation scenarios.

### 6.4 Platform title-bar strategy

Use a `TitleBarAdapter` with a native fallback.

| Platform | Default direction | Promotion gate |
|---|---|---|
| Windows | Custom Fluent candidate, native fallback | Snap Layouts, resize, system menu, keyboard, accessibility, mixed-DPI, and restore behavior pass |
| macOS | Native traffic lights with overlay/transparent title area | Native movement, full screen, focus, theme background, and safe-area behavior pass |
| Linux | Native decorations | Custom mode is unsupported until a named GNOME/KDE, Wayland/X11 matrix passes |

Windows custom chrome must preserve Snap Layout behavior. A custom maximize region requires appropriate native hit testing, including `HTMAXBUTTON` behavior where applicable. If this cannot be delivered safely and consistently, use native decorations with a styled in-app header.

Title-bar acceptance includes:

- Draggable non-interactive regions
- No drag on buttons or editable controls
- Minimize, maximize/restore, and close
- Double-click maximize/restore
- Platform system menu behavior
- Active/inactive visual states
- Accessible names and keyboard reachability
- Windows Snap Layouts and `Win+Z`
- 100%, 125%, 150%, and 200% scaling
- Mixed-DPI multi-monitor transitions
- RTL and text expansion
- Forced colors/high contrast
- Native-decoration fallback

### 6.5 Settings surface

Version 1 uses an in-window route/dialog:

- Settings navigation on the left
- Searchable settings
- Grouped rows/cards
- Immediate application for safe preferences
- Explicit confirmation for destructive actions
- Per-section reset
- Schema-versioned persistence

A separate native settings window requires a product ADR because it changes synchronization, capability, focus, and lifecycle behavior.

---

## 7. Fluent design system

### 7.1 Theme modes

Support:

- System
- Light
- Dark
- Windows system accent where available
- Default Fluent accent
- Custom accent seed with accessibility validation
- Forced colors/high contrast
- Reduced motion
- Reduced transparency
- Compact and comfortable density

Theme layers:

1. Fluent UI `webLightTheme` or `webDarkTheme`
2. Application semantic tokens
3. Platform capability/material adapter

### 7.2 Semantic shell palette

The following is the deterministic cross-platform solid fallback. Fluent tokens remain authoritative inside Fluent controls.

| Semantic role | Light | Dark |
|---|---|---|
| App/window shell | `#F5F5F5` | `#1F1F1F` |
| Title bar | `#F5F5F5` | `#1F1F1F` |
| Navigation rail | `#F5F5F5` | `#1F1F1F` |
| Context sidebar | `#FAFAFA` | `#1F1F1F` |
| Main workspace | `#FFFFFF` | `#292929` |
| Inspector | `#FAFAFA` | `#1F1F1F` |
| Bottom/output panel | `#F5F5F5` | `#141414` |
| Card/elevated surface | `#FAFAFA` | `#333333` |
| Code/terminal surface | `#FFFFFF` | `#141414` |
| Standard panel border | `#D1D1D1` | `#525252` |
| Subtle divider | `#E0E0E0` | `#3D3D3D` |
| Primary text | `#242424` | `#FFFFFF` |
| Secondary text | `#424242` | `#D6D6D6` |
| Tertiary text | `#616161` | `#ADADAD` |
| Disabled text | `#BDBDBD` | `#5C5C5C` |
| Overlay | `rgba(0,0,0,0.40)` | `rgba(0,0,0,0.50)` |

In dark themes, lighter neutral surfaces represent higher elevation. Pure black is not the default workspace background.

### 7.3 Interaction states

| State | Light | Dark |
|---|---|---|
| Subtle hover | `#F5F5F5` | `#383838` |
| Subtle pressed | `#E0E0E0` | `#2E2E2E` |
| Subtle selected | `#EBEBEB` | `#333333` |
| Workspace hover | `#F5F5F5` | `#3D3D3D` |
| Workspace pressed | `#E0E0E0` | `#1F1F1F` |
| Disabled surface | `#F0F0F0` | `#141414` |

Shell-state aliases are reconciled with the pinned Fluent version during Phase 2. Any intentional divergence is documented in the token snapshot.

### 7.4 Accent roles

Do not use one ambiguous `accentForeground` token. Use:

```ts
interface AccentTokens {
  accentBackground: string;
  accentBackgroundHover: string;
  accentBackgroundPressed: string;
  onAccentForeground: string;
  accentLinkForeground: string;
  accentSubtleBackground: string;
  focusStrokeInner: string;
  focusStrokeOuter: string;
}
```

Default aliases:

| Role | Light | Dark |
|---|---|---|
| Accent background | `#0F6CBD` | `#115EA3` |
| Accent hover | `#115EA3` | `#0F6CBD` |
| Accent pressed | `#0C3B5E` | `#0C3B5E` |
| Text/icon on accent | `#FFFFFF` | `#FFFFFF` |
| Accent link on neutral | `#115EA3` | `#479EF5` |
| Accent-subtle background | `#EBF3FC` | `#082338` |

Focus uses a dual-tone or otherwise contrast-verified treatment so it remains visible on neutral, accent, status, image, and translucent surfaces.

### 7.5 Status colors

| Status | Light foreground | Dark foreground | Light subtle background | Dark subtle background |
|---|---|---|---|---|
| Danger | `#B10E1C` | `#DC626D` | `#FDF3F4` | `#3B0509` |
| Success | `#0E700E` | `#54B054` | `#F1FAF1` | `#052505` |
| Warning | `#BC4B09` | `#FAA06B` | `#FFF9F5` | `#4A1E04` |

Status communication always includes text and/or an icon, not color alone.

### 7.6 Accent policy

```ts
type AccentMode =
  | { mode: "system" }
  | { mode: "default" }
  | { mode: "custom"; seedColor: string };
```

- Persist the selected mode rather than the resolved OS color.
- Rust resolves platform accent capability.
- Custom seeds generate a complete brand ramp.
- Reject or adjust a generated ramp that cannot meet approved contrast pairs.
- Theme changes propagate to portals, dialogs, menus, tooltips, and native window backgrounds without restart.

### 7.7 Mica, Acrylic, and fallback

Materials are capabilities, not guaranteed visuals.

- Mica may be used for long-lived Windows shell/title surfaces.
- Acrylic is limited to transient surfaces such as flyouts.
- Adjacent permanent Acrylic panes are not used.
- High contrast, reduced transparency, remote sessions, unsupported systems, or implementation failure resolve to solid semantic colors.
- macOS/Linux use stable native translucency only when it remains accessible and performant.
- Do not emulate Mica with screenshot or continuous custom-blur pipelines.

Separate window material from transient surface treatment:

```ts
type WindowMaterialPreference = "system" | "solid" | "mica" | "micaAlt";

interface MaterialCapabilities {
  mica: boolean;
  micaAlt: boolean;
  transparencyEnabled: boolean;
  forcedColors: boolean;
  reducedTransparency: boolean;
  reason?: "unsupported" | "user-disabled" | "accessibility" | "remote-session" | "battery" | "error";
}
```

### 7.8 High contrast and forced colors

- Prefer platform system colors and `forced-colors` behavior.
- Do not simulate Windows high contrast with a fixed custom palette.
- Use Fluent’s compatible high-contrast behavior and a small forced-colors stylesheet.
- Do not suppress OS-selected focus, text, link, or selection colors without a documented accessibility reason.

### 7.9 Typography and icons

- Windows: Segoe UI Variable/Segoe UI when provided by the operating system.
- macOS: platform system UI font.
- Linux: platform/common distro UI font or an explicitly licensed bundled fallback.
- Do not redistribute Segoe UI.
- Layout dimensions and line heights are token-driven rather than dependent on one font’s metrics.
- Use Fluent System Icons with a documented regular/filled selected-state convention.

### 7.10 Theme implementation contract

Initial design-token package:

```text
packages/design-tokens/
├── src/
│   ├── semantic-light.ts
│   ├── semantic-dark.ts
│   ├── accent.ts
│   ├── materials.ts
│   ├── forced-colors.css
│   └── contract.ts
├── snapshots/
│   ├── light.json
│   ├── dark.json
│   └── provenance.json
└── tests/
    ├── contract.test.ts
    ├── contrast-matrix.test.ts
    └── raw-color-policy.test.ts
```

Rules:

- Features consume semantic or Fluent tokens.
- Raw colors are prohibited in feature styling except reviewed categories such as charts, syntax highlighting, images, and user-authored content.
- Contrast tests use an explicit permitted foreground/background matrix.
- Disabled controls and decorative strokes use named WCAG exemptions rather than weakening the entire test.
- Rust resolves the theme or safe shell background before showing the window.
- A tiny HTML bootstrap applies a matching background before React starts to prevent light/dark launch flash.
- Token upgrades include a generated diff, visual review, and updated provenance.

### 7.11 Accessibility target

Target WCAG 2.2 AA for webview content plus platform-native acceptance tests:

- Complete keyboard navigation
- Visible focus without obstruction
- Accessible icon-only controls
- Correct landmarks and headings
- Keyboard and non-drag alternatives for splitters
- Focus restoration after dialog/panel closure
- Controlled screen-reader announcements for task progress
- Reduced-motion and reduced-transparency behavior
- Forced-colors support
- 200% text scaling/zoom resilience
- Target-size and dragging-alternative checks
- Manual screen-reader/title-bar checks on each supported platform

---

## 8. Component and feature architecture

### 8.1 Core shell components

```text
AppRoot
├── AppProviders
│   ├── FluentProvider
│   ├── QueryClientProvider
│   ├── RouterProvider
│   ├── ErrorBoundary
│   └── CommandProvider
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

Initial custom component candidates:

- `AppShell`
- `TitleBarAdapter`
- `WindowControls`
- `NavigationRail`
- `ContextSidebar`
- `WorkspaceHost`
- `InspectorPanel`
- `BottomPanel`
- `ResizablePane`
- `StatusBar`
- `SettingsShell`
- `CommandPalette`

Consume standard Fluent controls directly unless a wrapper enforces a real project convention.

### 8.2 Feature definition

The reference feature first uses a small internal definition:

```ts
export interface FeatureDefinition {
  id: string;
  routes: readonly AppRouteDefinition[];
  navigation?: readonly NavigationContribution[];
  commands?: readonly CommandContribution[];
  settings?: readonly SettingsContribution[];
  requiredOperations?: readonly BackendOperationName[];
}
```

Rules:

- Features are statically imported.
- Route, command, ID, and shortcut conflicts fail tests/startup validation.
- `requiredOperations` supports availability, documentation, and CI checks.
- `requiredOperations` is not a runtime security boundary.
- Rust authorization remains independent of feature metadata.
- Public `module-sdk`, feature migrations, and generators are deferred until a reference feature and a second small feature prove the contract.

---

## 9. Frontend state and persistence

### 9.1 State ownership

| State | Owner |
|---|---|
| Popovers, form drafts, hover, drag | Local component state |
| Theme selection, density, layout, navigation | Zustand |
| Bounded live task event reducer | Zustand |
| Fetchable entities, searches, backend snapshots | TanStack Query |
| Durable application settings and migrations | Rust settings repository |
| Domain data | Python-owned storage when a product requires it |

Do not mirror TanStack Query data into Zustand without a concrete editing/session requirement.

### 9.2 Persisted UI state

```ts
interface PersistedUiState {
  schemaVersion: number;
  themeMode: "system" | "light" | "dark";
  accent: AccentMode;
  windowMaterial: WindowMaterialPreference;
  density: "compact" | "comfortable";
  motion: "system" | "reduced";
  layout: PersistedLayout;
}
```

Resolved system accent, material capability, forced-colors state, and effective theme are runtime state and are not persisted.

### 9.3 Persistence rules

- Persist only approved fields.
- Rust performs schema validation and migration.
- Writes use temp-file plus atomic replacement where supported.
- Keep the previous valid copy.
- Recover invalid sections without discarding unrelated valid settings.
- Debounce frequent layout writes.
- Version 1 defaults to a single application instance.
- A second launch focuses the existing instance and forwards supported launch arguments.
- Multi-window writes, if introduced, go through Rust.

### 9.4 TanStack Query conventions

- Query keys derive from operation name and normalized validated parameters.
- Automatic retry is permitted only for operations explicitly marked idempotent.
- Write/destructive operations are never retried automatically.
- Query abort signals map to backend cancellation where the operation is cancellable.
- Backend change events invalidate affected keys through one central adapter.
- High-frequency progress is not stored as query data.

---

## 10. Rust-to-Python architecture

### 10.1 Sidecar trust and launch

The Python sidecar is trusted first-party native code running with the current user’s privileges. It is not sandboxed by Tauri or PyInstaller.

Rust launches it with:

- Exact bundled executable/resource path
- No shell
- Controlled arguments
- Deterministic working directory
- Minimal constructed environment
- Explicit UTF-8 and unbuffered protocol configuration
- Piped stdin/stdout/stderr
- Continuous concurrent draining of stdout and stderr
- Process-tree containment
- No automatic inheritance of credentials, proxy variables, or unrelated secrets

### 10.2 Packaging baseline

Use PyInstaller `onedir` first:

- Easier inspection and dependency diagnosis
- No per-launch extraction requirement
- More straightforward native-library signing and verification
- Better support for clean-machine and antivirus testing

Bundle the full sidecar directory as Tauri resources and let Rust resolve the executable from the resource directory.

`onefile` may be reconsidered only after measuring startup, temporary extraction, endpoint protection behavior, signing/notarization, crash cleanup, and native dependency compatibility.

Build separately on each supported OS/architecture.

### 10.3 Contract authority

Checked-in JSON Schema 2020-12 plus shared valid/invalid fixtures is the version-1 authority.

```text
packages/app-contracts/
├── schemas/
│   ├── handshake.schema.json
│   ├── envelope.schema.json
│   ├── errors.schema.json
│   ├── task-events.schema.json
│   └── operations/
├── fixtures/
│   ├── valid/
│   └── invalid/
├── generated/
│   └── typescript/
└── scripts/
    ├── generate.ts
    └── verify-drift.ts
```

- TypeScript/Zod may be generated.
- Rust serde and Python models may initially remain idiomatic handwritten types if all implementations run the same fixtures.
- CI fails on schema/generation drift.
- A schema-bundle hash is included in the handshake.
- A larger IDL is not introduced unless JSON Schema proves inadequate.

### 10.4 Transport

Use UTF-8 JSON Lines over stdin/stdout:

- One JSON object per line
- `stdout` contains protocol only
- `stderr` contains structured logs only
- Handshake maximum: 64 KiB
- Normal frame maximum: 1 MiB
- Log-line maximum: 64 KiB, then truncate with marker
- Bounded pending requests: initial limit 64
- Bounded backend event queue: initial limit 256
- Progress delivered to UI at most 10 times/second/task
- Binary and multi-megabyte values use an opaque artifact handle rather than inline JSON
- Invalid UTF-8, malformed JSON, oversized frames, unknown kinds, and unknown operations fail deterministically
- Stream corruption terminates that sidecar instance and fails pending work

CRLF is accepted as a normal line ending. The actual protocol risks are encoding, buffering, stray stdout output, malformed frames, and unbounded allocation.

### 10.5 Handshake

Python emits one `hello` frame before requests are accepted:

```json
{
  "protocol": "generic-app",
  "kind": "hello",
  "protocolMin": 1,
  "protocolMax": 1,
  "backendVersion": "0.1.0",
  "buildId": "sha256:...",
  "targetTriple": "x86_64-pc-windows-msvc",
  "pythonVersion": "3.x",
  "schemaHash": "sha256:...",
  "supportedOperations": ["spike.echo", "spike.count"]
}
```

Rust validates:

- Protocol overlap
- Expected bundled build/schema hash
- Supported target
- Required operation availability
- Startup/handshake deadline

The frontend receives only a safe `BackendStatus`.

### 10.6 Rust operation registry

The webview does not send an unrestricted method name through a universal tunnel.

Version 1 uses a compile-time Rust registry or generated operation enum:

```rust
enum BackendOperation {
    SpikeEcho(SpikeEchoRequest),
    SpikeCount(SpikeCountRequest),
}
```

Each operation declares:

- Request/response schema
- Risk class
- Initiating window(s)
- Maximum payload size
- Timeout
- Cancellable flag
- Idempotent flag
- Audit requirement
- Whether file/artifact references are permitted

Unknown operations are rejected in Rust before Python receives them.

### 10.7 Requests, tasks, and traces

Identifiers:

- `requestId`: one invocation
- `taskId`: assigned only after long-running work is accepted
- `traceId`: minted authoritatively by Rust and propagated through logs and Python

Rules:

- A request receives a synchronous terminal result, or `accepted` followed by one terminal result.
- Task event sequence increases monotonically.
- Progress may be coalesced; terminal events may not.
- Cancellation is best-effort.
- Cancellation acknowledgement means the request was received, not that work has already stopped.
- A success accepted by Rust wins over a later cancellation race.
- Timeouts trigger cancellation but do not imply side effects were rolled back.
- No operation is retried unless explicitly idempotent.
- No uncertain operation is automatically replayed after a crash.

### 10.8 Task states

Version 1 states:

- Queued
- Running
- Cancelling
- Succeeded
- Failed
- Cancelled
- TimedOut
- Interrupted

`Paused` is excluded until a real workload supports pause/resume.

Tasks are not durable across application restarts in version 1. Closing with active tasks warns the user and requests confirmation.

### 10.9 Cancellation model

Initial practical model:

- One active long-running sidecar task at a time unless Phase 0 workload evidence requires more.
- Short status/read operations may coexist if the Python runtime remains responsive.
- Python continuously reads control messages independently from task execution.
- Cooperative operations check cancellation at documented checkpoints.
- Rust coalesces progress and enforces deadlines.
- If an operation does not stop within the cancellation deadline, Rust may terminate and restart the sidecar, marking all affected in-flight work `Interrupted`.
- The app never claims rollback unless the operation itself provides transactional guarantees.
- Per-task child worker processes are deferred until native, GPU, uninterruptible, or concurrent workloads require them.

### 10.10 Lifecycle state machine

```text
Stopped
  → Starting
  → Ready
  → Busy
  → Ready

Starting → Faulted          invalid/late handshake
Ready/Busy → Restarting     unexpected exit
Restarting → Ready          one valid bounded restart
Restarting → Faulted        restart budget exceeded
Ready/Busy → Stopping       application shutdown
Stopping → Stopped          graceful exit or process-tree termination
```

Policy:

- Lazy start unless the first product requires backend readiness at launch.
- Fail all in-flight requests on unexpected exit with `BACKEND_CRASHED`.
- Never replay automatically.
- Restart once after short backoff.
- Repeated failures within 60 seconds open a circuit and require explicit user action.
- Graceful shutdown has a bounded deadline followed by process-tree termination.
- Host shutdown must leave no child or descendant process.

Containment candidates:

- Windows Job Object with kill-on-close behavior
- Unix process group/session and parent-death handling where available

Exact implementation is proven during the spike.

### 10.11 Rust-to-UI streaming

Use a Tauri channel or equivalent ordered Rust-managed subscription for task events:

- Deliver only to authorized/requesting windows.
- Coalesce progress before crossing into the webview.
- Keep terminal state and a queryable task snapshot so navigation does not lose state.
- Global broadcast events are reserved for low-frequency application-wide changes.

---

## 11. Data, files, and configuration

### 11.1 Single-writer ownership

| Store | Sole writer | Read access |
|---|---|---|
| UI settings/layout/theme | Rust | React through typed commands |
| Secrets | Rust/OS keychain | Rust only, operation-scoped disclosure if required |
| Product domain database | Python | Through typed backend operations |
| Rust logs | Rust | Diagnostics |
| Python logs | Python appends its own stream/file | Rust diagnostics aggregation |
| Cache/task files | Owning layer | Scoped brokered access |

Invariant: no durable file has more than one writing process.

### 11.2 Intent-centric file access

Prefer:

```text
React requests “select input document”
→ Rust opens native picker
→ Rust validates type, size, path, and policy
→ React receives an opaque DocumentRef
→ Python receives one scoped input for one operation
→ Result is imported to app storage or exported through a save picker
```

```ts
interface DocumentRef {
  id: string;
  displayName: string;
  size: number;
  mediaType?: string;
}
```

Rules:

- Feature code does not receive arbitrary native file APIs.
- User-selected paths are validated in Rust.
- Imported names, paths, text, Markdown, SVG, logs, and backend errors are untrusted display data.
- External text renders as text.
- Markdown, if supported, uses an allowlist sanitizer and no raw HTML.
- User SVG is rendered as an image, never inlined as privileged markup.
- Large files may use a validated operation-scoped path when copying is impractical.
- Create/write paths validate the parent and account for symlink/reparse and time-of-check/time-of-use risks.

### 11.3 Secret storage degradation

- Probe OS keychain capability at startup.
- Never silently fall back to plaintext secret storage.
- If unavailable, disable secret-dependent functionality or request the secret per session.
- Surface the capability state in diagnostics.

### 11.4 Configuration hierarchy

```text
built-in defaults
  → optional installation policy
  → user settings
  → product/workspace settings when a real workspace exists
  → session overrides
```

Every layer is schema-validated and versioned.

---

## 12. Security architecture

### 12.1 Webview and native policy

- Explicit Tauri capabilities
- No shell plugin permission for feature windows
- No generic shell execution
- No arbitrary file opener
- No arbitrary remote navigation
- No secrets in frontend bundles
- Rust operation registry
- Per-window operation checks
- Schema validation in Rust
- Payload and timeout limits per operation
- Audit logging for destructive/privileged operations when the product requires it

### 12.2 Content Security Policy

Start with a strict production CSP during shell implementation, not at release time.

Illustrative policy:

```text
default-src 'self';
script-src 'self';
style-src 'self' 'nonce-<per-launch>';
img-src 'self' data: asset:;
font-src 'self';
connect-src 'self' ipc:;
object-src 'none';
frame-ancestors 'none';
base-uri 'self';
form-action 'none';
```

The exact asset/IPC sources follow the pinned Tauri version.

- Use a per-launch nonce for Griffel style elements where required.
- Do not solve CSP failures with broad `unsafe-inline` without an ADR.
- Test a production/release build with CSP active in CI.
- Prevent duplicate Griffel runtime copies that bypass the configured renderer.
- Keep dangerous remote-domain IPC disabled.

### 12.3 Network policy

Version 1 is network-denied by default:

- All UI assets are bundled locally.
- Privileged webviews do not execute arbitrary remote code or HTML.
- A product may define named HTTPS endpoints through reviewed Rust or trusted Python services.
- Endpoint allowlists, timeout, certificate, proxy, and privacy behavior are documented per product.
- Update checking is a separate reviewed native network path.
- Telemetry is disabled by default.

### 12.4 Python supply chain

- Commit a hash-pinned dependency lock.
- No runtime package installation.
- No loading executable code from user-controlled paths.
- Record backend dependency/build metadata in the bundle and handshake.
- Generate a dependency/license inventory and SBOM for release artifacts.
- Build and sign from protected CI release environments.

### 12.5 Sidecar process security

- Start the exact bundled resource.
- Canonicalize and verify resource containment.
- Construct a minimal environment.
- Restrict working directories and operation inputs.
- Rate-limit restart loops.
- Capture only structured, bounded logs.
- Terminate the process tree on host shutdown.
- Treat optional OS sandboxing as a separate product decision; do not claim the sidecar is sandboxed by default.

---

## 13. Errors, logging, diagnostics, and privacy

### 13.1 Error taxonomy

```ts
type AppErrorCode =
  | "VALIDATION_ERROR"
  | "NOT_FOUND"
  | "CONFLICT"
  | "PERMISSION_DENIED"
  | "BACKEND_UNAVAILABLE"
  | "BACKEND_CRASHED"
  | "BACKEND_PROTOCOL_MISMATCH"
  | "PROTOCOL_ERROR"
  | "RESOURCE_EXHAUSTED"
  | "TIMEOUT"
  | "CANCELLED"
  | "INTERRUPTED"
  | "IO_ERROR"
  | "INTERNAL_ERROR";
```

Every user-facing error provides:

- Stable safe error code
- Plain-language message
- Recovery action where available
- Trace ID
- Technical details only in diagnostics

### 13.2 Structured logs

Allowed core fields:

```text
timestamp
level
component
event
traceId
requestId
taskId
durationMs
errorCode
buildId
```

Payloads, document contents, raw user input, credentials, authorization headers, environment dumps, and full paths are not logged by default.

### 13.3 Diagnostics

Diagnostics may show:

- App/Rust/Python versions and build IDs
- OS, CPU, webview, window system, and package type
- Theme/material/accent capability state
- Secret-storage capability
- Backend state and restart count
- Recent sanitized errors
- Log and data locations through narrow actions
- Contract/schema hash
- Supported backend operations

Default policy:

- Local logs only
- Seven-day and size-bounded retention, configurable by product policy
- Restrictive file permissions where supported
- Preview manifest before diagnostics export
- Export to a user-selected local path
- No upload without a separate privacy/consent decision

### 13.4 Crash behavior

- Rust panic hook writes bounded structured evidence.
- Webview failure shows a recovery/reload surface where the platform allows detection.
- Sidecar exit follows the lifecycle policy and never replays work.
- Remote crash reporting is absent by default.
- A product that adds crash upload requires a privacy ADR and explicit consent policy.

---

## 14. Testing strategy

### 14.1 Ownership by test layer

| Layer | Tool and purpose |
|---|---|
| React unit/component | Vitest, React Testing Library, axe |
| Renderer-only journeys | Optional Playwright or WebdriverIO browser mode with mocked IPC |
| Rust | `cargo test`, process/protocol integration fixtures |
| Python | pytest |
| Cross-language contract | Shared valid/invalid fixtures |
| Native Tauri E2E | WebdriverIO + `@wdio/tauri-service` |
| Installed artifact | Clean-machine package smoke tests |
| Platform behavior | Automated where possible plus versioned manual evidence |

The embedded WebDriver/testing plugin must not be present in production release artifacts.

### 14.2 Protocol and fault tests

Include:

- Fragmented reads and multiple frames per read
- Unicode and CRLF
- Invalid UTF-8/JSON
- Unknown kind and operation
- Boundary and oversized messages
- Slow consumer and bounded queue
- stderr flood while stdout continues
- Cancel before accept, during run, and after success
- Timeout while work continues
- Crash before handshake, during work, and after simulated side effect
- Repeated crash circuit breaker
- App close during active work
- No surviving child process
- Schema/protocol mismatch
- Stale sidecar version detection

### 14.3 Shell and accessibility tests

Include:

- Light/dark/system before first paint
- Theme switching
- Responsive states
- Mouse and keyboard splitters
- Focus restoration
- Forced colors
- Reduced motion/transparency
- 200% scaling/text zoom
- Mixed-DPI multi-monitor movement
- Title-bar/system-menu behavior
- Screen-reader pass
- RTL and 30–50% text expansion readiness

### 14.4 Cross-engine visual validation

- Storybook/Chromium validates component structure and token application.
- A small shell gallery is captured on Windows WebView2, macOS WKWebView, and Linux WebKitGTK.
- Use per-platform baselines and tolerance; visual equivalence is required, not identical pixels.
- No engine-specific CSS without a documented fallback.

### 14.5 CI gates

Pull requests:

- Formatting/lint
- Type checking
- Unit tests
- Contract fixtures
- Token/contrast checks
- Rust and Python tests
- Smoke build on declared targets

Release candidates:

- Native E2E
- Installed-artifact smoke
- Sidecar lifecycle/fault suite
- Signature/notarization verification
- Update-path verification
- SBOM/license output
- Manual platform evidence for non-automatable window/accessibility behavior

---

## 15. Repository layout

Start small and extract packages only when justified.

```text
generic-fluent-app/
├── apps/
│   └── desktop/
│       ├── src/
│       │   ├── app/
│       │   ├── shell/
│       │   ├── features/
│       │   ├── routes/
│       │   └── state/
│       ├── src-tauri/
│       │   ├── src/
│       │   │   ├── backend/
│       │   │   ├── commands/
│       │   │   ├── settings/
│       │   │   ├── security/
│       │   │   ├── platform/
│       │   │   └── lib.rs
│       │   ├── capabilities/
│       │   └── tauri.conf.json
│       └── package.json
├── packages/
│   ├── app-contracts/
│   └── design-tokens/
├── services/
│   └── python-backend/
├── tests/
│   ├── contract/
│   ├── e2e/
│   └── platform-smoke/
├── docs/
│   ├── architecture.md
│   ├── support-matrix.md
│   ├── threat-model.md
│   ├── release.md
│   └── adrs/
├── scripts/
│   ├── build-sidecar/
│   ├── package/
│   └── verify/
├── .github/workflows/
├── rust-toolchain.toml
├── pnpm-workspace.yaml
├── package.json
├── SECURITY.md
├── CODEOWNERS
└── THIRD-PARTY-NOTICES.md
```

Defer:

- `packages/ui`
- `packages/module-sdk`
- Broad shared `test-utils`
- Module migrations
- App generator

These become candidates during template extraction after a second consumer exists.

---

## 16. Architecture decision records

Create only decisions required for the current phase.

Initial ADRs:

1. Tauri 2 + React/Fluent instead of Electron
2. Rust and Python trust/privilege model
3. Python sidecar packaging candidate and spike evidence
4. Bounded JSON Lines protocol and JSON Schema authority
5. Platform-adaptive title-bar strategy
6. Supported target and distribution matrix
7. State/data ownership and single-writer policy
8. Security, CSP, and network-denied default
9. Signing, updater-key custody, and release channels
10. Diagnostics and privacy policy

Feature/module SDK decisions are written after the reference feature.

---

## 17. Development roadmap

Estimates are engineering working days for one experienced developer with AI-assisted review. External signing/account lead time is tracked separately.

### Phase 0A — Product constraints and risk definition
**Estimate:** 2–3 days

Deliverables:

- First representative product/workload statement
- Data classification
- Threat model
- Exact support/build/test matrix
- Installer and updater intent
- Network policy
- Signing account/certificate feasibility
- Spike measurement plan
- Initial ADRs 1–5

Exit:

- No unknown can invalidate Tauri + sidecar as the selected model.
- Every target/package has a test owner.
- Expected Python workload and dependency profile are recorded.

### Phase 0B — Cross-platform technical spike
**Estimate:** 7–12 days

Build only:

- Minimal Tauri/React/Fluent multi-pane shell
- Platform title-bar candidates with native fallback
- PyInstaller `onedir` sidecar
- Handshake
- Typed echo
- Ordered progress
- Cancellation
- Deliberate hang/crash/oversized frame
- Process-tree shutdown
- Minimal structured logs
- Unsigned/test-signed packages
- Native WebdriverIO smoke

Exit:

- Packaged app and sidecar launch without external Python.
- Shared fixtures pass in all languages.
- Progress remains responsive and bounded.
- Cancellation and terminal-race semantics pass.
- Crash fails in-flight work, bounded restart succeeds, and no work is replayed.
- Closing the host leaves no child process.
- Unsupported custom title-bar behavior uses native fallback.
- Installable CI artifacts and measurements are retained.

### Phase 1 — Architecture baseline and repository hardening
**Estimate:** 3–5 days

- Apply spike evidence to this architecture.
- Complete required ADRs.
- Pin toolchains and lockfiles.
- Establish contract generation/fixture checks.
- Establish CSP and capability baseline.
- Create contribution and release rules.
- Establish minimal CI.

Exit:

- Architecture claims match measured evidence.
- Schema, capability, lint, type, Rust, Python, and smoke-build gates pass.

### Phase 2 — Design system and application shell
**Estimate:** 8–12 days

- Fluent provider
- Light/dark/system/forced-colors
- Semantic tokens and snapshots
- Accent/material capability adapter
- Shell regions and responsive states
- Keyboard-resizable panes
- Selected title-bar adapters
- In-window settings shell
- Layout persistence
- Stories only for complex reusable shell components

Exit:

- Reference screenshot structures are composable.
- Palette and contrast matrix pass.
- Theme is correct before first visible frame.
- Keyboard, screen-reader, scaling, reduced-motion, forced-colors, and focus checks pass.
- Shell degrades to the 500 px compact state.

### Phase 3 — Backend and task infrastructure
**Estimate:** 6–10 days

- Productize contract schemas
- Rust operation registry
- Sidecar lifecycle state machine
- Bounded framing and queues
- Progress channels
- Timeout/cancel/crash/circuit breaker
- Intent-centric file/artifact handling
- Structured logs and trace correlation
- Per-platform packaging scripts

Exit:

- Fault-injection suite passes on every supported target.
- No orphan, replay, unbounded buffer, or protocol/log mixing occurs.
- Error and recovery behavior is documented.

### Phase 4 — Reference feature
**Estimate:** 5–8 days

Recommended product-neutral feature: synthetic local document analysis.

It exercises:

- Sidebar items
- Workspace content/status
- Inspector metadata
- Bottom task/output panel
- Search
- One Python analysis job
- Progress/cancel/recovery
- Settings contribution
- Empty/loading/error states

Exit:

- Feature uses only approved UI/native/backend boundaries.
- Performance budgets are measured.
- Architecture friction is recorded before extraction.

### Phase 5 — Extract feature and command contract
**Estimate:** 3–5 days

- Extract only extension points used by the reference feature.
- Validate route, command, ID, and shortcut conflicts.
- Add a second tiny feature.
- Confirm Rust authorization remains separate from feature metadata.

Exit:

- Both features register without shell edits.
- No false module security boundary is claimed.
- Public package extraction is justified by a real second consumer.

### Phase 6 — Settings, persistence, diagnostics, and recovery
**Estimate:** 5–8 days

- Searchable settings
- Atomic persistence and migrations
- Single-instance handling
- Diagnostics preview/export
- Log rotation/privacy tests
- Reset and repair workflows

Exit:

- Corrupt settings recover without destroying unrelated sections.
- Diagnostics include only allowlisted information.
- Migration and compatibility policy is tested.

### Phase 7 — Release engineering and hardening
**Estimate:** 10–20 days plus external lead time

Feasibility starts in Phase 0; this phase completes:

- Production capabilities and CSP
- Signed/notarized installers
- Updater artifacts and channels
- Updater private-key custody, backup, and rotation
- Publish authorization
- Dependency/license/SBOM output
- Installed-artifact E2E
- Performance/security review
- Backup/migration/update recovery

Exit:

- Clean-machine installs pass.
- App code signing and update signatures are independently verified.
- Stable/beta channels cannot cross accidentally.
- Previous installer and repair path are documented.
- No unresolved critical/high security finding remains.

### Phase 8 — Template extraction
**Estimate:** 4–6 days

- Extract justified UI and feature packages
- Remove reference product names
- Template configuration
- Second branded sample
- Generator or documented copy workflow
- Branding and conformance guides

Exit:

- A second application is created without modifying core shell internals.
- Branding, features, and Python domain logic can be replaced independently.
- Build/release documentation is complete.

---

## 18. Performance and reliability budgets

Spike targets are initial measurements, not universal product service levels.

| Metric | Initial target |
|---|---|
| Themed window visible | ≤ 700 ms on representative developer hardware |
| Shell interactive | ≤ 2 seconds |
| `onedir` sidecar handshake | ≤ 3 seconds cold |
| Theme switch | ≤ 100 ms to stable paint |
| UI progress delivery | ≤ 10 updates/second/task |
| Cancellation acknowledgement | ≤ 250 ms |
| Cooperative stop target | ≤ 2 seconds |
| Panel resizing | No visible event backlog; target ≥ 50 fps |
| Orphan process after forced close | Zero |
| Protocol queue/frame growth | Bounded by declared limits |

Record package size, idle memory, sidecar memory, cold/warm startup, and platform deviations during the spike. Product-specific budgets are established after the reference feature.

---

## 19. Release and update policy

### 19.1 Early feasibility

Start during Phase 0:

- Apple Developer enrollment/identity feasibility
- Windows signing mechanism feasibility
- Test-signed pipeline
- Initial installer formats
- Update public key inclusion strategy
- External lead-time tracking

A real production notarization credential is not required to complete architecture writing, but successful signing/notarization is mandatory before a public stable macOS release.

### 19.2 Update security

Document:

- Private key storage
- Offline backup
- Access control
- Rotation procedure through a release signed by the old key
- Stable/beta endpoints
- Kill switch or manifest withdrawal strategy
- Previous installer retention
- Data migration compatibility and downgrade policy

Do not enable user-visible automatic updating until the signed path has been tested end-to-end.

---

## 20. Version 1 non-goals

- Mobile targets
- Runtime third-party plugins
- Untrusted remote content in privileged webviews
- Generic shell execution
- Arbitrary filesystem access from React
- Multiple Python workers without workload evidence
- Durable resume of interrupted tasks
- Automatic replay after backend crash
- Cross-device synchronization
- Multiple workspaces without product need
- Separate settings window without product need
- Custom Linux title bar
- Identical compositor behavior across operating systems
- Universal UI library independent of this application
- Store distribution without a dedicated packaging/security decision

---

## 21. Open product decisions and defaults

| Decision | Default until product evidence changes it |
|---|---|
| First reference feature | Synthetic local document-analysis job |
| Settings | In-window route/dialog |
| Linux title bar | Native |
| Windows title bar | Custom candidate with native fallback |
| Workspaces | No workspace or one active workspace |
| Python workload | Short commands + one cancellable long CPU task; no GPU |
| Sidecar start | Lazy |
| Sidecar concurrency | One active long task |
| Runtime plugins | None |
| Network | Denied by default; named native endpoints only |
| Telemetry | None |
| Crash upload | None |
| Database encryption | Only after data classification requires it |
| Python bundle | `onedir` first |
| Contract authority | JSON Schema + shared fixtures |
| Native E2E | WebdriverIO Tauri service |
| Interrupted tasks | Not durable; no replay |
| Multiple app instances | Single instance |
| User file handling | Intent API + opaque references |
| Update | Plumbing early; enable before stable release if product policy allows |

Still requiring product input before Phase 0B is finalized:

- Exact first product/workload
- Data sensitivity
- Required CPU architectures
- Linux support breadth
- GPU/native scientific dependencies
- Enterprise deployment
- Mandatory installer formats
- Managed/offline update policy

---

## 22. Architecture acceptance criteria

The architecture is proven when the template can:

- Launch packaged artifacts on every declared target
- Require no separate Python installation
- Render the screenshot-inspired multi-pane shell
- Switch system/light/dark without restart or launch flash
- Match approved semantic palette and accent roles
- Use forced colors and reduced motion correctly
- Degrade from four panes to a compact 500 px shell
- Preserve native title-bar behavior or use native fallback
- Persist layout through Rust with migration/recovery
- Register two static features without shell edits
- Enforce backend operations in Rust
- Pass shared contracts across TypeScript, Rust, and Python
- Stream bounded ordered progress through a Tauri channel
- Cancel according to documented race semantics
- Fail and restart the sidecar without replay or orphan processes
- Render imported content safely
- Navigate fully by keyboard
- Pass WCAG 2.2 AA baseline and native accessibility checks
- Produce installed-artifact E2E evidence
- Build signed/notarized/releasable artifacts
- Verify a signed update path
- Create a second branded application without changing shell internals

---

## 23. Consolidation decision log

### Accepted directly

- Tauri/React/Fluent/Rust/Python core stack
- Python sidecar described as trusted, not sandboxed
- Rust compile-time operation registry
- JSON Schema and fixtures as initial contract authority
- Bounded JSON Lines transport
- Explicit request/task/cancel/crash semantics
- Process-tree containment
- WebdriverIO Tauri service for native E2E
- Cross-engine visual validation
- Platform-adaptive title bar with native fallback
- Exact support matrix requirement
- Single-writer persistence
- In-window settings
- Network denied by default
- Early signing/updater feasibility
- Diagnostics allowlist/privacy contract
- WCAG 2.2 AA target
- Reference feature before module SDK extraction

### Accepted with simplification

- PyInstaller `onedir` is the first candidate, not a permanent universal mandate.
- One active long task and sidecar restart escalation replace an immediate multi-process worker pool.
- Storybook is selective rather than a requirement for every component.
- JSON Schema generation may begin with shared fixtures and idiomatic Rust/Python models.
- Release pipeline feasibility starts early, but real notarization is a pre-release gate rather than a Phase 0 documentation gate.
- Exact raw palette values remain in the architecture baseline but are owned operationally by the token package.

### Deferred

- Runtime third-party plugins
- Public module SDK
- Multiple workspaces
- Multiple Python workers
- Durable task resume
- GPU-specific runtime
- Separate settings window
- Remote crash reporting
- Enterprise installer/policy system
- Database encryption without classified data
- Broad Linux distribution and package matrix
- Store distribution

### Rejected

- The claim that PyInstaller `onefile` is universally incompatible with macOS signing/notarization
- Treating every release concern as a Phase 0 blocker
- Requiring production notarization credentials before architecture work can proceed
- The claim that CRLF inherently corrupts JSON Lines
- Automatically introducing a child worker process for every long task before workload evidence
- Treating React feature metadata as a security boundary
- Designing a broad module SDK before a reference feature
- Generic enterprise rollout infrastructure as a mandatory core-template feature

---

## 24. Change log from v0.1.1

### Added

- Explicit trust model
- Initial support/distribution matrix
- Native title-bar promotion gates and fallback
- Correct accent/on-accent/focus token roles
- Forced-colors strategy
- Smaller feature definition
- Rust-owned settings and single-instance policy
- Contract authority and schema hash
- Bounded framing, queues, progress, and artifact path
- Precise task/cancel/crash semantics
- Sidecar lifecycle and process-tree containment
- Intent-centric file APIs
- CSP and network policy
- Diagnostics privacy contract
- Native Tauri E2E strategy
- Cross-engine validation
- Risk-first spike and revised roadmap
- Performance budgets
- Non-goals and consolidation decision log

### Changed

- Playwright is optional browser-only testing rather than packaged-app authority
- PyInstaller `onedir` is the first packaging candidate
- Custom title bar is conditional rather than assumed
- Settings default to in-window
- Module capabilities renamed to non-security required operations
- Module extraction occurs after a reference feature
- Sidecar recovery fails in-flight work and never replays
- Accessibility target raised to WCAG 2.2 AA
- Release feasibility begins early
- Repository starts with fewer packages

### Removed

- Ambiguous `accentForeground`
- Universal `Paused` task state
- Frontend timestamp requirement
- False per-module security claim
- Premature `module-sdk`, `packages/ui`, and module migrations
- Automatic crash recovery implication
- Broad “latest OS” support language

### Remaining risks

- Actual first-product workload and data classification
- PyInstaller bundle behavior with future native/GPU dependencies
- Windows custom-title-bar native integration
- macOS signing/notarization operations
- Linux webview/compositor variation
- Custom accent accessibility
- Update-key continuity and data migration rollback

---

## 25. Immediate next action

Run **Phase 0A**, then implement the **Phase 0B cross-platform technical spike only**. Do not begin the reusable module SDK or real product features until the spike report has validated the title bar, sidecar packaging, bounded protocol, cancellation, process-tree shutdown, native E2E, and target build matrix.
