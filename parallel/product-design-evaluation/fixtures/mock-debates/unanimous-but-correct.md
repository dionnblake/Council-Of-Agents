# Mock Debate: Unanimous but Correct

> Synthetic fixture. No provider was called. Evidence paths and hashes are illustrative.

## Metadata

```text
Debate ID: MOCK-001
Question: Should this Windows-first local-first tool keep SQLite for V1?
Mode: COMPARE
Product Type: Windows desktop application
Decision Type: Persistence architecture
Seats: Codex WSL, Claude, Antigravity
State: DECISION_RECORDED
```

## Preflight

| Seat | Requested model | Served status | Certification | State |
|---|---|---|---|---|
| Codex WSL | gpt-5.6-luna | PROVIDER_DOES_NOT_REPORT | PASS_WITH_DECLARED_LIMITATION | READY |
| Claude | claude-haiku-4-5 | VERIFIED | PASS | READY |
| Antigravity | gemini-3.7-flash-low | PROVIDER_DOES_NOT_REPORT | PASS_WITH_DECLARED_LIMITATION | READY |

Repository snapshot: `snap-sqlite-v1`, synthetic hash `sha256:mock001-snapshot`.

## R1 independent positions

### Codex WSL — WOULD_STAKE SQLite

- `C1`: `crates/state/src/store.rs:12-58` shows a single-writer queue around SQLite. `VERIFIED_EXACT`.
- `C2`: `docs/product.md:20-31` requires offline operation and no hosted dependency. `VERIFIED_EXACT`.
- `C3`: PostgreSQL would add a network/database lifecycle without a current multi-user requirement. `INFERENCE` from C1/C2.
- Risks: future sync and multi-user access would require a storage boundary and migration.
- Flip condition: concurrent users or server-authoritative sync become V1 requirements.
- Cost if wrong: moderate migration work, reduced by keeping repository/storage interfaces separate.
- Reversibility: moderate.

### Claude — WOULD_STAKE SQLite

- `CL1`: the current data volume and one-user workflow fit an embedded store. `INFERENCE` from `src/config/limits.ts:4-18`, `VERIFIED_EXACT`.
- `CL2`: offline durability is a hard constraint, while hosted database availability is not. `FACT` from `docs/product.md:20-31`, `VERIFIED_EXACT`.
- `CL3`: PostgreSQL solves a future topology problem rather than a demonstrated V1 problem. `INFERENCE`.
- Risks: backup/export and migrations must be tested.
- Flip condition: a measured concurrency or sync requirement appears.
- Cost if wrong: later migration and possible sync redesign.
- Reversibility: moderate.

### Antigravity — CONDITIONAL SQLite

- `A1`: SQLite is the best fit if the single-writer constraint remains enforced. `INFERENCE` from `crates/state/src/store.rs:12-58`, `VERIFIED_EXACT`.
- `A2`: a PostgreSQL path is not justified by scale language alone. `INFERENCE`.
- `A3`: the decision must include export/import and migration tests. `ASSUMPTION` until test evidence exists.
- Flip condition: sync or multiple writers become required.
- Cost if wrong: migration complexity.
- Reversibility: moderate.

## R2 cross-examination

| Peer claim | Codex WSL | Claude | Antigravity |
|---|---|---|---|
| `A1` single-writer condition | CONCEDE: it is the controlling invariant | CONCEDE: it matches the snapshot | CONCEDE: no contrary evidence |
| `CL3` future topology problem | CONCEDE: future need is not current need | CONCEDE: candidate set should not encode a cloud bias | CONCEDE: migration path remains necessary |
| `C2` offline hard constraint | CONCEDE with exact citation | CONCEDE | CONCEDE |

## R3 final positions

All three seats retain their recommendation. Antigravity changes `CONDITIONAL` to `WOULD_STAKE` after the shared snapshot confirms a single-writer queue and no server requirement.

Remaining dispute: none material. Minority position: no minority on winner; all seats preserve the migration warning.

## Human decision

```text
Action: APPROVE OPTION
Approved option: Keep SQLite for V1
Rationale: It satisfies offline/local constraints with lower operational burden.
Conditions: Preserve storage interface, export/import, backup, and migration tests.
Highest-impact flip condition: Multi-user or server-authoritative synchronization becomes required.
Master prompt: READY
```

## Intended UI/QA use

Render agreement without a vote counter. Show three attributable positions, exact evidence, a calm shared tradeoff, and a decision gate that still required human approval.
