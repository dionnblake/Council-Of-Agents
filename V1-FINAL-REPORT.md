# Council of Agents V1 Final Report

Date: 2026-08-16
Audited starting commit: `189f867a600633ce80be8b392959af0920b2c5d7`

## Executive verdict

```text
V1_STATUS = RELEASE_CANDIDATE
DEFINITION_OF_DONE = NOT MET: the current repository-grounded run stopped at the mandatory secret-review gate before provider dispatch; R1/R2/R3 positions, human decision, deterministic export, and live provider cancellation/recovery remain untested
M1 = NOT_STARTED
```

The implementation is present and the local controller, frontend policy, privacy, build, and NSIS installer gates are covered. The effective subscription-routing preflight is safe, but the current repository-grounded native run stopped at the mandatory snapshot secret-review gate before authenticated dispatch. No API-key fallback, billing change, credential change, or autonomous coding handoff was used.

The complete gate record is [docs/evidence/V1-PRODUCTION-CERTIFICATION.md](docs/evidence/V1-PRODUCTION-CERTIFICATION.md).

## Continuation checkpoint: persisted snapshot review

This continuation started from GitHub `main` at `26f51360ca601d8c9fcf5ca2f6b97fffff150902`. The dead-end secret-review behavior is now replaced by a persisted `SNAPSHOT_REVIEW_REQUIRED` state and review record. The record binds the debate ID, snapshot ID, manifest SHA-256, deterministic exclusion-set hash, safe relative exclusion metadata, source-tree fingerprint captured at snapshot build time, decision, fixed acknowledgment, and review timestamp. Approval is immutable, does not restore excluded files, and is invalidated when the relevant repository fingerprint changes. The IPC and React surface never returns excluded contents or matched values.

Automated coverage after the change is 42/42 Rust workspace tests and 7/7 frontend policy tests. The corrected native NSIS candidate is installed in the isolated test directory and the clean repository-grounded debate below is waiting for owner approval at the review gate. No provider turn or provider process has launched.

```text
DEBATE_ID = debate-5003e0d6-4635-46d4-a3d4-ddddab6690eb
STATE = SNAPSHOT_REVIEW_REQUIRED
SNAPSHOT_ID = snapshot-debate-5003e0d6-4635-46d4-a3d4-ddddab6690eb
MANIFEST_SHA256 = f7ae4901c5f14641f00c7fda8f1083b08861a38bcef347088f8d14c6a9bd66af
EXCLUSION_SET_SHA256 = f3e266280ff7a33d222af4a6032348c4c70b7c607ba859c8dd421403af628bc8
SECRET_EXCLUSION_COUNT = 3
PROVIDER_TURNS = 0
```

The three secret-scanner paths shown to the owner are relative paths only: `M0-RAW-RESULTS.md`, `crates/council-core/src/persistence.rs`, and `crates/council-core/src/snapshot.rs`. Their contents were not displayed or persisted in the review record. Provider dispatch remains paused pending that explicit human action.

## 1. Current commit and scope

The certification audit started from `189f867a600633ce80be8b392959af0920b2c5d7` on `main`. This pass finished V1 certification work only. It did not start M1, redesign the architecture, or add product features.

## 2. Live-provider evidence

Individual seat evidence is carried forward from [M0.8-FINDINGS.md](M0.8-FINDINGS.md) and [CODEX-WSL-FINAL-CERTIFICATION.md](CODEX-WSL-FINAL-CERTIFICATION.md): Claude 20/20 with verified model identity; Antigravity 17/20 with one repair and no served-model report; Codex WSL 20/20 with isolation/auth/sandbox/packet/stateless passes and no served-model report.

Those records do not prove a current Tauri debate. The corrected current-host preflight reported `SUBSCRIPTION ROUTING = SAFE`, with ambient credentials not inherited by provider commands. A native Tauri repository-grounded attempt created debate `debate-3794d2e0-6f0e-4390-9ee2-fc9503871826`, built the sanitized snapshot, then stopped at the mandatory secret-review gate before any provider position was dispatched.

