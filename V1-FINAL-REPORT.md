# Council of Agents V1 Final Report

Date: 2026-08-15

## Executive verdict

```text
V1_STATUS = IMPLEMENTED_WITH_DECLARED_RUNTIME_BLOCKER
DEFINITION_OF_DONE = IMPLEMENTATION_VERIFIED; LIVE_PROVIDER_CERTIFICATION_PENDING
M1 = NOT_STARTED
```

The V1 implementation is present across the Rust controller, SQLite persistence, safety boundary, provider contracts, Tauri desktop shell, React command center, schemas, fixtures, skills, and documentation. The final local verification pass passed formatting, 29 Rust tests, workspace checking, the React production build, CLI safety gates, synthetic snapshot/evidence checks, and a native Tauri window launch.

The product is not marked fully certified because the current Windows process environment reports a blocked billing/routing variable. The controller correctly refuses to route around that guard, so no authenticated Claude, Antigravity, or Codex WSL provider call was launched during this build pass. The prior M0.8 seat certifications remain the evidence basis for the lineup, but a current-host live three-seat R1/R2/R3 run is still pending.

## What was implemented

### Controller and domain

- Rust core owns intake validation, deterministic debate state, provider lifecycle, persistence, safety gates, structured output validation, semantic validation, repair policy, evidence, stateless reconstruction, evaluation metrics, and deterministic compilation.
- Debate lifecycle is R1 opening, R2 cross-examination, R3 final positions, human decision, deterministic compile, and local export.
- Independent-only evaluation is explicit and stops at the human gate after opening positions.
- R0 stack discovery is bounded, preserves the status quo, limits owner alternatives, and is embedded in stack-selection packets.
- R2 peer positions are anonymized before delivery. R3 revisions require a reason and prior-position hash. Claim relations, concessions, disputes, and unresolved disagreements are persisted.
- A degraded-seat path requires a human rationale and at least two remaining seats. The council never silently shrinks.
- Human decisions preserve unanimity, minority positions, selected option, modified decision, rationale, and verified/unverified evidence.

### Safety and evidence

- Real repositories are never passed directly to providers. The controller creates or reloads a Council-owned sanitized snapshot.
- Snapshot copying preserves file bytes, excludes instructions/config/hooks/MCP/Git/secret matches, rejects symlinks and reparse points, records exclusions, hashes files, and seals the evidence tree read-only.
- Provider packets are exact-byte, immutable, hashed files. Rendered skill names and versions remain in the packet.
- Codex payloads are streamed into `CouncilCodexWSL` over WSL stdin. The Windows repository is not mounted. Linux snapshot, packet, and schema hashes are checked before Codex dispatch, with separate Linux scratch.
- Evidence verification distinguishes `VERIFIED_EXACT`, `VERIFIED_CONTENT_FOUND_ELSEWHERE`, and `UNVERIFIED`, and persists content/file hashes.
- API-key, custom-provider, custom-base-URL, and alternate routing variables are rejected without printing values.

### Provider contracts

The registry contains exactly the three required seats:

```text
Claude Code       claude-haiku-4-5-20251001   Pass
Antigravity CLI   gemini-3.7-flash-low        PassWithDeclaredLimitation
Codex WSL         gpt-5.6-luna                PassWithDeclaredLimitation
```

Requested model IDs are persisted per debate and flow into provider dispatch. Reported served identity is never invented. Fresh processes, explicit timeouts, sanitized environments, raw artifacts, typed failures, deterministic call IDs, idempotent dispatch intents, and fail-closed restart recovery are implemented.

### Desktop

- Tauri 2 command bridge covers provider status/auth, R0 candidates, debate creation, recent debates, positions, evidence, evaluation metrics, explicit round dispatch, degraded continuation, cancel/resume, human decision, and deterministic export.
- React surfaces Home, New Debate, Active Debate, Decision, Export, and Settings.
- New Debate supports compare/discover intake, options, constraints, optional repository grounding, per-seat requested model IDs, priority, and independent-only evaluation.
- Active Debate shows seat state, packet/evaluation status, recovery/degraded controls, positions, dissent, and next deterministic transitions.
- Decision shows persisted evidence verdicts and requires a human decision/rationale before export.
- Export writes only to Council-owned application data and explicitly provides no implementation handoff.
- The app includes the Tauri 2 CLI as a development dependency. `npx tauri dev --no-watch` built and launched a responsive native `Council of Agents` window in this pass.

### Skills

The repository contains exactly five top-level reasoning-only V1 skill packages:

```text
architecture.v1
design-taste.v1
output-position.v1
protocol.v1
stack-selection.v1
```

## Final architecture

```text
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
```

The core remains testable without Tauri. Providers receive only fresh, file-based, immutable context. No provider can edit code, create branches, commit, push, deploy, open a coding harness, or become decision authority.

