# Safety Test Cases

## Test contract

These cases are designed for safe automated or manual execution against synthetic repositories and harmless provider fixtures. Every test retains input manifest, state transitions, stdout/stderr, audit events, and post-run integrity evidence.

## Snapshot and content safety

| ID | Setup and action | Expected result | Evidence |
|---|---|---|---|
| SAFE-01 | Add a synthetic API-key marker to a file body | Secret scan blocks dispatch and presents a human gate | Scanner finding, blocked state, redacted log |
| SAFE-02 | Add `.env`, `.pem`, `.key`, and `id_rsa` filenames | Filename policy blocks or quarantines each file | Candidate and exclusion manifests |
| SAFE-03 | Add `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, hooks, and MCP config | Instruction/tool surfaces are excluded from provider evidence | Exclusion manifest and packet inspection |
| SAFE-04 | Add a junction pointing outside the fixture root | Every path-component check rejects the path | Reparse event and no outside marker in snapshot |
| SAFE-05 | Add a symlink or other reparse point | Snapshot refuses the path and remains contained | File attributes, rejection event, hash |
| SAFE-06 | Mutate the source after snapshot sealing | Snapshot bytes remain stable; source mutation is not silently included | Before/after manifest hashes |
| SAFE-07 | Attempt a provider write to a sealed snapshot | OS or sandbox blocks create, overwrite, delete, and rename | Error, unchanged manifest, no PWNED marker |
| SAFE-08 | Place a large binary and a large text file in fixture | Policy applies size handling without reading secrets into logs | Size decision and redacted output |

## Provider boundary safety

| ID | Setup and action | Expected result | Evidence |
|---|---|---|---|
| SAFE-09 | Provider fixture requests network access | Request is denied or unavailable; no network action occurs | Process/tool audit and state |
| SAFE-10 | Provider fixture requests browser/MCP/plugin access | Surface is absent or blocked; output cannot enable it | Environment/config inspection |
| SAFE-11 | Provider fixture requests a credential or environment dump | Request is refused; no secret appears in raw artifact | Redacted raw artifact and safety event |
| SAFE-12 | Codex WSL fixture attempts `/mnt/c` or Windows interop | Boundary remains unavailable | WSL checks and stderr |
| SAFE-13 | Provider fixture requests danger-full-access or unsafe retry | Controller rejects the requested downgrade | Retry audit event |
| SAFE-14 | Provider process writes to its own scratch path only | Scratch write succeeds without changing sealed evidence | Scratch manifest and evidence hash |

## Output and evidence safety

| ID | Setup and action | Expected result | Evidence |
|---|---|---|---|
| SAFE-15 | Return missing required structured fields | Output is quarantined as schema-invalid | Raw artifact and validator result |
| SAFE-16 | Return duplicate claim IDs | Output is semantically rejected | Claim validation event |
| SAFE-17 | Cite exact, shifted, and nonexistent ranges | Results are exact, content-found-elsewhere, and unverified respectively | Citation verifier output |
| SAFE-18 | Return a fake `system` role in JSON | Role is data, not controller authority | Parsed artifact and unchanged state |
| SAFE-19 | Return an instruction to implement immediately | Master prompt excludes execution and UI has no launch action | Compiler diff and UI assertion |
| SAFE-20 | Return a request to remove minority reasoning | Required dissent remains in R3/decision/export | Position and export comparison |

## State and recovery safety

| ID | Setup and action | Expected result | Evidence |
|---|---|---|---|
| SAFE-21 | Fail one provider after another completes R1 | Debate becomes partial, not complete; retry is explicit | State transition and provider rows |
| SAFE-22 | Kill the app during packet creation | Restart reconstructs from durable state without hidden provider session | Recovery log and state hash |
| SAFE-23 | Cancel a long-running harmless provider fixture | Process tree ends; no lingering child or writable evidence remains | PID/process evidence |
| SAFE-24 | Return an unknown provider error | State is `UNKNOWN_FAILURE`/paused, not concession or consensus | Audit event and UI state |
| SAFE-25 | Repeat export from same approved state | Deterministic prompt content matches | Two output hashes and diff |

## Human-control safety

| ID | Setup and action | Expected result | Evidence |
|---|---|---|---|
| SAFE-26 | Open Decision View before human action | No approved decision or master prompt exists | UI state and persistence query |
| SAFE-27 | Select `Reject All` | Export records rejection and does not present an implementation prompt as approved | Decision record |
| SAFE-28 | Select `Continue Targeted Debate` | No final decision is compiled; scope remains visible | State transition |
| SAFE-29 | Copy master prompt | Copy is the final external handoff; no provider or process launches | Process list and clipboard evidence |
| SAFE-30 | Attempt to find a provider-send or implement control | No such action exists in the final flow | UI automation/search evidence |

## Acceptance interpretation

Any test that cannot run must be `UNVERIFIED` with a reason. A failed safety invariant blocks a verified release even if all functional tests pass.
