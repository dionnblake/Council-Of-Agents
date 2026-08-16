# Council of Agents V1 Boundary Contract

This document is the stable security contract for the local controller. It describes what the implementation must enforce and what the current verification does or does not prove.

## Context flow

The controller owns the only path from intake to provider:

~~~text
intake
  -> preflight
  -> sanitized snapshot or synthetic packet
  -> immutable file packet
  -> fresh provider process
  -> structured output validation
  -> persisted raw artifact and position
  -> human decision
  -> deterministic local export
~~~

The Council does not edit a repository, create branches, commit, push, deploy, or launch another coding harness.

## Repository and snapshot boundary

- A real repository is never passed directly to a provider.
- SnapshotBuilder copies bytes into a Council-owned destination.
- .git, provider directories, instruction files, hook files, MCP files, and configured secret patterns are excluded.
- Symlinks and Windows reparse points are rejected.
- The manifest records file size and SHA-256.
- The snapshot is sealed read-only; Windows ACLs are applied through the native security API, not icacls.
- The desktop run_round path creates or reloads a Council-owned sanitized snapshot before repository-grounded dispatch. A missing snapshot, secret review gate, hash mismatch, or bridge failure stops the round; there is no direct-repository fallback.

## Packet boundary

- Packets are file-based, immutable, byte-checked, and SHA-256 hashed.
- Provider prompts contain a short packet reference, not the full packet body.
- Each provider receives a fresh process. No resume or hidden provider session is used.
- Codex payloads are streamed through tar.exe into the dedicated WSL distribution over stdin. The real Windows repository is not mounted.
- Codex snapshot, packet, and schema hashes are checked from inside WSL before dispatch. The Linux snapshot is sealed read-only and scratch is separate.

## Provider boundaries

Claude uses its dedicated local configuration directory. Antigravity requires useG1Credits=false. Codex requires:

~~~text
distribution = CouncilCodexWSL
user = council
HOME = /home/council
CODEX_HOME = /home/council/.codex
working directory = /home/council/council/scratch/...
packet = /home/council/council/packet/...
~~~

The provider registry builds commands from typed configuration. React never spawns a provider process and never accepts a raw shell command.

## Billing and credentials

The controller rejects configured API keys, custom provider URLs, and alternate provider routing variables. It does not print their values. Council execution is account-based subscription authentication only.

## Failure behavior

- Timeout, process failure, authentication failure, provider limits, schema failure, semantic failure, refusal, and safety failures are typed.
- Claude and Codex use validation plus quarantine with no automatic repair.
- Antigravity is allowed one repair attempt because that is the certified seat policy.
- An incomplete three-seat round is quarantined and cannot advance the debate.
- A targeted round is human-requested and capped at one.
- If a seat is unavailable, the human may explicitly proceed with at least two remaining seats and a persisted rationale; the controller never silently shrinks the council.
- Independent-only evaluation stops after the opening positions and records deterministic evaluation metrics without treating the result as a production council decision.
- Process timeout uses Windows Job Object containment for native provider processes. The dedicated Codex WSL process is intentionally excluded from a Windows Job Object because this host's `wsl.exe` RPC rejects Job Object assignment; Codex remains bounded by the cleared `env -i` subscription environment, the fixed `CouncilCodexWSL` distribution and `council` user, read-only Codex sandbox, isolated Linux working directory, sealed snapshot and hash checks, and `wsl --terminate CouncilCodexWSL` as the hard timeout fallback.

## Human authority

Provider output is advisory. Majority is not authority. The human decision is required before compilation, and exports are written to Council application data for manual review and copy only.

## Evidence status

V1 runtime certification on the current host completed a controlled-repository three-seat debate through opening, cross-examination, final positions, human decision, and local export, with all three seats returning valid positions. Cancellation and restart recovery were also exercised. This remains host-specific evidence; future hosts may not have the same WSL distribution, authentication, or provider availability, so V1 still performs preflight, keeps unavailable seats visible, and requires an explicit degraded-mode choice before continuing.
