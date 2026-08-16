# V1 Production Certification Record

Date: 2026-08-16
Audited starting commit: `189f867a600633ce80be8b392959af0920b2c5d7`
Implementation and certification closeout commit: `23bffed2b39645437ac24898d6792f9fe1b5ccd6`
M1: `NOT_STARTED`

This record separates implementation evidence, automated evidence, current-host installer evidence, and live-provider evidence. It does not treat a synthetic adapter, a carried-forward seat record, or a successful compile as a completed product certification.

## Continuation checkpoint: persisted snapshot review

This continuation started from GitHub `main` at `26f51360ca601d8c9fcf5ca2f6b97fffff150902`. The secret-review dead end is now a persisted `SNAPSHOT_REVIEW_REQUIRED` state. Its review record binds the debate ID, snapshot ID, manifest SHA-256, deterministic exclusion-set hash, safe relative exclusion metadata, source fingerprint captured during snapshot creation, fixed acknowledgment, decision, and review timestamp. Approval cannot restore excluded files or expose matched values. A source change invalidates the prior review and blocks/reopens the gate for the changed sanitized snapshot.

The corrected native candidate reached the gate in a clean installed-app database:

```text
DEBATE_ID = debate-5003e0d6-4635-46d4-a3d4-ddddab6690eb
STATE = SNAPSHOT_REVIEW_REQUIRED
SNAPSHOT_ID = snapshot-debate-5003e0d6-4635-46d4-a3d4-ddddab6690eb
MANIFEST_SHA256 = f7ae4901c5f14641f00c7fda8f1083b08861a38bcef347088f8d14c6a9bd66af
EXCLUSION_SET_SHA256 = f3e266280ff7a33d222af4a6032348c4c70b7c607ba859c8dd421403af628bc8
SECRET_EXCLUSION_COUNT = 3
PROVIDER_TURNS = 0
```

The UI displayed only safe relative paths, exclusion reasons, and hashes. It did not display or persist excluded contents, matched substrings, or provider context. Owner approval is still required before any provider process can launch.

This implementation checkpoint was committed and pushed to `main` as `01d9657f7219754e35285dcb8e64808d336aea21`. GitHub Actions run `31943606032` completed successfully with pinned Rust formatting, workspace tests/check, frontend policy tests/build, and the reachable-history privacy audit passing.

## Verdict

```text
V1_STATUS = RELEASE_CANDIDATE
DEFINITION_OF_DONE = NOT MET: the current repository-grounded run stopped at the mandatory secret-review gate before provider dispatch; R1/R2/R3 positions, human decision, deterministic export, and live provider cancellation/recovery remain untested
M1 = NOT_STARTED
```

## Gate classifications

| Gate | Classification | Evidence and boundary |
|---|---|---|
| Rust controller, persistence, provider contracts, snapshot/packet controls, human gate, and manual export boundary | IMPLEMENTED | Repository source and Rust checks; Rust remains policy authority. |
| Rust library regression suite | AUTOMATED_TESTED | `cargo test --workspace`: 42/42 library tests and 0 doctests passed on the pinned toolchain. |
| Frontend policy regressions | AUTOMATED_TESTED | `npm test`: 7/7 passed. The tests protect visible UI guards; they do not replace Rust policy tests. |
| Rust formatting and workspace check | AUTOMATED_TESTED | The pinned 1.96 toolchain passed formatting and workspace checking. |
| Current-tree privacy/sanitation | AUTOMATED_TESTED | `node scripts/public-repo-audit.cjs` passed with 0 confirmed live-secret matches. |
| Reachable-history privacy audit | AUTOMATED_TESTED | `--history` passed with 0 confirmed live-secret matches and 2 non-secret identity metadata warnings for commit `189f867a6006`; history was not rewritten. |
| Individual provider-seat feasibility records | LIVE_TESTED | Carried forward from `M0.8-FINDINGS.md` and `CODEX-WSL-FINAL-CERTIFICATION.md`; not a current Tauri three-seat debate. |
| Current-host subscription routing preflight | AUTOMATED_TESTED | `SUBSCRIPTION ROUTING = SAFE`; ambient host credentials were present but not inherited. Claude, Antigravity, and Codex WSL all reported `preflight=READY`. |
| Current-host repository-grounded R1/R2/R3 provider debate | HUMAN_REVIEW_REQUIRED | Corrected native candidate created debate `debate-5003e0d6-4635-46d4-a3d4-ddddab6690eb`, sealed the real repository snapshot, persisted the exact review record, and is waiting for owner approval. No provider process launched. |
| Current-host Tauri human decision and deterministic export | NOT_TESTED | The installed-app test intentionally stopped before provider positions, decision, and export. |
| NSIS build, install, launch, SQLite creation, debate creation, cancellation, restart, reinstall, and uninstall | INSTALLER_TESTED | See the installer record below. |
| Provider process cancellation, WSL termination fallback, and interrupted-dispatch recovery | NOT_TESTED | The installed-app cancellation was a persisted debate cancellation, not a live provider-process interruption test. |
| Snapshot secret-review gate and unavailable-seat behavior | IMPLEMENTED / HUMAN_REVIEW_REQUIRED | The product fails closed, persists the review-required state across restart, exposes exact safe metadata, supports approve/reject semantics, and invalidates approval on source changes. The current repository still requires explicit owner review before live provider availability can be established. |

