# GFD-P7-WP02 — Signing, Notarization, and Signed Update Channels

## 1. Package identity and prompt status

**Package ID:** `GFD-P7-WP02`
**Phase:** `Phase 7`
**Title:** `Signing, Notarization, and Signed Update Channels`
**Task ID:** `GFD-P7-WP02`
**Prompt ID:** `PRIME-SHELL-GFD-P7-WP02-PROMPT`
**Prompt version:** `R1`
**Prompt lifecycle state:** `Provisional`
**Implementation status at authoring:** `Not started`
**Recommended future implementation model:** `GPT-5.6 Sol`
**Recommended future implementation reasoning/intelligence:** `Extra High`
**Authorization boundary:** Exactly one package, `GFD-P7-WP02`
**Execution status:** `Execution is not authorized`

Extra High is the minimum suitable level because this package joins several
independent trust systems without permitting them to collapse into one:
Windows application signing and timestamping, macOS nested signing and
notarization, updater signing, public-verifier distribution, stable/beta
channel isolation, anti-replay and downgrade policy, protected CI
credentials, key backup/rotation/revocation, native update recovery, and a
strict separation between controlled staging and public publication. A small
mistake could expose a private key, let untrusted code reach credentials,
accept the wrong channel or target, turn repair into a downgrade bypass,
misstate notarization as trust, or publish before authorization.

This prompt is complete for review but remains provisional and
non-executable. Chat Session must first accept it as `Approved provisional`
and later issue a separately activated revision after every predecessor,
credential boundary, target, service, and evidence owner is refreshed.
Prompt acceptance is not activation, implementation, key-generation,
credential-use, signing, notarization, updater enablement, publication, PR,
or merge authority.

**Direct prerequisite:** Accepted and merged `GFD-P7-WP01`, with exact
hardened unsigned artifact identities and hashes, target-bound dependency,
SBOM, license, finding, package, installed-native, recovery, and limitation
evidence.

**Direct dependent:** `GFD-P8-WP01 — Template Extraction and Second Branded
Application`, which remains unauthorized.

### Activation metadata

```text
Activation ID: Not activated — Chat Session must refresh and supply this exact value.
Activated by: Not activated — Chat Session must refresh and supply this exact value.
Activation UTC: Not activated — Chat Session must refresh and supply this exact value.
Authoritative main SHA: Not activated — Chat Session must refresh and supply this exact value.
Required fresh implementation branch: Not activated — Chat Session must verify absence and supply this exact value.
Accepted predecessors through GFD-P7-WP01: Not activated — Chat Session must supply exact merged commits, reports, artifacts, hashes, CI runs/jobs, native/manual evidence, measurements, deviations, fallbacks, limitations, accepted exceptions, and blockers.
Accepted hardened unsigned artifacts: Not activated — Chat Session must supply exact target, package, filename, format, architecture, version, metadata, byte length, digest, SBOM/license/finding state, installed-native result, and unsigned/untrusted classification.
Application-signing identities: Not activated — Chat Session must supply exact Windows and macOS identity, certificate chain, provider, key custody, expiry, revocation, timestamp/notarization role, owner, environment, and safely reportable identifier.
Updater-signing trust root: Not activated — Chat Session must supply exact algorithm, parameters, private-key custody, public-verifier distribution, key ID, metadata/artifact signature rules, transition format, owner, and accepted client compatibility.
Protected credential boundary: Not activated — Chat Session must supply exact secret manager or hardware provider, protected environments, trusted refs/runners, reviewers, token permissions, audit, cleanup, manual fallback, break-glass, and unavailable facts.
Windows signing and timestamping service: Not activated — Chat Session must supply exact tools, versions, identity, chain, subject/publisher policy, timestamp service, account/access/legal state, target runner, and independent verifier.
Apple signing and notarization service: Not activated — Chat Session must supply exact Developer ID identity, team, certificate chain, notarization account/profile, agreements, tool versions, hardened runtime, entitlements, target hardware, manual owner, and service access.
Updater implementation and metadata format: Not activated — Chat Session must supply exact Tauri/updater version, canonical schema/bytes, artifact format, public verifier, endpoint policy, redirect/cache/range behavior, target mapping, restart/install authority, and safe-error contract.
Stable and beta channel policy: Not activated — Chat Session must supply exact channel names, version namespaces, signing-root policy, eligibility, origins, storage paths, access, cache, retention, approvals, promotion rules, rollback/downgrade rules, and isolation owner.
Controlled non-public staging origin: Not activated — Chat Session must supply exact provider or accepted local equivalent, access boundary, URLs without secrets, retention, cache, audit, cleanup, trusted uploader, and proof that it is not public production publication.
Version and recovery policy: Not activated — Chat Session must supply exact semantic/build ordering, current/minimum versions, equal-version behavior, downgrade/rollback rule, stale/expiry policy, interrupted update behavior, repair/reinstall limits, settings compatibility, and no-replay authority.
Key backup, rotation, revocation, and compromise plan: Not activated — Chat Session must supply exact owners, custody providers, recovery method, backup/restore evidence, overlap rules, verifier transition, certificate renewal, emergency freeze, withdrawal authority, continuity plan, and blockers.
Target and manual evidence ownership: Not activated — Chat Session must refresh exact Windows/macOS/Linux OS versions, architectures, package formats, webview engines, runners, hardware, tools, installation methods, manual owners, and unavailable evidence.
Release/version/build identity: Not activated — Chat Session must supply exact application version, package identity, build metadata, artifact names, channel metadata version, previous accepted signed client, and activated update path.
Publication state and authority: Not activated — Chat Session must prove all public release, production channel, store, customer rollout, enterprise deployment, release-tag, and marketing actions remain separately unauthorized.
Accepted targeted amendments: Not activated — Chat Session must refresh and supply the exact list or None.
Predecessor deviations incorporated: Not activated — Chat Session must refresh and supply the exact list or None.
Unresolved blockers and assumptions: Not activated — Chat Session must refresh and supply the exact list or None.
Authorization boundary: Not activated — a future activation may authorize only GFD-P7-WP02.
Authorization invalidates when: Not activated — a future activation must invalidate on any base-SHA, predecessor acceptance, artifact identity/hash, target, signing identity, certificate/key, custody, service/account/agreement, tool, updater/schema/verifier, channel/origin, version/downgrade, hardware/owner, publication-state, repository-layout, or authorization-boundary change.
```

## 2. Repository and exact starting state

**Repository:** `Mirarsoft/prime-shell`
**Authoritative base branch:** `main`
**Required future starting commit:** Not activated — Chat Session must refresh
and supply the exact accepted and merged `main` SHA after `GFD-P7-WP01`
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
- approved-provisional P7-WP01 prompt documentation head:
  `3567523b7afff460ffa82ff52fa1a13972459618`;
- P7-WP01 prompt parent:
  `f385fcd7f29ba3de02259689b761cac302aa2b95`;
- P7-WP01 prompt tree:
  `6fe1528b6e49b065c4db71aec3bcf5bfe0d992ca`;
- P7-WP01 prompt blob:
  `478f4ae5c44db9ce8e95437a95cdfa3c6901516e`;
- P7-WP01 prompt SHA-256:
  `c33eae1a6bc1407f4d6b201010342b5ab6e075279ad0e60d580fe7978059e819`;
- accepted P7-WP01 authoring run:
  `20260728T191143Z`;
