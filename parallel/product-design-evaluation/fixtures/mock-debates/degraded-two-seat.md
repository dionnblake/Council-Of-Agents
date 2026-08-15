# Mock Debate: Degraded Two-Seat Council

> Synthetic fixture. Antigravity is unavailable; the human explicitly accepts a two-seat degraded council.

## Metadata

```text
Debate ID: MOCK-010
Question: Should this product use a local-first state layer or a cloud-first state layer?
Mode: COMPARE
Product Type: Local technical desktop tool
Decision Type: Data architecture
Seats selected: Codex WSL, Claude, Antigravity
State: DECISION_RECORDED_DEGRADED
```

## Preflight

| Seat | Status | Limitation |
|---|---|---|
| Codex WSL | READY | Served model not reported |
| Claude | READY | None |
| Antigravity | UNAVAILABLE | Standalone headless route not verified in current installation |

The UI states `DEGRADED COUNCIL: 2 of 3 selected seats available`. No Antigravity position is fabricated.

## Human continuation

```text
Action: CONTINUE WITH AVAILABLE SEATS
Reason: This is a reversible product direction question and the user accepts the limitation.
Constraint: Final record must name the missing seat and cannot call the result three-seat consensus.
```

## R1 positions

### Codex WSL — WOULD_STAKE local-first

- `C1`: local state supports offline technical work and keeps data under user control. `INFERENCE` from product packet.
- `C2`: cloud-first would add account and availability dependencies. `INFERENCE`.
- Flip condition: shared multi-user authority becomes a V1 requirement.

### Claude — CONDITIONAL local-first

- `CL1`: local-first fits the current Windows desktop scope. `INFERENCE`.
- `CL2`: backup, export, and future synchronization must be explicit. `ASSUMPTION` until tested.
- Strongest counterargument: cloud-first simplifies shared access and recovery.

## R2 cross-examination

| Claim | Codex WSL | Claude |
|---|---|---|
| Local-first is automatically safer | DISPUTE: local secrets and backup still need controls | DISPUTE: safety depends on storage and recovery design |
| Cloud-first is needed for future collaboration | NO_BASIS_TO_JUDGE: future collaboration is not specified | NO_BASIS_TO_JUDGE: no multi-user requirements supplied |

## R3 final positions

Codex retains `WOULD_STAKE local-first`. Claude revises from `CONDITIONAL` to `WOULD_STAKE local-first for V1`, with explicit export/backup and a future sync boundary.

Minority/limitation note: no third-seat perspective was available. Evidence quality is `CONDITIONAL`, because the debate is greenfield and contains no repository evidence.

## Human decision

```text
Action: APPROVE OPTION
Approved option: Local-first state layer for V1.
Conditions: backup/export, recovery, and migration boundary are acceptance criteria.
Declared limitation: two-seat degraded council; Antigravity unavailable.
Master prompt: READY WITH DEGRADED-COUNCIL NOTICE
```

## Intended UI/QA use

Render preflight unavailability, explicit degraded continuation, missing-seat disclosure in Decision View, and a valid decision without pretending to have three opinions.
