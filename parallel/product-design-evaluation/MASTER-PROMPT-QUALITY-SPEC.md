# Master Prompt Quality Specification

## Purpose

The master prompt is Council's final product artifact. It translates a human-approved decision record into a clear prompt that another human may manually copy into a coding harness. It is not a transcript, a vote, an implementation command, or a provider handoff.

## Quality contract

A good master prompt is:

- traceable to approved Council state;
- explicit about constraints and non-goals;
- honest about evidence quality and unresolved dissent;
- specific enough to guide implementation without inventing requirements;
- deterministic for the same approved state;
- provider-neutral unless the human approved a provider-specific constraint;
- bounded by an explicit manual-copy stop boundary.

## Required sections

Every approved master prompt contains, in this order:

1. **Decision identity:** debate ID, decision date, mode, product/decision type, and source state hash.
2. **Human decision:** exact action (`Approve Option`, `Approve Modified Decision`, or another recorded action), selected option, modifications, and rationale.
3. **Problem and outcome:** the problem being solved and the desired outcome, without restating irrelevant transcript.
4. **Hard constraints:** platform, offline, cost, security, time, team, compliance, and other approved constraints.
5. **Approved direction:** the selected architecture, stack, design, or decision in plain language.
6. **Implementation requirements:** only requirements traceable to approved claims, evidence, or human additions.
7. **Acceptance criteria:** observable tests and UX/operational conditions.
8. **Evidence and traceability:** claim IDs, source paths/ranges, verification status, and assumptions.
9. **Risks and dissent:** minority position, remaining disputes, strongest counterargument, and cost if wrong.
10. **Flip conditions and reversibility:** facts that should trigger reconsideration and how to leave the choice.
11. **Non-goals and boundaries:** what the implementation must not add, including no automatic provider action from Council.
12. **Manual handoff stop:** “This artifact is prepared for human copy. Council does not implement, run, send, publish, or deploy it.”

## Traceability requirements

Every important implementation requirement maps to one of:

```text
HUMAN_DECISION
APPROVED_CLAIM:<claim_id>
VERIFIED_EVIDENCE:<path:range>
APPROVED_CONSTRAINT
APPROVED_MODIFICATION
```

The compiler must reject or visibly label a requirement with no source. It must preserve `VERIFIED_CONTENT_FOUND_ELSEWHERE` and `UNVERIFIED` rather than upgrading them. It must not infer a missing acceptance criterion from a provider's prose.

## What the prompt must never do

- Add a framework, database, feature, or requirement that no human or approved Council state selected.
- Remove the strongest minority position, unresolved dispute, cost if wrong, or flip condition.
- Present an assumption as a fact.
- Present a requested model as a verified served model.
- Include secrets, credentials, private provider configuration, or uncontrolled repository instructions.
- Ask the receiving harness to bypass safety, approval, or repository boundaries.
- Start a command, call a provider, open a coding harness, publish, deploy, or send itself.
- Use vague implementation instructions such as “make it modern” without observable criteria.
- Convert `CONDITIONAL` into unconditional approval.

## Decision-type requirements

### Architecture decisions

Include boundary, data ownership, failure modes, migration path, operational burden, testing, and long-term ownership.

### Stack decisions

Include best fit, runner-up, winner weaknesses, disqualifiers, cost to leave, migration path, operational complexity, boring alternative, and AI-assisted-development tradeoff.

### Design decisions

Include hierarchy, density, typography, component philosophy, state coverage, accessibility, platform behavior, and visual acceptance criteria. “Premium” is not enough.

### Greenfield decisions

State `NO_REPOSITORY`. Separate product requirements, external facts, and assumptions. Do not invent file paths or repository evidence.

### Existing-repository decisions

Include repository snapshot ID/hash, exact citations, evidence status, and any files intentionally excluded from provider context.

## Conditional decision format

Use a visible block:

```text
DECISION STATUS: CONDITIONAL
APPROVED ONLY IF:
- observable condition one
- observable condition two
DO NOT PROCEED AS APPROVED IF:
- flip condition
```