- P7-WP01 authoring-report SHA-256:
  `f21cefe7dc8086b475c92d6ca83c1326fbd1ac02a6136f8785a732d0c2d63018`;
- P7-WP01 review-evidence-index SHA-256:
  `739c34ac73024106a8d9434a6255dffe4dd78adb208cce0eccf5e9b9449298ea`;
- P7-WP01 canonical handoff-manifest self-hash:
  `fbf954e65bd705c56c04cbb4790b87ba0f5555a4b809bf68d482c88826198b1f`;
- prompt-pack documentation branch:
  `docs/gfd-work-package-prompt-pack-v1`;
- package activation and implementation after WP01: `Not started`;
- `GFD-P7-WP02` implementation: `Not started`.

The authoring-time merged application remains the accepted WP01 Unicode echo
spike. It does not contain accepted Phase 2–7 implementation, production
signing identities, updater trust roots, protected release environments,
stable/beta endpoints, signed artifacts, notarization evidence, production
publication, or a customer release. A future activation must discover the
actual accepted merged predecessor paths and exact external trust facts. This
prompt must not invent credentials, subjects, key IDs, algorithms, endpoint
details, service access, package bytes, channel state, hardware evidence, or
publication authority.

Required future activation and Work Session preflight:

1. Verify exact access and write permission for `Mirarsoft/prime-shell`.
2. Record the default branch and exact accepted merged `main` SHA.
3. Verify every package through P7-WP01 is accepted and merged at that SHA.
4. Reconcile every accepted report, evidence index, manifest, snapshot,
   digest, workflow run/job/artifact, native/manual result, measurement,
   deviation, fallback, limitation, exception, amendment, and blocker.
5. Verify exact hardened unsigned inputs, package identities, target matrix,
   SBOM/license/finding closure, capabilities/CSP, installed-native behavior,
   migration/repair readiness, and package limitations.
6. Inventory current signing, updater, public-verifier, metadata, workflow,
   protected-environment, storage, channel, version, restart, repair,
   recovery, and publication surfaces without reading or exposing secret
   values.
7. Verify every activated credential, service, account, agreement, hardware,
   trusted runner, reviewer, manual owner, and publication prohibition.
8. Verify no P8, public publication, store, customer rollout, enterprise
   deployment, telemetry, extra target/format, or unrelated work exists.
9. Verify the named fresh implementation branch is absent.
10. Use a clean fresh clone or worktree at the exact activated commit.
11. Replace broad path categories with the smallest exact allowlist.
12. Stop if state differs, is ambiguous, would overwrite unrelated work,
    requires a substitute credential, or requires history rewriting.

The merged repository is authoritative over snapshots, documentation branches,
unmerged branches, chat summaries, provisional prompts, and superseded
handoffs. External signing, notarization, timestamping, storage, and credential
systems remain absent for implementation purposes until activation proves
exact access and policy.

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
10. accepted WP01 and P0B-WP02/P0B-WP03 source, prompts, activations,
    reviews, reports, artifacts, hashes, lifecycle, platform, package,
    process, measurement, deviation, fallback, and blocker evidence
11. accepted P1-WP01 through P3-WP02 prompts, activations, source, reports,
    manifests, snapshots, toolchains, schemas, commands, native intent,
    task/runtime, capabilities, package, process, measurements, deviations,
    and blockers
12. accepted P4-WP01 through P6-WP02 prompts, activations, source, reports,
    manifests, snapshots, feature journeys, settings, migration,
    single-instance, diagnostics, privacy, repair, recovery, process,
    package, measurements, deviations, and blockers
13. accepted P7-WP01 prompt, activation, source, report, evidence index,
    manifest, snapshot, exact hardened unsigned artifacts, package metadata,
    dependency closure, SBOM, license/notices, findings, clean-install/native
    evidence, workflow permissions, recovery readiness, measurements,
    exceptions, limitations, and blockers
14. every separately accepted targeted architecture amendment named by
    activation
15. current signing/notarization/timestamping/updater/channel configuration,
    manifests, schemas, public-verifier material, protected workflows,
    secret references, storage/origin definitions, version policy, recovery
    paths, target tools, manual procedures, and publication controls
16. the exact activated revision of this prompt

The first future implementation progress message must state:

> `AGENTS.md, repository authority, accepted Phase 0A, accepted and merged
> predecessors through GFD-P7-WP01, exact hardened unsigned artifact
> identities, activated signing/updater trust roots, protected credential
> boundaries, channel/version/recovery policy, and the activated GFD-P7-WP02
> prompt have been read and are active. Executing only GFD-P7-WP02 with
> GPT-5.6 Sol / Extra High. Public publication, customer rollout, stores,
> enterprise deployment, telemetry, P8-WP01, PR creation, and merge remain
> unauthorized.`

If a source is unavailable, stale, unreadable, or materially inconsistent,
stop before writing. Do not infer production credentials, service access,
hardware, signer trust, notarization acceptance, endpoint ownership, channel
isolation, release eligibility, or publication authority.

## 4. Dependencies, predecessor outputs, and prerequisite evidence

**Direct prerequisite:** Accepted and merged `GFD-P7-WP01`, including its
exact hardened unsigned Windows, macOS, and Linux candidates; commit, package
and installed-tree hashes; production capabilities/CSP; development/test/fault
separation; complete lock and packaged-component closure; target-bound SBOM;
license/notices and obligations; vulnerability/secret/provenance findings;
clean-target installed-native evidence; migration, diagnostics, repair,
recovery, no-replay, single-instance, and zero-descendant readiness; workflow
permissions; measurements; accepted exceptions; deviations; limitations; and
blockers.

**Inherited prerequisites:** Accepted and merged WP01 and every package from
P0B-WP02 through P6-WP02, including the accepted Rust operation, path,
settings, diagnostics, lifecycle, process, privacy, and recovery authority.

**Direct dependent:** `GFD-P8-WP01`, which remains unauthorized and cannot use
signed artifacts, release configuration, or trust material until this package
is accepted and separately followed by explicit authorization.

P7-WP02 cannot activate while P7-WP01 exists only as a provisional prompt,
unmerged branch, unaccepted evidence set, unsigned-input mismatch, unresolved
Critical/High blocker, or stale target/package result.

Activation must supply exact:

- merged commits, trees, changed paths, reports, evidence indexes, manifests,
  snapshots, byte lengths, and digests;
- hardened unsigned artifact filenames, formats, versions, architectures,
  package metadata, hashes, installed identities, SBOM/license/finding state,
  and unsigned/untrusted classifications;
- Windows signing identity, chain, key provider, subject/publisher policy,
  timestamping service, expiry/revocation behavior, runner, and verifier;
- Apple Developer ID identity, team, chain, provider, hardened-runtime and
  entitlement baseline, notarization profile/account, agreements, tools,
  hardware, Gatekeeper owner, and service availability;
- updater signing algorithm, key ID, private custody, public verifier,
  canonical metadata schema, artifact signature/hash rules, client support,
  and transition/rotation behavior;
- protected environments, trusted refs/runners, reviewers, permissions, audit,
  cleanup, manual fallback, and break-glass limits;
- stable/beta names, origins, namespaces, access, cache, retention, signing-root
  policy, version eligibility, promotion/withdrawal, and isolation rules;
- controlled non-public staging origin and proof of publication separation;
- version, stale/replay, downgrade, rollback, interruption, restart, repair,
  reinstall, settings compatibility, and no-replay policies;
- key backup/restore, rotation, revocation, compromise freeze, artifact
  withdrawal, continuity, certificate renewal, and service-loss procedures;
