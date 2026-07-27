# Generic Fluent Desktop Prompt-Pack Shared Ground Rules

**Document role:** Common execution contract inherited by every prompt-pack task
**Authority:** Subordinate to `AGENTS.md`, delivery rules, repository
architecture authority, accepted amendments, and the current explicit prompt

These rules do not authorize any package. A package may execute only through an
explicit Chat Session activation that names one package and exact repository
state.

## 1. Rules for prompt-pack authoring

1. Read and use repository authority in this order:
   `AGENTS.md`, `docs/authority/Core-Functionality-DeliveryRules.md`,
   `docs/authority/generic-fluent-desktop-project-handoff.md`,
   `docs/authority/generic-fluent-desktop-app-architecture-v0.2.md`,
   `docs/authority/architecture-review-consolidation-decision-log.md`,
   relevant accepted Phase 0A documents, and approved prompt-pack files.
2. The merged Git repository overrides old snapshots, chat drafts, and
   superseded handoffs.
3. Prompt-pack authoring is documentation and planning only. Do not implement
   WP02 or any later package while authoring the pack.
4. On the prompt-pack documentation branch, do not change application source,
   dependencies, CI workflows, architecture authority, packages, product
   features, release configuration, or runtime behavior unless a later
   explicit correction prompt authorizes that exact documentation-scope change.
5. Do not create per-package prompts during Stage 1. Author one provisional
   prompt at a time only after Chat Session accepts the foundation and
   authorizes that Stage 2 package.
6. Match package IDs, order, scope, ownership, dependencies, and model levels in
   `roadmap-index.md` and `dependency-matrix.md`.
7. Do not silently change frozen architecture. Record uncertainty as an
   activation-time fact, blocker, deferred decision, or targeted amendment
   proposal.
8. Do not create speculative frameworks, packages, services, plugins, workers,
   SDKs, UI libraries, databases, updaters, telemetry, or enterprise policy.
9. Keep repository filenames conventional. Use collision-resistant names only
   for external cross-session artifacts.
10. Keep Work Session progress concise and operational. Durable detail belongs
    in the required Markdown artifacts.

## 2. Rules for activation

1. Provisional prompts never execute directly. Only Chat Session may issue an
   activated execution prompt.
2. Activation starts from the latest accepted and merged `main`, not from a
   stale prompt SHA or unmerged predecessor branch.
3. Activation refreshes exact `main` SHA, predecessor outputs/deviations,
   repository paths, pinned versions, CI/platform capabilities, accepted
   targeted amendments, blockers, and changed assumptions.
4. Activation must confirm dependency ordering and that acceptance evidence is
   achievable on the actual environment.
5. Activation may update stale refs, filenames, commands, or versions and may
   narrow scope. It may not silently broaden scope, combine packages, change
   architecture, or authorize a successor.
6. A material scope or boundary change returns the prompt to review and, when
   necessary, requires a targeted architecture amendment.
7. An activated prompt authorizes exactly one package on one fresh branch.
8. Any change to the authoritative base, accepted predecessor evidence, or
   material activation assumptions invalidates the activation until Chat
   Session refreshes it.

## 3. Rules for implementation

### 3.1 Scope and architecture

1. Execute only the activated package.
2. Do not begin an adjacent, dependent, or later package.
3. Preserve the architecture order and explicit exclusions in the roadmap.
4. Do not add a public module SDK, `packages/ui`, or broader extraction before
   the named real second-consumer gate is satisfied.
5. When two safe solutions satisfy the package, choose the simpler direct one.
6. Stop before an unapproved architecture, repository-area, dependency,
   platform, signing, data-ownership, or product expansion.

### 3.2 Real functionality and evidence

1. Build the smallest complete real vertical slice required by the package.
2. Planning, interfaces, fixtures, mocks, documentation, test doubles, hard
   coded success, placeholder returns, and TODOs do not count as working
   functionality.
3. Never report unavailable, browser-only, mock-only, or fabricated evidence as
   a passed real/native path.
4. A build does not prove runtime, installed-artifact behavior, native UX,
   signing, notarization, or clean installation.
5. Run proportional tests that protect core behavior, critical failures, data
   integrity, important integration boundaries, and one small end-to-end path.
   Test count is not progress.
