# Mock Debate: Provider Timeout

> Synthetic fixture. One seat times out after another completes; the result must remain partial and visibly degraded.

## Metadata

```text
Debate ID: MOCK-007
Question: Should this import workflow use a separate worker process?
Mode: COMPARE
Product Type: Desktop data-import tool
Decision Type: Background execution
Seats: Codex WSL, Claude, Antigravity
State: PAUSED_FOR_HUMAN
```

## R1 attempt and preflight

| Seat | Preflight | Attempt | State | Artifact |
|---|---|---|---|---|
| Claude | READY | R1 completed in 11.2 s | COMPLETE | position hash `synthetic-claude-007` |
| Antigravity | READY | R1 completed in 18.4 s | COMPLETE | position hash `synthetic-agy-007` |
| Codex WSL | READY | deadline at 60 s; process tree cancelled | TIMEOUT | no position accepted |

Claude recommends a separate worker process. Antigravity recommends a database-backed durable job. Codex has no position and is not labelled `NO_BASIS_TO_JUDGE`.

## Failure event

```text
Event: PROVIDER_TIMEOUT
Detection: deadline exceeded; child processes terminated
Retry policy: one user-approved retry with the same packet
Council state: PARTIAL_ROUND -> PAUSED_FOR_HUMAN
Degraded mode: available only after explicit human continuation
```

## R2/R3 boundary

R2 is not automatically started because R1 independence is incomplete. The user can retry Codex, continue with two seats under degraded policy, or cancel. No final position exists.

## Human decision

```text
Action: CONTINUE TARGETED DEBATE
Choice: Retry Codex with the same immutable packet.
Reason: The question is important enough that a two-seat result is not sufficient.
Master prompt: NOT READY
```

## Intended UI/QA use

Render completed provider rows beside a timed-out row, cancellation evidence, retry policy, and a disabled R2 action until the human resolves the partial round.