## Current installed-app runtime record

The clean install used a harmless question and the three configured seats. It created this persisted debate through the installed Tauri binary:

```text
DEBATE_ID = debate-795181f0-43db-42c9-97ff-0af9b14fb9f0
QUESTION = Should Council keep a local-first desktop architecture for V1?
COUNCIL_SIZE = 3
CREATED_STATE = DRAFT
FINAL_TEST_STATE = CANCELLED
DATABASE_SHA256_AFTER_CLOSE = C64E111B930815B345AE08F3DB4D5B3237582950792014E071EE4A0DC1A66CA4
```

The application showed all three seats as pending/not ready, displayed the human retry/cancel/degraded controls, and did not dispatch a provider. The cancel action changed the persisted state to `CANCELLED`. A fresh launch and a reinstall launch both found the same persisted debate in SQLite. No raw database, provider output, or credential material is retained in the repository.

The corrected headless provider preflight reported effective routing and presence/status only:

```text
SUBSCRIPTION ROUTING = SAFE
HOST CREDENTIALS = PRESENT BUT NOT INHERITED
Claude Code = model=claude-haiku-4-5-20251001, certification=Pass, preflight=READY
Antigravity CLI = model=gemini-3.7-flash-low, certification=PassWithDeclaredLimitation, preflight=READY
Codex WSL = model=gpt-5.6-luna, certification=PassWithDeclaredLimitation, preflight=READY
```

`READY` here is effective subscription-routing preflight status, not proof that a live round completed. The repository snapshot secret-review gate stopped the native attempt before a provider process was launched.

The native attempt used the real repository path through the installed Tauri application. The snapshot builder recorded exclusions and sealed the sanitized destination, then the controller failed closed on the required human review. No API-key fallback, custom base URL, billing change, credential change, or provider handoff occurred. The temporary live-test app data was retained outside the repository and the user's original app data was restored.

For this current installed-app record, the controlled evidence hashes are explicitly absent because no provider round ran:

| Evidence item | Current record |
|---|---|
| R1/R2/R3 packet hashes | `NOT_CREATED` |
| Response schema hash | `NOT_CREATED` |
| Repository snapshot/manifest hash | `NOT_CREATED` |
| Final positions and citation hashes | `NOT_CREATED` |
| Human decision record hash | `NOT_CREATED` |
| Deterministic export hashes | `NOT_CREATED` |

## Carried-forward individual seat evidence

These records are durable evidence for individual seat boundaries only. They do not prove the current desktop application's complete R1/R2/R3 workflow.

| Provider | Requested model | Served-model status | Result |
|---|---|---|---|
| Claude Code | `claude-haiku-4-5-20251001` | Verified match in the carried-forward record | 20/20 schema, packet pass, stateless pass |
| Antigravity CLI | `gemini-3.7-flash-low` | Provider does not report served identity | 17/20 schema, packet/stateless pass, one repair |
| Codex WSL | `gpt-5.6-luna` | Provider does not report served identity | 20/20 schema, isolation/auth/sandbox/packet/stateless pass |

The Codex WSL record contains these preserved hashes:

