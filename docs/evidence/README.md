# Evidence Index

## Current local verification

The current implementation has been checked with:

~~~powershell
$env:CARGO_TARGET_DIR = 'C:\council-target'
rustup run 1.96.0-x86_64-pc-windows-msvc cargo fmt --all
rustup run 1.96.0-x86_64-pc-windows-msvc cargo test --workspace
rustup run 1.96.0-x86_64-pc-windows-msvc cargo check --workspace
Push-Location app
npx tauri --version
npm run build
npx tauri dev --no-watch
Pop-Location
rustup run 1.96.0-x86_64-pc-windows-msvc cargo run -p council-cli -- providers
rustup run 1.96.0-x86_64-pc-windows-msvc cargo run -p council-cli -- demo --output .\artifacts\verification-demo
~~~

The Rust workspace tests cover the deterministic core, fake provider execution, state transitions including independent-only opening, packet sizes, evidence controls, snapshot exclusions, WSL command planning, persistence/audit behavior, idempotent dispatch recovery, bounded R0 discovery, and stateless reconstruction. The current run passed 25 tests.

The native smoke built and launched `council-desktop.exe` with a responsive `Council of Agents` window, then stopped it. This proves the Windows Tauri shell can start, not that IPC or an authenticated provider call completed.

The CLI provider gate returned `BILLING BLOCKED_ENVIRONMENT_VARIABLE_PRESENT`; Claude, Antigravity, and Codex WSL preflight contracts were otherwise `READY`. The CLI synthetic demo, snapshot copy, and exact/shifted citation controls also passed in temporary directories.

The synthetic demo proves local SQLite and packet creation. It does not prove authenticated provider availability.

## Live evidence

- M0.8-FINDINGS.md is the final native feasibility decision.
- CODEX-WSL-FINAL-CERTIFICATION.md is the dedicated Codex WSL seat evidence.
- CODEX-WSL-CERTIFICATION.md is the preceding recovery record.

## Unverified here

- A live three-seat provider round was not run during V1 build verification because the current process environment reported a blocked billing variable and the implementation must not use API-key routing.
- Tauri IPC actions, authenticated repository-grounded R1/R2/R3 calls, live citation attachment, WSL transfer/cancellation, crash recovery from a real interrupted provider process, and packaged installer behavior remain unverified in this environment.
