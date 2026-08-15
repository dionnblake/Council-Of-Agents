# Mock Debate: Strong Minority

> Synthetic fixture. The minority seat has a credible constraint, not a decorative dissent label.

## Metadata

```text
Debate ID: MOCK-003
Question: Should a Windows-first local technical tool use Tauri or Electron?
Mode: COMPARE
Product Type: Windows desktop application
Decision Type: Desktop stack
Seats: Codex WSL, Claude, Antigravity
State: DECISION_RECORDED
```

## R1 positions

### Codex WSL — WOULD_STAKE Tauri

- `C1`: the product needs a small native boundary around filesystem and subprocess control. `INFERENCE` from `docs/boundaries.md:8-25`, `VERIFIED_EXACT`.
- `C2`: Windows-first delivery reduces the immediate value of a bundled Node runtime. `INFERENCE`.
- `C3`: a Rust core can remain outside the renderer. `FACT` from `crates/core/src/lib.rs:1-40`, `VERIFIED_EXACT`.
- Risk: WebView2 and native IPC become operational requirements.

### Claude — WOULD_STAKE Tauri

- `CL1`: Tauri better fits local resource discipline and a small Windows installer. `INFERENCE`.
- `CL2`: Electron's broader Node ecosystem is valuable only if required by actual dependencies. `ASSUMPTION` pending dependency inventory.
- `CL3`: cross-platform expansion is not a V1 hard constraint. `FACT` from question packet.
- Flip condition: required Node-native modules or a near-term macOS/Linux launch.

### Antigravity — CONDITIONAL Electron

- `A1`: the existing prototype uses Node-native process wrappers and assumes npm package access. `VERIFIED_CONTENT_FOUND_ELSEWHERE` at `prototype/package.json:4-12`; cited R1 range was wrong.
- `A2`: Electron reduces migration risk if those wrappers are required without a new IPC boundary. `INFERENCE`.
- `A3`: Tauri is preferable only if the native boundary is deliberately rewritten and tested. `INFERENCE`.
- Risk: Electron footprint and broader ambient surface.
- Flip condition: dependency inventory proves wrappers are replaceable.

## R2 cross-examination

| Claim | Codex WSL | Claude | Antigravity |
|---|---|---|---|
| `A1` Node-native wrappers exist | DISPUTE: cited range is wrong, but content is found elsewhere; inspect before dismissing | CONCEDE: the evidence status matters | CONCEDE: range correction weakens but does not remove claim |
| `C3` Rust core is separate | CONCEDE | CONCEDE | DISPUTE: existing prototype still couples shell code to npm wrappers |
| “Cross-platform is not V1” | CONCEDE | CONCEDE | CONCEDE: if the owner accepts a Windows-first boundary |

## R3 final positions

Codex and Claude retain Tauri. Antigravity revises from `CONDITIONAL Electron` to `CONDITIONAL Tauri` if the native process wrapper inventory is completed and a small IPC spike passes. The minority concern survives as a migration gate.

## Human decision

```text
Action: APPROVE MODIFIED DECISION
Approved option: Tauri, conditional on a native-wrapper inventory and IPC proof.
Minority preserved: Electron remains the fallback if Node-native dependencies are genuinely required.
Flip condition: Any required dependency that cannot cross the approved native boundary safely.
Master prompt: READY WITH TWO ACCEPTANCE GATES
```

## Intended UI/QA use

Render a credible minority panel, shifted citation status, conditional commitment, and decision approval that explicitly preserves the runner-up.
