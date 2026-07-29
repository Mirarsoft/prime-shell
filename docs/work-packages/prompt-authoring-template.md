# Generic Fluent Desktop Work-Package Prompt Authoring Template

**Template status:** Stage 1 foundation
**Use:** Author one provisional prompt for one package identified in
[`roadmap-index.md`](roadmap-index.md)
**Execution rule:** A provisional or approved-provisional prompt cannot execute.
Only Chat Session may issue an explicitly activated prompt.

Replace every `<PLACEHOLDER>` and remove author-only guidance before review.
Do not restructure or omit the 18 numbered sections. Select the minimum model
and reasoning level sufficient for the package risk; do not default to the
largest model.

## 1. Package identity and prompt status

**Package ID:** `<GFD-PHASE-WPnn>`
**Phase:** `<PHASE>`
**Task ID:** `<TASK_ID>`
**Prompt ID:** `<COLLISION_RESISTANT_PROMPT_ID>`
**Prompt version:** `<R1>`
**Prompt lifecycle state:** `<Draft | Provisional | Approved provisional | Activated>`
**Implementation status at authoring:** `<APPROVED_STATUS_LABEL>`
**Recommended model:** `<MODEL>`
**Reasoning/intelligence:** `<LEVEL>`
**Authorization boundary:** `<EXACTLY_ONE_PACKAGE>`

Authoring guidance:

- Use implementation status only from `Implemented`, `Partially implemented`,
  `Stub`, `Mock-only`, `Not started`, and `Blocked`.
- Prompt lifecycle state is separate from implementation status.
- State why the selected model is the minimum sufficient capacity for this
  package.
- A state other than `Activated` must say `Execution is not authorized`.

### Activation metadata

```text
Activation ID: <ACTIVATION_ID_OR_NOT_ACTIVATED>
Activated by: <CHAT_SESSION_OR_NOT_ACTIVATED>
Activation UTC: <YYYY-MM-DDTHH:MM:SSZ_OR_NOT_ACTIVATED>
Authoritative main SHA: <EXACT_40_CHARACTER_SHA_OR_NOT_ACTIVATED>
Accepted predecessor heads/evidence: <EXACT_REFS_OR_NOT_ACTIVATED>
Accepted targeted amendments: <LIST_OR_NONE>
Predecessor deviations incorporated: <LIST_OR_NONE>
Authorization expires/invalidates when: <EXACT_STATE_CHANGE_RULE>
```

## 2. Repository and exact starting state

**Repository:** `Mirarsoft/prime-shell`
**Authoritative base branch:** `main`
**Required starting commit:** `<EXACT_ACTIVATION_TIME_MAIN_SHA>`
**Required implementation branch:** `<EXACT_FRESH_BRANCH_NAME>`

Required preflight:

1. Verify exact repository access and write permission.
2. Verify the default branch and remote `main` SHA.
3. Verify every accepted predecessor ref and required file.
4. Verify the required branch does not already exist unexpectedly.
5. Start from a clean fresh clone/worktree at the exact starting commit.
6. Stop if current state differs, is ambiguous, or would require overwrite.

The merged repository is authoritative over snapshots and chat summaries.
Record exact observed refs before writing.

## 3. Mandatory authority and reading order

Read completely, in this order, before changing repository files:

1. `AGENTS.md`
2. `docs/authority/Core-Functionality-DeliveryRules.md`
3. `docs/authority/generic-fluent-desktop-project-handoff.md`
4. `docs/authority/generic-fluent-desktop-app-architecture-v0.2.md`
5. `docs/authority/architecture-review-consolidation-decision-log.md`
6. `<RELEVANT_ACCEPTED_PHASE_0A_FILES>`
7. `docs/work-packages/roadmap-index.md`
8. `docs/work-packages/shared-ground-rules.md`
9. `docs/work-packages/dependency-matrix.md`
10. `<ACCEPTED_PREDECESSOR_REPORTS_AND_AMENDMENTS>`
11. this activated execution prompt

State the required authority acknowledgement in the first progress message.
If a source is unavailable, unreadable, stale, or materially inconsistent, stop
before writing and report the exact conflict. Do not conduct a broad
architecture review.

## 4. Dependencies, predecessor outputs, and prerequisite evidence

**Direct package prerequisites:** `<PACKAGE_IDS_OR_ACCEPTED_EXTERNAL_PREDECESSOR>`
**Accepted predecessor commits:** `<EXACT_SHAS>`
**Required predecessor outputs:** `<FILES_ARTIFACTS_REPORTS>`
**Required prerequisite evidence:** `<CI_RUNS_RUNTIME_RESULTS_MANUAL_RESULTS>`
**Accepted deviations:** `<DEVIATIONS_OR_NONE>`
**Known blockers carried forward:** `<BLOCKERS_OR_NONE>`

Authoring guidance:

- Match the direct prerequisites in `dependency-matrix.md`.
- Name only evidence needed for this package.
- Never treat a build as runtime/native UX evidence.
- Never mark unavailable, mock-only, or predecessor-planned work as passed.
- If a prerequisite is not accepted, the package is `Blocked`.

