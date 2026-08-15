# Council of Agents Portfolio Story

## What this project demonstrates

Council of Agents is a local-first technical deliberation product whose engineering story is stronger than “three models answer a question.” The project makes model disagreement, provider boundaries, repository provenance, structured output, and human authority into explicit system behavior.

The story should be told as an evidence-backed sequence. It should not imply that every early assumption was correct or that a certification record proves more than it tested.

## Journey

### 1. Idea

The starting problem was practical: a builder wants multiple serious technical perspectives without manually copying a question, forwarding replies, and losing track of which claim changed. The product idea was a council that deliberates and ends at a human-controlled implementation prompt.

The first boundary was deliberate:

```text
Council decides with the human.
Another harness implements later.
```

### 2. Competitive and workflow research

The concept was compared against coding agents, chat aggregators, autonomous swarms, majority-vote systems, and project tools. Those products optimize for execution, conversation, or coordination. Council needed to optimize for visible reasoning, evidence, dissent, and a safe stop boundary.

### 3. Architecture debate

The architecture debate established several invariants:

- the controller owns turn-taking and state transitions;
- providers receive bounded packets, not hidden conversational context;
- repository evidence comes from a sanitized immutable snapshot;
- output is validated before it becomes a position;
- the human decision is a required gate;
- the final master prompt is a deterministic local artifact.

### 4. Feasibility spike

The feasibility work tested actual Windows process behavior, authentication routes, structured output, snapshots, secret scanning, read-only controls, packet transport, and cancellation. This was important because provider CLI behavior is not interchangeable and because a plausible architecture can fail at the process boundary.

### 5. Assumption failure

The spike found that an accepted assumption about Antigravity's headless CLI path did not hold for one installed version. It also found that native Windows Codex could load ambient user skills and hooks even when expected configuration suppression was used. These were not abstract security concerns; they changed the architecture.

### 6. Windows Codex contamination discovered

Native Windows Codex was observed loading user-level skill and instruction surfaces despite explicit isolation attempts. That meant a Council call could inherit context the packet did not declare. The right response was not another prompt instruction. It was a separate runtime boundary.

### 7. Antigravity IDE versus CLI distinction discovered

The feasibility work distinguished an IDE command that opens or drives a GUI from a standalone headless agent CLI that can accept a packet, return structured output, and report its execution state. Those are different capabilities. The product must certify the actual standalone route, not infer it from an IDE installation or a command named `chat`.

### 8. WSL Codex isolation architecture

The dedicated `CouncilCodexWSL` environment creates a separate Linux identity, `HOME`, Codex home, filesystem boundary, and sandbox. The certified route disables the Windows mount and interop, uses explicit Linux configuration, and keeps the Windows repository outside the distro. WSL is therefore a response to a tested contamination failure, not arbitrary complexity.

### 9. Provider certification

Certification makes provider differences visible:

```text
availability
authentication
schema reliability
repair policy
model reporting
isolation status
packet behavior
process cancellation
```

The project records declared limitations such as a provider not reporting served-model identity. A provider can be useful without being identical to another provider, but Council must not hide the difference.

The evidence set contains a material version/route boundary around Antigravity: one feasibility record reports no headless structured CLI for the installed product, while later certification records carry a certified Antigravity seat. The portfolio story should say that the route was investigated and certified only when the current certification artifact supports it. The application should fail closed and show the limitation when it does not.

### 10. Production build

The production build turns the lessons into a deterministic controller, validation and evidence layer, provider contracts, snapshot and packet boundaries, persistence, a CLI workflow, and a Tauri/React command center. The design goal is not to make the process look magical. It is to make every important boundary inspectable.

## Technical themes

### Multi-agent orchestration

The controller sends the same bounded R1 context to independent seats, then constructs explicit peer packets for R2 and final packets for R3. Provider identities remain attributable. A model does not choose the next turn.

### Deterministic state machines

Debate phases, provider attempts, validation, human decision, and export have explicit states. This supports crash recovery, partial-round handling, repeatable tests, and an honest answer to “what happened?”

### Context engineering

The important context is visible in packets: question, constraints, candidate set, repository evidence, prior positions, peer claims, and output contract. Stateless reconstruction costs more context transmission, but removes hidden provider memory from the decision record.