## Gold-standard examples

The examples below are synthetic and illustrate quality, not universal answers.

### Gold 1: Architecture

```text
DECISION: Keep SQLite for V1 of the Windows-first local-first product.
SOURCE: MOCK-001; human APPROVE OPTION.

REQUIREMENTS:
- Keep a single-writer persistence boundary.
- Add export/import and backup coverage.
- Keep the storage interface replaceable for a future sync/server requirement.

EVIDENCE:
- crates/state/src/store.rs:12-58 [VERIFIED_EXACT], claims C1/CL1.
- docs/product.md:20-31 [VERIFIED_EXACT], claim C2.

RISKS AND FLIP CONDITION:
SQLite is not approved as a universal scale answer. Reconsider when multiple writers or server-authoritative synchronization become V1 requirements. The strongest counterargument is future migration cost; preserve the boundary and migration tests.

STOP: This is a human-approved implementation brief. Council does not implement, run, or send it.
```

### Gold 2: Stack selection

```text
BEST FIT: Tauri + Rust core + React for the Windows-first local technical workstation.
RUNNER-UP: Electron + React if required Node-native dependencies cannot cross the native boundary.
WINNER IS BAD AT: broad Node ecosystem reuse and zero-cost cross-platform expansion.
DISQUALIFIERS: a required dependency cannot be safely bridged, or the platform boundary cannot be certified.
COST TO LEAVE: moderate shell/IPC rewrite; keep core logic outside the shell.
MIGRATION PATH: isolate domain logic and provider contracts behind stable interfaces.
BORING ALTERNATIVE: a conventional Windows native shell remains viable if React reuse is not a requirement.

ACCEPTANCE: prove IPC, cancellation, short-path packaging, and provider isolation before expanding scope.
```

### Gold 3: Design

```text
DESIGN DIRECTION: Use a claim-and-evidence command center with selective containers.

REQUIREMENTS:
- Keep question, round, unresolved dispute, and human decision visible above transcripts.
- Use aligned rows/tables for claims and evidence; reserve strong containers for provider status and the decision gate.
- Use deliberate title, section/body, metadata, and monospaced evidence typography.
- Make evidence statuses and keyboard focus understandable without color.

CANNOT_DETERMINE: dark-mode contrast and 125% scaling require visual verification.
NON-GOALS: no gradient hero, AI orb, provider-send control, or generic card grid.
```

### Gold 4: Conditional existing-repository decision

```text
DECISION STATUS: CONDITIONAL
APPROVED DIRECTION: Native Android for V1.
APPROVED ONLY IF: Bluetooth, offline queue recovery, and rugged-device lifecycle tests pass.
EVIDENCE: android/src/device/BluetoothScanner.kt:41-93 [VERIFIED_EXACT]; android/src/sync/OfflineQueue.kt:10-68 [VERIFIED_EXACT].
WITHDRAWN POSITION: React Native was withdrawn after exact device evidence changed the weighting.
FLIP CONDITION: the product adds a second platform before the device boundary is stable.
```

### Gold 5: Greenfield decision

```text
CONTEXT: NO_REPOSITORY. This is a greenfield local-first Windows workstation decision.
HUMAN DECISION: Approve Tauri + Rust core + React, subject to IPC and packaging gates.
ASSUMPTIONS: Windows is the only launch platform; no multi-user cloud control plane is required in V1.
REQUIREMENTS: local state, explicit process boundary, deterministic export, provider certification, and keyboard-completable review.
UNKNOWN: future web companion shape; do not implement it as part of this decision.
```

## Quality review questions

- Can every requirement be traced to the approved state?
- Is the human decision visible before implementation detail?
- Is dissent preserved without being treated as authority?
- Are assumptions, unknowns, and unverified citations labelled?
- Does the prompt contain observable acceptance criteria?
- Does it avoid provider-specific or execution-specific instructions unless approved?
- Would the same approved state compile to the same content?
- Does the prompt end at manual copy rather than automatic execution?
