# GFD-P7-WP01 — Production Security and Release Artifact Hardening

## 1. Package identity and prompt status

**Package ID:** `GFD-P7-WP01`
**Phase:** `Phase 7`
**Title:** `Production Security and Release Artifact Hardening`
**Task ID:** `GFD-P7-WP01`
**Prompt ID:** `PRIME-SHELL-GFD-P7-WP01-PROMPT`
**Prompt version:** `R1`
**Prompt lifecycle state:** `Provisional`
**Implementation status at authoring:** `Not started`
**Recommended future implementation model:** `GPT-5.6 Sol`
**Recommended future implementation reasoning/intelligence:** `Extra High`
**Authorization boundary:** Exactly one package, `GFD-P7-WP01`
**Execution status:** `Execution is not authorized`

Extra High is required because production capabilities, strict CSP, dependency
closure, SBOM and license truth, supply-chain findings, package contents,
clean-machine behavior, cross-target claims, recovery readiness, and release
workflow permissions are coupled security boundaries. A small inconsistency
could ship a test driver, omit a generated or packaged dependency, weaken
production policy, conceal a critical finding, or misstate an artifact as
release-ready.

This provisional prompt is complete for review but non-executable. Chat
Session must first accept it as an Approved provisional prompt and later issue
a separately activated revision after every prerequisite is accepted and
merged. Prompt acceptance is not activation, implementation authorization,
signing or publication authority, PR authority, or merge authority.

**Direct prerequisite:** Accepted and merged `GFD-P6-WP02`, including accepted
diagnostics, export, repair, recovery, privacy, package, process, and native
evidence.

**Direct dependent:** `GFD-P7-WP02 — Signing, Notarization, and Signed Update
Channels`, which remains unauthorized.

### Activation metadata

```text
Activation ID: Not activated — Chat Session must refresh and supply this exact value.
Activated by: Not activated — Chat Session must refresh and supply this exact value.
Activation UTC: Not activated — Chat Session must refresh and supply this exact value.
Authoritative main SHA: Not activated — Chat Session must refresh and supply this exact value.
Required fresh implementation branch: Not activated — Chat Session must verify absence and supply this exact value.
Accepted predecessors through GFD-P6-WP02: Not activated — Chat Session must supply exact merged commits, reports, artifacts, hashes, CI runs/jobs, native/manual evidence, measurements, deviations, fallbacks, limitations, and blockers.
Accepted P6-WP02 diagnostics/repair/recovery authority: Not activated — Chat Session must supply exact record/export schemas, retention/privacy rules, recovery/repair commands, package identities, report, evidence, and accepted status.
Accepted target matrix: Not activated — Chat Session must refresh exact OS versions, architectures, webview engines, package formats, runners, hardware/manual owners, and unavailable evidence.
Accepted package identities and metadata: Not activated — Chat Session must supply exact product name, identifier, version policy, publisher/vendor fields, icons, descriptions, filenames, install locations, upgrade identity, and uninstall behavior.
Production capability and CSP baseline: Not activated — Chat Session must supply exact Tauri commands, capabilities, permissions, CSP directives, navigation/resource rules, and target differences.
Production/test/evidence/fault separation: Not activated — Chat Session must supply exact feature flags, commands, capabilities, binaries, resources, fixtures, workflows, and package exclusions.
Locked dependency closure: Not activated — Chat Session must supply exact Rust, JavaScript, Python, Tauri/plugin, sidecar, packaging, generated, workflow-action, and toolchain manifests and locks.
SBOM, license, and notice policy: Not activated — Chat Session must supply exact formats, tools, versions, package-to-component mapping, license policy, notice owners, and output locations.
Security scanners and finding policy: Not activated — Chat Session must supply exact vulnerability, secret, provenance, integrity, and supply-chain tools, databases, severities, suppressions, expiry rules, and owners.
Release artifact budgets: Not activated — Chat Session must supply exact startup, memory, package size, installed size, task, diagnostics, recovery, cleanup, and reliability budgets or accepted measurement-only rules.
Clean-target evidence ownership: Not activated — Chat Session must supply exact clean machine or equivalent isolation definition, installation permissions, launch method, test journey, capture tools, and manual owners.
Accepted targeted amendments: Not activated — Chat Session must refresh and supply the exact list or None.
Predecessor deviations incorporated: Not activated — Chat Session must refresh and supply the exact list or None.
Unresolved blockers and assumptions: Not activated — Chat Session must refresh and supply the exact list or None.
Authorization boundary: Not activated — a future activation may authorize only GFD-P7-WP01.
Authorization invalidates when: Not activated — a future activation must invalidate on any base-SHA, predecessor acceptance, package identity, target, format, capability, CSP, dependency/lock, SBOM/license, scanner database/policy, workflow permission, recovery, tool, repository-layout, or authorization-boundary change.
```

## 2. Repository and exact starting state

**Repository:** `Mirarsoft/prime-shell`
**Authoritative base branch:** `main`
**Required future starting commit:** Not activated — Chat Session must refresh
and supply the exact accepted and merged `main` SHA after `GFD-P6-WP02`
acceptance and merge.
**Required future implementation branch:** Not activated — Chat Session must
verify absence and supply one exact fresh branch name.

Authoring-time documentation facts, for review only:

- accepted WP01 implementation head before squash merge:
  `bd502ffcd4c2d80bd3cd8817ebea4dfb18a76107`;
- accepted WP01 squash-merged `main`:
  `35d53bffec6e5b05aefdf8c7fed39a9bfdf288a2`;
- accepted WP01 Ubuntu evidence: Actions run `30285523445`, job
  `90042226699`, successful;
- accepted Stage 1 foundation head:
  `cf2c2613d1facca1e61ba03eab88078a0fc1ffbf`;
- approved-provisional P6-WP01 prompt documentation head:
  `8f35f88dac96f9ab2921ecd1962a59f3102e40f9`;
- approved-provisional P6-WP02 prompt documentation head:
  `f385fcd7f29ba3de02259689b761cac302aa2b95`;
- P6-WP02 prompt parent:
  `8f35f88dac96f9ab2921ecd1962a59f3102e40f9`;
- P6-WP02 prompt tree:
  `d1cbe091cc2a1a14c45ade94c23a97606e4a5f9a`;
- P6-WP02 prompt blob:
  `7eccafe6ac4a8be3bf942ca4cead5646e21d6383`;
- P6-WP02 prompt SHA-256:
  `7227ee5433ebef9ae518be24711449fbe34c1e18e79d28c9f27782d15504ae8b`;
- P6-WP02 authoring run:
  `20260728T183216Z`;
- P6-WP02 authoring-report SHA-256:
  `7ec730ea5dc5249bca9d7eca69f826f20be9a551008d22a6368fdc630f24417b`;
- P6-WP02 review-evidence-index SHA-256:
  `fae578918830718dec1dd7b8f3a13dac18f51c822b69f598d584adff077c78b0`;
- P6-WP02 canonical manifest self-hash:
  `14b9b1eb64f8556653a18ff5e1647a3e057e08786661be9bc784e0e234dde6b4`;
- prompt-pack documentation branch:
  `docs/gfd-work-package-prompt-pack-v1`;
- predecessor activation and implementation through P6-WP02: `Not started`;
- P7-WP01 implementation: `Not started`.

The authoring-time merged source remains the accepted WP01 echo spike. It has
no accepted Phase 2–6 implementation, production release identity, final
dependency closure, SBOM, release workflow, hardened installed-artifact
journey, or signing/update surface. A future activation must discover those
facts from accepted merged predecessors. This prompt must not invent their
future paths, versions, findings, package bytes, or acceptance.