- target OS/architecture/engine, signing/update tools, manual owners, hardware,
  unavailable evidence, measurements, limitations, and blockers;
- exact public publication state and separate authority owner.

No prerequisite may be accepted by inference. A build, signature file,
certificate listing, notarization request, CI badge, updater configuration,
mock server, or package upload cannot replace real cryptographic,
service-specific, installed-native, and target evidence.

## 5. Objective and measurable runnable outcome

Complete a controlled, review-ready signing and signed-update implementation
for the accepted P7-WP01 artifacts while stopping before any public release or
customer distribution.

The future package must prove this progression:

```text
accepted P7-WP01 hardened unsigned artifacts and exact hashes
→ activated identities, custody, protected environments, and services verified
→ application-signing and updater-signing trust roots separated
→ exact Windows artifact signed, timestamped, staged, and independently verified
→ exact macOS app and nested code signed with hardened runtime and entitlements
→ exact macOS artifact notarized, stapled where applicable, and Gatekeeper verified
→ Linux package integrity and explicit updater boundary verified truthfully
→ isolated stable and beta signed metadata/artifacts generated
→ controlled non-public origin serves exact channel/target/version content
→ older accepted signed client discovers only its allowed same-channel update
→ metadata, artifact, signature, hash, size, target, architecture, channel, and version verify before install
→ native update installs atomically or fails safely without uncertain-work replay
→ settings migration, diagnostics privacy, repair, backend recovery, and cleanup remain green
→ tamper, wrong key/channel/target, stale/replay, downgrade, interruption, and service failures fail safely
→ backup/rotation/revocation/compromise and release-continuity procedures are exercised
→ review-ready signed/staged evidence stops before public publication or customer rollout
```

A signing command exit code, certificate export, notarization submission,
staged file, update manifest, local mock that skips cryptographic verification,
browser-only test, or one platform substituting for another cannot satisfy the
outcome.

Activation must refresh the exact target matrix. The authoring-time expectation
is:

| Target | Required future trust result | Boundary |
|---|---|---|
| Windows 11 x64 | accepted binary/NSIS signing, trusted timestamp, independent Authenticode verification | no SmartScreen reputation claim, MSI, store, or extra architecture |
| activated macOS arm64 versions | nested Developer ID signing, hardened runtime, notarization acceptance, stapling where applicable, Gatekeeper/quarantine/native launch | no Mac App Store, PKG, universal/Intel, or unsupported target claim |
| Ubuntu 24.04 x64 `.deb` | retained package/hash/installed integrity and explicit supported-or-disabled updater state | no invented apt repository signing, extra format, or Linux channel claim |

This table is not activated acceptance. Activation may narrow a claim
truthfully but cannot silently add a platform, format, architecture, store,
repository, trust mechanism, or publication path.

## 6. Explicit in-scope work

A future activated Work Session may perform only the smallest coherent change
set needed for the following work.

### 6.1 Trust, credential, and publication inventory

Inventory every application signer, updater signer, public verifier,
certificate chain, key provider, secret reference, protected environment,
timestamp/notarization service, storage origin, channel authority, release
approval, version namespace, target owner, backup/recovery owner, and
publication boundary. Classify each as activated production trust, controlled
staging trust, CI-only, manual-only, public verification material, secret
material, derived metadata, unavailable, or blocked. Record one authoritative
source and one owner without reading, printing, or exporting secret values.

### 6.2 Separate application and updater trust roots

- Treat Windows/macOS application signing, updater metadata/artifact signing,
  public-verifier distribution, channel metadata authority, storage upload,
  repository tokens, notarization, and timestamping as distinct roles.
- Do not reuse one key silently across roles.
- If activation explicitly accepts shared material, require a threat analysis,
  role boundaries, independent approval, rotation impact, compromise blast
  radius, and exact verifier behavior.
- Freeze exact safely reportable key/certificate identifiers, algorithms,
  parameters, chain/expiry/revocation rules, and unknown-key failure.
- Never place private material in source, packages, logs, screenshots, CI
  output, diagnostics, SBOM/notices, staging content, or handoff files.

### 6.3 Windows signing, timestamping, and independent verification

- Sign only the activated hardened Windows x64 executables and NSIS installer.
- Sign required nested executables consistently under the activated package
  policy.
- Use the exact activated certificate chain and timestamp service.
- Verify digest algorithm, signature, timestamp, subject/publisher, chain,
  file hash, architecture, version, and package identity independently after
  signing and again after staging/download.
- Prove tamper, signature stripping, wrong subject/chain, wrong architecture,
  and wrong artifact identity fail.
- Keep cryptographic validity separate from SmartScreen reputation.
- A self-signed or staging certificate remains staging-only unless activation
  explicitly classifies it and blocks production trust claims.

### 6.4 macOS nested signing and hardened-runtime integrity

- Sign the activated arm64 `.app` and every nested executable, framework,
  dylib, helper, and sidecar in the correct inside-out order.
- Use the exact activated Developer ID Application identity.
- Enable hardened runtime and only necessary reviewed entitlements.
- Verify identity, designated requirement, entitlements, bundle identifier,
  architecture, sealed resources, and every nested signature.
- Construct the accepted distribution layout without mutating signed bytes
  afterward.
- Prove tamper, entitlement drift, missing nested signature, wrong identity,
  altered sealed resource, and unsupported architecture fail safely.

### 6.5 macOS notarization, stapling, Gatekeeper, and target proof

- Submit the exact activated artifact to the exact activated Apple service.
- Retain safely reportable submission identity, tool/service version, response,
  acceptance or rejection, and timing without credentials.
- Treat submission, upload, or processing as non-acceptance until the service
  returns the required accepted result.
- Staple the accepted ticket where applicable.
- Verify `codesign`, stapling, `spctl`, quarantine/Gatekeeper, offline-ticket
  behavior, clean installation, and native launch on real activated hardware.
- Classify unstapled/offline, service, agreement, account, and hardware limits
  truthfully.

### 6.6 Linux integrity and updater exclusion

- Preserve the exact accepted Ubuntu `.deb` hash, package metadata, contents,
  permissions, architecture, install/native evidence, and cleanup behavior.
- Freeze whether application update is unsupported, disabled, or narrowly
  activated on Linux.
- Prevent Windows/macOS metadata or artifacts from being offered to Linux.
- Verify target, architecture, channel, format, and verifier mismatch fails.
- Do not invent apt repository signing, repository publication, Snap, Flatpak,
  AppImage, RPM, extra distribution, or Linux release-channel support.

### 6.7 Canonical signed-update contract

Freeze the exact updater implementation and version, metadata schema/version,
canonical byte representation, artifact format/compression, signature
algorithm/parameters, key ID, public-verifier distribution, verification
order, artifact signature where applicable, hash, size, target OS,
architecture, channel, release version, minimum/current version, release
notes, and URL policy.

Bound metadata/artifact counts, field lengths, byte sizes, redirects, content
types, cache behavior, range requests, partial downloads, retries, timeouts,
temporary files, permissions, atomic replacement, restart, cleanup, and safe
error output. HTTPS is required where activated but never substitutes for
metadata and artifact verification.

### 6.8 Rust-owned updater authority and bounded UI

- Rust owns update discovery, metadata parsing and verification, target/
  architecture/channel/version selection, download, path and temporary-file
  policy, artifact verification, installation intent, restart, cleanup,
  settings writes, process containment, and single-instance coordination.
