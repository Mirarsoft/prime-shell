# ADR-0004: Bounded JSON Lines Contract

**Status:** Accepted for Phase 0A/0B validation
**Project:** `prime-shell`  
**Repository:** `prime-shell`

## Context

Rust and Python need a simple inspectable local protocol that supports handshake, typed requests, progress, cancellation, errors, and terminal events without unbounded buffering or cross-language contract drift.

## Decision

Use UTF-8 JSON Lines over sidecar stdin/stdout with JSON Schema 2020-12 and shared valid/invalid fixtures as contract authority.

- Protocol-only stdout; structured bounded logs on stderr.
- 64 KiB handshake, 1 MiB normal frame, and 64 KiB log-line limit.
- Initial maximum 64 pending requests and 256 backend events.
- UI progress coalesced to at most 10 events/second/task; terminal events are retained.
- CRLF is accepted.
- A schema-bundle hash, protocol range, build identity, target, and supported operations are checked at handshake.
- Rust rejects unknown operations and mints authoritative trace IDs.
- Invalid encoding/JSON/kinds/operations, oversize frames, and corruption fail deterministically.
- No automatic retry/replay unless an operation is explicitly proven idempotent; uncertain work is never replayed.

## Alternatives considered

- Unbounded ad hoc JSON/stdout: rejected due allocation, corruption, and drift risk.
- gRPC/Protobuf or another IDL immediately: deferred as unnecessary complexity for the local spike.
- Tauri feature metadata as authorization: rejected; Rust registry remains authoritative.
- Inline large/binary payloads: rejected; future large data uses opaque artifact handles.

## Consequences

- Line framing remains simple and testable but requires strict stdout discipline.
- Schema/fixture conformance must run in every language.
- Handwritten Rust/Python models are allowed initially only while fixtures remain authoritative.
- Backpressure and terminal-race semantics become explicit implementation duties.

## Spike evidence still required

- Unicode, CRLF, fragmented/multiple frames, invalid UTF-8/JSON, unknown kinds/operations.
- Boundary/oversized frames, slow consumer, queue limits, stderr flood.
- Handshake/version/schema mismatch and stale-sidecar rejection.
- Ordered/coalesced progress, cancellation races, terminal preservation.
- Crash corruption behavior and no-replay evidence.

## Revisit trigger

Revisit if shared schemas/fixtures cannot express or maintain required contracts, measured performance is inadequate, or product data requires a binary/artifact transport beyond opaque handles. A larger IDL requires evidence, not preference.
