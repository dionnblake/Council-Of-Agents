# Crates

## Purpose

Own the Rust workspace packages for Council of Agents.

## Ownership

- council-core owns the deterministic domain model, state machine, validation, evidence, snapshot safety, packets, provider contracts, process containment, orchestration, persistence, and compiler.
- council-cli owns the headless controller surface and must call council-core rather than duplicate policy.

## Local Contracts

- Provider execution must use explicit environment allowlists, clear the child environment before applying a command spec, reject configured API-key/custom-routing arguments, and use account-based subscription authentication.
- Windows Job Object containment remains enabled for native provider processes. The Codex WSL command uses the explicit WSL boundary controls instead because this host's `wsl.exe` RPC fails for any Job Object assignment; its dedicated distribution termination remains the timeout fallback.
- No provider adapter may implement autonomous coding, repository mutation, handoff, or external publishing.
- Snapshot and packet bytes must remain inspectable and hashable.
- A secret-looking snapshot exclusion enters the persisted `SNAPSHOT_REVIEW_REQUIRED` state. Review records bind the debate to the snapshot ID, manifest hash, exclusion-set hash, sanitized metadata, and source fingerprint; approval is explicit and stale source contents invalidate it.
- Repair policies and certification limitations are explicit data, not inferred from success.
- Provider model levels are provider-specific, validated against supported effort values, persisted with migration-safe defaults, and applied in each provider command without changing subscription routing.
- Antigravity model identifiers with embedded `-low`, `-medium`, or `-high` levels must be paired with that exact level, and the command must omit a conflicting `--effort` flag.
- Persisted latest-round turn statuses must remain queryable after reload so failed or partial provider rounds cannot appear complete.
- Changes to persisted schema require a migration-safe initialization path and tests.

## Verification

- Run rustup run 1.96.0-x86_64-pc-windows-msvc cargo fmt --all.
- Run cargo test --workspace with CARGO_TARGET_DIR=C:\council-target.
- Run the relevant CLI command against a synthetic workspace before claiming a workflow works.

## Child DOX Index

- council-core is governed here because its package is currently organized by Rust modules rather than child directories.
- council-cli is governed here because its package is currently organized by Rust modules rather than child directories.