## 5. Objective and measurable runnable outcome

**Objective:** `<ONE_BOUNDED_PACKAGE_OBJECTIVE>`

**Required runnable outcome:** `<USER_VISIBLE_OR_TECHNICALLY_RUNNABLE_REAL_PATH>`

Define the smallest coherent vertical slice and how an operator or test observes
success. Planning, interfaces, fixtures, documentation, or mock-only behavior
cannot substitute for the real execution path.

## 6. Explicit in-scope work

The package authorizes only:

1. `<IN_SCOPE_ITEM>`
2. `<IN_SCOPE_ITEM>`
3. `<IN_SCOPE_ITEM>`

Every item must contribute directly to the package outcome or proportional
evidence. This section authorizes one package only and grants no successor,
adjacent-package, release, merge, or publication authority.

## 7. Explicit exclusions and prohibited adjacent work

Do not:

- `<PACKAGE_SPECIFIC_EXCLUSION>`
- implement a predecessor correction without first stopping for review;
- begin any dependent or later package;
- silently change frozen architecture;
- create speculative packages, services, frameworks, plugins, workers,
  persistence, telemetry, updater, or enterprise systems;
- add `packages/ui` or a public module SDK before the required real second
  consumer proves the extraction;
- claim mocks, stubs, unavailable platforms, or documentation as working
  functionality;
- perform unrelated cleanup or refactoring.

List every nearby roadmap package whose work could otherwise be confused with
this scope.

## 8. Allowed repository areas and expected changes

**Allowed areas:**

```text
<PATH_OR_GLOB>
<PATH_OR_GLOB>
```

**Expected files or modules:** `<EXPECTED_CHANGE_INVENTORY>`
**Areas that must remain unchanged:** `<PROTECTED_PATHS>`

Authoring guidance:

- Prefer responsibility/area boundaries over an inaccurate exhaustive list,
  but identify expected concrete paths when known.
- Stop before touching an unlisted authority, application, workflow, release,
  or data-ownership boundary.
- A demonstrated necessary path change outside this section requires prompt
  review unless it is a mechanically generated/locked companion explicitly
  anticipated here.

## 9. Ordered implementation procedure

1. Complete authority and repository preflight.
2. Verify predecessor evidence and current real behavior.
3. `<IMPLEMENT_SMALLEST_REAL_VERTICAL_SLICE>`
4. `<ADD_ONLY_REQUIRED_FAILURE_AND_BOUNDARY_HANDLING>`
5. `<RUN_PROPORTIONAL_CHECKS_CONTINUOUSLY>`
6. `<RUN_REAL_RUNTIME_PLATFORM_OR_PACKAGE_EVIDENCE>`
7. Verify scope and status inventory.
8. Create the required commit, draft PR, and external handoff only as authorized.

The procedure must be specific enough to execute but must not solve work
outside the package. Prefer direct implementation over speculative
generalization.

## 10. Cross-cutting constraints

Complete only the subsections applicable to this package; write `Not applicable`
with a reason rather than silently omitting a boundary.

### Contract

`<SCHEMAS_FIXTURES_VERSIONING_LIMITS_AND_DRIFT_RULES>`

### Security and trust

`<REACT_RUST_PYTHON_AUTHORIZATION_CSP_PATH_SECRET_AND_CONTENT_RULES>`

### Lifecycle and concurrency

`<STATE_MACHINE_BOUNDS_CANCELLATION_TIMEOUT_CRASH_RESTART_NO_REPLAY_RULES>`

### Accessibility

`<KEYBOARD_FOCUS_SCREEN_READER_FORCED_COLORS_SCALING_MOTION_REQUIREMENTS>`

### Platform and packaging

`<TARGET_MATRIX_PACKAGE_TYPES_NATIVE_FALLBACKS_CLEAN_INSTALL_AND_SIGNING_SCOPE>`

### Data ownership and privacy

`<SINGLE_WRITER_PERSISTENCE_LOGGING_DIAGNOSTICS_AND_REDACTION_RULES>`

Do not weaken a higher-authority constraint inside this prompt. A material
boundary change requires an accepted targeted amendment before implementation.

## 11. Proportional tests and exact evidence

### Required automated checks

```text
<EXACT_FORMAT_LINT_TYPE_TEST_BUILD_COMMAND>
<EXACT_UNIT_CONTRACT_INTEGRATION_COMMAND>
```

### Required real execution evidence

```text
<EXACT_RUNTIME_OR_INSTALLED_ARTIFACT_COMMAND>
```

### Required CI evidence

- Workflow/job: `<EXACT_SCOPE_AND_TARGETS>`
- Retained result: `<RUN_JOB_ARTIFACT_AND_LOG_REQUIREMENTS>`

### Required manual/platform evidence

- `<PLATFORM_AND_CHECKLIST>`

For every check record the command/method, expected behavior, actual result,
platform/tool identity, and blocker. Keep tests proportional to core behavior,
critical failure paths, data integrity, and important integration boundaries.
Do not inflate test count or substitute browser mocks for native authority.

