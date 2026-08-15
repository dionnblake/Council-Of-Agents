# Council Evaluation Framework

## Purpose

This framework evaluates whether Council produces more useful, safer, and more auditable decisions than independent answers alone. It separates mechanical facts from human judgments and reports a vector of evidence instead of inventing one “Council Quality Score.”

## Evaluation principles

- A metric must have a clear unit, denominator, and interpretation.
- Missing evidence is `UNVERIFIED`, not a pass.
- Provider availability and reasoning quality are separate dimensions.
- A repair can make an output usable while still indicating provider/schema weakness.
- R1 and full-council results are compared on the same fixture and input packet.
- Human review asks whether the debate helped a decision, not whether the prose sounded impressive.
- Results retain raw artifacts, validator output, timing, configuration, and fixture identity.

## Evaluation unit

One evaluation unit is:

```text
fixture ID
mode
candidate set
repository snapshot or NO_REPOSITORY
hard constraints
primary priority
selected seats and requested models
round plan
run ID
```

The evaluator freezes the input packet before dispatch. It records provider versions and certification state without treating a provider outage as a reasoning failure.

## Automatically measurable signals

### Schema conformance

`usable_structured_outputs / validated_outputs_attempted` for each round and provider. A usable output satisfies the current validator, required fields, allowed vocabulary, semantic checks, and bounded claim rules. Report raw counts and failure categories such as missing field, illegal commitment, duplicate claim ID, malformed citation, semantic rejection, and truncation.

Do not convert an unavailable provider into a schema failure. Report it separately.

### Citation validity

For repository-grounded claims, report:

```text
exact_valid_citations / citations_submitted
content_found_elsewhere / citations_submitted
unverified_citations / citations_submitted
```

The verifier must distinguish `VERIFIED_EXACT`, `VERIFIED_CONTENT_FOUND_ELSEWHERE`, and `UNVERIFIED`. Greenfield claims do not receive a fake repository citation score; they are evaluated for correct absence and clear assumption labels.

### Repair rate

`outputs_requiring_repair / outputs_requiring_validation` and `repair_successes / repair_attempts`.

Report whether repair was attempted, why, how many attempts were allowed, and whether the repaired result was independently validated. A high repair-success rate does not erase the original provider failure.

### Provider failure rate

`provider attempts ending in timeout, auth failure, quota limit, malformed transport, process failure, unknown failure, or cancellation / provider attempts started`.

Report by provider and failure class. Cancellation is shown separately from accidental failure.

### Wall-clock time

Record per provider and per round:

```text
queue/start delay
process spawn time
provider execution time
validation time
repair time
round completion time
full debate elapsed time
```

Use medians and a small percentile summary across repeated deterministic fixture runs. Do not compare a live provider outage to a normal run as if it were a speed result.

### Claim count

Report claims per seat and per round. Flag positions outside the expected 5–7 load-bearing opening claims. The signal detects verbosity and scope drift; it does not reward a higher count.

### Peer-claim response rate

`peer claims receiving a valid R2 response / peer claims selected for response`.

A valid response names `CONCEDE`, `DISPUTE`, or `NO_BASIS_TO_JUDGE` and includes the required explanation. Repeating the claim without engaging it is not a response.

### Response-type distribution

Report separate rates for:

```text
CONCEDE responses / valid peer responses
DISPUTE responses / valid peer responses
NO_BASIS_TO_JUDGE responses / valid peer responses
```

Do not interpret a high `CONCEDE` rate as quality or a high `DISPUTE` rate as rigor. Use the human review and evidence validity to judge usefulness. Track `NO_BASIS_TO_JUDGE` explanations for shallow abstention.

### Attributed revision rate

`R3 positions with a named revision linked to a peer claim, evidence item, or corrected assumption / R3 positions`.

A position that does not change can still be valid, but it must explain why. This metric tests whether cross-examination changes reasoning rather than merely generating more text.

### R1/R3 recommendation change

Report:

```text
recommendations changed after R2/R3
commitments changed after R2/R3
claims withdrawn or materially revised
flip conditions added or changed
```

Use exact normalized comparison plus human inspection for semantic changes. Do not treat wording cleanup as a recommendation change.

### Duplicate argument rate

Cluster normalized claim statements across seats and report:

`claims that duplicate another seat's claim without distinct evidence or reasoning / all opening claims`.

