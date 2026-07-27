# AGENTS.md — Generic Fluent Desktop Project Execution Contract

## 1. Mandatory first action

Before executing **any** project prompt, task, work package, review, implementation, refactor, test change, packaging change, or documentation change:

1. Read this entire `AGENTS.md`.
2. Read `Core-Functionality-DeliveryRules.md`.
3. Read the project authority files in this exact order:
   1. `generic-fluent-desktop-project-handoff.md`
   2. `generic-fluent-desktop-app-architecture-v0.2.md`
   3. `architecture-review-consolidation-decision-log.md`
4. Inspect the current task prompt.
5. Confirm in the first progress message:

   > `AGENTS.md and the required authority files have been read and are active for this task.`

If any required file is unavailable, unreadable, or materially inconsistent with the task, stop before changing files and report the exact missing or conflicting item.

Do not rely on memory or summaries when the files are available.

---

## 2. Authority and conflict order

For project architecture and scope decisions, use:

1. The current explicit user/task prompt
2. `generic-fluent-desktop-project-handoff.md`
3. `generic-fluent-desktop-app-architecture-v0.2.md`
4. `architecture-review-consolidation-decision-log.md`

For execution discipline and delivery quality, this file and `Core-Functionality-DeliveryRules.md` are mandatory.

`Core-Functionality-DeliveryRules.md` specifically prohibits fake implementation, unnecessary complexity, inflated test suites, misleading completion claims, and documentation being presented as working functionality.

This file does not authorize overriding frozen architecture decisions. It governs how work is performed and delivered.

---

## 3. Scope discipline

Execute the current prompt precisely.

Do not:

- Begin a later phase.
- Add adjacent features because they appear useful.
- redesign settled architecture without concrete evidence.
- create speculative abstractions, frameworks, packages, or services.
- add a public module SDK before the reference feature and second consumer justify it.
- add `packages/ui` without a real second consumer.
- add runtime plugins, multiple Python workers, durable tasks, GPU orchestration, a production updater, telemetry, enterprise deployment, or multiple package formats unless explicitly required by the current approved milestone.
- perform broad cleanup or unrelated refactoring.
- silently change file formats, naming, or delivery structure required by the prompt.
- substitute planning, interfaces, mocks, fixtures, or tests for a working execution path.

When two safe implementations satisfy the task, choose the simpler and more direct one.

---

## 4. Quality standard

Produce high-quality, readable, maintainable work appropriate to the current phase.

For code:

- Implement the real execution path.
- Prefer explicit typed boundaries.
- Validate inputs at trust boundaries.
- Handle important failure paths.
- Keep functions and modules cohesive.
- Avoid premature generalization.
- Pin or lock dependencies as required by the architecture.
- Do not leave hidden stubs, placeholder returns, mock-only production paths, or unresolved `TODO` logic in claimed-complete functionality.
- Do not claim success without executing the relevant commands.
- Keep security boundaries consistent with the architecture:
  - React is least trusted.
  - Rust is the native policy and process authority.
  - Python is trusted native code, not a sandbox.
  - Unknown backend operations are rejected in Rust.
  - Backend failure never automatically replays uncertain work.

For documentation:

- Record decisions, constraints, evidence, commands, and unresolved items.
- Keep it concise and operational.
- Do not write speculative enterprise policy.
- Clearly distinguish facts, defaults, assumptions, evidence, and decisions.

For tests:

- Protect core behavior, critical failure paths, important integration boundaries, and one small end-to-end smoke path.
- Keep test volume proportional.
- Do not add low-value tests to inflate apparent progress.
- Do not validate only mocks or stubs and present that as functional evidence.

---

## 5. Truthful status rules

Use only these implementation-status labels when reporting work:

- `Implemented`
- `Partially implemented`
- `Stub`
- `Mock-only`
- `Not started`
- `Blocked`

A milestone is not complete unless the promised behavior can be run and verified without hidden stubs, hard-coded success, mock-only paths, or future work.

Every completion report must identify:

- Real production or spike functionality
- Supporting infrastructure
- Tests
- Documentation
- Generated code
- Fixtures/mocks
- Stubs/placeholders
- Incomplete or blocked work

