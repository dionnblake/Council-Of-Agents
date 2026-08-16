# Evidence Index

## Current local verification

The final local regression pass used:

~~~powershell
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
~~~

The full Rust workspace suite passed: 42 library tests, zero doctests, and zero failures. The frontend policy suite passed 7/7 tests. The frontend suite is intentionally dependency-free and protects the visible human-decision, preview, recovery, degraded-mode, limitation, snapshot-review, and manual-boundary contracts; Rust remains the policy authority.

The native Tauri installer path was tested separately. `npx tauri build --bundles nsis` produced the V1 NSIS artifact; a clean install launched a native `Council of Agents` window, created SQLite app data, opened a persisted debate, recorded cancellation, survived restart, survived reinstall, and uninstalled cleanly. This proves the packaged local shell and persistence path, not authenticated provider execution.

The complete gate record, installer hash, debate identifier, carried-forward seat hashes, and exact remaining gates are in [V1-PRODUCTION-CERTIFICATION.md](V1-PRODUCTION-CERTIFICATION.md).

## Live evidence

- [M0.8-FINDINGS.md](../../M0.8-FINDINGS.md) is the native feasibility decision.
- [CODEX-WSL-FINAL-CERTIFICATION.md](../../CODEX-WSL-FINAL-CERTIFICATION.md) is the dedicated Codex WSL seat evidence.
- [CODEX-WSL-CERTIFICATION.md](../../CODEX-WSL-CERTIFICATION.md) is the preceding recovery record.
- The carried-forward records prove individual seat controls only. They do not prove a current Tauri three-seat R1/R2/R3 debate.

## Current review gate and unverified work

The corrected native candidate created debate `debate-5003e0d6-4635-46d4-a3d4-ddddab6690eb` and persisted `SNAPSHOT_REVIEW_REQUIRED` for snapshot `snapshot-debate-5003e0d6-4635-46d4-a3d4-ddddab6690eb` with manifest hash `f7ae4901c5f14641f00c7fda8f1083b08861a38bcef347088f8d14c6a9bd66af` and exclusion-set hash `f3e266280ff7a33d222af4a6032348c4c70b7c607ba859c8dd421403af628bc8`. No provider process or turn has launched. Owner approval is required before live-provider evidence can begin.

The implementation checkpoint is pushed as `01d9657f7219754e35285dcb8e64808d336aea21`; GitHub Actions run `31943606032` passed all configured Windows verification steps.

## Unverified here

- A current-host repository-grounded three-seat provider round was attempted after `SUBSCRIPTION ROUTING = SAFE`, but the sanitized snapshot identified three secret-looking exclusions and stopped for mandatory human review before provider dispatch; no API-key routing was used.
- Current Tauri R1/R2/R3 positions, live citation attachment, human decision, deterministic export, real repository snapshot/WSL transfer, live provider cancellation, WSL termination fallback, and interrupted-dispatch recovery remain unverified.
- The global verifier returned `VERIFIED` with the pinned Rust toolchain. Both privacy audits returned `PASS` with 0 confirmed live-secret matches; the history audit reported 2 non-secret identity metadata warnings for commit `189f867a6006`, without rewriting history.