- React presents finite typed status and bounded user intent only.
- React cannot choose arbitrary URLs, paths, keys, packages, commands,
  channels, targets, or versions.
- Python has no signing, credential, updater, channel, storage, or publication
  authority.
- Unknown state, key, schema, target, channel, version, signature, or artifact
  fails closed with a safe error.

### 6.9 Stable and beta channel isolation

- Use distinct metadata namespaces and artifact paths.
- Freeze distinct eligibility, approval, retention, cache, and downgrade rules.
- Freeze client channel selection, persistence owner, and explicit
  user/admin-controlled channel movement.
- Do not silently cross, fall back, auto-promote, overwrite, or accept a beta
  artifact through the stable channel.
- Activation must decide whether channel keys are shared or separate and
  record threat, rotation, revocation, and compromise consequences.
- Retain independent channel evidence and withdrawal/recovery procedures.

### 6.10 Version, stale/replay, downgrade, and rollback policy

- Freeze accepted version ordering, build metadata, package identity,
  equal/current-version behavior, minimum supported source version, stale or
  expired metadata behavior, clock dependence and skew, and replay handling.
- Prohibit automatic downgrade unless activation names one narrow verified
  recovery path.
- Do not promise automatic rollback unless it is explicitly designed,
  bounded, implemented, and proven.
- Prevent repair or reinstall from silently bypassing anti-rollback.
- Preserve settings migrations, unsupported-future-version protection, user
  data, task truth, and no replay of uncertain work.

### 6.11 Controlled non-public staging origin

- Use exactly one activated non-public origin or accepted local equivalent for
  signed test metadata and artifacts.
- Bound access, upload authority, retention, cache, audit, cleanup, and
  credential handling.
- Prevent untrusted pull-request code from reading credentials or modifying
  staged content.
- Keep staging paths and authority separate from any production publication.
- Support deterministic tamper, wrong-key, stale, mismatch, interruption, and
  recovery fixtures.
- Remove, disable, or retain the origin non-publicly according to activation
  after evidence collection.

### 6.12 Protected CI and manual signing boundaries

- Execute credential-bearing work only from exact reviewed trusted refs,
  protected environments, approved trusted runners, and required reviewers.
- Minimize repository, artifact, identity, token, secret, storage, and
  environment permissions.
- Isolate target jobs and converge only on verified evidence.
- Attest exact input commit and unsigned hash, signed output hash,
  tool/service, safely reportable identity/key ID, timestamp/notarization
  result, metadata hash, channel state, and approval.
- Prevent fork/untrusted PR code, mutable scripts, caches, ordinary artifacts,
  workspace remnants, logs, dumps, and handoffs from reaching secrets.
- Require explicit staging-upload approval and a separate later publication
  authority.
- Make manual fallback obey equivalent custody, verification, audit, and
  cleanup rules.

### 6.13 Native install and signed-update journeys

Run real target journeys for clean signed/notarized install and first launch,
update check from an older accepted signed version, no-update/current version,
stable-to-stable and beta-to-beta update, download, verification, install,
restart, cleanup, settings migration, diagnostics privacy, repair, backend
recovery, single-instance, no-replay, and zero surviving descendants.

Separately exercise wrong channel, target, architecture, key, signature,
subject, chain, identity, artifact, hash, size, version, schema, URL, stale/
replayed metadata, tamper, truncation, interruption, outage, cancellation,
failed restart, downgrade, and repair-bypass paths. One platform or local mock
cannot substitute for another.

### 6.14 Key backup, rotation, revocation, and compromise recovery

- Record certificate/key inventory, ownership, expiry, renewal windows, and
  service dependencies.
- Prove an accepted encrypted backup, hardware/provider recovery, or other
  activated recovery method without exposing private bytes.
- Exercise safely permitted restore evidence.
- Prove planned updater-verifier rotation and old/new overlap only where
  explicitly safe.
- Prove certificate renewal and publisher continuity.
- Exercise revocation, compromise declaration, emergency channel freeze,
  staged artifact withdrawal, service loss, and release continuity procedures
  within activated safety limits.
- Report `Blocked` rather than generate substitute production identities when
  custody or recovery authority is unavailable.

### 6.15 Security, publication-control, and operations documentation

Reconcile source, workflows, metadata, packages, services, native behavior,
and evidence with focused documentation for application versus updater trust,
private versus public material, protected CI/manual custody, stable/beta
isolation, canonical verification order, version/downgrade/replay rules,
rotation/revocation/compromise, repair/recovery, known target/service
limitations, and separate publication authority. Documentation cannot
override contrary code or evidence.

### 6.16 Raw measurements and reliability evidence

Retain unrounded measurements per target for signing, timestamping,
notarization submission/acceptance, stapling, verification, staging upload,
metadata generation, discovery, download, verification, install, restart,
cleanup, backup/restore exercise, rotation/revocation exercise, repair, and
recovery. Record unsigned/signed artifact sizes, metadata/payload size, disk
and memory high-water use, repeated positive/negative outcomes, cache/
propagation delay, failures, variance, samples, hardware/runner, OS/engine,
tools/services, and limitations. Do not invent availability, rollout,
reputation, release, or service-level promises.

## 7. Explicit exclusions and prohibited work

The future package must not:

- activate or implement `GFD-P8-WP01` or another package;
- create substitute production certificates, identities, keys, accounts,
  services, endpoints, or publication authority when activated resources are
  unavailable;
- export, print, commit, archive, expose, copy into diagnostics, or place
  private keys, certificates with private material, tokens, passwords, API
  keys, secret values, recovery phrases, or sensitive paths in deliverables;
- let forks, untrusted PR code, arbitrary refs, mutable unreviewed scripts, or
  ordinary runners access signing, notarization, storage, or publication
  credentials;
- silently reuse one key for application signing, updater signing, storage,
  channel authority, and publication;
- weaken P7-WP01 capabilities, CSP, dependency closure, finding gates, package
  identity, production/test separation, installed evidence, migration,
  diagnostics privacy, no-replay, or process containment;
- accept unsigned, wrong-key, stale, replayed, wrong-channel, wrong-target,
  wrong-architecture, corrupted, truncated, equal/downgrade-disallowed, or
  otherwise mismatched metadata/artifacts;
- describe a self-signed/staging result as production trust, a notarization
  submission as acceptance, or signature validity as SmartScreen reputation;
- bypass macOS hardened runtime, nested signing, notarization, stapling where
  applicable, quarantine, Gatekeeper, or real-target evidence;
- invent Linux repository signing or unsupported updater behavior;
- publish a GitHub Release, public bucket/object, production update channel,
  store submission, release tag, customer notification/rollout, staged
  percentage rollout, enterprise deployment, public release notes, or
  marketing claim;
- add MSI, PKG, AppImage, Snap, Flatpak, RPM, extra architecture, extra
  distribution, store packaging, telemetry, crash upload, remote support,
  analytics, product/customer behavior, account, licensing, or monetization;
- add public SDKs, runtime plugins, `packages/ui`, templates, generators,
  second applications, or broad reusable release platforms;
- open a PR, enable auto-merge, merge, rebase, squash, force-push, rewrite
  accepted history, delete a branch, or change repository settings.

A missing credential, service, agreement, target, hardware owner, verifier,
channel-isolation fact, legal/account authorization, or required architecture
change is a stop condition. This package cannot fix a predecessor defect or
self-approve a scope expansion.

## 8. Allowed and protected paths