Required future activation and Work Session preflight:

1. Verify exact access and write permission for `Mirarsoft/prime-shell`.
2. Record the default branch and its exact 40-character SHA.
3. Verify WP01 and every package through P6-WP02 are accepted and merged at
   that SHA.
4. Verify every accepted report, evidence index, manifest, source snapshot,
   digest, CI run/job/artifact, native/manual result, measurement, deviation,
   fallback, limitation, amendment, and blocker.
5. Verify accepted capabilities, CSP, package formats, sidecar identity,
   diagnostics/export, settings migration/recovery, single-instance,
   process-containment, privacy, and no-replay authority.
6. Inventory current production/test capabilities, Tauri config, workflows,
   manifests, locks, generated outputs, sidecar resources, package metadata,
   notices, scanners, security docs, install tests, and target claims.
7. Verify no P7 hardening, signing, notarization, updater, publication,
   additional format, release channel, store, or deployment work exists
   unexpectedly.
8. Verify the named fresh implementation branch is absent.
9. Use a clean fresh clone or worktree at the activated starting commit.
10. Narrow every allowed and protected path against that exact merged tree.
11. Stop if repository state differs from activation, is ambiguous, would
    overwrite unrelated work, or would require rebasing or rewriting history.

The merged repository is authoritative over snapshots, documentation branches,
unmerged branches, chat summaries, provisional prompts, and superseded
handoffs.

## 3. Mandatory authority and reading order

Read completely, in this order, before changing repository files:

1. `AGENTS.md`
2. `docs/authority/Core-Functionality-DeliveryRules.md`
3. `docs/authority/generic-fluent-desktop-project-handoff.md`
4. `docs/authority/generic-fluent-desktop-app-architecture-v0.2.md`
5. `docs/authority/architecture-review-consolidation-decision-log.md`
6. all accepted Markdown under `docs/phase-0a/`
7. `docs/work-packages/roadmap-index.md`
8. `docs/work-packages/shared-ground-rules.md`
9. `docs/work-packages/dependency-matrix.md`
10. accepted WP01 source, review, report, evidence, manifest, snapshot, hashes,
    CI jobs, package, and native evidence named by activation
11. accepted P0B-WP02 and P0B-WP03 prompts, activations, sources, reviews,
    reports, manifests, snapshots, lifecycle, package, platform, process,
    measurement, deviation, fallback, and blocker evidence
12. accepted P1-WP01 prompt, activation, source, review, baseline report,
    manifest, snapshot, toolchain/dependency pins and locks, support claims,
    deviations, and blockers
13. accepted P2-WP01 and P2-WP02 prompts, activations, sources, reports,
    manifests, snapshots, theme, shell, layout, accessibility, CSP, native,
    package, fallback, deviation, and blocker evidence
14. accepted P3-WP01 and P3-WP02 prompts, activations, sources, reports,
    manifests, snapshots, schemas, operations, native intent, task/runtime,
    sidecar, process, capability, package, measurement, deviation, and blocker
    evidence
15. accepted P4-WP01 and P5-WP01 prompts, activations, sources, reports,
    manifests, snapshots, feature journeys, static contracts, settings,
    native/package, friction, measurement, deviation, and blocker evidence
16. accepted P6-WP01 and P6-WP02 prompts, activations, sources, reports,
    manifests, snapshots, settings/migration/single-instance, diagnostics,
    export/privacy, repair/recovery, package/process, measurement, deviation,
    and blocker evidence
17. every separately accepted targeted architecture amendment named by
    activation
18. current production/test configuration, capabilities, CSP, Tauri/Rust,
    JavaScript, Python, sidecar, generated, packaging, workflow, toolchain,
    lockfile, license, notice, SBOM, scanner, install-test, security, recovery,
    and support-claim surfaces
19. the exact activated revision of this prompt

The first future implementation progress message must state:

> `AGENTS.md, repository authority, accepted Phase 0A, accepted and merged
> predecessors through GFD-P6-WP02, their accepted reports and artifacts,
> accepted target/package identities, diagnostics/recovery authority, and the
> activated GFD-P7-WP01 prompt have been read and are active. Executing only
> GFD-P7-WP01 with GPT-5.6 Sol / Extra High. GFD-P7-WP02, signing,
> notarization, updater, publication, deployment, product/customer work, PR
> creation, and merge remain unauthorized.`

If a required source is missing, unreadable, stale, or materially
inconsistent, stop before writing. Do not substitute a broad architecture
review, rerun predecessors as acceptance evidence, or invent future package,
dependency, license, target, finding, signing, publication, or support facts.

## 4. Dependencies, predecessor outputs, and prerequisite evidence

**Direct prerequisite:** Accepted and merged `GFD-P6-WP02`, including its
bounded diagnostics record and retention authority, sanitized
preview-before-export, path-safe Rust export, privacy-marker evidence, P3
backend recovery integration, targeted P6-WP01 settings repair, no-replay
behavior, complete report, evidence index, manifest, source snapshot, hashes,
native/package evidence, measurements, deviations, limitations, and blockers.

**Inherited prerequisites:** Accepted and merged WP01, P0B-WP02, P0B-WP03,
P1-WP01, P2-WP01, P2-WP02, P3-WP01, P3-WP02, P4-WP01, P5-WP01, and
P6-WP01 source and evidence.

**Direct dependent:** `GFD-P7-WP02`, which owns signing, notarization,
stapling, protected signing identities, signed update channels, updater
enablement, and signed-release verification after this package is accepted.

P7-WP01 cannot activate while any prerequisite exists only as a provisional
prompt, unmerged branch, unaccepted report, stale target/package evidence, or
unresolved blocker affecting this scope.

Activation must supply exact:

- accepted merged commits and tree identities;
- reports, evidence indexes, manifests, snapshots, byte lengths, and digests;
- CI run/job/artifact identities and per-target native/manual evidence;
- the authoritative production command/capability/permission/CSP inventory;
- every direct, transitive, generated, vendored, tool, plugin, sidecar, and
  packaged dependency plus its lock/provenance source;
- target-specific product identity, package name, format, engine, OS,
  architecture, install location, resource inventory, upgrade identity, and
  uninstall behavior;
- SBOM format/tool, license policy, notice rules, vulnerability database,
  secret scanner, supply-chain checks, finding owners, and exception policy;
- clean-machine or equivalent isolation definition and exact installed-native
  journey;
- P6 migration, backup, previous-copy, repair, diagnostics export, backend
  recovery, single-instance, process-containment, and no-replay evidence;
- production performance, size, reliability, cleanup budgets or accepted
  measurement-only rules;
- measurements, deviations, fallbacks, amendments, limitations, and blockers.

Acceptance of this prompt does not accept predecessor implementation by
inference, activate this package, authorize P7-WP02, or approve signing,
notarization, updater work, publication, deployment, product/customer work,
stores, extra formats, extra architectures, or release claims.

## 5. Objective and measurable runnable outcome

Harden the accepted application and its declared installable artifacts so
they are reviewable production candidates before any signing or updater path
is enabled.

The future package must prove this exact progression for each activated target:

```text
accepted P6-WP02 source and package baseline
→ one authoritative production/test/fault surface inventory
→ least-privilege production commands, capabilities, permissions, and strict CSP
→ locked complete dependency and packaged-component closure
→ reproducible SBOM, license inventory, and required notices
→ vulnerability, secret, integrity, and supply-chain gates
→ target-specific unsigned installable artifact with correct metadata
→ package-content and forbidden-file inspection
→ clean-machine or equivalent isolated installation and native launch
→ accepted feature, settings, diagnostics, recovery, and cleanup smoke
→ measured security, size, performance, reliability, and recovery evidence
→ review-ready unsigned artifact and documentation, without publication
```

