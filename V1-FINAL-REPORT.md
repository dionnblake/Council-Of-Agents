# Council of Agents V1 Final Report

Date: 2026-08-15

## Executive verdict

~~~text
V1_STATUS = IMPLEMENTED_FOUNDATION_WITH_RUNTIME_GATES
DEFINITION_OF_DONE = NOT_YET_FULLY_CERTIFIED
M1 = NOT_STARTED
~~~

The repository now contains the V1 Rust core, provider safety contracts, SQLite persistence, immutable packet and snapshot primitives, Tauri command bridge, React desktop surface, five reasoning-only skill packages, schemas, fixtures, documentation, and automated verification.

The full product definition is not claimed complete because this environment did not permit a live three-seat provider round or a Tauri GUI smoke test. The controller intentionally refused the current process environment because a blocked billing/routing variable was present. No API-key fallback was used.

## What was built

### Core

- council-core owns the debate state machine, turn states, provider registry, explicit environment allowlists, model identity status, typed failure taxonomy, timeout/cancellation metadata, WSL fallback, structured output validation, semantic validation, controller-owned claim IDs, repair policy, packets, snapshots, evidence, stateless handoff checks, persistence, and deterministic compilation.
- council-cli exposes provider preflight, database initialization, intake validation, synthetic snapshot creation, citation verification, demo generation, and deterministic compilation.
- SQLite uses normal relational tables plus an append-only hash-chained audit log with update/delete triggers.
- Per-debate requested model overrides are stored and flow into round dispatch.

### Safety and evidence

- Snapshots copy bytes rather than using git archive.
- Provider instruction/config surfaces, Git metadata, ignored files, secret matches, symlinks, and reparse points are excluded or rejected and recorded.
- Windows snapshot sealing uses native ACL APIs; icacls is not the primary mechanism.
- Packets are immutable, exact-byte checked, SHA-256 hashed, and retain rendered skill names.
- Codex payloads are streamed through tar.exe and wsl.exe stdin to CouncilCodexWSL. The real repository is not mounted.
- WSL packet/schema hashes are checked before Codex dispatch.
- Evidence verification distinguishes VERIFIED_EXACT, VERIFIED_CONTENT_FOUND_ELSEWHERE, and UNVERIFIED.

### Deliberation and governance

- Default flow is R1 opening, R2 cross-examination, R3 final positions, then the human gate.
- A targeted round is explicit and capped at one.
- A round cannot advance if all three seats do not produce usable structured positions.
- Claude and Codex have no automatic repair; Antigravity has one repair attempt.
- Provider output never becomes authority by majority.
- The human rationale is persisted before deterministic export.
- Export creates master-prompt.md and decision-record.md in Council application data. There is no implementation, handoff, branch, commit, push, deploy, or automatic external-harness action.

### Desktop

- Tauri 2 commands cover provider status, debate creation, recent debates, position retrieval, explicit round dispatch, human decision, and deterministic export.
- React surfaces Home, New Debate, Active Debate, Decision, and Settings.
- New Debate supports discovery/compare mode, options, product and decision type, constraints, optional repository intake, per-seat requested model IDs, and priority.
- Debate shows packet hash, fresh/no-resume mode, seat state, stored recommendations, risks, flip conditions, and next deterministic transition.
- Decision shows stored positions, dissent, human rationale, and export hashes.

## Final architecture

~~~text
React + TypeScript
        |
        v
Tauri 2 typed commands
        |
        v
council-core
  |       |        |          |
SQLite  packets  snapshots  provider runner
                              |
             +----------------+----------------+
             |                |                |
        Claude Code     Antigravity CLI   wsl.exe -> CouncilCodexWSL
~~~

The core is testable without Tauri. Provider calls are fresh processes with explicit timeouts, sanitized environments, raw artifact retention, and typed failure handling.

## Provider status

### Certified evidence carried forward

~~~text
CLAUDE
CERTIFICATION: PASS
SCHEMA: 20/20
PACKET: PASS
STATELESS: PASS
MODEL_IDENTITY: VERIFIED_MATCH
REPAIR_POLICY: NO AUTOMATIC REPAIR