Future activation must replace these broad authoring-time categories with the
smallest exact path allowlist from accepted merged `main`:

```text
apps/desktop/src-tauri/tauri.conf.*             # exact updater configuration and public verification material
apps/desktop/src-tauri/capabilities/            # bounded updater/signing-related command exposure
apps/desktop/src-tauri/permissions/             # least-privilege exact permissions
apps/desktop/src-tauri/src/update/              # Rust-owned verified update state, policy, install, restart
apps/desktop/src/update/                        # bounded typed presentation and user intent
packages/app-contracts/                         # finite update metadata/state/error schemas when justified
scripts/signing/                                # target signing and independent verification
scripts/notarization/                           # submission, wait, staple, and verification
scripts/update/                                 # canonical metadata/artifact generation and staging checks
scripts/verify/                                 # channel, tamper, mismatch, downgrade, recovery, secret-negative checks
.github/workflows/                              # protected trusted-ref signing/notarization/staging jobs
release/ or accepted metadata location         # bounded channel schema/templates/public verification material
security/ or accepted documentation location  # custody, rotation, revocation, compromise, publication controls
manifests, locks, generated metadata           # mechanically required reviewed companions only
focused tests and synthetic fixtures           # non-secret keys/metadata for unit and negative tests only
```

Remove nonexistent and unnecessary entries during activation. Do not create a
directory merely because it appears above.

Protected by default:

- authority, Phase 0A records, Stage 1 governance, and provisional prompts;
- accepted P7-WP01 artifact identity, least-privilege capabilities, CSP,
  dependency closure, SBOM/license/finding truth, and hardened package
  semantics except exact signing/updater integration;
- accepted P3–P6 operation, path, settings, diagnostics, repair, recovery,
  privacy, single-instance, process, and no-replay authority;
- product/domain features, public SDK/plugin surfaces, template extraction,
  second application, telemetry, remote support, stores, enterprise
  deployment, and customer rollout;
- private keys, keychains, secret-manager state, external account settings,
  public release systems, and repository settings except exact separately
  activated interactions.

Stop when the exact implementation cannot fit the activated allowlist without
unrelated work.

## 9. Ordered future implementation sequence

All steps below are future requirements. This provisional prompt performs none
of them.

1. Read all activated authority and predecessor evidence; issue the exact
   acknowledgement and verify every stop condition.
2. Verify repository, exact accepted merged `main`, fresh branch, clean tree,
   permissions, exact artifacts, credential boundaries, protected
   environments, external services, target owners, publication state, and
   path allowlist.
3. Generate exactly one task-start UTC run ID, resolve all four implementation
   deliverable names, and stop on any collision.
4. Inventory application signing, updater signing, notarization, timestamping,
   storage, channels, secrets, CI, targets, versions, recovery, and
   publication surfaces without exposing secret values.
5. Freeze exact activated targets, hardened unsigned hashes, signing
   identities, updater trust root, algorithms, metadata schema, channel
   policy, anti-rollback rules, services, tools, hardware/manual owners, and
   budgets.
6. Produce one trust/custody map separating application signing, updater
   signing, public verification, storage, CI, notarization, timestamping, and
   publication authority.
7. Implement or reconcile finite typed update contracts, canonical
   verification order, target/channel/version matching, bounded state, and
   safe errors.
8. Implement trusted-ref, protected-environment, approval, permission, audit,
   and cleanup boundaries without exposing credentials to untrusted code.
9. Sign the exact hardened Windows artifacts, timestamp them, and
   independently verify every signed file and installer before and after
   staging.
10. Sign the exact macOS app and nested code with hardened runtime and exact
    entitlements; verify sealed resources, identity, architecture, and
    package bytes.
11. Submit the exact activated macOS artifact for notarization, verify
    acceptance, staple where applicable, and prove Gatekeeper/quarantine/
    native behavior on real target hardware.
12. Verify Linux package integrity and explicit updater/channel exclusion or
    exact narrowly activated behavior without inventing repository signing.
13. Generate signed update artifacts and canonical metadata for isolated
    stable and beta controlled-staging channels.
14. Configure or use the exact activated non-public origin with bounded
    access, retention, audit, cache, and cleanup policy.
15. Run real end-to-end signed install and same-channel update journeys from
    accepted older versions on every claimed target.
16. Run negative tests for wrong key, missing/stripped signature, tamper,
    truncation, stale/replayed metadata, channel crossing, wrong target/
    architecture, equal version, downgrade, interruption, cancellation, and
    service failures.
17. Prove settings migration/recovery, diagnostics privacy, repair, backend
    recovery, single-instance, no replay, restart, process containment, and
    zero-descendant behavior after update and failures.
18. Exercise backup/restore evidence, planned rotation, verifier transition,
    certificate renewal, revocation, compromise freeze, channel isolation,
    withdrawal, and release continuity within activated safety limits.
19. Reinspect workflows, logs, caches, artifacts, workspaces, staging storage,
    packages, metadata, diagnostics, and handoff files for secret/path leakage
    and unauthorized publication.
20. Capture raw signing, notarization, staging, update, recovery, size, memory,
    reliability, cleanup, service, and target measurements and limitations.
21. Reconcile security, key operations, channels, update, downgrade, repair,
    recovery, and publication-control documentation against source and
    evidence.
22. Run complete source, schema, test, security, secret-negative,
    signing-verification, notarization, channel, native update, recovery,
    process, accessibility, and documentation validation from a clean state.
23. Review the exact diff and evidence, commit one coherent package, perform
    an immediate remote-race check, and ordinary fast-forward push; do not
    open a PR or alter `main`.
24. Build the source snapshot from the exact final commit, verify safe paths,
    one root, CRC and digest, extract into a fresh empty directory, and rerun
    every source-reproducible check while retaining target/service/manual
    evidence separately.
25. Finalize exactly four deliverables with verified hashes and one run ID,
    verify remote/local parity and unchanged `main`, report every gate and
    status independently, return `READY FOR CHAT SESSION REVIEW`, and stop
    without public publication or P8-WP01.

Do not sign before identities and custody are verified, treat notarization
submission as acceptance, substitute mocks for cryptographic/native proof,
publish as a test shortcut, or infer publication authority from implementation
completion.

## 10. Cross-cutting rules and invariants

- Rust remains authority for update checks, verified metadata, download/install
  intent, native paths, settings writes, restart, process containment, and
  single-instance behavior.
- React/webview presents bounded typed state and user intent only; it cannot
  choose arbitrary URLs, keys, paths, packages, commands, targets, channels,
  versions, or publication actions.
- Python remains trusted first-party native code but has no signing, updater,
  channel, credential, storage-upload, or publication authority.
- Application signing and updater signing remain independent trust decisions.
- Private material never enters source, packages, logs, caches, screenshots,
  CI output, diagnostics, SBOM/notices, staging content, or deliverables.
- Public keys and certificate chains may be public verification material, but
  identity, distribution, trust, transition, and rotation remain controlled.
- Stable and beta channels are deterministic, bounded, and isolated.
- Transport security never substitutes for signed metadata and artifact
  verification.
- Metadata fields, signatures, key IDs, targets, architectures, channels,
  versions, URLs, redirects, retries, queues, files, timeouts, and retention
  are bounded.
- Unknown or failed schema, identity, signature, hash, size, target,
  architecture, channel, version, package, or policy checks fail closed.
- No unsigned fallback, wrong-channel fallback, stale/replay acceptance,
  automatic downgrade, repair bypass, or uncertain-work replay is allowed.