Planning, configuration alone, a source build, unit tests, a package file that
was not installed, an older predecessor artifact, a browser-only run, or one
target standing in for another cannot satisfy the outcome.

Activation must refresh the expected matrix before any write. The anticipated
scope is:

| Target | Runtime | Sole package claim before P7-WP02 |
|---|---|---|
| Windows 11 | x64 / WebView2 | one unsigned NSIS installer candidate |
| current and previous activated macOS major versions | arm64 / WKWebView | unsigned and therefore untrusted `.app` plus DMG-layout candidate |
| Ubuntu 24.04 | x64 / WebKitGTK, with Wayland/X11 classified separately | `.deb` only |

The table is authoring-time expectation, not activated acceptance. Activation
must state the exact retained versions and packages. It may narrow a claim
truthfully but may not silently add formats, architectures, Linux
distributions, stores, trust claims, signing, notarization, or publication.

## 6. Explicit in-scope work

The activated Work Session may perform only the smallest coherent change set
needed for the following work.

### 6.1 Evidence-driven release-surface consolidation

Inventory all production, development, test, evidence, fault, packaging, and
workflow surfaces. Classify each as retained production, retained build-only,
retained test-only, generated, packaged resource, removed, or blocked. Leave
one authoritative production configuration per target and no ambiguous
debug/release hybrid.

### 6.2 Production capabilities and permissions

- Enumerate every Tauri command, channel, event, capability, permission,
  plugin, sidecar, resource, dialog intent, file intent, process privilege,
  window, protocol, navigation target, and platform entitlement.
- Grant only capabilities used by accepted production journeys.
- Keep Rust authorization independent of frontend metadata.
- Deny generic shell/process/filesystem/network/native-path/payload tunnels.
- Keep fault injectors, privacy markers, test drivers, evidence writers,
  Webdriver commands, debug bypasses, alternate stores, and developer
  sidecars outside production capability files and packages.
- Verify generated permissions and plugin grants, not only handwritten files.

### 6.3 Strict production CSP and content boundary

- Use a strict release CSP with no wildcard, unsafe inline/eval, arbitrary
  remote navigation, remote script, remote frame, remote websocket, or
  developer-server exception unless activation identifies a narrowly required
  accepted directive.
- Keep development/test CSP separate from production.
- Prove packaged assets, Tauri asset protocol, fonts, icons, Fluent UI,
  portals, workers if any, and accepted local content function under the
  strict policy.
- Keep untrusted document, diagnostics, settings, and backend content rendered
  as text or bounded typed UI, never executable markup.
- Treat every exception as a named finding with owner and evidence.

### 6.4 Development, test, evidence, and fault separation

- Compile, register, enable, and package test/fault/evidence surfaces only
  through explicit non-production mechanisms.
- Make release builds fail if a forbidden command, capability, flag,
  fixture, evidence path, test route, Webdriver surface, privacy marker, fault
  operation, or debug resource is reachable or packaged.
- Do not rely on hidden UI, undocumented flags, tree shaking, or filename
  convention as the security boundary.
- Verify source, generated outputs, executable strings where appropriate, and
  final package inventory.

### 6.5 Locked production dependency closure

Build one reconciled production closure covering:

- Rust crates, features, build dependencies, transitive crates, Tauri core,
  plugins, target dependencies, and bundled native libraries;
- JavaScript/TypeScript packages, transitive packages, package manager,
  lockfile, build tooling, and production bundle;
- Python requirements, transitive packages, PyInstaller, hooks, hidden
  imports, runtime libraries, sidecar build inputs, and packaged onedir
  contents;
- generated bindings, schemas, fixtures required at runtime, code generators,
  packaging tools, CI actions, OS packages, and downloaded build inputs;
- every application binary, sidecar, dynamic library, framework, resource,
  license, notice, and generated component present in the artifact.

All production versions and integrity data must be lock-backed or explicitly
explained. Floating versions, mutable action tags where immutable references
are required, undeclared downloads, editable/local-path production
dependencies, and environment-dependent resolution are blockers.

### 6.6 SBOM, licenses, and notices

- Generate one activation-approved machine-readable SBOM format for each
  target artifact and one reconciled human-reviewable component inventory.
- Include Rust, JavaScript, Python, Tauri/plugin, PyInstaller, bundled native,
  generated, vendored, and packaged components rather than only direct
  manifests.
- Bind each SBOM to the exact commit, target, package filename, artifact
  digest, generator, version, and generation command.
- Reconcile SBOM contents against lockfiles and inspected package contents.
- Classify licenses, notices, attribution, source-offer or redistribution
  duties, unknown licenses, dual licenses, exceptions, and policy owners.
- Produce required third-party notices without copying secrets, local paths,
  user data, or irrelevant build-host metadata.
- Treat missing, unknown, conflicting, or prohibited license facts as
  blockers unless Chat Session accepts an exact limited exception separately.

### 6.7 Security and supply-chain gates

Run activation-pinned checks for:

- known vulnerabilities across Rust, JavaScript, Python, Tauri/plugins,
  sidecar, bundled libraries, packaging tools, and relevant OS components;
- secrets, credentials, private keys, signing material, tokens, native paths,
  environment dumps, user data, and sensitive fixtures;
- dependency integrity, lock drift, duplicate/mismatched versions,
  unreviewed build scripts, unsafe features, unexpected binaries, and
  undeclared network fetches;
- source/package forbidden files, suspicious permissions, archive/path
  hazards, writable executable locations, and provenance gaps.

Every finding must have exact component, version, source, scanner/database,
severity, reachability or exposure facts, disposition, owner, evidence, and
expiry where applicable. Unresolved `Critical` or `High` findings block
acceptance unless Chat Session separately accepts an exact time-bounded
exception naming the risk, scope, evidence, owner, mitigation, and expiry.
This package may record but cannot self-approve such an exception.

### 6.8 Package identity and metadata

- Freeze exact product name, executable, bundle identifier, semantic version,
  publisher/vendor text, copyright, descriptions, icons, package filenames,
  architecture labels, install scope, upgrade identity, and uninstall entry.
- Remove spike, echo, test, debug, placeholder, generic development, or
  contradictory metadata from production artifacts.
- Keep target metadata consistent with source authority and package contents.
- Do not claim a trusted publisher, signature, notarization, Gatekeeper
  acceptance, SmartScreen reputation, verified updater, or public release.

### 6.9 Target-specific unsigned artifact creation

- Build only the activated Windows x64 installer, macOS arm64 app/DMG-layout,
  and Ubuntu x64 `.deb` candidates on target-appropriate trusted runners.
- Keep target builds and results separate.
- Use locked tools and captured versions.
- Build from the exact reviewed commit with a clean dependency restore and no
  uncommitted source.
- Record all build inputs, environment classifications, output filenames,
  byte lengths, hashes, package metadata, and limitations.
- Do not sign, notarize, staple, publish, upload to a release, enable updater
  channels, or create an additional package.

### 6.10 Package content and forbidden-file inspection

Inspect each artifact and installed tree for the exact expected binaries,
sidecar, libraries, frameworks, assets, licenses, notices, and metadata.
Verify safe paths, bounded inventory, permissions, ownership, executable bits,
symlinks, install scripts, desktop entries, uninstall metadata, and absence of:

- source maps or source trees not explicitly accepted;
- test/fault/evidence commands, fixtures, drivers, privacy markers, or logs;
- caches, settings, diagnostics exports, user documents, temp files, or
  previous packages;
- credentials, signing files, environment dumps, native source paths, or CI
  metadata;
- system Python, source-tree sidecar fallback, developer server references,
  mutable downloads, or unsupported architectures;
- additional formats, update metadata, release-channel endpoints, telemetry,
  remote support, or customer/product content.

### 6.11 Clean-machine or equivalent installed-native proof

For each activated target:

- define and record the clean-machine or equivalent isolated baseline;
- install through the real target package mechanism;
- verify package identity, installed files, permissions, shortcuts/desktop
  integration where claimed, and no unintended startup or network behavior;
- launch the installed application natively, not from the build tree;
- prove WebView2, WKWebView, or WebKitGTK identity and version;
- run the accepted shell, feature, command, settings, single-instance,
  diagnostics preview/export, backend status/recovery, and safe shutdown smoke;
- uninstall or remove through the target mechanism and inspect bounded
  cleanup, while preserving user data only according to accepted policy;
- keep raw captures and limitations separate by target.

Unsigned/untrusted platform prompts are expected before P7-WP02 and must be
reported truthfully. They may not be bypassed or described as signed trust.

### 6.12 P6 migration, backup, repair, and recovery readiness

- Prove installation and upgrade-equivalent package transitions preserve the
  accepted canonical settings schema and supported migration chain.
- Exercise current-version, oldest-supported migration, corrupt-primary
  previous-copy recovery, interrupted-write cleanup, section-scoped reset,
  all-settings reset, unsupported-future-version refusal, and unrelated-valid
  section preservation as activated.
- Prove diagnostics preview/export remains sanitized and bounded in the
  installed application.
- Prove backend recovery restores availability only, obeys P3 restart/circuit
  and process-containment authority, and never replays uncertain work.
- Prove one primary instance owns settings and backend lifecycle; a secondary
  does not create a second writer or sidecar.
- Record exact backup, repair, migration, uninstall, and retained-data
  limitations without inventing updater or rollback behavior.

### 6.13 Workflow and CI least privilege

- Create or harden only release-candidate build, inspection, SBOM, license,
  scanner, installed-smoke, and evidence jobs required by this package.
- Pin actions/tools according to activated policy and minimize token,
  repository, artifact, package, and environment permissions.
- Use no signing, notarization, updater, publication, deployment, release,
  store, or customer credentials.
- Do not run untrusted code with privileged tokens or write permissions.
- Keep target jobs separate; allow target build jobs to parallelize only
  within this package, then converge before final acceptance.
- Retain exact workflow, run, job, artifact, tool, database, permission, and
  limitation identities.

### 6.14 Security and threat-boundary reconciliation

Update focused security/release documentation to match the implemented
boundary: trusted native Python is not a sandbox; Rust owns authorization,
paths, settings writes, export, process lifecycle, and production
capabilities; the webview is untrusted presentation; remote code/content is
not allowed; diagnostics are local and allowlisted; test/fault surfaces are
excluded; packages are unsigned; findings and target claims are exact.

Reconcile documentation against source, generated capability files, package
inventory, SBOM, scanners, and installed behavior. Documentation cannot
override contrary code or evidence.

### 6.15 Raw performance, size, and reliability evidence

Retain unrounded raw measurements for each target where applicable:

- clean dependency restore, build, package, SBOM, scan, install, first launch,
  warm launch, shutdown, uninstall, and cleanup time;
- package, installed tree, main binary, web assets, sidecar, runtime library,
  SBOM, notice, and evidence sizes;
- idle and journey memory, backend startup, feature task, diagnostics preview
  and export, settings write/migration/recovery, single-instance forwarding,
  backend recovery, and descendant cleanup;
- repeated install/launch/shutdown/uninstall and recovery outcomes;
- variance, sample count, runner/hardware, OS/engine, tools, failures, and
  limitations.

Use activated budgets where supplied. Otherwise report measurements without
inventing service-level objectives, release promises, or cross-target
comparability.

### 6.16 Release-readiness documentation without release action

Produce focused instructions for reproducing the exact unsigned artifacts,
dependency closure, SBOM, license/notice outputs, scanner gates, package
inspection, clean installation, native smoke, recovery checks, and known
blockers. State explicitly that signing, notarization, stapling, updater,
channel metadata, publication, deployment, stores, and customer release await
separate P7-WP02 and publication authority.

## 7. Explicit exclusions and prohibited work

The future package must not:

- activate or implement `GFD-P7-WP02` or any other package;
- create, import, request, expose, rotate, test, or use signing certificates,
  private keys, Apple identities, Windows signing identities, notarization
  credentials, updater keys, release tokens, or store credentials;
- sign, notarize, staple, enable Gatekeeper trust claims, enable SmartScreen
  trust claims, generate signed update metadata, or test signed channels;
- publish a GitHub release, upload a public artifact, deploy, distribute,
  submit to a store, create a release channel, or enable an updater;
- add MSI, AppImage, RPM, Flatpak, Snap, PKG, store packages, enterprise
  deployment formats, extra architectures, extra Linux distributions, or
  extra macOS/Windows claims;
- add telemetry, crash upload, remote support, analytics, a network endpoint,
  customer diagnostics, enterprise export, or data collection;
- add product/customer features, domain logic, databases, cloud sync,
  accounts, licensing, monetization, public SDKs, runtime plugins,
  `packages/ui`, templates, generators, or a second application;
- weaken CSP, capabilities, Rust authorization, path secrecy, settings
  single-writer ownership, export privacy, process containment, circuit
  behavior, or no-replay guarantees;
- resolve a finding by suppressing, downgrading, excluding, or relabeling it
  without exact evidence and activated policy;
- represent an unsigned, untrusted, uninstalled, partially tested, or
  unavailable artifact as signed, trusted, releasable, supported, or passed;
- open a PR, enable auto-merge, merge, rebase, squash, force-push, rewrite
  accepted history, delete a branch, or change repository settings.

## 8. Allowed and protected paths

A future activation must resolve exact paths against the accepted merged tree.
The allowlist may include only paths necessary for:

- production Tauri configuration, capabilities, permissions, CSP, package
  metadata, resources, and target configuration;
- Rust, frontend, Python, and sidecar changes strictly required to separate
  production from test/fault/evidence surfaces;
- manifests, lockfiles, generated dependency metadata, SBOM/license/notice
  rules, and target package inventories;
- release-candidate build, inspection, scanner, installed-smoke, recovery,
  performance, and evidence scripts/workflows;
- focused tests, fixtures containing synthetic non-sensitive markers, and
  package inspection rules;
- focused production security, supply-chain, package, recovery, and
  release-readiness documentation.

Protected unless a targeted amendment and refreshed activation explicitly
authorize otherwise:

- repository authority and accepted Phase 0A records;
- Stage 1 authoring governance and accepted provisional prompts;
- unrelated product/UI/domain source;
- public package or plugin surfaces;
- signing, notarization, updater, channel, publication, deployment, store, or
  enterprise-distribution configuration;
- credentials, secret stores, external release systems, and repository
  settings.

Do not pre-authorize paths that do not exist. Stop when the activated
allowlist cannot contain the required change without unrelated work.

## 9. Ordered future implementation sequence

All steps below are future requirements. This provisional prompt performs none
of them.

