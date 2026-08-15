# Provider Failure Matrix

## State vocabulary

```text
READY
PREFLIGHT_BLOCKED
AUTH_REQUIRED
QUOTA_LIMIT
OFFLINE
TIMEOUT
MALFORMED_OUTPUT
REPAIR_FAILED
UNKNOWN_FAILURE
PARTIAL_ROUND
PAUSED_FOR_HUMAN
CANCELLED
```

`DEGRADED COUNCIL` means the human has explicitly continued with fewer certified/available seats and the missing seat plus limitation are recorded. It is never an automatic fallback.

## Matrix

| Failure | Detection | User message | Retry policy | Council state | Can debate continue? | Degraded mode? | Audit event |
|---|---|---|---|---|---|---|---|
| Claude quota exhausted | Provider exit/error signature or certified quota response | “Claude quota is unavailable. No Claude position was accepted.” | No automatic repeat; allow recheck after user chooses wait, remove, or repair | `QUOTA_LIMIT` -> `PAUSED_FOR_HUMAN` | Only after explicit decision; completed artifacts remain visible | Yes, if two-seat policy permits | provider, quota class, timestamp, attempted round, user action |
| Claude auth expired | Auth check or provider auth error | “Claude authentication is required before this seat can run.” | User repairs auth, then reruns preflight; do not send packet while invalid | `AUTH_REQUIRED` | No with Claude required; otherwise explicit degraded mode | Yes, explicit | auth status, no credential values, repair result |
| Claude returns wrong model | Served identity reports a mismatch | “Claude served a different model than requested; review the limitation before continuing.” | No blind retry; user may select/accept a verified alternative | `PREFLIGHT_BLOCKED` or `PAUSED_FOR_HUMAN` | Only after human accepts the limitation | Possibly | requested, served, verification status, human choice |
| Codex WSL not running | Distro status/launch failure | “CouncilCodexWSL is not available. The Codex seat is paused.” | One bounded restart/recheck; then human repair or removal | `OFFLINE` | Other seats may finish; no false Codex position | Yes, if permitted | distro check, restart attempt, exit status |
| Codex WSL auth expired | Linux auth status or provider auth error | “Codex WSL authentication is required. No Windows credential was copied.” | Human authenticates inside certified distro; rerun preflight | `AUTH_REQUIRED` | Only without Codex if degraded mode is approved | Yes, explicit | auth category, distro identity, repair result |
| Codex distro corrupted | Boundary/config/hash/identity check fails | “The certified Codex environment failed integrity checks and is blocked.” | No automatic rebuild or credential migration; human repair/re-certification | `PREFLIGHT_BLOCKED` | No Codex; other seats depend on policy | Yes only with human approval | integrity failures, certification status |
| Codex packet transfer fails | Windows/Linux manifest or payload hash mismatch, bridge error | “The Codex packet was not verified in WSL. No position was accepted.” | One retry with same immutable packet; then pause | `UNKNOWN_FAILURE` or `PAUSED_FOR_HUMAN` | Other valid seats can remain; round incomplete | Yes, not as full council | packet hash, bridge result, attempt number |
| Antigravity quota exhausted | Certified provider quota/credit response | “Antigravity quota or credits are unavailable; no fallback billing path was used.” | No silent credit switch; user repairs or removes seat | `QUOTA_LIMIT` | Continue only under explicit degraded policy | Yes, explicit | credit guard result, no-spend assertion, user action |
| Antigravity credit guard changes | Preflight detects guard mismatch or unverifiable guard | “Antigravity credit protection changed or cannot be verified.” | Block dispatch until re-certified; no automatic update/install | `PREFLIGHT_BLOCKED` | No Antigravity; two-seat only if approved | Yes, explicit | guard version/status, certification block |
| Antigravity malformed JSON | Output parser/schema validation fails | “Antigravity returned unusable structured output. The raw response is quarantined.” | One bounded repair only if certified policy allows; otherwise stop | `MALFORMED_OUTPUT` | Other seats may remain; round is partial | Yes, if policy allows | raw hash, validator errors, repair attempt |
| Antigravity repair also fails | Repair output remains invalid or changes packet contract | “Repair did not produce a valid position. No repaired answer was accepted.” | No second automatic repair; human may retry same packet later | `REPAIR_FAILED` | Do not treat as abstention; round incomplete | Yes, explicit | original/repair hashes, reasons |
| Any provider times out | Wall-clock deadline and process supervision | “The provider timed out. No position was accepted from this attempt.” | Cancel process tree; at most one user-approved retry with same packet | `TIMEOUT` | Other seats can complete; final requires partial handling | Yes, explicit | deadline, process IDs, cancellation, retry choice |
| Any provider returns empty output | Exit success but no usable bytes/structured result | “The provider returned no usable output.” | One bounded retry after confirming process ended; then pause | `MALFORMED_OUTPUT` or `UNKNOWN_FAILURE` | Not as a valid position | Yes, explicit | byte count, exit status, attempt metadata |
| Any provider refuses | Structured refusal or refusal text under a valid process | “The provider declined this position. It is not a concession or evidence.” | Do not prompt-shop; user may inspect/remove/retry once | `PAUSED_FOR_HUMAN` | Other seats may continue; R2 cannot assume response | Yes, explicit | refusal category, packet hash, user action |
| Any provider crashes | Nonzero exit, signal, missing artifact, or process tree failure | “The provider process failed before producing a valid position.” | Verify no child remains; one user-approved retry if safe | `UNKNOWN_FAILURE` or `PARTIAL_ROUND` | Preserve completed seats; no automatic consensus | Yes, explicit | exit/signal, process cleanup, scratch state |

## General rules

- A failed provider is never represented as `CONCEDE`, `DISPUTE`, or `NO_BASIS_TO_JUDGE`.
- A retry uses the same immutable question, candidate set, snapshot, and requested model unless the human changes them and the attempt is recorded as a new input.
- Repair output is independently validated and cannot bypass schema, semantic, citation, or safety checks.
- Unknown failures pause the affected round and require human-visible recovery.
- Degraded mode names missing seats in the debate header, decision record, export, and master prompt.
- No failure path launches another provider, changes billing, or opens a coding harness without an explicit human action outside Council's runtime.