6. Record exact commands/methods, expected behavior, observed results,
   platform/tool identity, and blockers.
7. Use accepted native fallback behavior when platform promotion gates fail.
   Do not overstate cross-platform support.

### 3.3 Trust, lifecycle, and data

1. React remains least trusted; Rust remains native policy and process
   authority; Python remains trusted first-party native code, not a sandbox.
2. Do not expose generic shell, process, arbitrary filesystem, secret, or
   unrestricted operation access to React.
3. Preserve bounded protocol/frame/queue/log behavior, explicit operation
   authorization, safe content rendering, and minimal sidecar environment.
4. Preserve deterministic cancellation/terminal races, bounded restart and
   containment, no automatic replay of uncertain work, and zero-orphan
   shutdown.
5. Preserve single-writer durable-data ownership and privacy allowlists.
6. Accessibility, forced colors, keyboard/focus behavior, scaling,
   reduced-motion/transparency, and native checks are structural when relevant,
   not optional polish.

### 3.4 Status and blockers

Use only:

- `Implemented`
- `Partially implemented`
- `Stub`
- `Mock-only`
- `Not started`
- `Blocked`

Distinguish real functionality, supporting infrastructure, tests,
documentation, generated code, fixtures/mocks, stubs/placeholders, incomplete
work, and blocked work.

Stop on a material blocker. Report the failed gate, exact evidence, unchanged
scope, and smallest safe next action. Do not enter later work to hide or bypass
the blocker.

### 3.5 Branch and artifact discipline

1. Use a fresh implementation branch per package from the exact activated
   `main`.
2. Keep one coherent package change and a clean final tree.
3. Generate one UTC `RUN_ID=YYYYMMDDTHHMMSSZ` at task start and reuse it for
   every external artifact.
4. External filenames must include project marker `prime-shell`, Work/Chat
   session or role marker, package ID, the one UTC `RUN_ID`, artifact role, and
   revision suffix.
5. Calculate and report SHA-256 for external deliverables and any repository
   artifact the prompt requires.
6. When a source snapshot is required, make it a complete authoritative archive
   of the final commit, verify CRC and path safety, require one expected root,
   and extract into a fresh empty directory—never overlay an existing tree.
7. Documentation-only work normally uses the pushed Git branch as source
   authority and does not create an unnecessary source ZIP.

## 4. Rules for review and merge

1. Work Session ends with concise external reports, a review evidence index,
   a handoff manifest, individual links, and a short prompt addressed to Chat
   Session.
2. Chat Session reviews authority alignment, package scope, dependencies,
   ownership, real evidence, status, deliverables, and stop conditions.
3. Review outcomes are only `Accepted`, `Focused correction required`, or
   `Blocked`.
4. Keep implementation and documentation PRs draft until Chat Session accepts
   the work.
5. Do not enable or use auto-merge.
6. Chat Session acceptance does not itself authorize merge. Merge requires
   explicit user authorization.
7. Merge only the exact accepted branch/head and method the user authorized.
8. After merge, verify `main` contains the accepted result before deleting only
   the exact merged branch.
9. Acceptance of a provisional prompt does not authorize implementation;
   activation is separate. Acceptance of one implementation does not authorize
   its successor.
10. Do not reuse a stale provisional prompt. Activate it again against current
    repository and predecessor facts.

## 5. Model and communication efficiency

- Use GPT-5.6 Sol / Extra High for roadmap/dependency design and prompts whose
  risk is lifecycle, concurrency, security, cross-platform packaging, or
  recovery.
- Use High for substantial implementation and complex UI/integration prompts
  with known boundaries.
- Use Medium for focused corrections, activation updates, and ordinary review
  after the template is stable.
- Use Low for Git housekeeping, metadata, filenames, and formatting.
- Use the smallest sufficient level. Do not spend high-reasoning capacity on
  mechanical narration or repeat authority content in every report.
- Keep progress messages brief and cross-session return prompts short.

## 6. Permanent exclusions without new authority

No prompt-pack task may introduce runtime third-party plugins, untrusted code
execution, multiple Python workers, durable task resume, GPU/model
orchestration, remote telemetry/crash upload, broad Linux/store distribution,
or enterprise deployment unless a new hard requirement and accepted targeted
architecture decision explicitly authorize it.
