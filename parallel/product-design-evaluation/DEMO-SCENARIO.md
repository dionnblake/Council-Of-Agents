# Portfolio Demo Scenario: Electron or Tauri?

## Purpose

This is the primary portfolio demonstration for Council. It shows the product's actual differentiator: independent positions, evidence, challenge, revision, preserved minority reasoning, human authority, and a deterministic stop.

The demo must be honest about whether provider calls are live or synthetic. If a provider is unavailable, use the corresponding mock debate and label it `SYNTHETIC DEMO`; never present a staged fixture as live certification.

## Question

```text
Should this Windows-first local technical workstation use Electron or Tauri?
```

Context:

- Windows-first launch.
- React-capable desktop UI.
- Local filesystem and subprocess control.
- Existing subscriptions only; no normal API billing.
- Human manually copies the implementation prompt later.
- Small team and safety-sensitive local boundary.

## Demonstration setup

Use a repository snapshot containing:

```text
crates/core/src/lib.rs
src-tauri/src/commands.rs
prototype/package.json
docs/boundaries.md
```

The synthetic evidence deliberately contains a Node-native wrapper in `prototype/package.json` and a separate Rust core boundary. All paths are synthetic and hashes are recorded.

## Storyboard

| Stage | Visible moment | Product point |
|---|---|---|
| 1. Home | Click `New Debate` | Council starts from a decision, not a chat box |
| 2. Intake | Enter question, constraints, Compare, Electron/Tauri | Candidate set is bounded and shared |
| 3. Preflight | Three provider rows and model-status distinction | Seats are certified independently |
| 4. R1 | Three positions appear without peer visibility | Independence is visible |
| 5. R1 comparison | Codex/Claude favor Tauri; Antigravity favors Electron conditionally | Difference is attributable, not voted |
| 6. Evidence | Open Node-wrapper citation | Evidence matters more than rhetoric |
| 7. R2 | Claude challenges Codex's “no Node dependency” assumption | Cross-examination adds value |
| 8. R3 | Codex revises to conditional Tauri; Antigravity preserves Electron minority | Belief revision and dissent are visible |
| 9. Decision | Human approves Tauri with IPC gate | Human authority is explicit |
| 10. Master Prompt | Prompt shows constraints, evidence, dissent, acceptance | Final output is useful and traceable |
| 11. Stop | Copy only; no provider-send/implement action | Governance boundary is real |

## Planned R1 positions

### Codex WSL

`WOULD_STAKE Tauri` because the Rust core and local boundary fit the Windows-first safety goal. Initial assumption: no required Node-native dependency.

### Claude

`WOULD_STAKE Tauri` with a stronger emphasis on lower ambient surface and a replaceable shell. It asks for a dependency inventory before treating the choice as final.

### Antigravity

`CONDITIONAL Electron` because the prototype includes Node-native process wrappers and Electron lowers immediate migration cost. It names footprint and ambient-tool surface as costs.

## Evidence reveal and challenge

Open the exact synthetic `prototype/package.json` lines. Claude challenges Codex: “The snapshot does contain a Node-native wrapper. Why is Tauri unconditional?”

Codex responds:

```text
CONCEDE: the wrapper exists in the prototype.
DISPUTE: its presence does not prove it is required for the product boundary.
REVISION: Tauri becomes CONDITIONAL on replacing the wrapper with a tested native command boundary.
```

Antigravity's Electron concern survives as the minority position and flip condition.

## Final human decision

```text
Action: APPROVE MODIFIED DECISION
Approved: Tauri + Rust core + React.
Conditions: inventory Node-native dependencies; pass IPC, cancellation, and packaging tests.
Minority preserved: Electron is the fallback if a required native wrapper cannot be replaced safely.
Master prompt: READY WITH ACCEPTANCE GATES.
```

## Demo narration rules

- Say “the seats” or provider names, not “three AIs voted.”
- Point to claim IDs and evidence status before quoting prose.
- Show the revision attribution rather than claiming the model “learned.”
- Show the human action and the explicit stop boundary.
- Say when a result is synthetic, replayed, or live.

## Demo acceptance

- Three independent R1 opinions are visible.
- Claude's challenge addresses a Codex assumption.
- At least one exact evidence item is opened.
- One position changes in R3 with attribution.
- Antigravity's minority concern remains visible.
- Human approval is required and recorded.
- Master prompt includes evidence, conditions, dissent, and acceptance gates.
- Copy does not launch a provider or implementation process.
