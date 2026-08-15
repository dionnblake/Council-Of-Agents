# V1 Benchmark Intake Set

The synthetic benchmark set lives at `fixtures/benchmark-intakes.json`. It contains the six required comparison/design scenarios plus a true stack-discovery scenario:

1. SQLite vs PostgreSQL
2. Tauri vs Electron
3. REST vs GraphQL
4. Unity vs Godot
5. Native Android vs cross-platform
6. Design direction discovery
7. Stack candidate discovery with no owner-supplied options

These are controller and intake benchmarks, not claims of live provider output. Run the deterministic gate with:

```powershell
node "C:\Users\<USER>\.context\scripts\verify.js" "C:\Users\<USER>\Desktop\VIBE CODING PROJECTS\Council Of Agents"
cargo run -p council-cli -- benchmark fixtures\benchmark-intakes.json
```

The CLI validation command accepts one intake at a time. The desktop path is the authoritative multi-scenario path because it creates a debate, writes immutable packets, and records provider results in SQLite. Live benchmark completion remains gated by authenticated provider availability and the billing safety guard.
