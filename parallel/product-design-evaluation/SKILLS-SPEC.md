# V1 Reasoning Skill Package Specification

## Scope

Council V1 has exactly five top-level reasoning-only packages:

```text
protocol.v1
architecture.v1
stack-selection.v1
design-taste.v1
output-position.v1
```

These are prompt/context contracts for reasoning. They are not coding agents, runtime plugins, provider-specific personalities, or permissions to edit a repository. The production implementation may package them later, but this document does not create production skill files.

## Common package contract

Every package receives:

- the decision question;
- product type and decision type;
- mode (`COMPARE` or `DISCOVERY`);
- hard constraints and primary priority;
- repository evidence summary or explicit `NO_REPOSITORY`;
- selected candidate set, if any;
- the current round and allowed output shape;
- provider limitations and evidence status.

Every package must:

- distinguish fact, inference, assumption, and unknown;
- use bounded claims rather than a long shallow list;
- state what would change the recommendation;
- preserve uncertainty instead of filling missing evidence with confident prose;
- stay within the decision scope;
- return structured content that the controller can validate.

No package may authorize implementation, provider handoff, repository writes, or a majority vote.

## Package 1: `protocol.v1`

### Responsibility

`protocol.v1` governs how a seat reasons and responds across the council rounds. It is the shared deliberation discipline.

### Required behaviors

- Ground factual claims in supplied evidence or label them as assumptions.
- Separate fact from inference in wording and fields.
- Steelman the strongest plausible alternative before criticizing it.
- Prefer 5–7 load-bearing claims per opening position. Do not generate a catalog of every possible consideration.
- State the strongest counterargument to the recommendation.
- Describe cost if wrong, reversibility, and flip condition.
- Preserve scope. Decline adjacent implementation design unless it is necessary to answer the decision.
- Respond to peer claims directly in R2; do not merely repeat R1.
- Revise beliefs when a peer surfaces stronger evidence or a missed constraint.
- Preserve a minority position when it remains materially supported.
- Abstain when evidence is insufficient.

### Claim discipline

Each load-bearing claim has:

```text
claim_id
statement
claim_type: FACT | INFERENCE | ASSUMPTION | UNKNOWN
importance
evidence[]
impact_if_wrong
```

The controller assigns stable claim IDs. Seats must not invent IDs that collide with another seat.

### Peer response vocabulary

For each addressed peer claim, use exactly one response:

```text
CONCEDE
DISPUTE
NO_BASIS_TO_JUDGE
```

`CONCEDE` means the peer claim is accepted as materially correct, with an explanation of its consequence.

`DISPUTE` means the seat believes the claim is false, incomplete, or misapplied, with the specific counterargument and evidence.

`NO_BASIS_TO_JUDGE` means the seat cannot responsibly determine whether the claim holds from the available evidence. It must explain what evidence is missing and must not use abstention as a rhetorical way to avoid engaging.

### Belief revision

R3 must identify:

```text
surviving_claims
withdrawn_claims
revised_claims
remaining_disputes
revision_reason
```

No revision is also a valid result when the seat explains why peer interaction did not change its position.

### Commitment vocabulary

Use exactly:

```text
WOULD_STAKE
CONDITIONAL
WOULD_NOT_STAKE
```

`WOULD_STAKE` means the seat would choose or defend the recommendation under the stated constraints, while still naming cost if wrong and flip conditions.

`CONDITIONAL` means the seat recommends the option only if named conditions are satisfied or verified. Conditions must be observable, not “if it works well.”

`WOULD_NOT_STAKE` means the seat would not choose or defend the recommendation under the supplied constraints. It may still describe when the option would become viable.

Do not use numeric confidence percentages.

## Package 2: `architecture.v1`

### Responsibility

`architecture.v1` evaluates system-shape decisions and their consequences for the actual product, platform, team, and operating environment.

### Required lenses

The seat must consider the applicable parts of:

```text
current architecture
product requirements
platform requirements
complexity
maintainability
coupling
dependency impact
operational burden
performance
security
migration
reversibility
scalability
testing implications
failure modes
long-term ownership
```

The current architecture is `NO_REPOSITORY` in greenfield mode, not an invitation to invent one. In repository mode, architectural claims must cite snapshot evidence or identify the evidence gap.

### Tradeoff requirement

Every recommendation names:

