# Council Product Language

## Purpose

Consistent language makes Council feel like an intentional technical product and prevents the UI from drifting into generic AI or employment metaphors.

## Core vocabulary

| Use | Meaning | Avoid saying |
|---|---|---|
| **Council** | The bounded deliberation process and product | swarm, panel of bots |
| **Seat** | One provider participant in a council | employee, worker, bot, character |
| **Provider** | The local CLI/runtime that supplies a seat | brain, personality |
| **Participant** | A neutral alternative when seat attribution is not central | agent persona |
| **Question** | The technical decision the human needs to make | prompt, task (unless discussing input mechanics) |
| **Debate** | The bounded R1/R2/R3 deliberation | chat, conversation, argument |
| **Round** | A controller-defined stage: R1, R2, or R3 | turn loop, swarm cycle |
| **Position** | A seat's structured recommendation and commitment | answer, vote |
| **Claim** | A load-bearing statement with attribution and evidence | opinion blob |
| **Evidence** | Source material and verification status for a claim | proof (unless exact proof is established) |
| **Citation** | A path/range reference to evidence | link (for repository line references) |
| **Concede** | A seat accepts a peer claim with explanation | agree button |
| **Dispute** | A seat challenges a peer claim with reason | disagree reaction |
| **No basis to judge** | A seat cannot responsibly decide from available evidence | abstain without explanation |
| **Packet** | The explicit bytes sent to a provider for one purpose | hidden context |
| **Snapshot** | Sanitized, immutable repository evidence | live repo copy |
| **Preflight** | Provider and safety readiness checks | setup magic |
| **Certification** | Evidence that a provider route meets Council requirements | trusted automatically |
| **Declared limitation** | A known provider or evidence boundary | bug, unless it is a defect |
| **Degraded Council** | Human-approved deliberation with fewer available seats | partial consensus |
| **Human decision** | The owner's recorded choice or next action | final AI answer |
| **Master prompt** | Deterministic, human-copyable implementation brief | execution command, handoff automation |
| **Decision record** | Durable history of why the human chose a direction | transcript |

## Status language

Use exact, calm labels:

```text
READY
CHECKING
AUTH REQUIRED
CERTIFICATION WARNING
AVAILABLE
UNAVAILABLE
QUOTA LIMIT
TIMEOUT
UNKNOWN FAILURE
PROVIDER DOES NOT REPORT
VERIFIED EXACT
VERIFIED CONTENT FOUND ELSEWHERE
UNVERIFIED
PARTIAL ROUND
DEGRADED COUNCIL
DECISION REQUIRED
DECISION RECORDED
EXPORT READY
```

Do not call a provider “broken” when it is unavailable, uncertified, quota-limited, or outside the chosen route. Do not call an unverified citation “fake” without evidence that it is false; say `UNVERIFIED` or `CONTENT FOUND ELSEWHERE`.

## Commitment language

Use only:

```text
WOULD STAKE
CONDITIONAL
WOULD NOT STAKE
```

Avoid numeric confidence, stars, percentage certainty, and winner badges. Commitment describes what a seat would defend under stated constraints, not mathematical probability.

## Action language

Human actions:

```text
New Debate
Run Preflight
Repair
Remove Seat
Continue with Available Seats
Run R1
Run R2
Run R3
Continue Targeted Debate
Challenge Consensus
Approve Option
Approve Modified Decision
Reject All
Copy
Save
Export
```

Avoid `Launch`, `Send to Provider`, `Implement`, `Run Code`, `Deploy`, or `Make the AI Decide` in the Council product flow.

## Copy rules

- Lead with the human's decision and the current state.
- Use active, concrete sentences.
- Name the provider limitation instead of hiding it in a tooltip.
- Say “the position changed because claim C3 was supported by exact evidence,” not “the model learned.”
- Say “the packet was not verified in WSL,” not “Codex was weird.”
- Explain why a user action is blocked and what safe action is available.
- Preserve the distinction between a recommendation and a decision.

## Example microcopy

```text
Good: “Codex WSL timed out. No Codex position was accepted. Retry the same packet or continue explicitly with two seats.”
Bad: “Codex failed. The other AIs will decide.”

Good: “Served model: provider does not report. Requested model remains visible.”
Bad: “Model verified.”

Good: “Content found elsewhere. The claim may be relevant, but the original citation range is not exact.”
Bad: “Citation passed.”

Good: “Council prepared this prompt for manual copy. Nothing will run from here.”
Bad: “Send to your favorite coding agent.”
```

## Terms to reserve

- **Consensus:** use only to describe agreement as an observation, never as authority. Prefer “three seats agree” when precision matters.
- **Verified:** use only when the named verification actually ran and passed.
- **Certified:** use only for the provider route and certification scope, with timestamp/limitation.
- **Final:** use for a human-recorded decision or R3 final position, not an unreviewed provider response.