The headless preflight reported `SUBSCRIPTION ROUTING = SAFE` and `HOST CREDENTIALS = PRESENT BUT NOT INHERITED`; the three provider contracts reported `preflight=READY` with their requested model IDs. That READY status is not live execution evidence, and no provider process was launched after the snapshot review gate.

## 3. Repository and policy evidence

The Rust controller remains the policy authority for state transitions, packet/snapshot boundaries, provider process contracts, failure handling, recovery, the human decision gate, deterministic export, and the no-autonomous-implementation boundary. The React layer surfaces those states and now has focused policy regression tests.

The current installed-app record is:

```text
DEBATE_ID = debate-795181f0-43db-42c9-97ff-0af9b14fb9f0
QUESTION = Should Council keep a local-first desktop architecture for V1?
CREATED_STATE = DRAFT
FINAL_TEST_STATE = CANCELLED
DATABASE_SHA256_AFTER_CLOSE = C64E111B930815B345AE08F3DB4D5B3237582950792014E071EE4A0DC1A66CA4
```

No current-app R1/R2/R3 packet, response schema, repository snapshot, citation, decision, or export hash exists because the provider gate stopped dispatch before those artifacts were created. The carried-forward Codex WSL hashes remain preserved in the certification record and are explicitly labeled individual-seat evidence.

## 4. Cancellation and recovery

- The installed Tauri app created a real SQLite debate and exposed recovery controls when seats were unavailable.
- The installed-app cancel action persisted the debate as `CANCELLED` and showed the no-handoff/no-implementation notice.
- A fresh launch found the same persisted debate.
- A reinstall launch also found the same persisted debate.
- Live provider-process cancellation, WSL termination fallback, and interrupted-dispatch recovery remain untested in this current product certification.

## 5. Automated tests and local verification

| Check | Result | Command |
|---|---|---|
| Rust formatting | PASS | `rustup run 1.96.0-x86_64-pc-windows-msvc cargo fmt --all -- --check` |
| Rust workspace tests | PASS, 42 library tests and 0 doctests | `rustup run 1.96.0-x86_64-pc-windows-msvc cargo test --workspace` |
| Rust workspace check | PASS | `rustup run 1.96.0-x86_64-pc-windows-msvc cargo check --workspace` |
| Frontend policy regressions | PASS, 7/7 | `Push-Location app; npm test` |
| Frontend production build | PASS | `Push-Location app; npm run build` |
| Package-lock install | PASS | `Push-Location app; npm ci` |
| Current-tree public repository privacy/sanitation | PASS, 0 confirmed live-secret matches | `node scripts/public-repo-audit.cjs` |
| Reachable-history privacy audit | PASS WITH WARNING | `--history` passes with 0 confirmed live-secret matches and 2 non-secret identity metadata warnings for commit `189f867a6006`; history was not rewritten. |
| Global project verifier | VERIFIED | Pinned Rust toolchain; Evidence block reports `VERIFIED`. |

The frontend tests cover human decision rationale, preview non-persistence, failed/incomplete gating, explicit degraded action, recovery visibility, export-before-decision blocking, requested/served limitations, and no autonomous coding handoff. They are source-level contract tests, not a replacement for Rust policy tests or desktop UI smoke testing.

## 6. CI

`.github/workflows/windows.yml` runs on Windows for pushes, pull requests, and manual dispatch. It runs Rust formatting, full workspace tests, workspace checking, `npm ci`, frontend policy tests, the frontend build, and the reachable-history privacy audit. It contains no live-provider step. Ordinary Git identity metadata is reported as a warning; secret, private-path, credential, and token findings remain fatal.

## 7. NSIS build and artifact hash

Command:

```powershell
Push-Location app
npx tauri build --bundles nsis
Pop-Location
```

