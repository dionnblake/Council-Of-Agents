# Council of Agents Product Specification

## Product purpose

Council of Agents is a Windows-first, local-first technical deliberation application for people who use AI-assisted development and want stronger decisions before implementation begins.

The product turns one consequential technical question into a bounded, inspectable council process:

```text
QUESTION
  -> INDEPENDENT ANALYSIS
  -> CROSS-EXAMINATION
  -> FINAL POSITIONS
  -> HUMAN DECISION
  -> MASTER IMPLEMENTATION PROMPT
  -> MANUAL COPY
  -> STOP
```

Council reduces the work of manually asking several providers the same question, forwarding answers, and trying to preserve disagreement. Its value is not that several models agree. Its value is that independent positions, evidence, challenges, revisions, and remaining dissent are visible before the human commits to a direction.

## Target user

The primary user is a technically capable AI-assisted builder, consultant, or engineering lead who:

- has a real product, repository, or architecture decision;
- can judge technical tradeoffs but wants more independent perspective;
- wants to use existing Claude, Codex, and Antigravity access rather than normal API billing;
- needs a reviewable decision record, not a chat transcript;
- wants to keep final authority and manually choose whether to implement the result.

The product is designed for one human decision owner at a time. It is not a multi-user collaboration system in V1.

## Core value proposition

Council provides:

```text
multiple independent perspectives
+ controlled disagreement
+ evidence
+ cross-examination
+ preserved dissent
+ human authority
```

The output should answer not only “what is recommended?” but also:

- what each seat would stake its position on;
- which claims are load-bearing;
- what evidence supports or weakens those claims;
- what changed after peer challenge;
- what the minority position still says;
- what would cause the decision to flip;
- what the cost is if the decision is wrong;
- what the human approved and what remains outside the decision.

## What Council is not

Council is not:

- an AI coding agent or implementation harness;
- an autonomous swarm that edits a repository;
- a multi-agent execution framework;
- a majority-voting system;
- code-review automation;
- a project manager or issue tracker;
- a chat aggregator that places three conversations side by side;
- a replacement for a human architecture decision.

Council must not create commits, branches, pull requests, deployments, provider handoffs, or automatic implementation actions. The resulting master prompt is a prepared artifact for a human to copy into another coding harness.

## Default seats and provider truth

The intended V1 seats are:

1. Codex CLI through the dedicated certified CouncilCodexWSL boundary.
2. Claude Code through its isolated Windows configuration.
3. Antigravity through the standalone authenticated CLI route.

Seat availability is a runtime fact, not a product promise. A seat may be `UNAVAILABLE`, `UNVERIFIED`, quota-limited, authentication-expired, or otherwise restricted. Council must show that state and preserve the evidence. It must not fabricate a third position or silently substitute a different provider.

Requested model and served model are separate facts. If a provider does not report the served model, Council shows `PROVIDER_DOES_NOT_REPORT` or equivalent calm audit language. It never implies a verified match from a request alone.

## Product principles

### Human authority

The controller orchestrates turns and persists evidence. The human decides. Agreement among models is evidence of alignment, not proof of correctness.

### Independence before influence

R1 positions are generated without peer positions in the prompt. The controller makes the same question, constraints, repository evidence, and candidate set available to every seat.

### Evidence before confidence

Agents distinguish facts, inferences, assumptions, and unknowns. Repository claims cite the immutable evidence snapshot. Unsupported claims may remain useful only when labelled as assumptions or `NO_BASIS_TO_JUDGE`.

### Dissent is a product output

The final decision record preserves the strongest minority position and the issue most likely to change the decision. A majority is never shown as authority.

### Bounded depth

Council runs a small number of explicit rounds. It does not create endless conversational drift. The default debate has R1 independent positions, R2 cross-examination, and R3 final positions.

### Local control

Questions, snapshots, packets, raw provider artifacts, decisions, and exports remain inspectable locally. Existing subscriptions are used; there is no silent API-paid fallback.

## Core user journey

