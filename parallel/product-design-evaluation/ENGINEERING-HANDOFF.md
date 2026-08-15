# Engineering Handoff: Product, Design, Skills, and Evaluation Package

This handoff is for the primary engineering agent. It does not modify production implementation. Items below are requirements to consume, not commands to bypass existing architecture contracts.

## MUST IMPLEMENT

- Preserve the product boundary: Council deliberates, the human decides, the master prompt is manually copied, and the process stops.
- Keep exactly five V1 reasoning packages: `protocol.v1`, `architecture.v1`, `stack-selection.v1`, `design-taste.v1`, and `output-position.v1`.
- Enforce R1 independence. No peer position or peer claim may enter another seat's opening packet.
- Use controller-assigned stable claim IDs and validate bounded load-bearing claims.
- Require `CONCEDE`, `DISPUTE`, or `NO_BASIS_TO_JUDGE` in R2. Require an explanation for `NO_BASIS_TO_JUDGE`.
- Preserve R3 surviving claims, withdrawn claims, remaining disputes, strongest counterargument, acceptance criteria, implementation constraints, and revision attribution.
- Use exactly `WOULD_STAKE`, `CONDITIONAL`, and `WOULD_NOT_STAKE` for commitment. Do not use numeric confidence percentages.
- Distinguish `VERIFIED_EXACT`, `VERIFIED_CONTENT_FOUND_ELSEWHERE`, and `UNVERIFIED` evidence states.
- Keep requested model and served-model status separate. Use `PROVIDER_DOES_NOT_REPORT` when identity is not exposed.
- Make provider failure, partial completion, cancellation, and unknown failure visible and attributable.
- Require a human decision before deterministic master-prompt compilation.
- Do not expose provider-send, implement, run, or open-another-harness controls in the master-prompt flow.
- Make deterministic output reproducible from the same approved state.
- Apply the safety acceptance controls: snapshot isolation, reparse-point rejection, secret scan, config stripping, read-only evidence, WSL boundary, scratch separation, and process cancellation.

## SHOULD IMPLEMENT

- Use the claim/evidence relationship surface as the primary Debate View instead of three giant chat columns.
- Support Compare and Discovery as distinct flows. In Discovery, cap R0 nominations and bound the candidate union before R1.
- Support explicit two-seat continuation with the missing provider and declared limitation recorded.
- Keep a compact provider status rail visible through the debate.
- Implement keyboard navigation for claims, evidence, rounds, decision actions, and copy/export.
- Add a deterministic demo fixture for each major failure and acceptance state.
- Preserve a raw artifact and validation trail for each provider attempt.
- Use a vector of evaluation metrics rather than a single quality score.
- Add design acceptance checks for hierarchy, density, evidence readability, non-generic composition, loading/error polish, and dark-mode/focus quality.

## DESIGN REQUIREMENT

- Treat the UI as a technical deliberation command center, not a generic AI dashboard.
- Keep question, round, unresolved issue, and human decision above provider transcript detail.
- Use cards only for meaningful boundaries; use rows/tables/dividers for dense comparisons.
- Establish deliberate title, section/body, metadata, and code/citation typography.
- Do not use full-screen gradients, glowing AI orbs, excessive pills, rainbow providers, decorative glassmorphism, or centered marketing hero layouts.
- Preserve visual distinction for provider identities without turning the interface into branded panels.
- Show empty, loading, partial, failure, cancellation, disabled, and success states as first-class designs.
- Make evidence statuses and model-identity limitations legible without relying on color.
- Keep the human decision action prominent but never visually equivalent to automatic implementation.

## EVALUATION REQUIREMENT

- Add the 25-fixture benchmark corpus as the conceptual source for later executable fixtures. Do not require one universal answer where constraints are contextual.
- Evaluate schema conformance, citation validity, repair rate, provider failure rate, wall-clock, claim count, peer response rate, response-type distribution, attributed revision, recommendation change, duplicate arguments, packet size, availability, and deterministic export.
- Run R1-only versus full-council human checkpoints on the same fixture and packet.
- Retain evidence for every automated and human-reviewed result. Treat missing evidence as unverified.
- Include negative controls for malformed output, shifted citations, missing fields, duplicate IDs, secret/reparse inputs, hidden peer visibility, interruption, and timestamp-only export divergence.
- Use the Acceptance Gauntlet before claiming V1 complete.

## OPEN QUESTION

- **Antigravity route:** the project evidence contains a material route/version conflict. One feasibility record reports that the installed Antigravity product lacked a standalone headless structured CLI, while later certification records carry a certified Antigravity seat. Current installed-version certification must be the authority. Until recertified, keep the seat visible as `UNAVAILABLE` or `DECLARED_LIMITATION` rather than silently treating the IDE CLI as a headless provider.
- **Two-seat policy:** decide whether every debate type permits explicit two-seat continuation or whether some high-stakes types require a third certified seat.
- **Model identities:** confirm which providers report served identity and the exact status vocabulary accepted by runtime/UI.
- **Discovery union:** confirm the controller's normalization and five-candidate default limit against the existing architecture.
- **Prompt compiler schema:** align the conceptual R1/R2/R3 fields here with the production versioned schema without adding a sixth skill package.
- **Visual validation surface:** confirm whether the existing desktop shell can expose the claim relationship board without restructuring stable IPC contracts.

## DO NOT IMPLEMENT

- Do not add coding capability, automatic implementation, repository writes, GitHub access, pull requests, deployment, cloud SaaS, multi-user collaboration, mobile app, voice, agent marketplace, MCP ecosystem, or publishing.
- Do not add normal API billing or a silent paid fallback. Use existing Claude, Codex/ChatGPT, and Antigravity access only within certified boundaries.
- Do not create an LLM moderator or majority vote.
- Do not treat a provider timeout or malformed response as a concession, consensus, or valid abstention.
- Do not make requested model equal served model without provider evidence.
- Do not create a fake universal design score or numeric confidence percentages.
- Do not pass the live repository, credentials, provider config, hooks, or arbitrary ambient user skill trees to a provider.
- Do not inspect or rewrite the primary agent's incomplete production changes from this parallel branch.

## Parallel boundary and merge notes

This package was created in the isolated worktree:

```text
worktree: C:\Users\<USER>\Desktop\VIBE CODING PROJECTS\Council-Of-Agents-product
branch:   agent/product-design-evaluation
scope:    parallel/product-design-evaluation/
```

The primary checkout was not edited. This orphan branch contains only the parallel package and its local `AGENTS.md`; it does not contain the primary checkout's uncommitted production files.

## DOX UPDATE REQUIRED AT MERGE

The root `AGENTS.md` Child DOX Index currently lists `crates`, `app`, `docs`, `schemas`, `skills`, `fixtures`, and `artifacts`, but not this new parallel boundary. At merge, update the parent DOX chain to identify `parallel/product-design-evaluation/AGENTS.md` and its ownership. Do that after concurrent production work is stable so the shared root file does not conflict with the primary agent.
