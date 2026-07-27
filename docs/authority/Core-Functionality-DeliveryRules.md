# Core Functionality Delivery Rules

These rules override any tendency to over-engineer, over-document, inflate test counts, or create the appearance of progress without working software.

## 1. Working functionality is the primary deliverable

Every work package must produce a real, runnable, user-visible or technically usable capability from the approved roadmap.

Architecture, documentation, policies, governance, audits, abstractions, and tooling are supporting work only. They must never become the main output unless explicitly requested.

## 2. No fake implementation

Do not count any of the following as completed functionality:

* Empty methods or classes
* Placeholder return values
* Hard-coded success responses
* Mock-only implementations
* Fixtures presented as production data
* TODO-based logic
* Interfaces without working implementations
* Functions that only raise “not implemented”
* Tests that validate stubs, mocks, or meaningless wrappers

A feature is complete only when its real execution path works.

## 3. Build vertical slices first

Implement the smallest end-to-end path through the real system before expanding architecture.

Each major capability should be demonstrated through its complete flow:

```text
input → validation → core logic → persistence/integration → output
```

Do not build dozens of supporting components before one core feature works end to end.

## 4. Follow roadmap priority strictly

Work only on functionality required by the current roadmap milestone.

Do not add speculative frameworks, future-proof abstractions, governance systems, plugin architectures, policy engines, audit layers, or unrelated infrastructure unless they are immediately required by a current core feature.

## 5. Keep tests proportional

Write only tests that protect important behavior:

* Core business logic
* Critical failure paths
* Data integrity
* Important integration boundaries
* A small end-to-end smoke test

Do not create large test suites for trivial getters, wrappers, generated code, configuration files, stubs, fixtures, or low-value scaffolding.

Test count is never a measure of project progress.

## 6. Prove functionality continuously

At the end of every meaningful milestone, provide:

* Exact run command
* Exact test command
* A short manual verification procedure
* The real output or behavior expected
* A list of implemented core capabilities
* A list of anything incomplete or temporary

Do not postpone real execution and user testing until the end of the project.

## 7. Never hide incompleteness

Clearly label every item as:

```text
Implemented
Partially implemented
Stub
Mock-only
Not started
Blocked
```

Never describe scaffolding, interfaces, tests, documents, or planned components as completed product functionality.

## 8. Maintain a truthful implementation inventory

The project inventory must distinguish between:

* Production functionality
* Supporting infrastructure
* Tests
* Documentation
* Generated code
* Fixtures and mocks
* Stubs and placeholders

For every major feature, identify the actual files and execution path that implement it.

## 9. Stop unnecessary complexity

Before adding any abstraction, framework, dependency, policy, or new layer, ask:

```text
Is this required to make a current core feature work?
```

If the answer is no, defer it.

Prefer direct, readable, working code over elaborate architecture.

## 10. Communication must stay concise

When chatting, answering questions, reporting completion, or proposing the next step:

* Give the direct answer first.
* Keep explanations short and focused.
* Do not repeat known context.
* Do not produce long reports unless requested.
* Do not explain every internal decision.
* Mention risks, failures, and incomplete work plainly.
* Let the user ask when more detail is needed.

## Final completion rule

A project or milestone must never be called complete unless its promised core functionality can be run and verified locally without relying on fake data, hidden stubs, mock-only paths, or future implementation.