- One primary instance coordinates updater, settings, backend, install intent,
  restart, and process lifecycle.
- Tools, services, accounts, agreements, submissions, jobs, targets, and
  manual results are identified and classified exactly.
- One target, signer, key, service, channel, mock, or evidence class cannot
  substitute for another.
- Prompt acceptance, activation, credential authorization, implementation
  review, public publication, PR, merge, and P8 progression are separate
  decisions.
- Implementation status uses only `Implemented`, `Partially implemented`,
  `Stub`, `Mock-only`, `Not started`, or `Blocked`.
- Evidence status uses only `Passed`, `Failed`, `Partial`, `Blocked`, or
  `Not run`.
- `Partially implemented`, `Partial`, and `Blocked` require a concise reason
  and exact evidence reference.

## 11. Required validation, evidence, and measurements

Future validation must keep these evidence classes separate:

1. authority, activation, exact base, branch, permissions, and allowlist;
2. accepted hardened unsigned artifact identities, hashes, and P7-WP01 state;
3. signing identities, certificate chains, key custody, public verifiers, and
   safely reportable identifiers;
4. protected environments, trusted refs/runners, approvals, permissions,
   audit, cleanup, manual fallback, and untrusted-code isolation;
5. Windows signing, timestamping, independent verification, staging/download,
   and tamper failures;
6. macOS nested signing, hardened runtime, entitlements, notarization,
   stapling, Gatekeeper, quarantine, native launch, and hardware;
7. Linux package integrity and updater exclusion/limitation;
8. updater schema, canonical bytes, algorithms, signatures, hashes, sizes,
   target/version/channel mapping, and public-verifier distribution;
9. stable/beta namespace, artifact, eligibility, storage, access, cache,
   retention, signing-root, promotion, withdrawal, and isolation;
10. controlled non-public origin and publication-negative evidence;
11. update discovery, download, verification, install, restart, and cleanup;
12. wrong-key, unsigned, tamper, stale/replay, mismatch, downgrade,
    interruption, cancellation, outage, and failed-restart evidence;
13. settings migration, diagnostics privacy, repair, backend recovery,
    single-instance, no-replay, process containment, and zero descendants;
14. backup, restore, rotation, verifier transition, renewal, revocation,
    compromise freeze, withdrawal, continuity, and manual-owner evidence;
15. external service/account/agreement identities, results, outages, expiry,
    legal limitations, tool versions, and safely reportable submission IDs;
16. raw signing, notarization, staging, update, performance, size, memory,
    reliability, recovery, service, and cleanup measurements;
17. security/key/channel/update/recovery/publication-control documentation;
18. source snapshot, fresh extraction, reproducible checks, hashes, and path
    safety;
19. unavailable, blocked, partial, accepted-exception, deviation, limitation,
    and manual evidence.

For every command or check, retain exact method, exit status, expected and
actual result, commit, target OS/version/architecture, engine, package and
artifact hash, safely reportable key/certificate identifier, tool/service/
version, submission or job identity, capture time, raw evidence artifact,
classification, and limitation.

Future scenarios must include:

- trusted and untrusted refs have mechanically different credential access;
- secret values are absent from source, history scope named by activation,
  logs, caches, packages, screenshots, diagnostics, SBOM/notices, staging
  content, and all deliverables;
- application and updater signatures cannot substitute for each other;
- Windows valid signatures/timestamps verify independently and tamper,
  stripping, wrong subject/chain, architecture, or identity fails;
- macOS nested code, entitlements, sealed resources, notarization, stapling,
  quarantine, Gatekeeper, and native launch verify on real hardware;
- rejected/incomplete notarization, missing nested signatures, entitlement
  drift, tamper, wrong identity, and offline limitations remain safe/truthful;
- Linux remains isolated from unsupported updater/channel claims;
- canonical metadata and artifacts bind key ID, hash, size, target,
  architecture, channel, version, URL, and package identity;
- stable and beta cannot cross, fall back, overwrite, or auto-promote;
- staging remains controlled and non-public;
- current/equal/no-update, wrong key/channel/target/architecture, unsigned,
  corrupted, truncated, stale/replayed, downgrade, interruption, outage, and
  cancellation paths fail deterministically;
- allowed same-channel updates install/restart/clean up through the real native
  path;
- settings migrations, diagnostics privacy, repair, backend recovery,
  single-instance, no-replay, and zero descendants remain green;
- backup/restore, rotation, verifier transition, renewal, revocation,
  compromise freeze, withdrawal, and continuity are exercised without private
  material exposure;
- no public publication, release tag, store submission, customer rollout,
  enterprise deployment, telemetry, or P8 work occurs;
- source snapshot extracts safely and reproduces every source-level result.

The review evidence index must map every acceptance gate independently to
source/tests, command and exit status, workflow run/job/artifact, target/manual
proof, tool/service/submission identity, artifact/metadata hashes, safely
reportable key/certificate identifier, measurement, classification,
limitation, and digest.

## 12. Measurable acceptance gates

All 30 gates are unsatisfied while this prompt remains provisional:

1. Exact accepted and merged predecessors through P7-WP01 were used from the
   exact clean activated `main`.
2. The final diff remains inside one-package allowlist with no P8,
   publication, store, enterprise, telemetry, product/customer, prompt-pack,
   repository-setting, or unrelated work.
3. Exact hardened unsigned artifacts, hashes, package metadata, targets,
   SBOM/license/finding state, installed-native results, and P7-WP01
   limitations are preserved.
4. Application signing, updater signing, public verification, storage,
   notarization, timestamping, CI, channel, and publication roles have
   explicit separate owners and trust boundaries.
5. No private key, token, password, credential, secret value, sensitive path,
   recovery phrase, or signing material appears in source, packages, logs,
   caches, CI output, screenshots, diagnostics, SBOM/notices, staging content,
   or deliverables.
6. Signing and staging workflows run only from exact reviewed trusted refs
   with protected approvals, trusted runners, least-privilege permissions,
   audit, and cleanup; untrusted code cannot reach credentials.
7. Activated Windows binaries and installer are signed and timestamped with
   the exact activated identity and independently verify after staging and
   download.
8. Windows tamper, stripped signature, wrong subject/chain, wrong
   architecture, and wrong artifact identity fail safely; SmartScreen
   reputation is not misclaimed.
9. The activated macOS app and every nested executable, framework, library,
   helper, and sidecar are signed correctly with exact identity, hardened
   runtime, necessary entitlements, architecture, and sealed resources.
10. The exact macOS artifact is accepted by notarization, stapled where
    applicable, and passes signing, stapling, Gatekeeper, quarantine, clean
    installation, and native-launch checks on activated hardware.
11. macOS tamper, entitlement drift, missing nested signature, wrong identity,
    altered sealed resource, rejected/incomplete notarization, and
    offline/stapling limitations remain safe and truthful.
12. Ubuntu `.deb` integrity and target identity remain green, and unsupported
    updater/repository-signing/channel behavior is disabled and not claimed.
13. Signed update metadata and artifacts use exact activated algorithms,
    canonical bytes, key IDs, hashes, sizes, schema, target, architecture,
    channel, version, package identity, and URL policy.
14. Rust verifies every required metadata and artifact field and signature
    before installation; transport security is not treated as a substitute.
15. Stable and beta metadata, artifacts, eligibility, verifier policy,
    storage, access, retention, cache, approval, and client selection are
    isolated with no silent crossing, fallback, overwrite, or auto-promotion.
