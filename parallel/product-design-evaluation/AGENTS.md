# Product, Design, and Evaluation Package

## Purpose

This subtree contains the parallel specification package for Council of Agents. It defines product behavior, desktop UX, design taste, reasoning-skill contracts, stack-selection methods, benchmark fixtures, security tests, mock debates, evaluation, acceptance, portfolio explanation, and the engineering handoff.

## Ownership

This subtree is owned by the parallel product/design/evaluation workstream. It does not own Rust, Tauri, React, provider adapters, snapshots, schemas, production skills, production fixtures, or runtime configuration. The `fixtures/mock-debates/` area contains documentation-only synthetic debates for UI and QA design.

## Local Contracts

- Keep all work in this subtree during parallel development.
- Treat the repository and certification records as evidence. Mark unresolved or conflicting evidence as unresolved; do not silently promote an assumption to fact.
- The Council remains a local-first technical deliberation product. It advises; the human decides; manual copy is the final handoff boundary.
- The five V1 reasoning packages are exactly `protocol.v1`, `architecture.v1`, `stack-selection.v1`, `design-taste.v1`, and `output-position.v1`.
- These documents may specify production behavior but must not implement production code or create production JSON Schema files.
- Security material describes adversarial cases and expected fail-closed behavior; it must not contain real credentials, exploit payloads against external systems, or runnable attack tooling.
- Mock debates are synthetic, provider-free, and safe to render. They must not be presented as live provider evidence.

## Work Guidance

- Prefer concrete states, inputs, outputs, and acceptance checks over adjectives.
- Preserve dissent and provider limitations.
- Avoid generic AI dashboard patterns and fake aggregate quality scores.
- Keep benchmark fixtures contextual. Do not force one universal architecture answer.
- Keep security tests and failure matrices tied to observable detection, user state, retry policy, and audit evidence.

## Verification

- Check every required deliverable from the assignment exists.
- Search for prohibited product expansion, automatic implementation, provider-send controls, numeric confidence percentages, and unsupported completion claims.
- Review the final diff and run the repository verifier only if it can run against this documentation-only orphan worktree.
- At merge, the parent DOX index must identify this durable boundary.

## Child DOX Index

- `security/AGENTS.md` owns the threat model, prompt-injection corpus, and safety test cases.
- `fixtures/AGENTS.md` owns the parallel synthetic fixture boundary.
