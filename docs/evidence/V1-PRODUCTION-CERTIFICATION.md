# V1 Production Certification Record

Date: 2026-08-16/17 local certification run
Audited starting commit: `a6636daecbe755862574c5f987b75a3866e47ff7`
Current implementation checkpoint: `bbf339df4942ad95a433f2b8e3cc978ac4ca34f6`
M1: `NOT_STARTED`

This record separates implementation, automated, installer, provider-feasibility, and live-product evidence. It does not treat a synthetic adapter, carried-forward seat record, successful compile, or stale snapshot review as current three-seat production certification.

## Verdict

```text
V1_STATUS = RELEASE_CANDIDATE
DEFINITION_OF_DONE = NOT MET: fresh current-head snapshot approval and live three-seat R1/R2/R3 evidence are absent; human decision/export and live provider cancellation/recovery remain unverified
M1 = NOT_STARTED
```

## Exact model/reasoning configuration boundary

The current source distinguishes provider boundary certification from exact configuration certification. `ModelSelection` and persisted `turns`/`raw_artifacts` retain:

- provider;
- requested model;
- requested reasoning level;
- reported served model, when available;
- serving identity status;
- exact configuration status and evidence;
- `council-provider-boundary.v1` certification boundary.

Exact configuration is fail-closed. Historical seat records did not include exact reasoning-level tuples, so current configurations are not promoted to `CERTIFIED` merely because their provider seat has a pass record. This prevents aliases such as `sonnet` or `opus`, and unrecorded Codex/Claude levels, from being presented as reproducible certified configurations. The UI exposes provider-specific model and level dropdowns and rejects unsupported combinations before provider dispatch.

Antigravity identifiers with embedded `-low`, `-medium`, or `-high` levels are treated as fixed-level model identifiers. Their command does not receive a conflicting generic `--effort` flag. Reloaded failed/partial latest-round turn statuses remain visible and are never presented as final positions.

## Automated verification

| Gate | Result | Evidence |
|---|---|---|
| Rust format | PASS | Pinned `1.96.0-x86_64-pc-windows-msvc` `cargo fmt --all -- --check` |
| Rust workspace tests | PASS | 49 library tests, 0 doctests, 0 failures |
| Rust workspace check | PASS | Pinned `cargo check --workspace` |
| Frontend dependencies | PASS | `npm ci`, 73 packages, 0 vulnerabilities |
| Frontend policy tests | PASS | 9/9 |
| Frontend build | PASS | TypeScript and Vite production build |
| Global verifier | VERIFIED | `RUSTUP_TOOLCHAIN=1.96.0-x86_64-pc-windows-msvc` |
| Current-tree public audit | PASS | 0 confirmed live-secret matches |
| Reachable-history public audit | PASS | 0 confirmed live-secret matches; 2 old identity-metadata warnings |

The verifier without the pinned environment was separately `UNRUNNABLE` for this host because the default stable toolchain's `rustdoc.exe` was not applicable. The pinned verifier is the authoritative project result.

## Provider CLI checks

Harmless installed CLI checks were used before any live provider call:

| Provider | Check | Result |
|---|---|---|
| Claude Code | `--help` | `--effort` accepts low/medium/high/xhigh/max |
| Antigravity CLI | `--help` | `--effort` accepts low/medium/high |
| Codex WSL | isolated `codex exec --strict-config --help` | model/config/schema/json/ephemeral boundary flags present; no live inference |

Rust command-construction tests verify forwarding, invalid-level rejection, environment filtering, and fixed-level Antigravity behavior. No paid live model call was performed during this closeout.

## GitHub Actions

```text
WORKFLOW = Windows verification
RUN_ID = 31983611106
COMMIT = bbf339df4942ad95a433f2b8e3cc978ac4ca34f6
CONCLUSION = SUCCESS
URL = https://github.com/dionnblake/Council-Of-Agents/actions/runs/31983611106
```

The run completed checkout, pinned Rust setup, formatting, workspace tests, workspace check, `npm ci`, frontend tests, frontend build, and the reachable-history privacy audit. CI performs no live provider calls.

## Final NSIS candidate

```text
BUILD_COMMIT = bbf339df4942ad95a433f2b8e3cc978ac4ca34f6
PATH = C:\council-target\release\bundle\nsis\Council of Agents_0.1.0_x64-setup.exe
SIZE = 3,491,307 bytes
SHA256 = D20BCBD83AFC910829A55D7743559862C933B241123B8B951FD2D6248A0B7F56
APP_SHA256 = 5B2524AFDCDF1E4932ADEDA8F273A5BF5C623AB897D801D39393FA0DD8F7B003
```

The exact artifact installed with NSIS exit `0` into `CouncilV1CertificationFinal-bbf339d`. The installed binary launched from that exact directory with title `Council of Agents`. After close, relaunch succeeded and SQLite app data remained present. The exact uninstaller exited `0` and removed the install directory while app data remained. Model/level controls are covered by the frontend policy suite and the exact source build; no provider round was run.

## Snapshot and live-provider gate

The previous persisted review record was created before the current model/configuration hardening and is stale. It cannot authorize the current head. The attempted current installed-app input could not be delivered to the WebView, so no fresh debate ID, snapshot ID, manifest hash, exclusion-set hash, or owner approval exists for this checkpoint. No provider process was launched and no approval was taken on the owner's behalf.

The following gates are therefore not complete:

1. fresh repository-grounded snapshot build and human secret-exclusion review;
2. live three-seat R1 independent analysis;
3. live R2 cross-examination and R3 final positions;
4. current packet/schema/manifest/citation/served-identity evidence;
5. human decision and deterministic export;
6. live Claude, Antigravity, and Codex WSL cancellation;
7. interrupted-dispatch recovery, restart reconstruction, and WSL termination fallback in the full product path.

## Carried-forward provider evidence

`M0.8-FINDINGS.md` and `CODEX-WSL-FINAL-CERTIFICATION.md` remain individual-seat evidence only: Claude 20/20 with verified requested/served model, Antigravity 17/20 with one repair and provider-reported identity limitation, and Codex WSL 20/20 with the documented isolation boundary and served-model limitation. They do not certify the current Tauri product flow.

## Safety and ownership

No secret values, credential material, API-key fallback, billing change, target-repository edit, autonomous coding handoff, or public release was performed. The product remains human-controlled at snapshot review, decision, and manual-copy boundaries. M1 remains unopened.
