# ADR-0002: Rust/Python Trust and Privilege Model

**Status:** Accepted for Phase 0A/0B validation
**Project:** `prime-shell`  
**Repository:** `prime-shell`

## Context

The frontend needs domain computation from packaged Python without exposing generic native capabilities. A Python process launched for the user is not sandboxed merely because it is a Tauri sidecar or PyInstaller artifact.

## Decision

Treat React as the least-trusted application layer, Rust as the native policy and process-lifecycle authority, and Python as trusted first-party native code running with current-user privileges. Rust launches the exact bundled sidecar without a shell, constructs a minimal environment, validates operation-scoped inputs, rejects unknown operations, drains bounded stdout/stderr, contains the process tree, and never automatically replays uncertain work.

## Alternatives considered

- Treat Python as sandboxed: rejected as an inaccurate security claim.
- Let React invoke arbitrary sidecar methods/files/processes: rejected because it bypasses native policy.
- Run every task in a separate worker process immediately: deferred until workload evidence requires uninterruptible/concurrent isolation.
- Remove Python and implement all computation in Rust: rejected because packaged Python domain work is a project requirement.

## Consequences

- Python dependencies and build pipeline are trusted supply-chain inputs.
- Compromise or defect in the sidecar can act with user privileges.
- Rust must maintain an explicit operation registry and lifecycle policy.
- Secrets stay in Rust unless one reviewed operation explicitly needs a value.
- Optional OS sandboxing would require a separate product decision.

## Spike evidence still required

- Exact bundled-resource launch with no external Python or shell.
- Unknown-operation rejection in Rust.
- Minimal environment and safe path/resource containment.
- Crash/hang/cancel lifecycle, one bounded restart, and no replay.
- Zero sidecar/descendant processes after normal and forced host close.
- Structured bounded logs with no raw content, environment dump, or full path by default.

## Revisit trigger

Revisit if a product must execute untrusted Python/code, requires stronger OS isolation, needs secrets inside Python, or introduces native/GPU/uninterruptible/concurrent workloads that invalidate the single trusted sidecar model.
