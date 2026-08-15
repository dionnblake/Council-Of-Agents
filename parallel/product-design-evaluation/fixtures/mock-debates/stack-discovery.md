# Mock Debate: Stack Discovery

> Synthetic fixture. R0 nominations are brief; R1 evaluates the same bounded union.

## Metadata

```text
Debate ID: MOCK-008
Question: What stack should a small team choose for a local-first Windows technical workstation?
Mode: DISCOVERY
Product Type: Greenfield desktop application
Decision Type: Full stack discovery
Seats: Codex WSL, Claude, Antigravity
State: DECISION_RECORDED
```

## R0 candidate discovery

| Seat | Candidate | Brief reason | Concern |
|---|---|---|---|
| Codex WSL | Tauri + Rust core + React | Local shell and native core fit | WebView2 and IPC need proof |
| Codex WSL | Electron | Fast React desktop path | Larger ambient/runtime surface |
| Claude | Tauri + Rust core + React | Small local boundary | Native integration learning curve |
| Claude | Electron | Mature Node ecosystem | Footprint and isolation burden |
| Antigravity | Tauri + Rust core + React | Windows-first local control | Provider route limitation must be certified |
| Antigravity | .NET desktop shell | Strong Windows integration | React reuse and cross-platform story weaker |

## Controller union

```text
Candidate set for R1 (bounded to four):
1. Tauri + Rust core + React
2. Electron + React
3. .NET desktop shell + web UI
4. Status quo: a native Windows-only shell with minimal web UI
```

The union includes the status quo, a boring established alternative, and a newer challenger. No late framework list is added.

## R1 positions

- Codex WSL: `WOULD_STAKE` Tauri, with Rust core and explicit IPC boundary.
- Claude: `CONDITIONAL` Tauri, with Electron as runner-up if Node-native dependencies are proven.
- Antigravity: `CONDITIONAL` Tauri, but asks for current provider certification before relying on the third seat.

Shared load-bearing claims: local process control, inspectable state, small team, and no cloud control plane. The seats disagree on how much ecosystem maturity justifies Electron.

## R2/R3

All seats concede that the candidate union is bounded correctly. Claude disputes the assumption that cross-platform expansion is near-term. Codex adds an acceptance criterion for short workspace paths and process cancellation. Antigravity preserves the provider-certification concern as an operational constraint, not a stack winner.

Final positions:

```text
BEST FIT: Tauri + Rust core + React
RUNNER-UP: Electron + React
WHAT WINNER IS BAD AT: Node-native ecosystem breadth and zero-cost cross-platform reuse
COST TO LEAVE: moderate shell/IPC rewrite; core can remain portable
FLIP CONDITION: required dependency cannot cross the native boundary safely
```

## Human decision

```text
Action: APPROVE OPTION
Approved option: Tauri + Rust core + React
Conditions: IPC spike, provider boundary certification, cancellation, and short-path packaging tests.
Master prompt: READY
```

## Intended UI/QA use

Render the R0-to-R1 transition, bounded candidate union, discovery rationale, runner-up, and explicit stack-selection output fields.
