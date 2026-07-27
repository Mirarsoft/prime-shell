# Generic Fluent Desktop — Phase 0B Work Package 01 Execution Prompt

**Prompt ID:** `GFD-P0B-WP01-20260727T111826Z-R1`  
**Prompt artifact:** `prime-shell-chat-gfd-p0b-wp01-execution-prompt-20260727T111826Z-r1.md`  
**Authorized phase:** Phase 0B technical spike  
**Authorized work package:** WP01 — Packaged Unicode Echo Vertical Slice  
**Execution status:** Authorized  
**Later work packages:** Not authorized

## 1. Required model and intelligence

Use:

- **Model:** GPT-5.6 Sol
- **Reasoning/intelligence:** **Medium**
- **Severity:** Moderate, bounded cross-language implementation
- **Token policy:** Do not use Pro or Extra High. Do not broaden the task to justify higher reasoning.

Medium is the minimum safe setting for this prompt because it requires coordinated React/TypeScript, Tauri/Rust, Python, schema, packaging, and verification work, but the scope is limited to one real echo path.

If a problem would require redesigning a frozen boundary, stop and report it instead of consuming additional reasoning on a speculative workaround.

## 2. Mission

Create the first real, runnable Phase 0B vertical slice in a fresh repository:

```text
user enters Unicode text
→ React sends one typed Tauri request
→ Rust validates and authorizes `spike.echo`
→ Rust communicates with an exact packaged PyInstaller onedir Python sidecar
→ Python returns a validated response
→ Rust returns safe typed output
→ React displays the exact Unicode result
```

This must be a real execution path. It must not be a frontend mock, hard-coded success, stub, or direct React-to-Python shortcut.

Stop after WP01. Do not implement Phase 0B WP02 or WP03 functionality.

## 3. Mandatory inputs

Before executing, ensure these files are available:

### Execution controls

- `AGENTS.md`
- `Core-Functionality-DeliveryRules.md`

### Architecture authority, read in this order

1. `generic-fluent-desktop-project-handoff.md`
2. `generic-fluent-desktop-app-architecture-v0.2.md`
3. `architecture-review-consolidation-decision-log.md`

### Accepted Phase 0A artifacts

- `phase-0a-definition.md`
- `phase-0a-spike-plan.md`
- `ADR-0001-tauri-react-fluent-stack.md`
- `ADR-0002-rust-python-trust-model.md`
- `ADR-0003-python-sidecar-onedir-spike.md`
- `ADR-0004-bounded-json-lines-contract.md`
- `ADR-0005-platform-adaptive-title-bar.md`
- `phase-0a-readiness-report.md`
- `work-session-handoff-manifest.md`

### Chat Session authorization

- `prime-shell-chat-gfd-p0a-review-20260727T111826Z-r1.md`
- this prompt file

If any mandatory authority/control file is unavailable, stop before creating or changing source.

## 4. Mandatory first response

Before work, read `AGENTS.md`, `Core-Functionality-DeliveryRules.md`, all authority files, the accepted Phase 0A artifacts, the Chat Session review, and this prompt.

Then state:

> `AGENTS.md, delivery rules, architecture authority, accepted Phase 0A artifacts, and the WP01 authorization prompt have been read and are active. Executing GFD-P0B-WP01 with GPT-5.6 Sol / Medium only.`

Do not perform another broad architecture review.

## 5. Collision-resistant artifact naming

The Chat Session and Work Session share one File Library.

At task start, generate one UTC run identifier:

```text
RUN_ID=YYYYMMDDTHHMMSSZ
```

Use it unchanged for every deliverable exposed outside the repository.

Required external naming form:

```text
prime-shell-work-gfd-p0b-wp01-<RUN_ID>-<descriptor>-r1.<extension>
```

Examples:

```text
prime-shell-work-gfd-p0b-wp01-20260727T120000Z-source-snapshot-r1.zip
prime-shell-work-gfd-p0b-wp01-20260727T120000Z-implementation-report-r1.md
prime-shell-work-gfd-p0b-wp01-20260727T120000Z-handoff-manifest-r1.md
```

Rules:

- Do not expose generic names such as `report.md`, `handoff.md`, or `source.zip`.
- Conventional filenames **inside the repository** remain conventional (`package.json`, `Cargo.toml`, `README.md`, source filenames, and so on).
- If a deliverable is regenerated, increment the revision suffix.
- All exposed reports, reviews, inventories, and manifests must be Markdown.
- A complete source snapshot is necessarily a ZIP archive and is the only required non-Markdown deliverable.

## 6. Repository initialization

Create the implementation in a fresh empty directory.

Requirements:

1. Initialize a Git repository.
2. Use branch:

   ```text
   phase0b/wp01-packaged-echo
   ```

3. Include root `AGENTS.md`.
4. Preserve the authority/control and accepted Phase 0A documents in a clearly named documentation area without modifying their content.
5. Record the starting state as no prior implementation repository.
6. Make focused commits with meaningful messages.
7. Finish with a clean working tree.
8. The delivered ZIP must be a complete authoritative repository snapshot.
9. Instruct the user to extract it into a fresh empty directory, never overlay an older snapshot.

