# Stack Selection and Candidate-Set Specification

## Purpose

Stack selection is a first-class Council decision. The goal is not to list technologies. The goal is to identify the best fit for a specific product and make the cost of each choice visible.

## Evaluation contract

Every stack-selection debate starts with:

```text
product type
target platform
users and usage shape
hosting/deployment context
offline or online requirement
performance and graphics needs
team skill and ownership horizon
hard constraints
primary decision priority
```

The council evaluates the same candidate set, constraints, and evidence for each seat. Candidates are not scored by a hidden universal rubric; each dimension is reasoned in context and supported by evidence or marked as an assumption.

## Required dimensions

The position should cover the relevant dimensions below:

- **Product type:** desktop tool, web app, mobile app, game, library, service, or other.
- **Target platform:** Windows, macOS, Linux, Android, iOS, browser, console, or a constrained mix.
- **Hosting and deployment:** local-only, self-hosted, managed cloud, app store, installer, static hosting, or hybrid.
- **Performance:** startup, latency, memory, concurrency, rendering, battery, or throughput expectations.
- **Offline requirements:** offline-first behavior, local persistence, sync, conflict handling, or no offline need.
- **Graphics requirements:** 2D, high-end 3D, GPU access, animation pipeline, or no special graphics.
- **Ecosystem maturity:** package quality, tooling, documentation, community, support, and release stability.
- **Maintenance:** upgrade burden, breaking-change risk, operational ownership, and staffing reality.
- **Cost:** licensing, hosting, tooling, support, build infrastructure, and likely paid dependencies.
- **Security:** attack surface, secrets, identity, sandboxing, update path, and data boundary.
- **Observability:** logs, metrics, traces, crash reporting, diagnostics, and local supportability.
- **Testing:** unit, integration, UI, device, performance, migration, and reproducibility implications.
- **Vendor lock-in:** proprietary runtime, account dependency, hosted data model, or migration friction.
- **Migration difficulty:** data conversion, API rewrite, UI rewrite, asset/toolchain conversion, and dual-run options.
- **AI-assisted development quality:** how reliably the stack can be explained, tested, and maintained with AI help; do not treat model familiarity as correctness.
- **Developer experience:** debugging, local setup, build speed, editor support, and documentation.
- **Long-term support:** release cadence, support horizon, platform policy, and ownership confidence.

## Required conclusions

Every final position must contain:

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

`BEST FIT` is a commitment under current constraints, not a claim that the winner is universally best. `RUNNER-UP` explains the tradeoff that kept it from winning. `WHAT WINNER IS BAD AT` prevents advocacy from hiding costs. `DISQUALIFIERS` names facts that would remove a candidate. `COST TO LEAVE` and `MIGRATION PATH` turn reversibility into a concrete planning concern.

## Compare mode

In `COMPARE` mode, the human provides the candidate set. The controller validates that it is bounded and presents the same ordered list to every seat.

Rules:

- No seat may add a new candidate to R1 as a substitute for evaluating the supplied list.
- A seat may name an outside alternative only in a clearly marked `outside_candidate` note, with no replacement of the required comparison.
- Initial descriptions and constraints are identical across seats.
- The candidate order is stable and must not encode a preferred winner.
- Candidate-specific evidence is either supplied equally or identified as a missing evidence gap.
- The final record distinguishes a human-supplied candidate from a seat-suggested alternative.

Recommended initial candidate limit: 2–5. More than 5 requires an explicit user confirmation because comparison quality degrades and the result becomes framework mush.

## Discovery mode

In `DISCOVERY` mode, the human supplies requirements and constraints, not necessarily candidate options.

### R0 Candidate Discovery

Each seat nominates possible solutions with an extremely brief justification. R0 is not a recommendation round. It should contain only:

```text
candidate name
candidate category
one-sentence fit reason
one disqualifier or concern
```

R0 should not exceed 3 nominations per seat. Nominations may be duplicates.

### Controller candidate union

The controller normalizes names, removes obvious duplicates, and creates a bounded R1 set. When relevant, the set should include:

```text
status quo or simplest viable path
boring established alternative
newer challenger
```

The union limit is 5 candidates by default and 6 only with a recorded reason. If the union exceeds the limit, the controller retains candidates that represent distinct tradeoffs and records omitted nominations in the debate artifact. It does not silently drop a candidate because one provider mentioned it late.

### R1 evaluation

All seats receive the same normalized candidate set and the same product requirements. R1 then follows the normal independent-position protocol. Candidate discovery cannot give one seat an information advantage during evaluation.

## Bias controls

Each seat must explicitly check:

- **Hype bias:** is popularity standing in for fit?
- **Recency bias:** is a newer release being treated as better without a requirement?
- **Model familiarity bias:** is the provider recommending what it can generate fluently?
- **Status-quo bias:** is the current choice protected despite a named problem?
- **Tool-count bias:** are many packages being mistaken for maturity?
- **Migration denial:** is the exit cost omitted because the first choice is attractive?
- **AI optimism:** is a stack recommended because a model can scaffold it quickly while ignoring debugging and ownership?

The bias check must point to a specific claim or decision criterion. It is not a generic disclaimer.

## Evidence rules

Repository mode may use only the sanitized immutable snapshot. A stack claim about the current repository cites the relevant file and line range. Greenfield mode has no repository evidence; the position must distinguish product requirements, external facts, and assumptions.

Evidence quality is not the same as recommendation strength. A recommendation can be conditional because key facts are missing even when the reasoning is coherent.

## Human decision surface

The Decision View shows candidate tradeoffs, winner weaknesses, disqualifiers, exit cost, and the minority position before approval controls. The human may approve a candidate, approve a modified constraint set, continue a targeted evidence round, challenge consensus, or reject all options.

## Example of a useful conclusion

```text
BEST FIT: Tauri for a Windows-first local tool with a small web UI and strict local boundaries.
RUNNER-UP: Electron if cross-platform desktop parity becomes a first-order requirement.
WHAT WINNER IS BAD AT: teams that need a broad Node-native desktop ecosystem or rapid cross-platform plugin reuse.
DISQUALIFIERS: required platform lacks a supported WebView2-equivalent or the app depends on native Node modules that cannot be bridged safely.
COST TO LEAVE: UI can move with moderate effort; native shell and IPC must be rewritten.
MIGRATION PATH: keep domain logic in a platform-neutral core and isolate shell commands behind an interface.
OPERATIONAL COMPLEXITY: low to moderate, with installer and WebView2 support as explicit risks.
BORING ESTABLISHED ALTERNATIVE: Electron remains viable if cross-platform delivery outweighs footprint and native boundary concerns.
```

The example is a format illustration, not a universal answer.
