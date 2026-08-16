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
- The desktop run_round path creates or reloads a Council-owned sanitized snapshot before repository-grounded dispatch. A missing snapshot, hash mismatch, or bridge failure stops the round; there is no direct-repository fallback.
- A secret-looking exclusion creates a persisted `SNAPSHOT_REVIEW_REQUIRED` state. The review record binds the exact snapshot ID, manifest hash, deterministic exclusion-set hash, safe relative paths/reasons, and source fingerprint captured during snapshot creation. Approval keeps excluded files absent, stores only a fixed safe acknowledgment, and becomes invalid when relevant source contents change. Rejection transitions to `SAFETY_ABORT`.

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

The controller builds each provider command from an explicit host-environment allowlist and then clears the child environment before applying that command spec. Ambient host credentials are not inherited by Claude, Antigravity, or Codex WSL. Configured API keys, custom provider URLs, alternate routing variables, and prohibited routing arguments are rejected at settings and command-construction boundaries. It does not print their values. Council execution is account-based subscription authentication only.

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

The repository contains durable individual-seat evidence for Claude, Antigravity, and Codex WSL, plus implementation and automated checks for the controller boundary. That evidence does not substantiate a current Tauri three-seat debate through R1, R2, R3, human decision, and local export.

The current installed-app certification run created debate `debate-795181f0-43db-42c9-97ff-0af9b14fb9f0`, showed the unavailable-seat recovery/degraded controls, persisted cancellation, and verified restart/reinstall persistence. The corrected native repository-grounded candidate created debate `debate-5003e0d6-4635-46d4-a3d4-ddddab6690eb`, persisted the exact review gate for snapshot `snapshot-debate-5003e0d6-4635-46d4-a3d4-ddddab6690eb`, and is waiting for owner approval before provider dispatch. Current-host live positions, citation attachment, decision, export, provider-process cancellation, and interrupted-dispatch recovery remain unverified. The exact gate classifications and hashes are recorded in [V1-PRODUCTION-CERTIFICATION.md](../evidence/V1-PRODUCTION-CERTIFICATION.md).

Accordingly, the current product status is `RELEASE_CANDIDATE`, not `PRODUCTION_CERTIFIED`. V1 continues to perform preflight, keep unavailable seats visible, and require an explicit degraded-mode choice before continuing.
