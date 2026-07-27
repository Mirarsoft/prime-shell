# ADR-0005: Platform-Adaptive Title Bar

**Status:** Accepted for Phase 0A/0B validation
**Project:** `prime-shell`  
**Repository:** `prime-shell`

## Context

The application should reflect Fluent/Windows 11 styling without sacrificing platform-native window reliability, accessibility, resizing, system menus, Snap Layouts, traffic lights, or Linux compositor compatibility.

## Decision

Use a `TitleBarAdapter` concept with native fallback:

- Windows 11: custom Fluent title bar is a spike candidate only. Promote it only if native drag, resize, system menu, Snap Layout/`Win+Z`, keyboard, accessibility, DPI, restore, and forced-colors behavior pass.
- macOS: retain native traffic lights, using overlay/transparent treatment only when movement, full screen, focus, theme background, and safe-area behavior remain correct.
- Ubuntu: use native decorations on Wayland and X11. Custom Linux chrome is outside the spike.

If the Windows candidate fails, use native decorations with a styled in-app header.

## Alternatives considered

- One identical custom title bar everywhere: rejected because native semantics and compositor behavior differ.
- Native decorations everywhere: valid fallback but does not test the desired Windows Fluent candidate.
- Custom Linux decorations: rejected for the initial matrix due unsupported compositor breadth and no current product need.

## Consequences

- Platform visuals may differ while product structure remains consistent.
- Manual native evidence is required in addition to automation.
- A documented fallback can be a successful spike outcome.
- Title-bar code must not expand into a cross-platform window framework.

## Spike evidence still required

- Windows controls, drag exclusions, double-click, system menu, `Win+Z`/Snap Layouts, scale factors, mixed DPI, active/inactive, RTL/text expansion, forced colors, and accessible names/keyboard reachability.
- macOS traffic-light, full-screen, focus, background, and safe-area behavior.
- Ubuntu native decorations under Wayland and X11.
- Proof that fallback activates when promotion gates fail.

## Revisit trigger

Revisit when Phase 0B produces concrete native failure evidence, a supported platform changes its window APIs materially, or a hard product requirement mandates a different title-bar behavior. Failure of the custom candidate should normally select the existing fallback, not trigger broad redesign.
