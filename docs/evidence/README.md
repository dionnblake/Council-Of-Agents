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

The full Rust workspace suite passed: 32 library tests, zero doctests, and zero failures. The frontend policy suite passed 6/6 tests. The frontend suite is intentionally dependency-free and protects the visible human-decision, preview, recovery, degraded-mode, limitation, and manual-boundary contracts; Rust remains the policy authority.

The native Tauri installer path was tested separately. `npx tauri build --bundles nsis` produced the V1 NSIS artifact; a clean install launched a native `Council of Agents` window, created SQLite app data, opened a persisted debate, recorded cancellation, survived restart, survived reinstall, and uninstalled cleanly. This proves the packaged local shell and persistence path, not authenticated provider execution.

The complete gate record, installer hash, debate identifier, carried-forward seat hashes, and exact remaining gates are in [V1-PRODUCTION-CERTIFICATION.md](V1-PRODUCTION-CERTIFICATION.md).

## Live evidence

- [M0.8-FINDINGS.md](../../M0.8-FINDINGS.md) is the native feasibility decision.
- [CODEX-WSL-FINAL-CERTIFICATION.md](../../CODEX-WSL-FINAL-CERTIFICATION.md) is the dedicated Codex WSL seat evidence.
- [CODEX-WSL-CERTIFICATION.md](../../CODEX-WSL-CERTIFICATION.md) is the preceding recovery record.
- The carried-forward records prove individual seat controls only. They do not prove a current Tauri three-seat R1/R2/R3 debate.

## Unverified here

- A current-host authenticated three-seat provider round was not run because the safe billing/routing guard blocked dispatch; no API-key routing was used.
- Current Tauri R1/R2/R3 positions, live citation attachment, human decision, deterministic export, real repository snapshot/WSL transfer, live provider cancellation, WSL termination fallback, and interrupted-dispatch recovery remain unverified.
- The global verifier returned `VERIFIED` with the pinned Rust toolchain. The current-tree privacy audit returned `PASS` with 0 confirmed live-secret matches. The reachable-history audit remains `FAIL` only because commit `189f867a6006` carries a pre-existing author/committer email; history was not rewritten in this pass.
