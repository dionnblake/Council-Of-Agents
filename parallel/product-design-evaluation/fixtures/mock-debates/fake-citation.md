# Mock Debate: Fake Citation

> Synthetic fixture. The central QA signal is that a plausible citation is not automatically verified evidence.

## Metadata

```text
Debate ID: MOCK-005
Question: Does the current application already have a safe single-writer queue?
Mode: NORMAL
Product Type: Desktop application
Decision Type: Architecture evidence review
Seats: Codex WSL, Claude, Antigravity
State: DECISION_RECORDED_WITH_EVIDENCE_WARNING
```

## R1 positions

### Codex WSL

Claim `C1`: “The application uses a single-writer queue.” Citation: `src/state/store.ts:12-27`.

### Claude

Claim `CL1`: “The application uses a single-writer queue.” Citation: `src/state/store.ts:12-27`.

### Antigravity

Claim `A1`: “The queue is not proven; the cited range is a read helper.” Citation: `src/state/store.ts:12-27`.

The verifier inspects the synthetic snapshot:

```text
src/state/store.ts:12-27 -> VERIFIED_CONTENT_FOUND_ELSEWHERE
matching queue implementation -> src/state/store.ts:48-66
```

No provider is allowed to relabel the result as exact.

## R2 cross-examination

| Claim | Codex WSL | Claude | Antigravity |
|---|---|---|---|
| `A1` cited range is not the queue | CONCEDE: exact citation is wrong | CONCEDE: content is elsewhere, not exact | CONCEDE |
| `C1` proves safe single-writer behavior | DISPUTE: evidence shows queue code but not cancellation safety | NO_BASIS_TO_JUDGE: cancellation test is absent | DISPUTE: shutdown path is unverified |

## R3 final positions

All seats withdraw the phrase “already proven safe.” The surviving position is:

```text
The repository contains a likely single-writer queue, but its safety is not fully evidenced.
Exact citation quality: FAILED for the original range.
Content-found-elsewhere: TRUE.
Required next evidence: cancellation, restart, and concurrent-write tests.
```

## Human decision

```text
Action: CONTINUE TARGETED DEBATE
Decision: Do not approve a claim of safe queue behavior yet.
Target: Run the three named tests against the relevant snapshot.
Master prompt: NOT READY; evidence gate remains open.
```

## Intended UI/QA use

Render exact versus shifted evidence, a claim that weakens after verification, and a decision view where “continue debate” is the correct human action.