1. Read all activated authority and accepted predecessor evidence, verify
   permissions, exact base SHA, clean tree, fresh branch, path allowlist,
   absent P7-WP02/release work, target ownership, and stop conditions.
2. Generate exactly one task-start UTC run ID, resolve all four deliverable
   names, verify no collisions, and record the fixed evidence-set identity.
3. Inventory source, generated, build, test, fault, evidence, dependency,
   capability, CSP, package, workflow, install, recovery, and documentation
   surfaces against accepted predecessor evidence.
4. Freeze exact activated target/package matrix, clean-target definition,
   package identity, tools/versions, lock policy, production capability/CSP
   baseline, SBOM/license formats, scanners/databases, owners, and budgets.
5. Classify every inventoried surface and reconcile one authoritative
   production path without changing accepted architecture.
6. Harden named production commands, capabilities, permissions, plugins,
   resources, navigation, and Rust authorization to least privilege.
7. Harden strict production CSP and content/navigation/resource rules while
   preserving accepted packaged journeys.
8. Separate development, test, evidence, privacy-marker, fault, Webdriver,
   and debug surfaces from production source reachability, capabilities, and
   package contents.
9. Lock and reconcile the complete Rust, JavaScript, Python, Tauri/plugin,
   sidecar, generated, packaging, action, toolchain, native-library, and
   packaged-component dependency closure.
10. Generate and reconcile target-bound SBOMs, license inventory, policy
    results, third-party notices, and redistribution obligations.
11. Run pinned vulnerability, secret, integrity, lock-drift, unsafe-feature,
    build-script, provenance, and supply-chain checks over source and complete
    artifact inputs.
12. Resolve every finding within scope; stop on any unresolved Critical or
    High unless Chat Session separately accepts an exact time-bounded
    exception.
13. Build only the activated unsigned Windows x64, macOS arm64, and Ubuntu
    x64 artifact candidates from the exact clean commit on appropriate
    target runners.
14. Inspect each package and installed-tree inventory, permissions, metadata,
    binaries, sidecar, libraries, assets, licenses/notices, scripts, and
    forbidden files.
15. Install each artifact on its clean-machine or accepted equivalent target
    and prove native launch with the exact engine and unsigned/untrusted
    classification.
16. Run the accepted shell, theme, accessibility, feature, command, task,
    path-secrecy, settings, single-instance, diagnostics preview/export,
    backend-status, recovery, repair, privacy, CSP, capability, and production
    exclusion journeys in the installed application.
17. Prove migration, previous-copy recovery, interrupted-write cleanup,
    section/all reset, unsupported-future safety, unrelated-section
    preservation, uninstall/remove, retained-data policy, and no replay.
18. Prove bounded shutdown, cancel/timeout/crash/hang/restart/circuit behavior
    relevant to smoke, one backend owner, process-tree containment, cleanup,
    and zero surviving descendants.
19. Capture raw security, dependency, package, install, launch, task,
    diagnostics, settings, recovery, memory, size, reliability, uninstall,
    and cleanup measurements and limitations per target.
20. Reinspect production source, generated output, capabilities, workflows,
    packages, SBOMs, notices, and extracted installed files after all fixes.
21. Reconcile threat/security/release-readiness documentation, finding
    register, target claims, package identity, dependency closure, SBOM,
    license duties, measurements, deviations, limitations, and blockers.
22. Run the complete source, generation, lock, build, test, security, package,
    install/native, accessibility, recovery, production-exclusion, and
    documentation validation from a clean state; fix only in-scope failures
    and rerun affected plus full required checks.
23. Review the exact diff and artifacts, commit one coherent package, perform
    an immediate remote-race check, and ordinary fast-forward push; do not
    open a PR or alter `main`.
24. Build the source snapshot from the exact final commit, verify safe paths,
    one root, CRC and digest, extract into a fresh empty directory, and rerun
    the complete activated validation against that snapshot.
25. Finalize exactly four deliverables with verified hashes and one run ID,
    verify remote/local parity and unchanged `main`, report every gate and
    status independently, return `READY FOR CHAT SESSION REVIEW`, and stop.

Do not reorder security closure after package acceptance, treat one target as
another, begin signing/updater work, or publish an artifact as a shortcut.

## 10. Cross-cutting rules and invariants

- Rust remains authority for native paths, operations, capabilities, settings
  writes, diagnostics export, sidecar lifecycle, process containment, and
  single-instance behavior.
- Python remains trusted native code rather than a sandbox and receives only
  bounded authorized data through accepted contracts.
- React/webview remains untrusted presentation and cannot authorize native or
  backend work through mutable metadata.
- There is one production configuration per target, one dependency closure,
  one package identity, and one finding register for this package.
- Production, test, evidence, fault, and development surfaces are explicit
  and mechanically separated.
- CSP and capabilities are least privilege; no generic tunnel or remote
  content is introduced.
- All queues, files, archives, exports, settings, diagnostics, logs, package
  inventories, scanner outputs, and external artifacts are bounded.
- Paths, credentials, signing material, environment dumps, user content,
  opaque-reference backing values, raw diagnostics, and secrets never enter
  UI, logs, screenshots, CI output, SBOM metadata, notices, or handoff files.
- Every target, package, engine, tool, scanner database, finding, and evidence
  layer is identified exactly and classified independently.
- No automatic retry or replay of uncertain/non-idempotent work is added.
- Unsigned/untrusted status is truthful and cannot be promoted by wording.
- Implementation status uses only `Implemented`, `Partially implemented`,
  `Stub`, `Mock-only`, `Not started`, or `Blocked`.
- Evidence status uses only `Passed`, `Failed`, `Partial`, `Blocked`, or
  `Not run`.
- `Partially implemented`, `Partial`, and `Blocked` require a concise reason
  and exact evidence reference.
- Prompt acceptance, activation, implementation review, PR authorization,
  merge authorization, signing, and publication are separate decisions.

## 11. Required validation, evidence, and measurements

Future validation must keep these evidence classes separate:

1. authority, activation, exact-base, branch, permission, and path evidence;
2. source and generated production/test/fault-surface inventory;
3. command, capability, permission, plugin, resource, and CSP evidence;
4. Rust, JavaScript, Python, Tauri/plugin, sidecar, generated, packaging,
   action, toolchain, native-library, and package dependency closure;
5. SBOM generation, reconciliation, artifact binding, and reproducibility;
6. license inventory, policy, notices, obligations, and unknowns;
7. vulnerability findings by ecosystem, scanner, database, and reachability;
8. secret, credential, signing-material, native-path, and sensitive-data
   negative evidence;
9. integrity, lock drift, unsafe feature, build-script, provenance, and
   undeclared-download evidence;
10. target-specific source build and unsigned package creation;
11. package/archive/installed-tree content, metadata, permission, script,
    binary, sidecar, and forbidden-file inspection;
12. clean-machine or equivalent isolation, install, native launch, exact
    engine, uninstall/remove, and cleanup evidence;
13. accepted feature, accessibility, settings, single-instance, diagnostics,
    export, recovery, repair, privacy, no-replay, CSP, and capability
    installed-native smoke;
14. P3 process containment, backend lifecycle, cancellation, timeout, crash,
    hang, restart, circuit, shutdown, and zero-descendant regression;
15. target workflow permissions, runs, jobs, tools, databases, artifacts, and
    retention evidence;
16. raw performance, size, memory, reliability, migration, recovery, and
    cleanup measurements;
17. threat-boundary and release-readiness documentation reconciliation;
18. source snapshot, fresh extraction, clean rerun, digest, and path safety;
19. unavailable, blocked, partial, accepted-exception, deviation, limitation,
    and manual-owner evidence.

