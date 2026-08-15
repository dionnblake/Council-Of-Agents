# Decision Record and ADR Specification

## Purpose

The decision record preserves why a human chose a direction. The master prompt translates that approved direction for later implementation. They are related artifacts with different jobs.

## Record properties

- Durable and locally exportable.
- Append-only after a decision, with amendments rather than silent rewrites.
- Attributable to debate, seats, provider certifications, evidence snapshot, and human action.
- Reconstructable without hidden provider session state.
- Clear about current status, limitations, and supersession.

## Required record format

```text
Decision ID
Record version
Date/time and timezone
Debate ID
Repository path label or NO_REPOSITORY
Repository SHA or snapshot ID/hash
Question
Product type
Decision type
Mode: COMPARE | DISCOVERY
Candidate options
Hard constraints
Primary decision priority
Selected seats
Requested models
Served-model status
Provider certifications and limitations
Round plan and state
Opening positions
Key claims and evidence
Disagreements and concessions
NO_BASIS_TO_JUDGE explanations
Position revisions
Final positions
Human decision and rationale
Approved modifications
Minority position
Risks and cost if wrong
Flip condition
Reversibility and migration path
Acceptance criteria
Implementation constraints
Unresolved questions
Master prompt ID/hash
Exports and artifact hashes
Audit events
Supersedes / superseded by
```

## Field rules

### Identity and provenance

`Decision ID`, debate ID, snapshot/hash, record version, and source timestamps identify the exact decision context. A repository SHA is included only when a repository was supplied and the value is available; greenfield records say `NO_REPOSITORY`.

### Provider and model fields

Requested model is what the human selected. Served model is provider evidence, `VERIFIED`, or `PROVIDER_DOES_NOT_REPORT`. Certification status is recorded at preflight time and is not retroactively inferred from a successful response.

### Positions and evidence

Opening and final positions retain seat attribution, commitment, claim IDs, evidence statuses, risks, assumptions, reversibility, cost if wrong, and flip conditions. The record stores withdrawn claims and revision attribution instead of replacing history.

### Human decision

The record identifies the exact human action and rationale. If the human approves a modified decision, the modification is explicit and traceable. `Reject All` and `Continue Targeted Debate` are valid outcomes and must not create an approved implementation direction.

### Amendment and supersession

After export, corrections create a new record version with:

```text
amendment ID
author/action
reason
fields changed
prior record hash
new record hash
```

Do not silently edit the historical decision. A later debate can supersede an earlier decision while preserving both.

## ADR-style lifecycle

```text
DRAFT
-> DEBATED
-> HUMAN_DECISION_RECORDED
-> EXPORTED
-> ACCEPTED_FOR_IMPLEMENTATION (human-controlled external status)
-> SUPERSEDED or AMENDED
```

Council may create through `EXPORTED`. It must not claim that an external coding harness implemented or accepted the decision.

## Example record

```text
Decision ID: ADR-2026-0017
Question: Keep SQLite or migrate to PostgreSQL for V1?
Repository: snapshot=snap-sqlite-v1; hash=sha256:synthetic
Mode: COMPARE
Seats: Codex WSL, Claude, Antigravity
Requested models: recorded per seat
Served status: Claude VERIFIED; Codex/Antigravity PROVIDER_DOES_NOT_REPORT
Options: SQLite; PostgreSQL
Constraints: Windows-first, local-first, one active user, no hosted dependency
Final positions: all retain SQLite; migration warning preserved
Human decision: APPROVE OPTION, keep SQLite for V1
Rationale: satisfies offline/local requirements with lower operational burden
Risks: future sync or multi-user requirements may force migration
Flip condition: server-authoritative synchronization becomes V1 scope
Reversibility: moderate; preserve storage interface and migration tests
Master prompt: MP-2026-0017, hash=sha256:synthetic
Status: EXPORTED
```

## Decision-record acceptance

- Record can be opened without provider availability.
- Human action and rationale are visible before the master prompt.
- Dissent, evidence limitations, and flip condition survive export.
- Same approved state reproduces the same substantive record.
- Amendment creates a new version and preserves prior hash.
- No record claims an external implementation occurred unless supplied as an external human update.