```text
ROOTFS_SHA256 = 2a790896740b14d637dbdc583cce1ba081ac53b9e9cdb46dc09a2f73abbd9934
SNAPSHOT_MANIFEST_SHA256 = 8ced118af4fa5561ace9544fa0b5bf9c909aa79cef7687dabea9e323ce8b4e2f
PACKET_50KB_SHA256 = cbc834ede2510aea49e4effc3575e8644260f005d5c661170d65017d0e923570
PACKET_200KB_SHA256 = 297b7ccaee809fec600185717b22faf6065f605c0c005073e1087f5b0af59513
PACKET_500KB_SHA256 = 5199030b1c75d45719f144a793b3ef408f0556feb5af6a7843792c6a542ca2ab
SCHEMA_SHA256 = be7cc9a7c2becd924bfa97b1dc5d33e0b68df4dcd6ec43bbd093b33866be7295
TURN1_PACKET_SHA256 = 4af01d5ee6ab53a3918cf9a60cec03b18f460e5bdd2b30efcc0be516ee9f8e36
TURN2_PACKET_SHA256 = e18c92ac6bc0ab1e7f6ea655ce93765a75a82a0b0325f858f6c6785acd67058d
TURN1_OUTPUT_SHA256 = 9d0a700f1a540082f25b8a338f0f561c90d15cd3a015edb982b8fa10e5bc935a
```

The repository's synthetic demo packet hash `1393345ab992ebfd9a7f7740cba24c552c761f1d1252e44c5435ab76460fa19d` appears only in demo artifacts and is not live certification evidence.

## Installer evidence

Command:

```powershell
Push-Location app
npx tauri build --bundles nsis
Pop-Location
```

Generated artifact:

```text
PATH = target/release/bundle/nsis/Council of Agents_0.1.0_x64-setup.exe
SIZE = 3,465,281 bytes
SHA256 = 60CB7FCBBB228DF25502513436CB1F259DC96B2618DE29CA45224C522A30046D
```

The final rebuilt installer was run silently into an isolated temporary directory. It exited `0`, produced `council-desktop.exe` and `uninstall.exe`, launched a native window titled `Council of Agents`, and its uninstaller exited `0` and removed the install directory. The clean app-data install, persisted debate flow, cancellation, restart, and reinstall evidence was exercised against the earlier same-source NSIS build (`5843b1b542e3ca2a60a79036819619f13c451567b845ea40f2dd96d243153782`, 3,434,551 bytes); the final rebuilt artifact was separately install/launch/uninstall verified.

The user's pre-existing application data was moved to a recoverable temporary backup during the clean test and restored afterward. The clean test database remains outside the repository under a temporary evidence directory for recovery and is not a source artifact.

## Cancellation and recovery

| Scenario | Classification | Result |
|---|---|---|
| Installed-app cancel action | INSTALLER_TESTED | Persisted debate changed from `DRAFT` to `CANCELLED`; user-facing notice stated no provider handoff or implementation action occurred. |
| Installed-app restart persistence | INSTALLER_TESTED | Fresh launch reopened the local command center and the same debate row remained in SQLite. |
| Installed-app reinstall persistence | INSTALLER_TESTED | Fresh install launched and found the persisted debate in the restored app-data test state. |
| Live provider process cancellation | NOT_TESTED | No safe live provider dispatch occurred. |
| WSL termination fallback/interrupted dispatch | NOT_TESTED | Existing individual Codex evidence covers a harmless cancellation fixture, not this full product gate. |

## Environment and commands

The certification work used Windows, Rust toolchain `1.96.0-x86_64-pc-windows-msvc`, Tauri CLI `2.5.0`, Node.js `24.16.0`, the repository's checked-in `app/package-lock.json`, and the existing `CouncilCodexWSL` evidence record. Commands used included:

```powershell
rustup run 1.96.0-x86_64-pc-windows-msvc cargo fmt --all -- --check
rustup run 1.96.0-x86_64-pc-windows-msvc cargo test --workspace
rustup run 1.96.0-x86_64-pc-windows-msvc cargo check --workspace
Push-Location app
npm ci
npm test
npm run build
Pop-Location
node scripts/public-repo-audit.cjs
node scripts/public-repo-audit.cjs --history
```

No live provider call, credential change, billing change, public release, or autonomous coding handoff was performed.

## Remaining gates before V1 production certification

1. Complete human review and clearance of the current secret-looking snapshot exclusion without exposing its contents.
2. Run a current desktop Tauri debate against the real sanitized repository path through R1, R2, and R3.
3. Verify live packet, schema, snapshot, citation, response, and served-model evidence for every seat.
4. Exercise the human decision and deterministic export path from those live positions.
5. Exercise live provider cancellation, WSL fallback, restart persistence, and interrupted-dispatch recovery.

Until those gates have durable evidence, V1 remains a release candidate and M1 remains unopened.