## 12. Measurable acceptance gates

The package passes only when:

1. `<MEASURABLE_GATE>`
2. `<MEASURABLE_GATE>`
3. `<MEASURABLE_GATE>`
4. all required real-path and platform evidence is retained;
5. no forbidden adjacent work or hidden stub is present;
6. repository scope, status, and handoff checks pass.

Each gate must be decidable from retained evidence and depend only on accepted
predecessor work plus this package. Do not make acceptance depend on an
unimplemented successor.

## 13. Stop conditions and blocker reporting

Stop without improvising when:

- the repository or base SHA differs from activation;
- a prerequisite or required authority file is missing or conflicting;
- a frozen boundary would need to change;
- the package requires an unapproved path, dependency, platform, credential,
  consumer, or adjacent package;
- the real execution path cannot be built or verified;
- only fake, mock-only, browser-only, unavailable, or fabricated evidence would
  support a claimed pass;
- `<PACKAGE_SPECIFIC_STOP_CONDITION>`.

Return `Blocked` or `Partially implemented` using the approved labels. Name the
exact failed gate, observed evidence, unchanged scope, and smallest safe next
action. Do not continue into later work as a workaround.

## 14. Required functional and status inventory

Report every applicable category with one approved implementation-status label:

```text
Package outcome: <STATUS>
Real production/spike functionality: <STATUS>
Supporting infrastructure: <STATUS>
Tests: <STATUS>
Documentation: <STATUS>
Generated code: <STATUS>
Fixtures/mocks: <STATUS>
Stubs/placeholders: <STATUS>
Incomplete work: <STATUS>
Blocked work: <STATUS>
Next package: Not started
```

Add package-specific categories. Documentation may be `Implemented` while
application behavior remains `Not started`; never conflate them.

## 15. One RUN_ID and collision-resistant external naming

At task start generate exactly one UTC identifier:

```text
RUN_ID=YYYYMMDDTHHMMSSZ
```

Use it unchanged for all external artifacts. Names must contain:

```text
prime-shell-<session-or-role>-<package-id-lowercase>-<RUN_ID>-<artifact>-rN.<ext>
```

Repository filenames remain conventional and do not receive timestamps unless
their repository contract requires versioning. Do not reuse a prior run's
filename or mix multiple run IDs.

## 16. Required deliverables, hashes, and evidence index

**External reports:**

1. `<COLLISION_RESISTANT_IMPLEMENTATION_OR_REVIEW_REPORT>.md`
2. `<COLLISION_RESISTANT_HANDOFF_MANIFEST>.md`
3. `<COLLISION_RESISTANT_REVIEW_EVIDENCE_INDEX>.md`
4. `<OTHER_EXPLICITLY_REQUIRED_ARTIFACT_OR_NONE>`

The handoff manifest must record:

- repository, branch, exact base and final commit;
- files changed and responsible execution paths;
- commands and actual results;
- CI/runtime/manual evidence;
- truthful status inventory and limitations;
- SHA-256 of every external deliverable;
- SHA-256 of repository documents/artifacts when required;
- a documented self-hash convention for the manifest if used;
- exact next controlled action.

Create a complete authoritative source snapshot only when this package changes
source implementation or the activated prompt explicitly requires one. It must
represent the final commit, include its SHA-256, pass CRC/path-safety checks,
contain one expected root, and be extracted into a fresh empty directory for
verification—never overlaid onto an existing tree. Documentation-only packages
normally use the pushed Git branch as source authority and do not create a ZIP.

The review evidence index maps each requirement and acceptance gate to a file,
section, command/result, and unresolved item without duplicating full reports.

## 17. Branch, draft-PR, review, merge, and deletion controls

1. Create only the fresh branch named in section 2.
2. Commit a coherent package change; do not mix unrelated work.
3. Push only that branch.
4. Open or update one **draft** PR only when this activated prompt authorizes it.
5. Keep the PR draft until Chat Session accepts the package.
6. Do not enable auto-merge.
7. Do not merge, publish, or delete a branch without explicit user
   authorization after Chat Session acceptance.
8. After an explicitly authorized merge, verify `main`, then delete only the
   exact merged branch.

Report the exact branch/head/PR state. Prompt acceptance alone is not
implementation activation, and implementation acceptance alone is not merge
authorization.

## 18. Completion response and return prompt

Keep progress and completion messages concise. Lead with the truthful package
result, then report model, repository state, files, evidence, status inventory,
external artifacts, exclusions, and next action.

End with a short prompt addressed to Chat Session:

```text
Chat Session: Review <PACKAGE_ID> on branch <BRANCH> at <FINAL_SHA>.
Read <EXTERNAL_ARTIFACT_FILENAMES>. Verify <KEY_ACCEPTANCE_GATES> and return
Accepted, Focused correction required, or Blocked. No successor package was
started, and this handoff does not authorize merge or later implementation.
```

Do not include authorization for another package. Do not continue after
returning the handoff.
