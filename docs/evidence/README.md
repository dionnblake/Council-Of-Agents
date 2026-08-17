# Evidence Index

## Current local verification

The current pinned regression pass used:

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

Results: 49 Rust library tests passed, zero doctests failed, 9/9 frontend policy tests passed, the frontend build passed, and both public-repository audits found zero confirmed live-secret matches. The pinned global verifier returned `VERIFIED`. The unpinned verifier was not applicable on this host because its default stable `rustdoc.exe` was unusable.

The exact NSIS candidate was installed, launched, relaunched, and uninstalled successfully. The install directory was removed while local SQLite app data remained. This proves the packaged local shell and persistence path, not authenticated provider execution.

The complete gate record, installer hash, CI run, exact model/level semantics, and remaining blockers are in [V1-PRODUCTION-CERTIFICATION.md](V1-PRODUCTION-CERTIFICATION.md).

## Provider feasibility evidence

- [M0.8-FINDINGS.md](../../M0.8-FINDINGS.md) records the individual Claude and Antigravity feasibility results and the closed native-Codex route.
- [CODEX-WSL-FINAL-CERTIFICATION.md](../../CODEX-WSL-FINAL-CERTIFICATION.md) records the dedicated Codex WSL seat boundary.
- [CODEX-WSL-CERTIFICATION.md](../../CODEX-WSL-CERTIFICATION.md) is the preceding WSL recovery record.

These records do not prove a current Tauri three-seat R1/R2/R3 debate.

## Current unverified work

No fresh current-head snapshot review exists. The earlier persisted snapshot-review metadata is stale after the model/configuration hardening commits and cannot authorize this source. Current Tauri R1/R2/R3 positions, live citations, human decision, deterministic export, live provider cancellation, and interrupted-dispatch recovery remain unverified. No provider process was launched during this closeout.
