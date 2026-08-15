# Evidence Index

## Current local verification

The current implementation has been checked with:

~~~powershell
$env:CARGO_TARGET_DIR = 'C:\council-target'
rustup run 1.96.0-x86_64-pc-windows-msvc cargo fmt --all
rustup run 1.96.0-x86_64-pc-windows-msvc cargo test --workspace
rustup run 1.96.0-x86_64-pc-windows-msvc cargo check -p council-desktop
Push-Location app
npm run build
Pop-Location
rustup run 1.96.0-x86_64-pc-windows-msvc cargo run -p council-cli -- providers
rustup run 1.96.0-x86_64-pc-windows-msvc cargo run -p council-cli -- demo --output .\artifacts\verification-demo
~~~

The Rust workspace tests cover the deterministic core, fake provider execution, state transitions, packet sizes, evidence controls, snapshot exclusions, WSL command planning, persistence/audit behavior, and stateless reconstruction.

The synthetic demo proves local SQLite and packet creation. It does not prove authenticated provider availability.

## Live evidence

- M0.8-FINDINGS.md is the final native feasibility decision.
- CODEX-WSL-FINAL-CERTIFICATION.md is the dedicated Codex WSL seat evidence.
- CODEX-WSL-CERTIFICATION.md is the preceding recovery record.

## Unverified here

- A live three-seat provider round was not run during V1 build verification because the current process environment reported a blocked billing variable and the implementation must not use API-key routing.
- Tauri desktop visual smoke was not run because the cargo tauri subcommand and browser automation were unavailable. The Vite dev server returned HTTP 200 with the expected root shell, and the production frontend build passed.
