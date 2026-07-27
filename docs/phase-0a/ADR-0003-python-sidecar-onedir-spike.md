# ADR-0003: PyInstaller `onedir` Sidecar Spike

**Status:** Accepted for Phase 0A/0B validation
**Project:** `prime-shell`  
**Repository:** `prime-shell`

## Context

Users must not install Python separately. The first packaging candidate must work across Windows 11 x64, macOS arm64, and Ubuntu 24.04 x64 and remain inspectable for dependency, signing, startup, cleanup, and clean-install evidence.

## Decision

Build the Python sidecar separately on each target OS/architecture with pinned Python/PyInstaller versions using PyInstaller `onedir`. Bundle the complete directory as a Tauri resource and let Rust resolve and launch the contained executable.

## Alternatives considered

- PyInstaller `onefile`: retained as a measured future alternative, but not first because extraction, startup, endpoint protection, cleanup, native library, and signing behavior add uncertainty.
- Require system Python/virtual environment: rejected because supported releases must be self-contained.
- Multiple Python workers: deferred; no workload evidence requires them.
- Other freezing tools: deferred unless `onedir` fails a declared target or hard dependency.

## Consequences

- Artifacts contain multiple files and may be larger/more visible to users.
- Per-target builds and nested-binary signing/notarization planning are required.
- Inspection, dependency diagnosis, and clean-machine testing are simpler.
- The result validates only the standard-library-first CPU spike profile.

## Spike evidence still required

- Clean launch without external Python on every target.
- Correct target architecture and resource resolution.
- Cold handshake time, bundle/package size, idle/active memory.
- Normal/forced-close cleanup and antivirus/Gatekeeper limitations.
- macOS nested-code signing order/layout feasibility.
- `.deb`, NSIS candidate, and `.app` resource integration.

## Revisit trigger

Revisit if `onedir` cannot be packaged/launched/signed on a declared target, package/startup costs are unacceptable, or future native/GPU/model dependencies materially change bundle behavior. Do not switch solely for cosmetic single-file preference.