## Certified seat evidence carried forward

```text
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
```

Sources: [M0.8-FINDINGS.md](M0.8-FINDINGS.md), [CODEX-WSL-FINAL-CERTIFICATION.md](CODEX-WSL-FINAL-CERTIFICATION.md).

## Current host provider gate

The final safe provider preflight returned:

```text
BILLING = BLOCKED_ENVIRONMENT_VARIABLE_PRESENT
Claude Code = READY
Antigravity CLI = READY
Codex WSL = READY
```

The command reports presence state only. No credential values were printed, no API key was used, and no provider process was launched after the billing guard failed.

## Verification matrix

| Area | Result | Evidence |
|---|---|---|
| Rust formatting | PASS | `cargo fmt --all -- --check` |
| Rust workspace tests | PASS, 29/29 | `cargo test --workspace` |
| Workspace type/check validation | PASS | `cargo check --workspace` |
| React TypeScript and production build | PASS | `npm run build` |
| Tauri CLI | PASS | `npx tauri --version`, version 2.5.0 |
| Native Tauri shell | PASS | `npx tauri dev --no-watch`; responsive native window observed |
| CLI provider safety gate | PASS_WITH_RUNTIME_BLOCKER | billing guard blocked dispatch; all provider preflights READY |
| SQLite synthetic demo | PASS | `council-cli demo` created DB and sealed packet in a temporary directory |
| Snapshot copy | PASS | fixture snapshot preserved hashes and excluded `AGENTS.md` |
| Evidence exact control | PASS | `VERIFIED_EXACT` |
| Evidence shifted control | PASS | `VERIFIED_CONTENT_FOUND_ELSEWHERE` |
| Deterministic compiler | PASS | unit test |
| State machine and dispatch recovery | PASS | unit tests |

## Safety and workflow coverage

| Control | Status |
|---|---|
| Snapshot byte copy and exclusion | PASS in unit/CLI checks |
| Secret/config/instruction exclusion | PASS in unit/CLI checks |
| Reparse/symlink rejection | PASS in snapshot implementation/tests |
| Native Windows ACL sealing | IMPLEMENTED; live write matrix pending |
| Packet size/marker handling | PASS in unit tests |
| WSL bridge avoids `/mnt/c` | PASS in bridge tests and Codex certification evidence |
| Linux payload hash verification | IMPLEMENTED; live current-host transfer pending |
| Explicit provider environment allowlist | PASS in provider tests |
| Antigravity `useG1Credits=false` guard | PASS in provider tests |
| Codex subscription-only guard | PASS in provider contract and certification evidence |
| Job Object/process containment | IMPLEMENTED; live provider cancellation pending |
| `wsl --terminate CouncilCodexWSL` fallback | IMPLEMENTED; live cancellation pending |
| Deterministic call identity/idempotency | PASS in persistence tests; live crash injection pending |
| No automatic implementation handoff | PASS |
| Human final authority | PASS |

## Remaining runtime gates

These are the only material unverified items after this build pass:

1. Clear the current unsafe billing/routing environment variable without adding an API key, then run the authenticated three-seat provider flow.
2. Exercise the Tauri IPC commands through the native window, including R1/R2/R3, evidence attachment, human decision, and export.
3. Run a real repository-grounded round to verify live Windows snapshot creation, WSL transfer, Linux hash comparison, citations, and read-only enforcement.
4. Run live cancellation, WSL termination fallback, restart persistence, and interrupted-dispatch recovery against the dedicated runtime.
5. Produce and verify an installer only after the runtime gates pass. Bundling remains intentionally disabled in this checkout.

These gates require safe provider availability or explicit host/runtime access. They are not permission to route through Platform API billing or to weaken the isolation boundary.

## Exact verification commands

```powershell
Set-Location 'C:\Users\<USER>\Desktop\VIBE CODING PROJECTS\Council Of Agents'
$env:CARGO_TARGET_DIR = 'C:\council-target'
rustup run 1.96.0-x86_64-pc-windows-msvc cargo fmt --all -- --check
rustup run 1.96.0-x86_64-pc-windows-msvc cargo test --workspace
rustup run 1.96.0-x86_64-pc-windows-msvc cargo check --workspace
rustup run 1.96.0-x86_64-pc-windows-msvc cargo run -p council-cli -- providers
Push-Location app
npm run build
npx tauri --version
npx tauri dev --no-watch
Pop-Location
```

Do not start a live provider round until the provider status gate reports safe subscription-only routing and the required isolated configurations are available.

## Closeout

The V1 implementation is verified locally with a declared live-runtime blocker. M1 remains unopened. No external repository, provider configuration, account, credential, billing setting, or public release was modified by this pass.
