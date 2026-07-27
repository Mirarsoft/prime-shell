# ADR-0001: Tauri 2 + React/Fluent Stack

**Status:** Accepted for Phase 0A/0B validation
**Project:** `prime-shell`  
**Repository:** `prime-shell`

## Context

The project needs a reusable cross-platform desktop foundation with a Fluent 2 visual direction, native packaging/window integration, a least-privilege webview boundary, and a small-team implementation model. The risky assumptions must be proven before broader shell or product work.

## Decision

Use React + TypeScript + Vite with Fluent UI React v9 in a Tauri 2 desktop host. Rust owns native policy, capabilities, window behavior, and process lifecycle. Phase 0B will implement only a minimal themed window and the narrow sidecar spike.

## Alternatives considered

- Electron: reconsider only if a hard requirement needs identical Chromium behavior or Node-only APIs that cannot be cleanly brokered through Tauri.
- Platform-native UI stacks: rejected for the initial reusable cross-platform foundation because they fragment the UI implementation and Fluent component strategy.
- Browser/PWA only: rejected because packaged sidecar lifecycle, native window behavior, and distribution are core requirements.

## Consequences

- Smaller native host footprint is plausible but must be measured.
- UI rendering differs across WebView2, WKWebView, and WebKitGTK; equivalence, not pixel identity, is required.
- Rust capability/command design becomes a primary security boundary.
- Toolchains and dependencies must be pinned and built on each target.
- Electron remains a contingency, not parallel scope.

## Spike evidence still required

- Minimal themed Fluent window launches on every declared runtime target.
- Release CSP preserves Fluent/Griffel rendering.
- WebdriverIO can drive the real Tauri application.
- Target builds/packages succeed with retained tool/version evidence.
- Startup, memory, package size, and cross-engine deviations are recorded.

## Revisit trigger

Revisit only if Phase 0B shows a declared target cannot support the required host/webview behavior, native E2E, CSP rendering, or packaged sidecar model without a material workaround, or a new hard product requirement requires Chromium/Node-specific behavior.