### Provider abstraction

The abstraction is not “all models are equivalent.” It captures command invocation, authentication category, model request, served-model evidence, schema behavior, repair policy, isolation, scratch paths, failure classes, and cancellation.

### CLI orchestration

Fresh-process provider calls, packet files, stdout/stderr retention, and explicit exit handling make the headless workflow inspectable and testable outside the desktop UI.

### Security boundaries

The system layers input sanitization, secret scanning, reparse-point rejection, immutable snapshots, provider environment allowlists, OS-level write protection, and process cancellation. Each layer addresses a different failure mode.

### Windows process control and WSL isolation

Windows process semantics, `.cmd` shims, ACLs, temp-file writes, junctions, and WSL lifecycle behavior are part of the architecture. The project tests the actual OS boundary instead of assuming a CLI flag is sufficient.

### Repository provenance and evidence verification

Providers do not inspect the live repository. Snapshot manifests, file hashes, line ranges, packet hashes, and citation statuses make evidence traceable. The system distinguishes exact support, content found at another range, and unverified claims.

### Structured output validation

Positions are not accepted because they sound persuasive. Required fields, commitment vocabulary, bounded claims, semantic checks, and citation verification keep the decision record usable.

### Human-in-the-loop governance

The human approves an option, approves a modified decision, requests targeted debate, challenges consensus, or rejects all. The controller records this action. The council cannot turn its own agreement into authority.

### Evaluation design

The benchmark corpus includes architecture, stack, testing, operations, security, and design decisions with both repository and greenfield context. Evaluation separates automatic conformance and timing from human-perceived decision value.

### Skill design

Exactly five reasoning packages separate protocol discipline, architecture tradeoffs, stack selection, design taste, and output shape. This avoids a single undifferentiated “be a good architect” prompt and lets each responsibility be tested.

### Model-selection transparency

Requested and served models are shown as separate facts. If a provider cannot report served identity, the product says so. Transparency is more valuable than a falsely reassuring model badge.

## Architecture explanations

### Why no LLM moderator

The controller owns orchestration because turn-taking, retries, termination, context construction, persistence, and failure handling are control-plane responsibilities. Giving a model authority over those actions makes workflow behavior less reproducible and lets the thing being evaluated change the evaluation protocol.

Models may reason about the question. They do not decide who speaks next or when the council is complete.

### Why no majority vote

Agreement among models is evidence of alignment, not proof of correctness. Models can share an assumption, miss the same evidence, or echo a common pattern. Council preserves the minority position and asks the human to decide.

### Why stateless packets

Stateless reconstruction provides:

```text
reproducibility
visible context
crash recovery
provider uniformity
auditability
no hidden conversational state
```

The tradeoff is higher context transmission and packet management. That cost is accepted because a visible, hashable packet is more trustworthy than a provider session whose hidden state cannot be reconstructed.

### Why Codex uses WSL

Native Windows authenticated Codex continued loading user-level skills and related context despite explicit isolation attempts. The dedicated WSL2 environment creates a separate Linux identity, HOME, Codex home, filesystem boundary, and sandbox. It is the certified boundary because testing proved the native route was not clean enough for Council.

### Why snapshots exist

Agents never inspect the live repository. Snapshots provide:

```text
write isolation
provenance
stable evidence
secret scanning
instruction/config exclusion
repeatability
human trust
```

The snapshot algorithm must inspect every path component for reparse points, because a Windows junction can escape a repository even when a file list looks ordinary.

### Why file-based packets

Antigravity does not reliably accept large stdin prompts, Windows argv has a practical size limit, immutable files provide exact bytes and hashes, and one packet mechanism can be bridged across providers. File packets also make stateless reconstruction and forensic review possible.

### Why provider certification exists

Claude, Codex, and Antigravity expose different command surfaces, output guarantees, model reporting, authentication routes, and failure behavior. Certification records what each provider can actually prove. Council uses those facts to choose a safe run shape instead of pretending every seat is interchangeable.

## Honest limits

This project does not prove that a recommendation is correct. It proves that the recommendation process is bounded, attributable, evidence-aware, and subject to human judgment. It does not make providers independent of all shared training assumptions. It does not eliminate the need for domain expertise, current external research, or human verification of consequential decisions.
