# Generic Fluent Desktop Provisional Prompt Activation Procedure

**Purpose:** Prevent an approved but stale work-package prompt from executing
against the wrong repository, predecessor, toolchain, platform, or architecture
state.

Only Chat Session may issue an activated execution prompt. Activation authorizes
exactly one package on one fresh branch; it does not authorize a successor,
merge, publication, release, or repository-setting change.

## 1. Prompt lifecycle states

| State | Meaning | Executable |
|---|---|---|
| `Draft` | Incomplete authoring copy; not yet reviewed. | No |
| `Provisional` | Complete candidate prompt awaiting Chat Session review. | No |
| `Approved provisional` | Content accepted as the reusable package specification, but repository facts may become stale. | No |
| `Activated` | Chat Session refreshed and verified current facts and issued this exact version for one package. | Yes, for the named package and state only |
| `Superseded` | A newer prompt or activation replaces this copy. | No |
| `Retired` | Package was completed, removed, or materially redesigned through accepted control. | No |

Implementation remains `Blocked` until an explicit `Activated` copy exists.
Prompt lifecycle state is not an implementation-status label.

## 2. Activation entry conditions

Chat Session may begin activation only when:

1. the Stage 1 foundation is accepted;
2. the package has one `Approved provisional` prompt;
3. every direct prerequisite in `dependency-matrix.md` is accepted and merged;
4. no competing implementation branch is active for the same package;
5. the user has requested or authorized activation of that package;
6. the package remains the next valid dependency-ordered action.

If any entry condition fails, return `Blocked` and the smallest safe next action.

## 3. Establish current repository truth

Activation starts from the latest accepted and merged `main`.

Verify and record:

- exact repository `Mirarsoft/prime-shell`;
- default branch `main`;
- current remote `main` 40-character SHA;
- clean/fresh repository access and expected branch absence;
- authority and accepted prompt-pack files at that SHA;
- merged predecessor commits and repository paths;
- absence of unexpected implementation that would duplicate or invalidate the
  package.

Do not reuse the SHA embedded in a provisional prompt without verification. A
branch or unmerged PR is not accepted predecessor authority.

## 4. Refresh predecessor facts

For every direct prerequisite, record:

- accepted merged commit;
- accepted reports, manifests, evidence indexes, source/artifact hashes, CI
  runs, runtime/manual results, and platform limitations relevant to this
  package;
- functional/status inventory;
- accepted deviations and unresolved blockers;
- targeted architecture amendments accepted since the provisional prompt;
- paths or ownership boundaries actually produced.

Compare these facts with the provisional prompt. Missing or materially
conflicting predecessor evidence blocks activation.

## 5. Refresh volatile execution facts

Update only facts that may have become stale:

1. current authoritative `main` SHA;
2. predecessor outputs and accepted deviations;
3. current repository paths and expected files;
4. pinned Node, pnpm, Rust, Python, Tauri, Fluent, packaging, test, and other
   applicable tool/dependency versions;
5. actual CI runner labels, permissions, quotas, retained-artifact behavior,
   and platform capabilities;
6. available target hardware/manual-test ownership;
7. signing/notarization/update credentials and protected environment
   availability when the package requires them;
8. accepted targeted architecture amendments;
9. known blockers and changed assumptions;
10. filenames, commands, refs, and external artifact revisions.

Do not broaden Linux formats, architectures, stores, workers, persistence,
networking, plugins, telemetry, product features, or release scope merely
because a tool could support them.

## 6. Revalidate package scope and dependencies

Confirm:

- the package ID, objective, boundaries, and direct prerequisites still match
  the accepted roadmap and matrix;
- no package was skipped and no dependency cycle or missing target was
  introduced;
- no predecessor now owns or completes the same scope;
- expected repository ownership does not conflict with another active package;
- a required real second consumer exists before extraction;
- the package still produces one coherent runnable vertical slice;
- no acceptance gate depends on an unimplemented successor.

Activation may narrow scope to fit new evidence or remove already-completed
work. It may not add an adjacent capability or combine packages.

## 7. Revalidate evidence achievability

For each acceptance gate, verify:

- exact automated command remains valid;
- real runtime or installed-artifact path can be exercised;
- required CI targets and permissions are available;
- manual/native evidence owner and environment are named;
- build, runtime, native UX, clean install, signing/notarization, and update
  claims remain separately provable;
