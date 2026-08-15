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

Units 1 through 9 have a working local implementation and automated coverage. The Tauri bridge exposes explicit round dispatch, but the desktop runtime has not been live-smoked in this environment because the Tauri CLI is not installed. The frontend is production-build verified and the Vite shell responds locally.

The controller refuses repository-grounded execution until an explicit snapshot bridge is selected. Synthetic packet runs are supported and are the safe default for local smoke checks.

Live provider execution remains opt-in. The current process environment reports a blocked billing variable, so no authenticated provider round was attempted during this build. The controller must continue to reject that environment rather than route through Platform API billing.

## Verification posture

Live provider calls are opt-in and never run as part of ordinary unit tests. Fake adapters exercise orchestration and failure handling. Certified live smoke tests use the exact isolated provider configurations recorded in the certification findings. See docs/evidence/README.md for the commands and current unverified items.