For every command or check, retain exact command/method, exit status, expected
and actual result, commit, target OS/version/architecture, engine, package,
runner/hardware, tool/version, scanner database/time, capture time, raw output
artifact, classification, and limitation.

Future scenarios must include at minimum:

- release and development capabilities/CSP remain separate;
- every accepted production command works and every forbidden command is
  unavailable;
- remote navigation/content, unsafe evaluation, generic invoke, shell,
  process, filesystem, path, and payload tunnels are rejected;
- test, fault, privacy-marker, evidence, debug, and Webdriver surfaces are
  absent from production;
- locks reproduce dependency closure and drift fails;
- SBOM maps to locks and inspected packaged components;
- license/notice obligations are complete and unknown/prohibited cases block;
- clean secret scans include source, history scope named by activation,
  generated files, packages, SBOMs, notices, and artifacts;
- every vulnerability finding has exact disposition and unresolved
  Critical/High blocks;
- package metadata and architecture are exact per target;
- package inspection detects each synthetic forbidden-file rule;
- safe install, launch, smoke, shutdown, uninstall/remove, and cleanup occur
  per target;
- unsigned/untrusted prompts are recorded without bypass or trust claims;
- settings persist, migrate, recover, reset, and preserve unrelated sections;
- unsupported-future settings remain untouched;
- secondary instance neither writes settings nor starts a backend;
- diagnostics preview precedes export and contains only frozen allowlisted
  sanitized data;
- failed/cancelled export leaves no partial output;
- backend recovery obeys restart/circuit/process/no-replay rules;
- accepted feature and task journeys retain safe rendering/path secrecy;
- slow/failed backend and recovery journeys leave truthful terminal state;
- shutdown and fault paths leave zero surviving descendants;
- restricted/network-free expectations are observed where activated;
- repeated package/install/native journeys retain raw reliability results;
- source snapshot extracts safely and complete rerun reproduces results.

The review evidence index must map every acceptance gate to exact source/tests,
command and exit status, workflow run/job/artifact, native/manual evidence,
measurement, classification, limitation, accepted exception if any, and
deliverable digest.

## 12. Measurable acceptance gates

All 29 gates are unsatisfied while this prompt remains provisional:

1. Exact accepted and merged predecessors through P6-WP02 were used from the
   exact clean activated `main`.
2. The final diff remains inside the activated allowlist with no P7-WP02,
   signing, notarization, updater, publication, deployment, product/customer,
   store, extra-format, prompt-pack, or unrelated work.
3. One authoritative production configuration and one explicit
   production/development/test/evidence/fault classification exist per target.
4. Every production Tauri command, capability, permission, plugin, channel,
   resource, window, protocol, and native intent is named, least privilege,
   used, and independently authorized by Rust where required.
5. Production source, capabilities, generated outputs, and packages exclude
   fault injectors, privacy markers, test drivers, evidence writers,
   Webdriver/debug commands, alternate writers, developer sidecars, and
   generic tunnels.
6. Strict production CSP forbids unaccepted remote content/navigation,
   wildcards, unsafe evaluation, developer exceptions, and executable
   untrusted content while every accepted installed journey remains usable.
7. One locked production dependency closure covers direct, transitive,
   generated, vendored, tool, plugin, sidecar, packaging, workflow, native,
   and packaged components across Rust, JavaScript, Python, and Tauri.
8. Clean restoration and generation from locks are deterministic; floating
   versions, mutable unapproved action/tool references, undeclared downloads,
   editable paths, and lock/package drift fail.
9. Target-bound SBOMs are reproducible, bind to exact commit and package
   digest, cover all packaged ecosystems/components, and reconcile with locks
   and inspected contents.
10. License inventory, policy classification, third-party notices, and
    redistribution obligations are complete; unknown, conflicting, or
    prohibited cases block unless separately accepted exactly.
11. Vulnerability, secret, integrity, unsafe-feature/build-script,
    provenance, and supply-chain checks use activated pinned tools/databases
    and retain exact raw findings.
12. No unresolved Critical or High finding remains unless Chat Session
    separately accepted one exact time-bounded exception with risk, scope,
    owner, evidence, mitigation, and expiry.
13. No credential, private key, signing material, token, native path,
    environment dump, user data, raw diagnostic, opaque-reference value, or
    sensitive fixture appears in source, packages, scans, logs, screenshots,
    CI, SBOM/notices, or deliverables.
14. Exact production name, identifier, version, publisher/vendor wording,
    icons, descriptions, filenames, architecture, install scope, upgrade
    identity, and uninstall metadata are consistent and contain no spike,
    echo, test, debug, or false trust claim.
15. Only the activated Windows 11 x64/NSIS, macOS arm64/unsigned app and
    DMG-layout, and Ubuntu 24.04 x64/`.deb` candidates are built; each target,
    engine, format, runner, result, and limitation remains separate.
16. Every package and installed tree has safe paths, expected metadata,
    permissions, binaries, sidecar, libraries, resources, notices, and exact
    bounded inventory with all forbidden files absent.
17. Each declared artifact installs through its real target mechanism on a
    clean machine or accepted equivalent, launches the installed application
    with the exact engine, and retains truthful unsigned/untrusted status.
18. Installed-native shell, theme, accessibility, P4/P5 feature, command,
    settings, single-instance, diagnostics preview/export, backend
    status/recovery, repair, privacy, CSP, capability, and safe-rendering
    smoke passes proportionally on every claimed target.
19. Settings current/oldest-supported migration, atomic write,
    previous-valid-copy recovery, interrupted-write cleanup, scoped/all reset,
    future-version refusal, and unrelated-section preservation remain green.
20. Diagnostics retention and preview/export remain bounded, sanitized,
    allowlisted, path-safe, explicit, privacy-verified, and free of partial
    output on failure or cancellation.
21. Backend recovery restores availability only, respects P3 restart/circuit,
    package identity, process containment, and no-replay rules, and never
    fabricates task success.
22. One primary instance owns settings/backend lifecycle, secondary launch
    forwarding is bounded and allowlisted, and no duplicate writer, backend,
    task replay, or descendant survives.
23. Workflow permissions, tokens, artifacts, environments, and untrusted-code
    boundaries are least privilege, target-specific, and contain no signing,
    release, publication, deployment, or customer credential path.
24. Source, generated, scanner, package-creation, installed/native, engine,
    filesystem, process, accessibility, CI, manual, and unavailable evidence
    remains separate and truthful per target.
25. Raw dependency, scan, build, package, size, install, launch, task,
    diagnostics, settings, recovery, memory, reliability, uninstall, and
    cleanup measurements are retained without fabricated service-level or
    release claims.
26. Security/threat and release-readiness documentation matches source,
    capabilities, CSP, locks, SBOM, licenses/notices, findings, packages,
    installed behavior, limitations, and unsigned status.
27. Exactly four implementation deliverables use one task-start run ID, have
    verified hashes, contain no sensitive data, and represent the reviewed
    final commit.
28. The source snapshot contains one safe expected root, passes CRC/path and
    exact-inventory checks, extracts into a fresh empty directory, and passes
    the complete activated rerun.
29. The implementation branch matches its remote, `main` is unchanged, no PR
    or auto-merge exists, and the handoff stops at
    `READY FOR CHAT SESSION REVIEW` without starting P7-WP02 or release action.