Generated installer:

```text
target/release/bundle/nsis/Council of Agents_0.1.0_x64-setup.exe
SIZE = 3,465,281 bytes
SHA256 = 60CB7FCBBB228DF25502513436CB1F259DC96B2618DE29CA45224C522A30046D
```

## 8. Install, launch, persistence, reinstall, and uninstall results

The final rebuilt installer exited `0` into an isolated temporary install directory and produced `council-desktop.exe` and `uninstall.exe`. It launched a native `Council of Agents` window and was removed cleanly by its uninstaller. The clean app-data install, SQLite debate creation, provider limitation surface, cancellation, restart, and reinstall evidence was exercised against the earlier same-source NSIS build (`5843b1b542e3ca2a60a79036819619f13c451567b845ea40f2dd96d243153782`, 3,434,551 bytes); the final rebuilt artifact was separately install/launch/uninstall verified.

The first silent uninstall exited `0` and removed its install directory. The same NSIS artifact was installed into a second isolated directory, launched successfully with the persisted canceled debate present, and its silent uninstall exited `0` and removed that directory. Existing user app data was backed up recoverably during the clean test and restored afterward. Temporary clean-test data remains outside the repository and contains no provider credential material.

This is installer and local persistence certification. It is not live-provider certification.

## 9. Gate classifications

| Classification | V1 result |
|---|---|
| IMPLEMENTED | Controller, persistence, snapshot/packet boundary, provider contracts, Tauri bridge, UI states, human gate, deterministic local export, no-handoff boundary |
| AUTOMATED_TESTED | Rust library 42/42, frontend 7/7, formatting, workspace check, frontend build, privacy/sanitation audit |
| LIVE_TESTED | Individual provider-seat records only, carried forward with declared limitations |
| INSTALLER_TESTED | NSIS build, clean install/launch/app data/debate/cancel/restart/reinstall/uninstall |
| KNOWN_LIMITATION | Snapshot secret-review gate requires explicit owner approval before the current repository can be dispatched; the corrected UI now persists and restores that gate |
| SAFETY_BLOCKED | Current native repository-grounded Tauri attempt stopped before provider launch |
| NOT_TESTED | Current Tauri R1/R2/R3 positions/citations, human decision/export from live positions, live provider cancellation/recovery, interrupted dispatch, WSL fallback in this product run |

## 10. Limitations and remaining gates

Before changing the verdict to `PRODUCTION_CERTIFIED`, the project needs durable current-host evidence for:

1. owner human review and clearance of the current secret-looking snapshot exclusion without exposing its contents;
2. a real sanitized repository-grounded Tauri R1/R2/R3 run for all selected seats;
3. packet, schema, snapshot, citation, response, and served-model evidence;
4. the human decision and deterministic export path from those live positions;
5. live provider cancellation, WSL termination fallback, restart persistence, and interrupted-dispatch recovery.

## 11. Files changed in this certification pass

- `app/package.json`
- `app/tests/AGENTS.md`
- `app/tests/policy-regression.test.mjs`
- `.github/workflows/windows.yml`
- `docs/evidence/V1-PRODUCTION-CERTIFICATION.md`
- `docs/evidence/README.md`
- `docs/security/BOUNDARY-CONTRACT.md`
- `docs/architecture/V1-IMPLEMENTATION-PLAN.md`
- `README.md`
- `app/AGENTS.md`
- `docs/evidence/AGENTS.md`
- provider command routing and environment boundary implementation/tests
- public repository audit severity policy
- this report

## 12. Exact commit and closeout

The exact audited starting commit for the earlier report is `189f867a600633ce80be8b392959af0920b2c5d7`. This continuation started from `26f51360ca601d8c9fcf5ca2f6b97fffff150902`; the continuation implementation commit is pending the required owner review and final certification gates. The GitHub target is `github.com/dionnblake/Council-Of-Agents`, branch `main`.

No M1 work was started.
