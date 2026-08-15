# ADR 0001: Local-First Three-Seat Boundary

## Status

Accepted for V1.

## Decision

Council of Agents is a Windows-first local controller with three certified provider seats:

- Claude Code through its isolated local configuration.
- Antigravity CLI with the no-G1-credit guard.
- Codex CLI through the dedicated CouncilCodexWSL boundary.

The Rust core owns state, persistence, provider lifecycle, packet and snapshot safety, structured validation, evidence, repair policy, and deterministic compilation. React is a human-facing surface over typed Tauri commands.

## Consequences

- A provider outage or missing authentication is visible and does not silently become another provider.
- Provider identity limitations remain in the debate record.
- Cross-provider context is transferred as immutable files, not hidden session state.
- A human decides before export.
- The architecture can be tested with fake executors without spending provider quota.
- Repository-grounded runs require a controller-created snapshot bridge; the controller never falls back to reading or dispatching the real repository directly.

## Rejected alternatives

- Native Windows Codex user-context isolation was closed by M0.8 evidence.
- API-key routing is not a billing fallback for ChatGPT-subscription Codex.
- Majority vote is not a human-decision substitute.