16. The controlled staging origin is non-public, bounded, auditable, isolated
    from production publication, and inaccessible to untrusted code.
17. Older accepted signed clients complete allowed same-channel signed updates
    through real native discovery, download, verification, install, restart,
    and cleanup.
18. Current/equal-version, no-update, wrong-channel, wrong-target,
    wrong-architecture, wrong-key, unsigned, corrupted, truncated, stale, and
    replayed metadata/artifacts fail deterministically.
19. Downgrade and rollback behavior follows exact activated policy; repair or
    reinstall cannot silently bypass anti-rollback.
20. Interrupted download/install, failed restart, partial file, service
    outage, cancellation, and cleanup failure leave no accepted partial
    update, secret, unsafe state, false success, or orphaned process.
21. Settings migration, previous-copy recovery, section preservation,
    unsupported-future settings, diagnostics privacy, repair, and backend
    availability remain truthful after successful and failed updates.
22. One primary instance owns update/settings/backend lifecycle; no duplicate
    writer, backend, install intent, replayed task, or surviving descendant
    exists.
23. Key backup/restore evidence, planned rotation, public-verifier transition,
    certificate renewal, revocation, compromise freeze, channel isolation,
    artifact withdrawal, and release continuity are complete and exercised
    within activated safety limits.
24. External signing, notarization, timestamping, storage, and secret-provider
    identities, results, outages, expiries, agreements, account/legal
    limitations, and manual owners are recorded exactly without secret values.
25. Source, cryptographic, service, workflow, package, installed-native,
    target, channel, manual, and unavailable evidence remains separate and
    truthful.
26. Raw signing, notarization, staging, update, size, memory, reliability,
    recovery, service, and cleanup measurements are retained without
    fabricated availability, reputation, rollout, or release claims.
27. Security, key, channel, update, downgrade, repair, recovery, and
    publication-control documentation matches source, workflows, metadata,
    artifacts, services, native behavior, and limitations.
28. Exactly four implementation deliverables use one task-start run ID, have
    verified hashes, contain no sensitive data, and represent the reviewed
    final commit.
29. The source snapshot has one safe root, passes CRC/path/inventory checks,
    extracts cleanly, and reproduces every source-reproducible validation while
    non-reproducible target/service evidence remains separately identified.
30. The implementation branch matches remote, `main` is unchanged, no PR or
    auto-merge exists, and handoff stops at review without public publication,
    customer rollout, store submission, enterprise deployment, telemetry, or
    P8-WP01.

If a gate is unmet, report `Blocked`, `Not run`, `Partial`, or the exact
accepted limitation as an evidence outcome. Never aggregate independent
target, signer, service, channel, recovery, or publication facts into a vague
pass.

## 13. Stop conditions and blocker reporting

Stop before writing or at the last safe non-destructive point when:

- activation is absent, provisional, stale, incomplete, or overbroad;
- repository, exact base, branch, clean tree, allowlist, target, artifact,
  package identity, credential state, or publication state differs;
- P7-WP01 or another predecessor is unaccepted or unmerged;
- hardened unsigned artifacts, hashes, SBOM/license/finding closure,
  installed evidence, or limitations are missing or inconsistent;
- an exact signing identity, updater key, public verifier, custody provider,
  protected environment, notarization/timestamping service, staging origin,
  target hardware, manual owner, or legal/account authorization cannot be
  verified;
- work would expose private material or allow untrusted code to access
  credentials;
- application and updater trust roles cannot be separated safely;
- Windows signing/timestamping or macOS signing/notarization/Gatekeeper cannot
  be proven on the claimed target;
- canonicalization, signature verification, target/channel/version mapping,
  anti-replay, anti-rollback, interruption cleanup, repair, or recovery policy
  is ambiguous;
- stable/beta isolation or staging/publication separation cannot be proven;
- a public endpoint/release, store, customer rollout, extra format/
  architecture, telemetry, remote support, product/customer behavior, public
  SDK/plugin, or another package is required;
- one target, key, service, channel, mock, build, or evidence class would have
  to substitute for another;
- snapshot integrity or clean extraction rerun fails;
- unauthorized or unrelated files enter the diff;
- remote state changes, push is non-fast-forward, or history rewriting would
  be required.

When blocked:

1. Stop at the last clean non-destructive state.
2. Do not broaden scope, generate substitute production trust, weaken a gate,
   publish, or enter P8-WP01.
3. Record the failed fact, command/service, expected and observed state,
   target/channel/safely reportable key identifier, affected gates, changed
   paths, tree state, evidence class, and unaffected exclusions.
4. Distinguish repository defect, stale activation, missing authority,
   credential/custody, external service/account/agreement, target/hardware,
   updater/channel design, environment, permission, or publication decision.
5. Use `Blocked` or `Partially implemented` for implementation status and
   `Partial` only for explicitly labeled evidence.