ANTIGRAVITY
CERTIFICATION: PASS_WITH_DECLARED_LIMITATION
SCHEMA: 17/20
PACKET: PASS
STATELESS: PASS
MODEL_IDENTITY: PROVIDER_DOES_NOT_REPORT
REPAIR_POLICY: ONE REPAIR ATTEMPT

CODEX WSL
CERTIFICATION: PASS_WITH_DECLARED_LIMITATION
ISOLATION: PASS
AUTH: PASS
SANDBOX: PASS
SNAPSHOT_BRIDGE: PASS
PACKET: PASS
SCHEMA: 20/20
STATELESS: PASS
MODEL_IDENTITY: PROVIDER_DOES_NOT_REPORT
REPAIR_POLICY: NO AUTOMATIC REPAIR
~~~

Source records: M0.8-FINDINGS.md and CODEX-WSL-FINAL-CERTIFICATION.md.

### Current host runtime

The headless provider command returned:

~~~text
BILLING = BLOCKED_ENVIRONMENT_VARIABLE_PRESENT
Claude Code preflight = READY
Antigravity CLI preflight = READY
Codex WSL preflight = READY
~~~

The provider command reports only the presence state and never prints credential values. Because the environment was blocked, no provider process was launched during this build verification.

## Safety verification

| Control | Result | Evidence |
|---|---|---|
| Snapshot byte copy and exclusions | PASS | Rust snapshot test |
| Secret/config/instruction exclusion | PASS | Rust snapshot test |
| Symlink/reparse rejection | PASS | Snapshot implementation and test path |
| Native Windows ACL sealing | IMPLEMENTED, live write matrix not rerun here | snapshot.rs |
| Packet exact bytes and 50/200/500 KB markers | PASS | Rust packet test |
| WSL bridge plan avoids /mnt/c | PASS | Rust bridge test |
| WSL payload hash verification | IMPLEMENTED, live transfer not rerun here | Tauri bridge path |
| Explicit environment allowlist | PASS | Provider command test |
| API-key/custom-routing rejection | PASS | Provider status and provider guard |
| Antigravity no-G1-credit guard | PASS | Provider test |
| Windows Job Object containment | IMPLEMENTED, live provider process not rerun here | Runner implementation |
| Codex WSL terminate fallback | IMPLEMENTED, live cancellation not rerun here | Provider/runner contract |
| No automatic handoff | PASS | No command or UI action exists |

## Debate verification

| Area | Result |
|---|---|
| Deterministic lifecycle | PASS |
| Per-agent turn states | PASS |
| R1/R2/R3 round command path | IMPLEMENTED |
| One targeted round | PASS in state-machine/core and Tauri guard |
| Controller-owned claim IDs | PASS |
| Structured schema validation | PASS |
| Semantic validation | PASS |
| Certified repair policies | PASS in fake-executor tests |
| Raw stdout/stderr retention | IMPLEMENTED |
| Stateless handoff packet/reconstruction | PASS in core test |
| Human decision persistence | IMPLEMENTED |
| Deterministic compiler | PASS |
| Live three-seat R1/R2/R3 | NOT RUN |
| Live citation verification against a repository snapshot | NOT RUN |

## Verification commands and results

Environment:

~~~text
Windows PowerShell
Rust toolchain: 1.96.0-x86_64-pc-windows-msvc
CARGO_TARGET_DIR=C:\council-target
Node: 24.16.0
npm: 11.13.0
~~~

Commands:

~~~powershell
rustup run 1.96.0-x86_64-pc-windows-msvc cargo fmt --all
rustup run 1.96.0-x86_64-pc-windows-msvc cargo test --workspace
rustup run 1.96.0-x86_64-pc-windows-msvc cargo check -p council-desktop
Push-Location app
npm run build
Pop-Location
rustup run 1.96.0-x86_64-pc-windows-msvc cargo run -p council-cli -- providers
rustup run 1.96.0-x86_64-pc-windows-msvc cargo run -p council-cli -- demo --output .\artifacts\verification-demo
rustup run 1.96.0-x86_64-pc-windows-msvc cargo run -p council-cli -- verify-evidence .\docs security/BOUNDARY-CONTRACT.md:1-3
~~~