Do not create a monorepo abstraction beyond what this vertical slice currently consumes.

## 7. Authorized implementation scope

### 7.1 Minimal frontend

Implement only enough React + TypeScript + Vite + Fluent UI React v9 UI to prove the path:

- a minimal Fluent-themed window;
- a backend readiness indicator;
- one text input accepting Unicode;
- one `Echo` button;
- one result area;
- a concise safe error state.

No navigation framework, settings system, inspector, bottom task panel, command palette, feature SDK, design-token package, or final shell layout is required in WP01.

### 7.2 Tauri/Rust host

Implement:

- Tauri 2 native host;
- explicit capability/command permission for the one required frontend command;
- one typed frontend command for echo;
- a Rust operation enum/registry containing only the currently implemented operation;
- unknown-operation rejection before Python receives it;
- exact sidecar resource resolution;
- no shell invocation;
- deterministic working directory;
- a minimal constructed environment;
- piped stdin/stdout/stderr;
- concurrent stdout/stderr draining sufficient to prevent deadlock;
- handshake validation before accepting echo;
- stable safe error mapping to the frontend;
- bounded shutdown of the sidecar when the host exits.

Do not implement generic method tunneling, arbitrary process execution, arbitrary filesystem APIs, secrets, updater logic, settings persistence, diagnostics export, or product data.

### 7.3 Contract authority

Create the minimum JSON Schema 2020-12 authority and shared fixtures currently consumed by WP01:

- handshake/hello;
- request/response envelope;
- safe error envelope;
- `spike.echo` request and response.

Implement and enforce:

- UTF-8 JSON Lines;
- CRLF acceptance;
- protocol-only stdout;
- structured stderr;
- handshake maximum 64 KiB;
- normal frame maximum 1 MiB;
- deterministic malformed/oversized rejection;
- protocol range;
- backend version/build identity;
- target identity;
- schema-bundle hash;
- supported operation list.

Do not create schemas for future operations solely as placeholders. Future operations will be added when implemented.

Generated TypeScript validation or an equivalent pinned runtime validator is acceptable. Rust and Python may use idiomatic typed models if they both verify the same fixtures.

### 7.4 Python sidecar

Implement real Python behavior:

- emits one valid `hello` frame before requests;
- supports only `spike.echo`;
- validates the request;
- returns the exact Unicode text through a typed response;
- rejects unknown operations;
- rejects malformed or oversized input deterministically;
- writes protocol data only to stdout;
- writes bounded structured logs only to stderr;
- exits cleanly on EOF/shutdown.

No hard-coded success response is allowed. The returned text must originate from the validated request.

Use a CPU-only standard-library-first dependency profile. Do not add database, network, GPU, model, scientific, plugin, or worker-pool dependencies.

### 7.5 PyInstaller onedir

For the Work Session’s executable host platform:

- pin Python and PyInstaller;
- build the sidecar using `onedir`;
- bundle the complete sidecar directory as Tauri resources;
- prove the app or Rust integration path launches the packaged sidecar without relying on an external Python interpreter;
- record the bundle layout, size, hash, command, and observed result.

Source-mode Python execution may exist as a clearly labeled developer convenience, but it cannot be the only verified path or be reported as packaged-sidecar success.

### 7.6 CSP

Establish the release-mode CSP baseline now:

- local assets only;
- no arbitrary remote navigation/content;
- no broad CSP relaxation merely to make Fluent/Griffel work;
- exercise the minimal Fluent control styling in a release build;
- document any nonce/style handling used.

Do not build a full final security policy package in WP01.

## 8. Explicit exclusions

Do not implement:

- `spike.count`;
- progress streaming;
- cancellation;
- timeout escalation;
- `spike.crash`;
- `spike.hang`;
- restart budget/circuit breaker;
- no-replay fault demonstration;
- full process-tree containment/descendant tests;
- `spike.largeRejected` as a separate frontend operation;
- Windows custom-title-bar candidate;
- macOS overlay behavior;
- WebdriverIO;
- cross-platform package matrix;
- real document import or document analysis;
- final multi-pane shell;
- Zustand, TanStack Query, or React Router unless the current tiny UI has a concrete unavoidable need;
- settings/persistence;
- module SDK or `packages/ui`;
- runtime plugins;
- database;
- updater;
- telemetry or crash upload;
- enterprise deployment;
- production signing/notarization.

Basic malformed and oversized protocol rejection is in scope because it is necessary for a real bounded echo boundary. The dedicated fault-operation suite belongs to later work packages.

## 9. Proportional tests

Create only tests that protect the implemented path:

1. shared valid and invalid contract fixtures;
2. Python protocol tests for handshake, Unicode echo, unknown operation, malformed input, and size boundary;
3. Rust tests for operation authorization, handshake validation, frame bounds, and safe error mapping;
4. one real Rust-to-packaged-Python integration test or executable verification harness;
5. minimal frontend test for request/result/error rendering if it adds real value.

Do not add broad snapshot tests, trivial getter tests, placeholder tests, or WebdriverIO in this package.

Tests must not replace real packaged execution evidence.