The signal is diagnostic. Some agreement is expected; duplicates with independent evidence are not automatically bad. The evaluator should also report distinct load-bearing perspectives retained.

### Packet size

Record bytes and token estimate for each input packet, separated into question, constraints, repository evidence, prior positions, peer claims, and instructions. Track packet size against transport limits and provider context behavior. A smaller packet is not automatically better if it removes decision-critical evidence.

### Provider availability

Report each preflight state:

```text
installed
authenticated
certified
available
requested model
served-model status
declared limitations
```

Use this as a readiness vector, not a percentage score. A provider that does not report served identity is `PROVIDER_DOES_NOT_REPORT`, not a verified match or automatic failure.

### State and export determinism

Run the same approved state twice and compare normalized prompt bytes, ordering, included claims, citations, and metadata rules. Report exact match or the first divergence. Timestamps and run IDs must be explicitly excluded or placed in a declared metadata block if they are not meant to be deterministic.

## Human-reviewed signals

Use a short form after R1 and after R3. Each item can be answered `Yes`, `Partly`, or `No`, followed by one sentence of evidence.

### Decision usefulness

- Did Council surface a material consideration you had not identified?
- Did cross-examination add something beyond the independent answers?
- Did the Council change your decision?
- Did the Council change why you made the decision?
- Was the preserved minority position useful?
- Was the highest-impact unresolved issue clear?
- Were the flip conditions actionable rather than vague?
- Was the final master prompt implementation-ready for a separate human-controlled coding step?
- Would you use Council for another decision of similar importance?

### Evidence and trust

- Could you tell which statements were facts, inferences, assumptions, or unknowns?
- Could you verify the important repository claims without reading every provider transcript?
- Did the UI make provider limitations and missing evidence visible without creating false alarm?
- Did the human decision remain clearly yours rather than a model vote?

### Design and usability

- Could you identify the current question and round state immediately?
- Could you find a disputed claim and its evidence in two actions or fewer?
- Were loading, partial, failure, and empty states understandable?
- Could you complete review and export using the keyboard?
- Did the interface feel like a technical deliberation tool rather than a generic AI dashboard?

## R1 versus full-council evaluation

The purpose of this comparison is to measure whether debate adds value beyond three independent answers.

### R1 checkpoint

After R1, before showing peer responses, ask the human:

```text
Would you make a decision now?
What would you choose?
What considerations matter most?
What evidence is still missing?
```

Record the answer without coaching. The human may choose “not enough information.”

### Full-council checkpoint

After R2/R3 and before or after the human decision, ask:

```text
Did your decision change?
Did your reasoning change?
What new information or framing appeared?
Was cross-examination worth the additional time?
Which minority or dissenting point mattered?
```

Compare the R1 and final responses for:

- option change;
- priority or rationale change;
- newly identified risk or evidence gap;
- changed commitment or flip condition;
- human-perceived value of debate;
- additional time and provider cost.

### Controlled comparison variants

For benchmark batches, use:

1. `R1_ONLY`: independent positions and a human checkpoint.
2. `FULL_COUNCIL`: R1, R2, R3, human checkpoint.
3. Optional `SINGLE_BEST`: one seat under the same input packet, only when useful as a baseline.

Keep fixture order, candidate set, and human instructions stable. Rotate order when repeated exposure could bias the result.

## Negative controls and mutation tests

The evaluator should include cases that deliberately:

- shift a citation one line away;
- cite a real file with a nonexistent range;
- remove a required output field;
- introduce a duplicate claim ID;
- return an illegal commitment;
- hide a provider response during R1;
- inject a secret or reparse point into a snapshot candidate;
- interrupt one seat after another seat completes;
- change only a timestamp before deterministic export.

Each mutation must produce a named failure or a documented invariant. A green test that does not detect the mutation is a coverage gap.

## Evidence package

Each run retains:

```text
fixture and input packet hash
selected providers and requested models
preflight state
round state transitions
raw provider artifacts
validation and repair records
citations and verification results
timings
human review form
export hash
failure or cancellation evidence
```

## Reporting format

Report each fixture as a compact evidence table plus human notes. Include provider availability and limitations first, then automatic signals, then human observations, then residual risks. Do not headline a single score. A release summary may state a decision such as `PASS_WITH_DECLARED_LIMITATIONS` only when every blocking gate and its evidence are explicit.
