# Council of Agents V1 Final Report

Date: 2026-08-16/17 local certification run
Audited starting commit: `a6636daecbe755862574c5f987b75a3866e47ff7`
Current implementation checkpoint: `bbf339df4942ad95a433f2b8e3cc978ac4ca34f6`

## Executive verdict

```text
V1_STATUS = RELEASE_CANDIDATE
DEFINITION_OF_DONE = NOT MET: no fresh current-head snapshot approval or live three-seat R1/R2/R3 evidence; human decision/export and live provider cancellation/recovery remain unverified
M1 = NOT_STARTED
```

The implementation and local release controls are in place. The exact requested provider, model, reasoning level, served identity, and certification boundary now persist through debates, turns, raw artifacts, and reloads. Unsupported exact configurations remain `UNVERIFIED_CONFIGURATION`; no provider-level pass is widened into a model-level claim.

The live production gate remains closed. The earlier snapshot-review record is stale after the current source changed and is not reused. No provider process, human decision, export, or autonomous implementation handoff was claimed in this pass.

## 1. Scope and boundary

This pass remained V1 certification work only. M1 was not started. Council still stops at `MANUAL COPY -> STOP`; it does not edit a target repository, create branches, commit, push, deploy, or launch a coding harness.

## 2. Exact model and reasoning certification

- Provider boundary certification and exact configuration certification are separate visible fields.
- Each debate and turn persists provider, requested model, requested reasoning level, reported served model, serving identity status, exact configuration status/evidence, and boundary version.
- Exact configuration certification is fail-closed because the historical records did not record exact reasoning-level tuples. Current defaults and newly selected combinations therefore display `UNVERIFIED_CONFIGURATION` until a matching evidence record exists.
- Claude exposes low/medium/high/xhigh/max; Antigravity exposes low/medium/high; Codex WSL exposes low/medium/high/xhigh/max, with `ultra` limited to the configured Sol/Terra model choices.
- Antigravity model identifiers that embed `-low`, `-medium`, or `-high` are paired with that level and do not receive a conflicting `--effort` flag.
- Reloaded failed or partial rounds remain visibly partial and cannot become a decision surface.

## 3. Provider CLI verification

Installed harmless help/config checks confirmed Claude accepts `--effort` values low, medium, high, xhigh, and max; Antigravity accepts low, medium, and high; and the isolated Codex WSL CLI exposes the configured model/config/schema/ephemeral boundary. Invalid or unsupported model-level combinations fail before dispatch in Rust tests. No paid live model call was made during this closeout.

## 4. Automated verification

| Check | Result |
|---|---|
| Rust format | PASS, pinned `1.96.0-x86_64-pc-windows-msvc` |
| Rust workspace tests | PASS, 49 library tests, 0 doctests, 0 failures |
| Rust workspace check | PASS |
| `npm ci` | PASS, 73 packages audited, 0 vulnerabilities |
| Frontend policy tests | PASS, 9/9 |
| Frontend production build | PASS |
| Global verifier | VERIFIED with `RUSTUP_TOOLCHAIN=1.96.0-x86_64-pc-windows-msvc` |
| Current-tree privacy audit | PASS, 0 confirmed live-secret matches |
| Reachable-history privacy audit | PASS, 0 confirmed live-secret matches; 2 old identity-metadata warnings |

The unpinned global verifier invocation was separately `FAILED` because the system stable toolchain could not run its `rustdoc.exe`; this was an environment/toolchain selection failure. The required pinned invocation returned `VERIFIED`.

## 5. GitHub Actions

Windows workflow run `31983611106` passed on commit `bbf339df4942ad95a433f2b8e3cc978ac4ca34f6`. Checkout, pinned Rust, format, workspace tests/check, `npm ci`, frontend tests/build, and the reachable-history privacy audit all passed. No live provider calls run in CI.

## 6. Final NSIS artifact

```text
BUILD_COMMIT = bbf339df4942ad95a433f2b8e3cc978ac4ca34f6
PATH = C:\council-target\release\bundle\nsis\Council of Agents_0.1.0_x64-setup.exe
SIZE = 3,491,307 bytes
SHA256 = D20BCBD83AFC910829A55D7743559862C933B241123B8B951FD2D6248A0B7F56
APP_SHA256 = 5B2524AFDCDF1E4932ADEDA8F273A5BF5C623AB897D801D39393FA0DD8F7B003
```

## 7. Installer lifecycle

The exact candidate installed silently with exit `0` into `CouncilV1CertificationFinal-bbf339d`, containing `council-desktop.exe` and `uninstall.exe`. The exact installed executable launched with the expected `Council of Agents` title and path. Close/relaunch preserved SQLite app data. The exact uninstaller exited `0` and removed the install directory while retaining app data. This is packaged-shell and local-persistence evidence, not provider execution evidence.

## 8. Live certification status

No fresh current-head three-seat debate reached the usable snapshot-review record in this run. The last recorded review metadata from an earlier source checkpoint is stale after the model/configuration hardening commits and is not an approval for the current source. The UI automation attempt could not deliver input to the installed WebView, so no human approval was taken on the owner's behalf.

## 9. Unverified gates

The following remain unverified or blocked:

- fresh current-head snapshot build and human review;
- live Claude, Antigravity, and Codex WSL R1 independent analysis;
- R2 cross-examination and R3 final positions;
- live repository packet, schema, manifest, citation, exclusion, and served-identity evidence;
- human decision and deterministic export;
- live provider cancellation for all three seats;
- interrupted dispatch, restart reconstruction, and WSL termination fallback in the full product path.

## 10. Closeout

The code fixes, automated evidence, exact NSIS lifecycle, CI result, and conservative release status are pushed to GitHub `main`. V1 remains a release candidate until a human operates the fresh current-head snapshot gate and the remaining live evidence exists. No M1 work was started.