6. Return the smallest safe Chat Session decision, focused correction,
   refreshed activation, or separate authority required.

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
GFD-P7-WP01 provisional prompt: Implemented and accepted as Approved provisional
GFD-P7-WP01 activation/implementation: Not started
GFD-P7-WP02 package outcome: Not started
Trust and credential inventory: Not started
Application/updater trust-root separation: Not started
Windows application signing: Not started
Windows timestamping and independent verification: Not started
macOS nested signing and hardened runtime: Not started
macOS notarization, stapling, and Gatekeeper: Not started
Linux package integrity and updater boundary: Not started
Canonical signed-update contract: Not started
Rust updater authority and bounded UI: Not started
Stable channel implementation and evidence: Not started
Beta channel implementation and evidence: Not started
Stable/beta isolation: Not started
Controlled non-public staging origin: Not started
Protected CI signing/notarization/staging boundary: Not started
Manual signing fallback: Not started
Version, stale/replay, downgrade, and rollback policy: Not started
Signed install/update native journeys: Not started
Negative tamper/mismatch/interruption journeys: Not started
Settings migration and diagnostics privacy after update: Not started
Repair, backend recovery, single-instance, and no replay: Not started
Backup and restore evidence: Not started
Key and certificate rotation/renewal: Not started
Revocation, compromise freeze, and artifact withdrawal: Not started
Release continuity procedure: Not started
Security and publication-control documentation: Not started
Raw measurements and reliability evidence: Not started
Supporting infrastructure: Not started
Tests: Not started
Documentation for implementation: Not started
Generated code: Not started
Fixtures/mocks: Not started
Stubs/placeholders: Not started
Incomplete work: Not started
Blocked work: Blocked — GFD-P7-WP01 is not accepted and merged and GFD-P7-WP02 has no activated prompt or verified credential/service boundary
GFD-P8-WP01: Not started
Public publication/customer rollout/store/enterprise deployment: Not started
Telemetry/crash upload/remote support: Not started
Product/customer features: Not started
Public SDK/plugin/packages-ui/template extraction: Not started
PR creation: Not started
Merge: Not started
Branch deletion: Not started
```

At future completion, update every line independently with one approved
implementation-status label. Evidence outcomes may use only `Passed`,
`Failed`, `Partial`, `Blocked`, or `Not run` when explicitly labeled as
evidence. Keep functionality, infrastructure, tests, documentation, generated
code, fixtures, stubs, incomplete work, blocked work, credential availability,
service availability, and manual evidence separate.

## 15. One RUN_ID and collision-resistant external naming

At the start of future implementation, after mandatory authority reads and
preflight but before any repository or deliverable write, generate exactly one
UTC run ID in basic ISO-8601 form:

```text
YYYYMMDDTHHMMSSZ
```

Resolve all four implementation filenames immediately and reuse the one value
unchanged. Never substitute a second run ID, local timestamp, target
timestamp, random suffix, branch label, or service time.

Before any write, verify none of the four names exists in the destination. A
collision is a stop condition: report `Blocked` without generating another
ID. The run ID identifies the evidence set, not a credential, release,
channel, signer, or version.

## 16. Required deliverables, hashes, and evidence index

Produce exactly these four future implementation deliverables:

1. `prime-shell-work-gfd-p7-wp02-<RUN_ID>-source-snapshot-r1.zip`
2. `prime-shell-work-gfd-p7-wp02-<RUN_ID>-signing-notarization-signed-update-channels-report-r1.md`
3. `prime-shell-work-gfd-p7-wp02-<RUN_ID>-review-evidence-index-r1.md`
4. `prime-shell-work-gfd-p7-wp02-<RUN_ID>-handoff-manifest-r1.md`

### Source snapshot

- Build from the exact reviewed final commit with one top-level
  `prime-shell/` root and repository-relative paths.
- Include source, typed contracts, public verification material, synthetic
  non-secret fixtures, scripts, protected workflow definitions, metadata
  schemas/templates, manifests, locks, tests, and focused security/operations
  documentation needed for review.
- Exclude `.git`, environments, caches, build output, signed installers,
  notarized artifacts, staging content, native captures, keychains, secret
  files, certificates with private material, tokens, settings, diagnostics,
  user data, and unrelated files.
- Verify SHA-256, ZIP CRC, safe paths, expected root, and exact inventory.
- Extract into a fresh empty directory and rerun every source-reproducible
  activated check.

### Signing, notarization, and signed-update report

Record activation, model/reasoning, repository, branch, base/final commit,
parent, tree, exact changed paths, snapshot, accepted predecessors, hardened
unsigned inputs, signed outputs and hashes, package identities, trust/custody
map, safely reportable certificate/key identifiers, secret-negative evidence,
Windows signing/timestamping/verification, macOS nested signing/entitlements/
notarization/stapling/Gatekeeper/hardware, Linux boundary, canonical update
format, stable/beta isolation, staging origin, version/downgrade/replay/
interruption rules, protected CI/manual custody, native update journeys,
backup/rotation/revocation/compromise/continuity, all 30 gates, measurements,
external services, unavailable evidence, limitations, status inventory, and
publication-negative confirmation.

### Review evidence index

Include one row for every final gate. Map exact source/tests, command and exit
status, workflow run/job/artifact, target/manual proof, tool/service/submission
identity, artifact/metadata hashes, safely reportable key/certificate
identifier, measurement, classification, limitation, and digest. Do not leave
blank cells; use `None`, `Not run`, `Blocked`, or `Partial` explicitly.

### Handoff manifest and hashing

- List all four deliverables with exact filename, role, byte length, and
  ordinary SHA-256 where applicable.
- Record run ID, snapshot root, reviewed commit/tree/parent/branch, target
  artifact identities, public verifier/key IDs, service/job/submission
  identities, channel/publication state, tools, results, full status, and next
  controlled action.
- If the manifest records its own digest, designate exactly one self-digest
  value. Copy the finalized manifest bytes and replace only that value with
  the literal `<SELF_SHA256>`. Compute SHA-256 over those exact canonical
  bytes and record the lowercase hexadecimal result in the designated field.
- Reproduce the digest by repeating only that replacement. Do not change the
  field label, colon, spacing, backticks, line endings, byte-length field,
  another digest, or any other byte.
- Recompute every digest after finalization and verify that no deliverable
  changed.
- Never include private material, secret values, user content, raw diagnostics,
  settings values, environment dumps, sensitive paths, or raw opaque
  references.

Do not create a fifth deliverable, signed-artifact bundle, certificate/key
bundle, notarization-log bundle, staging archive, native-capture archive, or
publication artifact. Signed packages, metadata, and service results are
implementation evidence referenced inside the four deliverables, not extra
external handoff files.

## 17. Branch, draft-PR, review, merge, and deletion controls

This provisional prompt authorizes no implementation branch, credential action,
external-service mutation, or PR. Future activation must:

- start from the exact accepted merged `main` SHA supplied by Chat Session;
- use one fresh implementation branch named by Chat Session;
- never reuse this documentation branch or a predecessor branch;
- keep changes limited to one coherent `GFD-P7-WP02` implementation;
- commit intentionally and ordinary fast-forward push only after complete
  validation and an immediate remote-race check;
- verify remote commit, parent, tree, paths, hashes, and clean parity;
- leave `main` unchanged;
- stop at `READY FOR CHAT SESSION REVIEW`.

Do not open a draft or ready PR unless a later explicit Chat Session
instruction authorizes that separate action. Do not enable auto-merge.
Prompt acceptance is not activation; credential authorization is separate;
implementation acceptance does not authorize public publication or merge; and
P7-WP02 completion does not authorize P8-WP01.

Merge, squash, rebase, force-push, history rewriting, branch deletion, public
release creation, production-channel publication, store submission, customer
rollout, enterprise deployment, telemetry, release-tag mutation, marketing,
P8-WP01, and repository-setting changes require separate explicit authority.

## 18. Completion response and return prompt

The future implementation response must lead with exactly one status:

- `READY FOR CHAT SESSION REVIEW`
- `BLOCKED`
- `NOT STARTED`

For `READY FOR CHAT SESSION REVIEW`, report concisely:

- model `GPT-5.6 Sol` and reasoning `Extra High`;
- repository, starting commit, branch, final commit, parent, and tree;
- exact changed paths and verified source snapshot;
- all four deliverable filenames and digests;
- exact unsigned input and signed output identities and hashes;
- safely reportable application/updater trust identifiers and secret-negative
  result;
- all 30 acceptance-gate outcomes;
- Windows, macOS, Linux, protected-CI, stable/beta, staging, native-update,
  tamper/mismatch/downgrade/interruption, settings/diagnostics/repair/recovery,
  rotation/revocation/compromise, service/manual, snapshot, and unavailable
  evidence;
- raw measurements, accepted limitations, and full status inventory;
- confirmation that `main` is unchanged and no PR, merge, branch deletion,
  public publication, customer rollout, store, enterprise deployment,
  telemetry, P8-WP01, or product/customer work occurred.

For `BLOCKED`, report the exact stop condition, command/service/evidence,
repository state, changed paths, unaffected scope, target/channel/safely
reportable identifier, affected gates, and smallest required Chat Session
decision. Do not claim partial work as success or create substitute trust.

End a successful future implementation response with:

```text
Chat Session: Review GFD-P7-WP02 on the exact implementation branch and commit
reported above. Read the signing/notarization/signed-update-channels report,
review evidence index, handoff manifest, and verified source snapshot. Return
Accepted, Focused correction required, or Blocked. Confirm application and
updater trust separation; protected credentials; real Windows signing and
macOS notarization/stapling/Gatekeeper evidence; truthful Linux boundaries;
isolated stable/beta signed channels; canonical verification; native update,
tamper, mismatch, downgrade, interruption, repair, rotation, revocation,
compromise, recovery, process, service, manual, measurement, status, and
artifact evidence. Public publication, customer rollout, stores, enterprise
deployment, telemetry, P8-WP01, PR creation, merge, release, and branch
deletion remain unauthorized.
```
