# Mock Debate: Evidence Changes Position

> Synthetic fixture. One seat changes recommendation because repository evidence changes the constraint interpretation.

## Metadata

```text
Debate ID: MOCK-004
Question: Should this field app use React Native or native Android?
Mode: COMPARE
Product Type: Android field-service app
Decision Type: Mobile stack
Seats: Codex WSL, Claude, Antigravity
State: DECISION_RECORDED
```

## R1 positions

| Seat | R1 commitment | Recommendation | Load-bearing claims |
|---|---|---|---|
| Codex WSL | CONDITIONAL | React Native | Shared UI could preserve future iOS option; device features may require native modules |
| Claude | WOULD_STAKE | React Native | Team has JavaScript experience; common form UI is cross-platform |
| Antigravity | WOULD_NOT_STAKE | Native Android | Offline Bluetooth workflow and rugged-device lifecycle favor native |

Repository evidence available to R1 includes only the product brief, not device code. The R1 packet correctly labels hardware assumptions as assumptions.

## R2 evidence challenge

Antigravity supplies a peer request for the following snapshot evidence:

```text
android/src/device/BluetoothScanner.kt:41-93
android/src/sync/OfflineQueue.kt:10-68
android/src/device/RuggedLifecycleReceiver.kt:5-44
```

The controller verifies all three as `VERIFIED_EXACT`. They show custom Bluetooth framing, a foreground-resume requirement, and a device vendor lifecycle callback not covered by the selected React Native modules.

| Peer claim | Codex WSL | Claude | Antigravity |
|---|---|---|---|
| Native device behavior is load-bearing | CONCEDE: new exact evidence changes the platform constraint | CONCEDE: shared UI is no longer the primary risk | CONCEDE |
| React Native preserves iOS optionality | CONDITIONAL: only if native module ownership is accepted | DISPUTE: optionality is not a current requirement | CONCEDE as a future benefit, not a V1 winner |

## R3 final positions

- Codex WSL revises `CONDITIONAL React Native` to `WOULD_STAKE Native Android` because the exact device boundary is now a hard constraint.
- Claude revises `WOULD_STAKE React Native` to `CONDITIONAL Native Android`, naming React Native as a future UI option only after device integration is proven.
- Antigravity retains `WOULD_STAKE Native Android`.

Surviving claim: device integration and lifecycle reliability outweigh speculative iOS reuse for V1.

## Human decision

```text
Action: APPROVE OPTION
Approved option: Native Android for V1.
Rationale: The new repository evidence changed the weighting, not merely the prose.
Acceptance criteria: Bluetooth fixture tests, offline queue recovery, rugged lifecycle test, and an explicit future UI extraction boundary.
Master prompt: READY
```

## Intended UI/QA use

Show a visible revision attribution from R1 to R3, evidence arriving in R2, and the difference between a future option and a current requirement.
