# Council UI State Catalog

## Purpose

This catalog defines the states the desktop application must render. A state is not just a spinner label. It has an entry condition, visible facts, available actions, forbidden assumptions, and a recovery or exit path.

## State rules

- Never render a missing result as success, concession, abstention, or consensus.
- Keep the question and debate identity visible in every debate state.
- Preserve completed artifacts when another seat fails.
- Unknown and unverified states are visible and actionable.
- Loading states show what is running and what the user can safely do.
- A human decision state is distinct from agent recommendation and provider availability.

## Application lifecycle states

| State | Entry | Show | Actions | Exit |
|---|---|---|---|---|
| `APP_FIRST_RUN` | No local setup record | Product boundary, provider setup, privacy/safety summary | Start setup, read boundary, exit | `ONBOARDING_PROVIDER_SETUP` or Home |
| `APP_READY` | Local state and safe defaults load | Recent debates, provider health, new debate | New, resume, settings, recheck | Home action |
| `APP_LOADING` | Shell starts or state reloads | Progress stage and last known local state | Cancel only if safe | Ready or `APP_RECOVERY_REQUIRED` |
| `APP_RECOVERY_REQUIRED` | Persistence or migration cannot load | Exact local error and recovery options | Retry, backup/export, open diagnostics, quit | Ready or blocked |
| `APP_SHUTTING_DOWN` | User closes or update begins | Save/cancel status | Wait, cancel close if supported | Closed |

## Intake and debate lifecycle states

| State | Entry | Show | Actions | Forbidden assumption |
|---|---|---|---|---|
| `DEBATE_CREATED` | Valid question saved | Question, mode, product/decision context, draft status | Edit, save, preflight | No provider is ready yet |
| `PREFLIGHT_RUNNING` | User starts checks | Per-seat progress, requested models, elapsed time | Cancel checks, inspect details | Slow seat is not failed until deadline |
| `PREFLIGHT_READY` | Required checks pass | Installed/authenticated/certified/available, limitations | Start R1, change seats/models | Served identity is not implied |
| `PREFLIGHT_PARTIAL` | Optional seat unavailable | Available seats, missing seat, degraded eligibility | Repair, remove, continue explicitly | Two seats are not three-seat consensus |
| `PREFLIGHT_BLOCKED` | Required safety/certification failure | Blocking reason and evidence | Repair, recheck, cancel | No dispatch may start |
| `SNAPSHOT_SCANNING` | Repository debate prepares evidence | Candidate count, scan stages, progress | Cancel | Snapshot is not ready evidence |
| `SECRET_FOUND` | Scanner finds synthetic/real-looking secret | File/path, detection class, redacted marker, blocking reason | Exclude, cancel, inspect safely | `.gitignore` alone is not proof |
| `SNAPSHOT_READY` | Snapshot sealed and verified | Snapshot ID/hash, exclusions, read-only status | Review, start preflight/R1 | Live repository was not passed |
| `SNAPSHOT_FAILED` | Scan/copy/hash/reparse failure | Exact stage and safe recovery | Retry, choose another path, cancel | Partial snapshot is not usable |
| `R1_READY` | Inputs and seats ready | Packet summary, independence guarantee | Run R1 | No peer positions exist yet |
| `R1_RUNNING` | R1 dispatch begins | Provider rows, elapsed time, packet hash, cancel | Cancel round, inspect safe diagnostics | Empty output is not completion |
| `CLAUDE_COMPLETE` | Claude R1 validates | Valid position summary, claims, evidence, time | Open position/evidence | Other seats are not complete |
| `CODEX_RUNNING` | Codex WSL process active | Distro boundary, transfer state, elapsed time | Cancel Codex, inspect transfer | Windows access is not available |
| `ANTIGRAVITY_FAILED` | Antigravity attempt fails | Failure class, raw artifact status, repair path | Repair, retry, remove, continue degraded | Failure is not abstention |
| `R1_PARTIAL` | One or more seats incomplete | Completed positions and failed rows | Retry failed seat, continue degraded, cancel | R2 cannot assume independence is complete |
| `R1_COMPLETE` | All selected seats valid | Claims, positions, evidence quality, provider status | Run R2, inspect, save | Agreement is not approval |
| `R2_READY` | R1 positions frozen | Peer claim selection and packet summary | Run R2, adjust bounded scope, cancel | Peer text has not influenced R1 |
| `R2_RUNNING` | Cross-examination dispatch begins | Claim response progress and response types | Cancel, inspect | Missing response is not concession |
| `R2_PARTIAL` | Some responses valid, some fail | Per-claim response status and provider failures | Retry allowed failure, continue to R3 only if policy allows | Failure is not `NO_BASIS_TO_JUDGE` |
| `R2_COMPLETE` | Required peer responses validated | Concede/dispute/no-basis matrix, explanations | Run R3, inspect evidence | Revision has not happened yet |
| `R3_RUNNING` | Final position dispatch begins | Revision progress, surviving/withdrawn claims | Cancel | Unchanged position still needs explanation |
| `R3_COMPLETE` | Final positions validated | Final recommendations, dissent, flip conditions | Open Decision View | Human has not decided |
| `DECISION_REQUIRED` | R3 complete and no decision | Final positions, highest-impact issue, risks, evidence quality | Approve, modify, continue, challenge, reject | Consensus is not human approval |
| `DECISION_RECORDED` | Human action persisted | Action, rationale, approved constraints, limitations | View record, compile prompt | Approval does not launch implementation |
| `TARGETED_DEBATE_REQUIRED` | Human requests more evidence | Target claim, question, bounded next round | Run targeted round, edit scope, cancel | Existing decision is not final |
| `DEGRADED_COUNCIL` | Human accepts fewer seats | Missing seats, reason, limitations, available positions | Continue permitted round, repair, cancel | Result is not full-seat consensus |
| `REJECTED` | Human rejects all options | Rejection rationale and unresolved requirements | Export record, start new debate | No approved master prompt |

