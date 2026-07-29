# Generic Fluent Desktop Prompt-Pack Review Checklist

**Reviewer:** Chat Session
**Applies to:** Stage 1 foundation, each provisional package prompt, each
activation update, and final cross-package readiness
**Review outcomes:** `Accepted`, `Focused correction required`, or `Blocked`

Mark each applicable item `Pass`, `Fail`, `Blocked`, or `Not applicable` with a
short evidence reference. `Not applicable` requires a reason.

## 1. Common authority and truth checks

- [ ] Repository authority was read in the required order.
- [ ] The reviewed artifact identifies its role, version, package or stage, and
      provisional/activated state.
- [ ] Architecture claims match current authority and accepted targeted
      amendments.
- [ ] No frozen decision changed silently.
- [ ] Current project position is accurate: accepted work is not repeated and
      future work is not described as implemented.
- [ ] Only approved implementation-status labels are used.
- [ ] Documentation, tests, fixtures/mocks, generated code, stubs, and real
      functionality are distinguished.
- [ ] No fake, hard-coded, mock-only, unavailable, or fabricated evidence is
      presented as passed.
- [ ] No hidden authorization exists for a later package, merge, publication,
      release, or repository-setting change.

## 2. Stage 1 foundation review

### Roadmap index

- [ ] WP01 is recorded as `Implemented` and excluded from future package
      inventory.
- [ ] WP02 preserves progress, cancellation, timeout, crash/hang, bounded
      restart, no-replay, and process-tree containment boundaries.
- [ ] WP03 preserves title-bar evidence/fallback, native WebdriverIO,
      cross-platform build/package evidence, measurements, and final Phase 0B
      spike closure.
- [ ] Remaining packages preserve the architecture order from baseline
      hardening through template extraction.
- [ ] Packages are coherent runnable/reviewable slices—not phase-sized
      monoliths or low-value mechanical fragments.
- [ ] Every package has stable ID, phase, title, objective, prerequisites,
      in-scope outcome, exclusions, acceptance summary, expected ownership,
      status, and minimum suitable model/reasoning level.
- [ ] Lifecycle, concurrency, security, platform, packaging, accessibility, and
      recovery risks receive sufficient review capacity.
- [ ] Second-consumer gates precede feature/command, public SDK, `packages/ui`,
      or broader extraction as applicable.
- [ ] No per-package implementation procedure or complete execution prompt is
      embedded in the index.

### Prompt-authoring template

- [ ] Exactly 18 required numbered sections are present and usable without
      restructuring.
- [ ] Identity includes task/prompt/package/phase/version/status/model fields.
- [ ] Repository/base SHA/branch preflight is exact.
- [ ] Authority files are in exact reading order.
- [ ] Dependencies, accepted predecessor outputs/deviations, and evidence are
      mandatory.
- [ ] Objective requires a measurable real runnable path.
- [ ] Scope, exclusions, allowed areas, and ordered procedure are distinct.
- [ ] Contract, security, lifecycle, accessibility, platform/packaging, and
      data-ownership constraints are explicit or reasoned `Not applicable`.
- [ ] Proportional tests and CI/runtime/manual evidence are exact.
- [ ] Acceptance gates are measurable and do not depend on successor work.
- [ ] Stop conditions require truthful blocker reporting.
- [ ] Functional/status inventory uses approved labels.
- [ ] Exactly one UTC `RUN_ID` and collision-resistant naming are required.
- [ ] Reports, hashes, manifest, evidence index, and conditional source snapshot
      rules are complete.
- [ ] Branch, draft PR, review, merge, and deletion controls require explicit
      authorization.
- [ ] The final return prompt is short and addressed to Chat Session.
- [ ] Provisional/activated marker and activation metadata block are mandatory.
- [ ] Exact activation-time `main` SHA, predecessor deviations, and accepted
      amendments are mandatory.
- [ ] No package authorizes another package.
- [ ] Model selection is minimum sufficient rather than automatically maximal.

### Shared rules and activation

- [ ] Authoring, activation, implementation, and review/merge rules are clearly
      separated.
- [ ] Repository-first authority, scope discipline, no silent architecture
      changes, real-path evidence, and stop-on-blocker behavior are preserved.
- [ ] Prompt-pack authoring prohibits application-source and WP02/later
      implementation.
- [ ] External naming includes project, role/session, package, one UTC
      `RUN_ID`, artifact role, and revision while repository names remain
      conventional.
- [ ] Source snapshots, when required, use a complete final commit, hashes,
      path safety, one root, and fresh empty extraction.
- [ ] Draft PR, no-auto-merge, Chat Session acceptance, and explicit user merge
      authorization controls are unambiguous.
- [ ] Activation lifecycle and record prevent stale provisional prompts from
      executing.
- [ ] Activation may refresh/narrow but cannot silently broaden or combine
      packages.

### Dependency matrix

- [ ] Package row inventory and order exactly match `roadmap-index.md`.
- [ ] Every direct prerequisite exists or is the accepted WP01 predecessor.
- [ ] The graph is acyclic.
- [ ] Required predecessor outputs/evidence are sufficient.
- [ ] Dependents, sequencing, and parallelism rules are explicit.
- [ ] Major repository ownership is complete enough to expose conflicts.
- [ ] No duplicated scope or incompatible concurrent ownership remains.
- [ ] No acceptance gate relies on work not yet completed.
- [ ] No extraction precedes its real second-consumer evidence.
- [ ] No release/package claim precedes necessary platform evidence.
- [ ] Stale activation facts and conflict risks are identified.
- [ ] Model recommendations are neither materially excessive nor insufficient.