Results:

~~~text
22 Rust core tests passed
0 Rust tests failed
Desktop cargo check passed
React TypeScript check passed
Vite production build passed
Synthetic SQLite and packet demo passed
Exact citation lookup returned VERIFIED_EXACT
Vite dev server returned HTTP 200 with the expected root shell
~~~

The frontend build produced app/dist. It is ignored as a generated build output.

## Known limitations and remaining work

These are real limitations, not hidden TODOs:

1. A live provider round was not run because the current environment reported a blocked billing/routing variable. The product correctly refuses to route around that guard.
2. The Tauri CLI is not installed in this environment. The frontend was built and served through Vite, but the native desktop window, IPC calls, and actual provider availability were not visually smoke-tested.
3. The desktop intake records an optional repository path, but run_round currently refuses repository-grounded execution until the full snapshot-to-provider context integration is selected. The snapshot and WSL bridge primitives are implemented and tested independently.
4. The current desktop decision surface retrieves stored positions and displays recommendations/risk summaries, but mechanical evidence-index verification is not yet automatically attached to every live position in the Tauri round command.
5. Deterministic call IDs are present in persisted attempt identity, but crash recovery does not yet implement a complete running/unknown/completed recovery UI.
6. Runtime settings are currently inspectable defaults and preflight status. Editing provider binary paths, timeout overrides, and certification records through the Settings screen is not yet implemented.
7. R0 candidate discovery and independent-only evaluation are represented in the domain direction but are not exposed as separate desktop workflows.
8. A packaged installer was not produced because Tauri bundling is disabled in the current configuration and the Tauri CLI was unavailable.

## Exact run instructions

### Core and CLI

~~~powershell
Set-Location 'C:\Users\<USER>\Desktop\VIBE CODING PROJECTS\Council Of Agents'
$env:CARGO_TARGET_DIR = 'C:\council-target'
rustup run 1.96.0-x86_64-pc-windows-msvc cargo test --workspace
rustup run 1.96.0-x86_64-pc-windows-msvc cargo run -p council-cli -- providers
rustup run 1.96.0-x86_64-pc-windows-msvc cargo run -p council-cli -- demo --output .\artifacts\demo
~~~

### Frontend

~~~powershell
Set-Location 'C:\Users\<USER>\Desktop\VIBE CODING PROJECTS\Council Of Agents\app'
npm install
npm run build
npm run dev -- --host 127.0.0.1
~~~

### Native desktop

Install a compatible Tauri 2 CLI in the development environment, then:

~~~powershell
Set-Location 'C:\Users\<USER>\Desktop\VIBE CODING PROJECTS\Council Of Agents\app'
npm install
cargo tauri dev
~~~

Do not launch a live provider round until the provider status screen shows the required isolated configuration and the billing guard is clear.

## Packaging instructions

Packaging is intentionally not claimed verified. After installing the Tauri 2 CLI and confirming the Windows WebView2/Rust prerequisites:

~~~powershell
Set-Location 'C:\Users\<USER>\Desktop\VIBE CODING PROJECTS\Council Of Agents\app'
npm run build
cargo tauri build
~~~

Before enabling distribution, set the Tauri bundle configuration, run the native desktop smoke test, verify application-data export paths, and repeat provider preflight on the target host.

## Portfolio-ready description

Council of Agents is a local-first Windows technical deliberation system that coordinates Claude Code, Antigravity CLI, and Codex through a dedicated WSL isolation boundary. A deterministic Rust core manages immutable evidence packets, provider safety, stateless multi-round reasoning, structured outputs, repair/quarantine policy, append-only audit history, and a human decision gate. The Tauri/React command center makes provider identity limitations, dissent, packet hashes, and final export provenance visible without giving the Council any ability to modify code or autonomously hand off implementation.

## Closeout

The implementation is committed as a coherent initial repository snapshot. No external repository, account, provider configuration, credentials, or billing setting was modified by this build.