If a gate is unmet, report `Blocked`, `Not run`, `Partial`, or the exact
accepted limitation as an evidence outcome. Do not claim success by inference
or aggregate independent gates into one statement.

## 13. Stop conditions and blocker reporting

Stop before writing if:

- activation is absent, provisional, stale, incomplete, or overbroad;
- repository, base, branch, clean-tree, path, permission, target, or package
  state differs;
- P6-WP02 or another predecessor is unaccepted or unmerged;
- accepted target/package, capability/CSP, dependency/lock, diagnostics,
  recovery, privacy, process, or native evidence is missing or inconsistent;
- current source and accepted authority materially disagree;
- one complete production dependency closure or package inventory cannot be
  determined;
- SBOM/license/notice tools cannot cover a packaged ecosystem truthfully;
- a Critical or High finding remains unresolved without a separately accepted
  exact exception;
- a secret, signing material, sensitive value, user content, diagnostic
  payload, environment dump, or native path appears in any output;
- test/fault/evidence/debug surface exclusion, strict CSP, least privilege,
  artifact integrity, clean installation, native smoke, migration/recovery,
  privacy, no replay, or zero descendants cannot be proven;
- an unsigned or unavailable target would have to be described as trusted,
  releasable, passed, signed, notarized, or supported;
- work requires signing, notarization, updater, publication, deployment,
  store, extra package/architecture, telemetry, remote support,
  product/customer behavior, public SDK/plugin, or another package;
- one target or evidence class would have to substitute for another;
- snapshot integrity or clean extraction rerun fails;
- unauthorized or unrelated files enter the diff;
- remote state changes, push is non-fast-forward, or history rewriting would
  be required.

When blocked:

1. Stop at the last clean non-destructive state.
2. Do not broaden scope, weaken a gate, suppress a finding, or enter P7-WP02.
3. Record the exact failed fact, command/path/component, expected value,
   observed value, severity, affected gates, changed files, tree state, and
   unchanged exclusions.
4. Distinguish repository defect, stale activation, missing authority or
   evidence, dependency/license gap, security finding, environment,
   permission, target/runner, package/tool, platform, or design decision.
5. Use `Blocked` or `Partially implemented` for implementation status and
   `Partial` only for explicitly labeled evidence.
6. Return the smallest safe Chat Session decision, accepted-exception request,
   focused correction, or refreshed activation required.

## 14. Required functional and status inventory

Before activation, the truthful inventory is:

```text
Stage 1 foundation: Implemented and accepted
GFD-P0B-WP02 provisional prompt: Implemented and accepted as Approved provisional
GFD-P0B-WP02 activation/implementation: Not started
GFD-P0B-WP03 provisional prompt: Implemented and accepted as Approved provisional
GFD-P0B-WP03 activation/implementation: Not started
GFD-P1-WP01 provisional prompt: Implemented and accepted as Approved provisional
GFD-P1-WP01 activation/implementation: Not started
GFD-P2-WP01 provisional prompt: Implemented and accepted as Approved provisional
GFD-P2-WP01 activation/implementation: Not started
GFD-P2-WP02 provisional prompt: Implemented and accepted as Approved provisional
GFD-P2-WP02 activation/implementation: Not started
GFD-P3-WP01 provisional prompt: Implemented and accepted as Approved provisional
GFD-P3-WP01 activation/implementation: Not started
GFD-P3-WP02 provisional prompt: Implemented and accepted as Approved provisional
GFD-P3-WP02 activation/implementation: Not started
GFD-P4-WP01 provisional prompt: Implemented and accepted as Approved provisional
GFD-P4-WP01 activation/implementation: Not started
GFD-P5-WP01 provisional prompt: Implemented and accepted as Approved provisional
GFD-P5-WP01 activation/implementation: Not started
GFD-P6-WP01 provisional prompt: Implemented and accepted as Approved provisional
GFD-P6-WP01 activation/implementation: Not started
GFD-P6-WP02 provisional prompt: Implemented and accepted as Approved provisional
GFD-P6-WP02 activation/implementation: Not started
GFD-P7-WP01 package outcome: Not started
Production surface inventory and classification: Not started
Least-privilege production commands/capabilities/permissions: Not started
Strict production CSP and content boundary: Not started
Development/test/evidence/fault separation: Not started
Locked Rust dependency closure: Not started
Locked JavaScript dependency closure: Not started
Locked Python and sidecar dependency closure: Not started
Tauri/plugin/generated/packaging/workflow dependency closure: Not started
Target-bound SBOM generation and reconciliation: Not started
License inventory and policy: Not started
Third-party notices and obligations: Not started
Vulnerability and supply-chain gates: Not started
Secret and sensitive-data exclusion: Not started
Critical/high finding closure: Not started
Production package identity and metadata: Not started
Windows unsigned NSIS candidate hardening: Not started
macOS unsigned app and DMG-layout candidate hardening: Not started
Ubuntu .deb candidate hardening: Not started
Package content and forbidden-file inspection: Not started
Clean-machine or equivalent installation: Not started
Installed-native accepted journey smoke: Not started
P6 migration/backup/repair/recovery readiness: Not started
Single-instance and zero-descendant regression: Not started
Workflow and CI least privilege: Not started
Security/threat-boundary reconciliation: Not started
Performance, size, and reliability measurements: Not started
Release-readiness documentation: Not started
Supporting infrastructure: Not started
Tests: Not started
Documentation for implementation: Not started
Generated code: Not started
Fixtures/mocks: Not started
Stubs/placeholders: Not started
Incomplete work: Not started
Blocked work: Blocked — P6-WP02 is not accepted and merged and P7-WP01 has no activated prompt
GFD-P7-WP02: Not started
Signing/notarization/updater: Not started
Publication/deployment/store distribution: Not started
Product/customer features: Not started
Public SDK/plugin/packages-ui: Not started
PR creation: Not started
Merge: Not started
Branch deletion: Not started
```

At future completion, update every line independently using only:

- `Implemented`
- `Partially implemented`
- `Stub`
- `Mock-only`
- `Not started`
- `Blocked`

Evidence outcomes may use `Passed`, `Failed`, `Partial`, `Blocked`, or
`Not run` only when explicitly labeled as evidence. Keep functionality,
infrastructure, tests, documentation, generated code, fixtures, stubs,
incomplete work, and blocked work separate. `Partially implemented` and
`Blocked` require a concise reason and exact evidence reference.

## 15. One RUN_ID and collision-resistant external naming

At the start of the future implementation task, after mandatory authority
reads and preflight but before any repository or deliverable write, generate
exactly one UTC run ID in basic ISO-8601 form:

```text
YYYYMMDDTHHMMSSZ
```

Resolve all four target filenames immediately from that one value and reuse it
unchanged for every implementation deliverable. Never generate or substitute
a second run ID, timestamp, branch label, local time, random suffix, or target
timestamp in the same task.

Before any write, verify none of the four resolved names exists in the
destination. A collision is a stop condition: report `Blocked` without
generating another run ID.

The run ID identifies the evidence set, not the implementation version. The
manifest must record exact reviewed commit, tree, parent, branch, snapshot
root, target artifacts, and every deliverable digest.

## 16. Required deliverables, hashes, and evidence index

Produce exactly these four future implementation deliverables:

1. `prime-shell-work-gfd-p7-wp01-<RUN_ID>-source-snapshot-r1.zip`
2. `prime-shell-work-gfd-p7-wp01-<RUN_ID>-production-security-release-artifact-hardening-report-r1.md`
3. `prime-shell-work-gfd-p7-wp01-<RUN_ID>-review-evidence-index-r1.md`
4. `prime-shell-work-gfd-p7-wp01-<RUN_ID>-handoff-manifest-r1.md`

