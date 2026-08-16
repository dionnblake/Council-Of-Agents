# Crates

## Purpose

Own the Rust workspace packages for Council of Agents.

## Ownership

- council-core owns the deterministic domain model, state machine, validation, evidence, snapshot safety, packets, provider contracts, process containment, orchestration, persistence, and compiler.
- council-cli owns the headless controller surface and must call council-core rather than duplicate policy.

## Local Contracts

- Provider execution must use explicit environment allowlists and account-based subscription authentication.
- Windows Job Object containment remains enabled for native provider processes. The Codex WSL command uses the explicit WSL boundary controls instead because this host's `wsl.exe` RPC fails for any Job Object assignment; its dedicated distribution termination remains the timeout fallback.
- No provider adapter may implement autonomous coding, repository mutation, handoff, or external publishing.
- Snapshot and packet bytes must remain inspectable and hashable.
- Repair policies and certification limitations are explicit data, not inferred from success.
- Changes to persisted schema require a migration-safe initialization path and tests.

## Verification

- Run rustup run 1.96.0-x86_64-pc-windows-msvc cargo fmt --all.
- Run cargo test --workspace with CARGO_TARGET_DIR=C:\council-target.
- Run the relevant CLI command against a synthetic workspace before claiming a workflow works.

## Child DOX Index

- council-core is governed here because its package is currently organized by Rust modules rather than child directories.
- council-cli is governed here because its package is currently organized by Rust modules rather than child directories.