- required source snapshot and external handoff rules are feasible;
- unavailable evidence will be reported `Blocked`, not substituted or
  fabricated.

If an essential gate cannot be achieved in the authorized environment, do not
activate unless the accepted prompt explicitly defines truthful partial/blocker
handling that preserves the package boundary.

## 8. Apply allowed activation updates

Chat Session may:

- replace stale SHA, branch, path, filename, command, version, runner, and
  evidence references;
- add accepted predecessor deviations or targeted amendments;
- remove work already accepted in a predecessor;
- narrow evidence or implementation scope without defeating the package
  objective;
- strengthen a stop condition or clarify an existing acceptance gate.

Chat Session may not silently:

- broaden scope;
- combine two package prompts;
- change a frozen architecture, trust, lifecycle, platform, accessibility,
  packaging, or data-ownership boundary;
- add a new product requirement;
- waive real execution evidence;
- authorize a successor, merge, publication, or release.

A material change to objective, ownership, dependencies, public contract,
security boundary, supported platform, package type, data model, or acceptance
meaning returns the prompt to provisional review. An architecture boundary
change also requires an accepted targeted amendment before prompt review.

## 9. Create the activation record

Embed this completed block in the activated prompt:

```text
Activation ID: PRIME-SHELL-CHAT-<PACKAGE-ID>-ACT-<UTC-RUN-ID>-R<N>
Prompt ID and version: <PROMPT-ID> / <VERSION>
Package ID: <PACKAGE-ID>
Lifecycle state: Activated
Activated by: Chat Session
Activation UTC: <YYYY-MM-DDTHH:MM:SSZ>
Repository: Mirarsoft/prime-shell
Authoritative base branch: main
Authoritative main SHA: <EXACT-40-CHARACTER-SHA>
Required fresh branch: <EXACT-BRANCH>
Accepted predecessor commits: <EXACT-SHAS>
Accepted predecessor evidence: <FILES-RUNS-HASHES>
Accepted deviations/amendments: <LIST-OR-NONE>
Activation-time changes applied: <CONCISE-LIST-OR-NONE>
Current tools/CI/platform facts: <CONCISE-FACTS>
Unresolved blockers/assumptions: <LIST-OR-NONE>
Authorization boundary: Execute exactly <PACKAGE-ID>; no successor or merge
Invalidation condition: Any change to base SHA, accepted predecessor state,
material platform capability, or named authorization boundary
```

Use one collision-resistant activation ID. Preserve the approved provisional
prompt identity and increment its revision only according to the prompt-pack
versioning rule; do not disguise a material redesign as a metadata refresh.

## 10. Issue and control the activated prompt

Before issuing it, Chat Session verifies:

- lifecycle state is exactly `Activated`;
- exact current `main` SHA and fresh branch are present;
- all placeholders are resolved;
- one package only is authorized;
- required model/reasoning remains minimum sufficient;
- predecessor evidence and deviations are current;
- exclusions, acceptance gates, stop conditions, status inventory, deliverable
  naming, hashes, draft-PR controls, and return prompt remain intact.

Chat Session then provides the activated prompt to Work Session. Work Session
must re-run repository and authority preflight before writing. If observed state
differs from the activation record, Work Session stops rather than rebasing or
improvising.

## 11. Invalidation, supersession, and retirement

- A changed `main` SHA before work starts invalidates the activation.
- A material predecessor correction, accepted amendment, tool/platform
  capability change, or branch collision invalidates the activation.
- Chat Session may issue a refreshed activation after repeating affected checks.
- A newer activated revision marks the prior activation `Superseded`.
- Completion/merge of the package marks its prompt `Retired` for execution; it
  remains historical evidence.
- A failed or blocked implementation does not automatically reactivate the same
  prompt. Chat Session reviews evidence and chooses focused correction, refreshed
  activation, or targeted amendment.

## 12. Activation review outcome

Use:

- `Accepted`: issue the exact one-package activated prompt.
- `Focused correction required`: correct only identified prompt/metadata gaps
  and repeat activation review.
- `Blocked`: name the repository, dependency, authority, environment,
  credential, platform, or evidence blocker and stop.

Activation acceptance authorizes implementation of the named package only. It
does not authorize merge, publication, release, or the next package.