## 10. Toolchain policy

At preflight:

1. Resolve mutually compatible stable versions from official documentation/tool output.
2. Pin or record:
   - Node;
   - pnpm;
   - Rust toolchain;
   - Tauri CLI/crates;
   - Python;
   - PyInstaller;
   - Fluent UI;
   - schema/runtime validation dependencies.
3. Commit lockfiles.
4. Record exact runner/host OS, architecture, and webview/build dependencies.
5. Do not perform a broad comparison of alternative tools.

If the exact macOS versions or GitHub runner labels are not used in WP01, record them as unresolved for later packaging work rather than pretending they were verified.

## 11. Required verification

Run every applicable command and record actual results.

At minimum:

### Repository

```text
git status --short
git log --oneline --decorate -n 5
git diff --check
```

### Frontend

```text
pnpm install --frozen-lockfile
pnpm typecheck
pnpm lint
pnpm test
pnpm build
```

Use the actual scripts provided by the repository; do not invent success for missing scripts.

### Rust

```text
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Run from the appropriate manifest directory/workspace.

### Python

Run the pinned test command and PyInstaller build command.

### Packaged vertical slice

Demonstrate:

- packaged `onedir` sidecar exists;
- no external Python is required by the executed integration path;
- handshake succeeds;
- Unicode echo round-trip succeeds;
- unknown operation is rejected in Rust;
- malformed/oversized input fails deterministically;
- stdout remains protocol-only;
- stderr logging does not block protocol traffic;
- sidecar exits on bounded host/test shutdown.

### Release UI

Build or run the release-mode UI path and verify Fluent rendering under the configured CSP where the environment permits.

### Archive

- verify ZIP integrity;
- reject unsafe archive paths;
- compute SHA-256;
- verify a fresh extraction;
- run at least the non-GUI validation suite from the fresh extracted snapshot.

If GUI execution is impossible in the Work Session environment, mark GUI runtime evidence `Blocked`, provide the exact reason, and still deliver build and non-GUI integration evidence. Do not claim a visible-window pass.

## 12. Truthful completion gate

WP01 may be called complete only when the real packaged Unicode echo execution path works on the available host environment.

Use the required labels:

- `Implemented`
- `Partially implemented`
- `Stub`
- `Mock-only`
- `Not started`
- `Blocked`

The report must distinguish:

- real spike functionality;
- supporting infrastructure;
- tests;
- documentation;
- generated code;
- fixtures/mocks;
- stubs/placeholders;
- later Phase 0B work.

No hidden stub, hard-coded response, mock-only production path, or unresolved `TODO` may be present in any item reported `Implemented`.

## 13. Required external deliverables

Using the single `RUN_ID`, create and expose:

1. `prime-shell-work-gfd-p0b-wp01-<RUN_ID>-source-snapshot-r1.zip`
   - complete authoritative repository snapshot;
   - clean committed state;
   - extract into a fresh empty directory.

2. `prime-shell-work-gfd-p0b-wp01-<RUN_ID>-implementation-report-r1.md`
   - scope;
   - architecture conformance;
   - actual execution path;
   - files/modules responsible;
   - commands and observed results;
   - manual verification;
   - status inventory;
   - exclusions;
   - limitations.

3. `prime-shell-work-gfd-p0b-wp01-<RUN_ID>-handoff-manifest-r1.md`
   - prompt ID;
   - model/intelligence used;
   - repository branch and commit;
   - deliverable inventory;
   - SHA-256 for every external deliverable where non-self-referential;
   - source tree/file counts;
   - commands;
   - verification;
   - incomplete/blocked items;
   - cross-session transfer instructions;
   - exact next controlled action.

4. `prime-shell-work-gfd-p0b-wp01-<RUN_ID>-review-evidence-index-r1.md`
   - concise map from each WP01 acceptance requirement to its evidence file, command, or log;
   - no duplicate narrative.

Do not expose internal repository Markdown files individually unless requested. The uniquely named source snapshot is the source authority.

## 14. Cross-session delivery

At completion:

1. Upload/expose all four uniquely named external deliverables to the shared Project/File Library when supported.
2. Provide individual download links.
3. State plainly whether each file was successfully exposed.
4. Do not assume the Chat Session can see the files until exposure/upload is confirmed.
5. Do not begin WP02.

The user will return the artifacts to the Chat Session. The Chat Session will review the snapshot and either issue a correction prompt or authorize WP02.

## 15. Completion response format

Use exactly these headings:

### WP01 result

### Model used

State:

```text
GPT-5.6 Sol / Medium
```

### Deliverables

Provide the four individual links.

### Functional status

At minimum report:

```text
Packaged Unicode echo vertical slice: <status>
Phase 0B WP02 lifecycle functionality: Not started
Phase 0B WP03 cross-platform evidence: Not started
Product feature implementation: Not started
```

### Verification

### Blocked or unverified evidence

### Cross-session transfer

### Next controlled action

End with:

> `Return the four GFD-P0B-WP01 artifacts to the Chat Session for review. Do not begin WP02 until a uniquely named Chat Session authorization prompt is supplied.`

Do not continue beyond WP01.
