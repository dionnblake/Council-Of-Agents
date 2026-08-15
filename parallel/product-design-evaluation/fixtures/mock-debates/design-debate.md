# Mock Debate: Design Debate

> Synthetic fixture. Design reasoning is expressed as observable hierarchy and state requirements, not a universal beauty score.

## Metadata

```text
Debate ID: MOCK-009
Question: Should the technical deliberation app keep its card dashboard or move to a claim-and-evidence command center?
Mode: COMPARE
Product Type: Windows desktop application
Decision Type: Product visual direction
Seats: Codex WSL, Claude, Antigravity
State: DECISION_RECORDED
```

## R1 positions

| Seat | Recommendation | Commitment | Load-bearing claim |
|---|---|---|---|
| Codex WSL | Claim/evidence command center | WOULD_STAKE | The decision and unresolved disputes must outrank transcript detail |
| Claude | Claim/evidence command center | WOULD_STAKE | Rows and evidence links support technical density better than equal cards |
| Antigravity | Hybrid: command center with selective cards | CONDITIONAL | Provider status and decision gate need bounded containers |

Evidence is visual fixture evidence: `screens/home-current.png`, `screens/debate-current.png`, and annotated task notes. Status: `OBSERVED_SYNTHETIC`, not live product proof.

## R2 cross-examination

| Claim | Codex WSL | Claude | Antigravity |
|---|---|---|---|
| Every section is a card in the current shell | CONCEDE | CONCEDE | CONCEDE |
| A claim table can replace all cards | DISPUTE: decision gate still needs emphasis | DISPUTE: use selective boundary containers | DISPUTE: provider status benefits from a rail |
| “Premium” requires more whitespace | NO_BASIS_TO_JUDGE: no task evidence supports that tradeoff | DISPUTE: density is a product requirement | NO_BASIS_TO_JUDGE: visual evidence lacks scaling states |

## R3 final positions

The seats converge on a hybrid command center:

- question, round, claims, and disputes use aligned rows/table structures;
- provider status uses a compact rail;
- evidence viewer uses a monospaced excerpt surface;
- human decision uses one clearly bounded action area;
- no full-screen gradient, excessive pills, or decorative AI orb;
- loading, partial, and failure states preserve the same hierarchy.

`CANNOT_DETERMINE`: dark-mode contrast and 125% scaling cannot be judged from the supplied images.

## Human decision

```text
Action: APPROVE MODIFIED DECISION
Approved direction: Claim/evidence command center with selective containers.
Acceptance criteria: question and decision remain visible; evidence statuses are non-color-only; keyboard flow works; loading/failure states are designed.
Master prompt: READY
```

## Intended UI/QA use

Render design-taste findings, `CANNOT_DETERMINE`, selective card rationale, and a decision that is more specific than “make it modern.”
