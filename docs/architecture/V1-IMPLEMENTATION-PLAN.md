# Council of Agents V1 Implementation Plan

## Invariants

- The Rust core owns orchestration, persistence, safety, provider lifecycle, and compilation.
- Provider processes receive sanitized, immutable, file-based packets and never receive the real repository.
- All provider turns are fresh processes. No resume, continue, or hidden provider memory is used between rounds.
- Models produce reasoning only. Deterministic code controls state transitions, retries, repair policy, cancellation, and human approval.
- A human decision is required before deterministic exports are generated.
- Exports are written to Council-owned application data, never into the analyzed repository.
- There is no automatic implementation handoff, provider launch button, repository write capability, or majority authority.

## Implementation units

1. Rust domain and state machine
2. SQLite persistence and append-only audit chain
3. Snapshot, reparse-point, secret, and read-only sealing services
4. Packet and evidence services
5. Provider command contracts and preflight guards
6. Headless CLI
7. Tauri command bridge
8. React command-center UI
9. Skills, fixtures, documentation, and acceptance tests
10. Final live-safe verification and V1-FINAL-REPORT.md

## Current implementation posture

Units 1 through 9 have a working local implementation and automated coverage. The Tauri bridge exposes explicit round dispatch, repository snapshot creation, Linux snapshot verification, evidence attachment, recovery status, R0 candidate discovery, independent-only evaluation, explicit degraded-seat continuation, and deterministic export. The native Tauri 2 shell was built and smoke-launched successfully in the current environment; IPC/provider dispatch remains intentionally unrun.

Repository-grounded execution creates or reloads a sanitized Council-owned snapshot and dispatches only from that snapshot. Synthetic packet runs remain available for safe local checks. Snapshot, packet, and schema hashes are persisted and checked before provider dispatch, with Codex payloads bridged into the dedicated WSL distribution without mounting the Windows repository.

Live provider execution remains opt-in. The current process environment reports a blocked billing/routing variable, so no authenticated provider round was attempted during this build. The controller must continue to reject that environment rather than route through Platform API billing. M0.8 certification evidence for the three seats is carried forward separately, but a current-host live round is still an explicit runtime gate.

## Verification posture

Live provider calls are opt-in and never run as part of ordinary unit tests. Fake adapters exercise orchestration and failure handling. Certified live smoke tests use the exact isolated provider configurations recorded in the certification findings. See docs/evidence/README.md for the commands and current unverified items.