1. **Open Home.** The user sees recent debates, incomplete work, recent decisions, and provider health without being forced through setup.
2. **Start New Debate.** The user enters one decision question, chooses `Compare` or `Discovery`, supplies product and decision context, and optionally selects a repository.
3. **Review preflight.** Council shows each selected seat's installation, authentication, certification, availability, requested model, served-model status, and declared limitations. The human can continue with two seats when a third is unavailable if the evidence state permits it.
4. **Run R1.** Council dispatches independent positions. Each position is stored with claim IDs, evidence, risks, assumptions, commitment, reversibility, cost if wrong, and flip condition.
5. **Inspect the debate.** The user sees claims and evidence first, with provider transcripts available as supporting detail. The UI surfaces unresolved disputes rather than rewarding verbosity.
6. **Run R2.** Each seat receives the bounded peer claims and responds with `CONCEDE`, `DISPUTE`, or `NO_BASIS_TO_JUDGE`, including reasons and evidence where applicable.
7. **Run R3.** Each seat states its surviving claims, withdrawn claims, remaining disputes, strongest counterargument, revised recommendation if any, and acceptance constraints.
8. **Make a human decision.** The user approves an option, approves a modified decision, continues a targeted debate, challenges the apparent consensus, or rejects all options. This action is recorded as the human decision, not an agent vote.
9. **Review the master prompt.** Council compiles the approved state into a deterministic implementation prompt that contains decision context, constraints, accepted requirements, evidence references, risks, dissent, and explicit boundaries.
10. **Copy or export.** The user copies or saves the artifact manually. The product stops there. There is no provider launch button.

## Debate types

### Normal debate

Use when the user has a focused question and enough context to compare a small set of options. The question, constraints, evidence, and decision criteria are held constant across seats.

### Greenfield debate

Use when no repository exists. Agents reason from product requirements, platform, users, constraints, and priorities. The UI explicitly says `NO REPOSITORY`; evidence requirements distinguish external facts from product assumptions. The Council must not invent repository citations.

### Repository-grounded debate

Use when a repository is provided. Council creates a stable, sanitized, immutable evidence snapshot. Agents cite only snapshot paths and line ranges. The live repository is not passed to providers, and the snapshot is not writable by provider processes.

### Compare mode

The human supplies a bounded candidate set. Every seat evaluates exactly that same set with the same initial context. The controller must not give one candidate extra framing merely because another seat proposed it.

### Discovery mode

Agents first nominate candidates in a thin R0 with brief justifications. The controller forms a bounded union that includes the status quo, a boring established alternative, and a newer challenger when relevant. R1 then evaluates the same bounded list independently. Discovery is not permission to compare an unbounded framework ecosystem.

### Two-seat mode

When one selected provider is unavailable, the user sees the missing seat and declared limitation. Council may run with two seats only when the product has enough independent perspective for the chosen mode and the user explicitly continues. The decision record records the missing seat and does not label two-seat output as a three-seat consensus.

### Three-seat mode

The default mode when all selected seats are certified and available. Each seat remains independently attributable. A third seat adds another perspective; it does not add voting authority.

## Important product states

### Provider unavailable

The UI identifies the provider, exact state, last check, and next safe action. Examples include `NOT_INSTALLED`, `NOT_AUTHENTICATED`, `NOT_CERTIFIED`, `QUOTA_LIMITED`, `TIMEOUT`, `MODEL_LIMITATION`, and `UNKNOWN_FAILURE`. Unknown failure pauses the round. The user can repair, remove the seat, or cancel; Council does not silently retry into a different billing or provider path.

### Partial round

Completed seats remain inspectable. Incomplete seats show their failure artifact and retry status. The user may retry only the failed seat if the round inputs are unchanged, or cancel the round. A partial round cannot be presented as a complete council result.

### Human decision

The decision gate is explicit and required. Available actions are `Approve Option`, `Approve Modified Decision`, `Continue Targeted Debate`, `Challenge Consensus`, and `Reject All`. The selected action, rationale, approved constraints, and unresolved issues are stored.

### Export

Export creates a local, deterministic artifact from the approved state. It includes a clear status such as `DECISION_APPROVED` or `DECISION_REJECTED`, an evidence summary, dissent, and the master prompt. Export does not run code, call a provider, or publish anything.

## Product success criteria

Council succeeds when a human can:

- understand what decision is being made without reading every transcript;
- see which claims are supported, disputed, or unjudgeable;
- distinguish provider limitations from reasoning disagreement;
- identify what would change the decision;
- preserve a useful minority position;
- make and audit a human decision;
- copy a deterministic master prompt without accidentally starting implementation.

It fails when it hides uncertainty, turns agreement into authority, mixes independent and peer-informed reasoning, passes the live repository to providers, or makes the final handoff feel like an automatic execution command.
