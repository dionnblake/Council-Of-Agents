# Mock Debate: No Basis to Judge

> Synthetic fixture. `NO_BASIS_TO_JUDGE` is used with a concrete missing-evidence explanation, not as a silent skip.

## Metadata

```text
Debate ID: MOCK-006
Question: Is cloud-first architecture safe for this offline casework product?
Mode: COMPARE
Product Type: Sensitive casework application
Decision Type: Data architecture
Seats: Codex WSL, Claude, Antigravity
State: DECISION_RECORDED_AS_REJECTED_PENDING_EVIDENCE
```

## R1 positions

| Seat | Recommendation | Commitment | Key claim |
|---|---|---|---|
| Codex WSL | Local-first with explicit sync | CONDITIONAL | Offline work is a stated field constraint |
| Claude | Cloud-first with offline cache | CONDITIONAL | Shared access may simplify support |
| Antigravity | Local-first | CONDITIONAL | Sensitive records need device and sync policy |

## R2 cross-examination

Claude asks whether a cloud-first design can meet offline conflict resolution and device-loss requirements. Codex WSL responds:

```text
Response: NO_BASIS_TO_JUDGE
Explanation: The packet contains no identity model, conflict policy, device-loss procedure, or data-retention requirement. I cannot responsibly determine whether cloud-first can satisfy those constraints.
Missing evidence: identity/authorization model, offline duration, conflict authority, encryption and recovery requirements.
```

Antigravity disputes the idea that cloud-first is automatically simpler, citing only the product brief. Claude concedes that “offline cache” is underspecified and withdraws its unconditional support.

## R3 final positions

- Codex WSL: `CONDITIONAL Local-first`, pending explicit sync and identity design.
- Claude: `CONDITIONAL`, no winner until conflict and device-loss requirements are specified.
- Antigravity: `CONDITIONAL Local-first`, with a cloud sync boundary rather than cloud authority by default.

Strongest counterargument: cloud authority may reduce multi-user conflict if the product can tolerate connectivity and secure device recovery.

## Human decision

```text
Action: REJECT ALL
Rationale: The current candidate definitions are too vague for a safe architecture decision.
Required next inputs: offline duration, conflict authority, identity, encryption, retention, and device-loss requirements.
Master prompt: NOT READY; no implementation direction approved.
```

## Intended UI/QA use

Show an abstention explanation, unresolved evidence panel, rejected decision, and the difference between “no basis” and provider failure.