Never describe scaffolding or documentation as implemented functionality.

---

## 6. Required execution workflow

For every task:

### A. Preflight

- Read the mandatory files.
- Inspect the repository and current branch/state when a repository exists.
- Confirm the exact requested scope.
- Identify existing relevant files before creating new ones.
- Do not ask questions already answered by the authority files or current prompt.
- Use safe defaults when explicitly authorized.
- Ask only when a missing fact would materially change the implementation or invalidate the task.

### B. Work

- Follow the prompt’s ordered steps.
- Build the smallest complete vertical slice required by the milestone.
- Run verification continuously.
- Keep changes limited to the approved scope.
- Maintain a concise change inventory while working.

### C. Verification

At minimum, provide as applicable:

- Exact run command
- Exact test command
- Exact build/package command
- Short manual verification procedure
- Expected real behavior
- Actual observed result
- Any platform limitation or unverified claim

Do not invent execution results. If a command cannot run in the environment, state that clearly and provide the exact reason.

### D. Delivery

Produce all explicitly requested files.

All reports, plans, ADRs, manifests, handoffs, reviews, and prompts must be Markdown (`.md`).

When source code is requested:

- Deliver a **complete authoritative repository snapshot**, not an incremental overlay, unless the prompt explicitly requests a patch.
- Package the snapshot as an archive.
- Instruct the user to extract it into a fresh empty directory.
- Include a Markdown handoff manifest listing the archive name, hash, branch/commit if applicable, implemented scope, verification evidence, and known limitations.

---

## 7. Cross-session handoff protocol

The project uses two cooperating conversations:

- **Chat Session:** architecture control, prompt drafting, review, scope decisions, and quality gatekeeping.
- **Work Session:** execution, file creation, implementation, tests, packaging, and evidence gathering.

The sessions do not reliably share a live filesystem or hidden context. Therefore, use explicit file handoffs.

### Work Session → Chat Session

At the end of every assigned task, the Work Session must:

1. Create all required Markdown reports.
2. Create `work-session-handoff-manifest.md` containing:
   - Task name and phase
   - Exact files produced
   - Purpose of each file
   - SHA-256 hash of each deliverable where possible
   - Repository branch and commit, if applicable
   - Commands executed
   - Verification results
   - Status inventory
   - Known limitations and unresolved items
   - Exact recommended next controlled action
3. Provide individual downloadable links.
4. If source code exists, provide one complete authoritative snapshot archive and its hash.
5. Expose the files to the shared Project/File Library when the interface supports it.
6. If direct project exposure is unavailable, provide download links so the user can upload the files to the Chat Session.

### Chat Session → Work Session

After review, the Chat Session should produce a Markdown review or next-phase prompt with:

- Accepted outputs
- Required corrections
- Scope decisions
- Exact next task
- Required input filenames
- Required output filenames

The user then uploads or exposes that Markdown file to the Work Session.

Do not assume the other session can see a file until it is explicitly attached, uploaded to the project, or provided through a downloadable artifact.

---

## 8. Required reporting format

Unless the current prompt specifies a stricter format, finish with:

### Scope executed

### Files created or changed

### Functional status inventory

### Commands executed

### Verification results

### Incomplete or blocked items

### Scope exclusions respected

### Deliverables and download links

### Next controlled action

Keep conversational text concise. Put durable detail in the requested Markdown files.

---

## 9. Stop conditions

Stop and report instead of improvising when:

- A required authority file is missing.
- The prompt requests a direct violation of a frozen decision without acknowledging a new hard requirement or spike evidence.
- Required credentials or unavailable hardware make a claimed verification impossible.
- Continuing would require entering a later phase.
- The only possible result would be fake, stubbed, or mock-only functionality presented as complete.
- Existing repository state is unsafe to overwrite and the prompt does not authorize replacement.

A stop report must identify the blocker and the smallest safe next action.

---

## 10. Permanent project principle

Build a working, polished, secure-enough cross-platform desktop foundation without turning it into a speculative framework or enterprise platform.

Working functionality and measured evidence take priority over architecture theater, documentation volume, test counts, and apparent progress.