## 3. Provisional package prompt review

- [ ] Package ID exists once in the accepted roadmap and dependency matrix.
- [ ] Package scope is stable, bounded, and limited to one package.
- [ ] Direct prerequisites, predecessor commits, outputs, deviations, and
      evidence match current accepted state.
- [ ] Objective yields a measurable user-visible or technically runnable result.
- [ ] In-scope work is sufficient for the outcome without adjacent scope.
- [ ] Explicit exclusions name nearby packages and prohibited speculative work.
- [ ] Allowed repository ownership matches the dependency matrix and does not
      conflict with another active package.
- [ ] Ordered procedure builds a real vertical slice before expansion.
- [ ] Security/trust/lifecycle/data-ownership constraints match authority.
- [ ] Accessibility requirements are present wherever UI/native behavior is
      affected.
- [ ] Platform/package requirements distinguish build, runtime, native UX,
      clean install, signing, and manual evidence.
- [ ] Tests are proportional and commands are plausible for current repository
      paths and tools.
- [ ] CI/runtime/manual evidence is exact and cannot be satisfied by mocks alone.
- [ ] Acceptance gates are measurable, complete, and package-local.
- [ ] Stop conditions cover repository drift, missing prerequisites, boundary
      changes, environment limits, and unavailable real evidence.
- [ ] Status inventory and external deliverables are complete.
- [ ] One `RUN_ID`, collision-resistant filenames, reports, hashes, manifest,
      evidence index, and conditional source snapshot are correct.
- [ ] Required model/reasoning is minimum sufficient for actual risk.
- [ ] The prompt is marked `Provisional` or `Approved provisional`, not
      `Activated`.
- [ ] Acceptance of the provisional prompt is explicitly not implementation
      authorization.

## 4. Activation update review

- [ ] Activation was issued by Chat Session for exactly one approved
      provisional prompt.
- [ ] Latest accepted/merged `main` and exact 40-character SHA are verified.
- [ ] All direct predecessors are accepted and merged.
- [ ] Predecessor outputs, evidence, deviations, and targeted amendments are
      refreshed.
- [ ] Repository paths, pinned tool/dependency versions, CI runners, platform
      capabilities, credentials, manual-test owners, blockers, and assumptions
      are current.
- [ ] Scope and dependency order remain valid.
- [ ] Every acceptance gate remains achievable with actual capabilities.
- [ ] Stale filenames, commands, versions, refs, and paths were updated without
      broad redesign.
- [ ] Activation narrows or refreshes; it does not broaden, combine packages, or
      authorize a successor.
- [ ] A material scope/boundary change returned to prompt or architecture review.
- [ ] Activation ID, timestamp, current SHA, predecessor evidence, changes,
      blockers, invalidation condition, and authorization boundary are complete.
- [ ] The state is explicitly `Activated`; all other lifecycle copies are
      superseded or remain non-executable.
- [ ] A fresh implementation branch name is exact and not unexpectedly present.

## 5. Final cross-package prompt-pack review

- [ ] Every roadmap package has exactly one approved provisional prompt.
- [ ] No prompt is missing, duplicated, stale, or attached to the wrong package.
- [ ] Package IDs, phases, titles, dependencies, ordering, ownership, status,
      and model levels agree across all prompt-pack files.
- [ ] Dependency cycle and missing-target checks pass.
- [ ] No scope is omitted or duplicated.
- [ ] No two packages claim incompatible repository or data ownership.
- [ ] Acceptance gates do not conflict or require unimplemented successor work.
- [ ] Second-consumer gates are enforced before extraction.
- [ ] Cross-platform and release claims follow retained evidence.
- [ ] Lifecycle, security, privacy, accessibility, packaging, and recovery
      constraints remain consistent end to end.
- [ ] Deliverable naming and single-`RUN_ID` rules are consistent.
- [ ] Every prompt has proportional evidence and truthful stop conditions.
- [ ] Model recommendations follow the token-efficiency policy.
- [ ] Activation procedure can refresh each prompt without redesign.
- [ ] The final readiness report lists limitations and unresolved blockers.
- [ ] Documentation PR remains draft; no merge or implementation is implicitly
      authorized.

## 6. Review outcomes and next allowed action

| Outcome | Meaning | Next allowed action |
|---|---|---|
| `Accepted` | All applicable checks pass and no material blocker remains. | For Stage 1, retain the foundation and wait for a separate one-package Stage 2 authorization. For a provisional prompt, mark it `Approved provisional` and wait for later activation. For activation, issue the one-package activated prompt. For final pack review, request explicit user authorization for the documentation merge; do not merge automatically. |
| `Focused correction required` | Scope remains valid but one or more bounded, correctable checks fail. | Issue or apply one correction limited to the named failures, retain the same stage/package identity, and repeat the affected review. Do not activate, merge, or begin a successor. |
| `Blocked` | Authority, dependency, architecture, repository state, environment, credential, platform, or real-evidence failure prevents safe acceptance. | Record the exact blocker and smallest safe next action. Do not broaden scope, fabricate evidence, activate, merge, or advance. |

Acceptance of a prompt is never implementation authorization unless Chat Session
has separately completed activation and issued the activated execution prompt.