- the capability gained;
- the cost introduced;
- the boundary or dependency it creates;
- the failure mode it makes more likely;
- the exit or migration path;
- the boring alternative that was considered.

“It depends” is incomplete. The seat must say what it depends on, which side of the dependency the current constraints occupy, and what fact would flip the recommendation.

### Complexity discipline

Do not recommend services, queues, databases, frameworks, or abstractions merely because they are available or familiar. New machinery needs a named requirement, a measurable benefit, an ownership plan, and an operational cost.

## Package 3: `stack-selection.v1`

### Responsibility

`stack-selection.v1` selects a practical technology stack or candidate option for the named product, not the globally fashionable stack.

### Required lenses

Evaluate the relevant dimensions:

```text
product type
target platform
hosting
deployment
performance
offline requirements
graphics requirements
ecosystem maturity
maintenance
cost
security
observability
testing
vendor lock-in
migration difficulty
AI-assisted development quality
developer experience
long-term support
```

### Required output conclusions

Each seat must provide:

```text
BEST FIT
RUNNER-UP
WHAT WINNER IS BAD AT
DISQUALIFIERS
COST TO LEAVE
MIGRATION PATH
OPERATIONAL COMPLEXITY
BORING ESTABLISHED ALTERNATIVE
```

The candidate set is bounded. In Compare mode it is human-supplied. In Discovery mode it is controller-formed after a thin R0 nomination step.

### Bias controls

The seat explicitly checks for:

- hype bias: popularity or novelty substituted for fit;
- recency bias: newer release treated as inherently better;
- model familiarity bias: the provider recommends what it can explain most fluently;
- incumbent bias: the status quo is protected without testing its cost;
- ecosystem-counting bias: many libraries mistaken for a better product outcome.

## Package 4: `design-taste.v1`

### Responsibility

`design-taste.v1` evaluates visual and interaction intentionality, hierarchy, density, platform fit, originality, and AI-slop risk. Its complete contract is in `DESIGN-TASTE-SPEC.md`.

### Required lenses

At minimum, reason about visual hierarchy, typography, spacing, composition, information density, color discipline, component philosophy, interaction philosophy, platform conventions, originality, asset coherence, distinctive identity, intentionality, and visual polish. For games, add art direction, silhouettes, environments, HUD, lighting, animation, effects, and scene composition.

### Required output behavior

Translate vague taste language into observable criteria. Mark unseen evidence `CANNOT_DETERMINE`. Distinguish deliberate use of a visual pattern from generic template repetition. Do not output a universal numerical beauty score.

## Package 5: `output-position.v1`

### Responsibility

`output-position.v1` turns a seat's reasoning into a bounded, attributable position that can be validated, compared, challenged, revised, and compiled into the final decision record.

### R1 required fields

Conceptually, R1 contains:

```text
recommendation
commitment
claims[]
evidence[]
risks[]
assumptions[]
alternatives[]
flip_condition
cost_if_wrong
reversibility
strongest_counterargument
acceptance_criteria[]
```

Claims are load-bearing and usually limited to 5–7. Evidence identifies exact snapshot paths and line ranges where repository support exists. Greenfield positions explicitly identify product facts versus assumptions.

### R2 required fields

Conceptually, R2 contains:

```text
peer_responses[]
response_type: CONCEDE | DISPUTE | NO_BASIS_TO_JUDGE
response_explanation
new_evidence[]
candidate_revision
```

Every selected peer claim receives a response or an explicit reason it is outside scope. `NO_BASIS_TO_JUDGE` requires missing-evidence explanation.

### R3 required fields

R3 adds:

```text
surviving_claims
withdrawn_claims
remaining_disputes
strongest_counterargument
acceptance_criteria
implementation_constraints
final_recommendation
final_commitment
position_revision_attribution
```

The package records whether the final position changed because of a specific peer claim, new evidence, a corrected assumption, or no material change.

### Validation intent

The controller should be able to reject missing fields, illegal vocabulary, unsupported citations, duplicate claim IDs, unbounded claim lists, and shallow `NO_BASIS_TO_JUDGE` responses before the result enters the decision view.

## Package boundaries

These packages do not define provider transport, process control, snapshot implementation, persistence schema, or UI components. Those are runtime responsibilities. The packages provide reasoning instructions and output expectations that the runtime can enforce or evaluate.