## Evidence and claim states

| State | Meaning | UI treatment |
|---|---|---|
| `CLAIM_UNREVIEWED` | Valid claim awaiting evidence review | Neutral pending marker |
| `VERIFIED_EXACT` | File and range match the snapshot | Verified word, icon, source/range |
| `VERIFIED_CONTENT_FOUND_ELSEWHERE` | Content exists but cited range is wrong/shifted | Warning word, corrected location, claim weakened |
| `UNVERIFIED` | Missing file, invalid range, absent repository, or failed verifier | Explicit unverified label and reason |
| `CLAIM_CONCEDED` | Peer accepted claim with explanation | Attribution and response link |
| `CLAIM_DISPUTED` | Peer challenged claim with reason | Dispute marker and evidence links |
| `CLAIM_NO_BASIS` | Peer cannot judge and says what is missing | Abstention explanation, not blank space |
| `CLAIM_WITHDRAWN` | Seat removed or materially revised claim | Original retained with revision link |

## Provider states

| State | Meaning | User action |
|---|---|---|
| `READY` | Installed, authenticated, certified, available | Run permitted |
| `AUTH_REQUIRED` | Authentication missing/expired | Repair, recheck |
| `QUOTA_LIMIT` | Provider usage/credit limit blocks call | Wait, repair, remove; no silent fallback |
| `OFFLINE` | Binary, distro, or service unavailable | Recheck/restart/repair |
| `TIMEOUT` | Deadline exceeded and process handled | Retry same packet or continue explicitly |
| `CERTIFICATION_WARNING` | Certified with declared limitation | Review limitation; human acceptance may be required |
| `MODEL_MISMATCH` | Served model explicitly differs from request | Review, change request, or accept limitation |
| `PROVIDER_DOES_NOT_REPORT` | Served identity unavailable | Show requested-only status |
| `MALFORMED_OUTPUT` | Output cannot pass structural/semantic validation | Quarantine and bounded repair |
| `REPAIR_FAILED` | Repair output also invalid | Stop retry loop; human action |
| `UNKNOWN_FAILURE` | Unclassified process/provider failure | Pause; preserve evidence; no consensus inference |
| `CANCELLED` | User or controller stopped attempt | Show cancellation separately from failure |

## Export states

| State | Entry | Show | Actions |
|---|---|---|---|
| `EXPORT_NOT_READY` | No approved human decision | Missing prerequisite | Return to Decision View |
| `EXPORT_COMPILING` | Approved state is being compiled | Source debate ID, deterministic stage | Cancel if safe |
| `EXPORT_READY` | Prompt and decision record validated | Hash, sections, traceability, stop boundary | Copy, save, export |
| `EXPORT_FAILED` | Compilation or filesystem write fails | Exact failure and no-final-artifact warning | Retry, choose folder, save later |
| `COPIED_MANUALLY` | User copies prompt | Copy confirmation and artifact hash | Return, save, export |

## Crash, restart, and recovery states

- `RESTART_RECONSTRUCTING`: durable state is loaded; hidden provider session resume is not assumed.
- `STALE_PROVIDER_PROCESS`: process identity remains after cancellation; block new dispatch until cleanup is verified.
- `HASH_MISMATCH`: snapshot, packet, or export bytes differ; quarantine and pause.
- `AUDIT_WRITE_FAILED`: do not claim the event was recorded; pause the affected action.
- `RECOVERY_UNVERIFIED`: some required state could not be reconstructed; user must choose retry, export available evidence, or abandon.