### Source snapshot

- Build it from the exact reviewed final commit.
- Use one top-level `prime-shell/` root and repository-relative paths.
- Include source, generated bindings/metadata, fixtures, tests, scripts,
  manifests, locks, workflows, SBOM/license/notice rules, capability/CSP
  configuration, and focused security/release documentation needed for review.
- Exclude `.git`, environments, caches, build output, installers, installed
  trees, native captures, settings, diagnostics, exports, logs, scanner
  databases, credentials, signing material, user content, and unrelated files.
- Verify SHA-256, ZIP CRC, safe paths, expected root, and exact inventory.
- Extract into a fresh empty directory and rerun complete activated
  verification.

### Production-security and release-artifact-hardening report

Record:

- activation, model, reasoning, repository, branch, base/final commit, parent,
  tree, and exact changed paths;
- every accepted predecessor commit, report, artifact, digest, evidence,
  deviation, fallback, limitation, amendment, and blocker;
- before/after production/test/fault/evidence surface inventory and ownership;
- exact capabilities, permissions, commands, plugins, resources, CSP, and
  production exclusions;
- complete ecosystem/tool/generated/packaged dependency closure, locks,
  integrity sources, and drift results;
- target-bound SBOMs, reconciliation, license policy, notices, obligations,
  unknowns, and exact output identities;
- every security/supply-chain finding, tool/database, severity, reachability,
  disposition, accepted exception if any, owner, mitigation, expiry, and raw
  evidence;
- package identity, target matrix, build/package inputs, target artifact
  filenames/digests, metadata, package/installed inventories, permissions,
  and forbidden-file results;
- clean-target install/native smoke, exact engines, unsigned/untrusted status,
  migration/backup/repair/recovery, single-instance, process/no-replay,
  uninstall, cleanup, and limitations;
- workflow permissions, raw performance/size/reliability measurements,
  threat-boundary reconciliation, all 29 gates, and full status inventory;
- confirmation that P7-WP02, signing, notarization, updater, publication,
  deployment, stores, extra formats/architectures, product/customer, PR, and
  merge work did not start.

### Review evidence index

- Include one row for each of the 29 acceptance gates.
- Map every gate to exact source/tests, command and exit status, workflow
  run/job/artifact, package/native/manual evidence, tool/database,
  measurement, classification, limitation, accepted exception if any, and
  digest.
- Keep capability/CSP, production exclusion, dependency/lock, SBOM,
  license/notice, vulnerability, secret, integrity/provenance, package
  creation/inspection, installed/native, engine, accessibility, settings,
  diagnostics/privacy, recovery/process, workflow/CI, performance, snapshot,
  manual, and unavailable evidence separate.
- Record target OS/version/architecture, filesystem, engine, package, runner
  or hardware, commit, capture time, expected and actual result.
- Use `None`, `Not run`, `Blocked`, or `Partial` explicitly rather than blank
  cells.

### Handoff manifest and hashing

- List all four deliverables with exact filename, role, byte length, and
  ordinary SHA-256 where applicable.
- Record run ID, snapshot root, reviewed commit, tree, parent, branch,
  generation commands, target artifact identities/digests, target matrix,
  tools/databases, paths, exact results, status inventory, and next controlled
  action.
- Record ordinary SHA-256 for the snapshot, hardening report, and evidence
  index.
- If the manifest records its own digest, designate exactly one self-digest
  value. Copy the final manifest bytes, replace only that designated
  self-digest value with the literal `<SELF_SHA256>`, and compute SHA-256 over
  those exact canonical bytes. Record the lowercase hexadecimal result in the
  designated value.
- Reproduce the self-digest by repeating only that replacement. Do not change
  the field label, spacing, line endings, byte-length field, any other digest,
  or any other byte.
- Recompute every digest after finalization and verify no deliverable changed.
- Never include credentials, secrets, signing material, user content, native
  paths, diagnostics payloads, settings values, environment dumps, raw opaque
  references, or sensitive scanner inputs.

Do not create a fifth deliverable, source SBOM bundle, installer bundle,
scanner database bundle, native-capture archive, or publication artifact.
Target installers and SBOM/license outputs are implementation evidence named
inside the four deliverables, not additional external handoff deliverables.

## 17. Branch, draft-PR, review, merge, and deletion controls

This provisional prompt authorizes no implementation branch or PR. Future
activation must:

- start from exact accepted merged `main` SHA supplied by Chat Session;
- use one fresh implementation branch named by Chat Session;
- never reuse the documentation branch or a predecessor branch;
- keep changes limited to one coherent `GFD-P7-WP01` implementation;
- commit intentionally and ordinary fast-forward push only after complete
  validation and an immediate remote-race check;
- verify final remote commit, parent, tree, paths, hashes, and clean parity;
- leave `main` unchanged;
- stop at `READY FOR CHAT SESSION REVIEW`.

Do not open a draft or ready PR unless a later explicit Chat Session
instruction authorizes that separate action. Do not enable auto-merge. Prompt
acceptance is not activation, review acceptance does not authorize merge, and
P7-WP01 completion does not authorize P7-WP02, signing, or release.

Merge, squash, rebase, force-push, history rewriting, branch deletion,
signing, notarization, stapling, updater/channel work, release creation,
publication, deployment, store submission, customer distribution, and
repository-setting changes require separate explicit authorization.

## 18. Completion response and return prompt

The future implementation response must lead with exactly one status:

- `READY FOR CHAT SESSION REVIEW`
- `BLOCKED`
- `NOT STARTED`

For `READY FOR CHAT SESSION REVIEW`, report concisely:

- model `GPT-5.6 Sol` and reasoning `Extra High`;
- repository, starting commit, branch, final commit, parent, and tree;
- exact changed paths and source snapshot;
- all four deliverable filenames and digests;
- exact target artifact identities, hashes, and unsigned/untrusted status;
- all 29 acceptance-gate outcomes;
- capability/CSP, production exclusion, dependency/lock, SBOM,
  license/notice, vulnerability/secret/provenance, package/installed-native,
  engine, accessibility, settings/diagnostics/recovery/process,
  workflow/CI/manual, performance, snapshot, and unavailable evidence;
- raw measurements, accepted exceptions, limitations, and full status
  inventory;
- confirmation that `main` is unchanged and no PR, merge, branch deletion,
  P7-WP02, signing, notarization, updater, publication, deployment, store,
  extra format/architecture, or product/customer work started.

For `BLOCKED`, report the exact stop condition, command or evidence,
repository state, changed files, unaffected scope, affected gates, finding
severity if applicable, and smallest required Chat Session decision. Do not
claim partial work as implementation success.

End a successful future implementation response with:

```text
Chat Session: Review GFD-P7-WP01 on the exact implementation branch and commit
reported above. Read the production-security/release-artifact-hardening report,
review evidence index, handoff manifest, and verified source snapshot. Return
Accepted, Focused correction required, or Blocked. Confirm least-privilege
production capabilities and strict CSP; production/test/fault separation;
complete locked dependency closure; target-bound SBOM, licenses, notices, and
finding gates; exact unsigned package identities and contents; clean-target
installed-native journeys; migration, repair, recovery, no-replay, and process
containment readiness; workflow permissions; target limitations; raw
measurements; status inventory; and artifact hashes. GFD-P7-WP02, signing,
notarization, updater, publication, deployment, product/customer work, PR
creation, merge, release, and branch deletion remain unauthorized.
```
