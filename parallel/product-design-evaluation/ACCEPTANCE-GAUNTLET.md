# Council of Agents V1 Acceptance Gauntlet

## Purpose

This is the completion gate for the primary engineering build. It is a manual plus automated sequence. It assumes nothing from a passing unit test or a polished screenshot. Every pass requires retained evidence.

The gauntlet validates product boundary, provider certification, safety, debate semantics, deterministic handoff, failure recovery, and visual usability.

## Acceptance rules

- Run from a clean, identified checkout and record the commit or workspace state.
- Use synthetic fixtures for destructive, secret, reparse-point, and provider-isolation tests.
- Do not use a real secret or a real production repository as a test fixture.
- Mark a check `PASS`, `FAIL`, `UNVERIFIED`, or `NOT_APPLICABLE` with reason. `UNVERIFIED` is not a pass.
- A provider limitation may be accepted only when it is declared, visible, and compatible with the chosen test.
- A failed required gate blocks a `V1 VERIFIED` claim.

## Gate 0: Test identity and baseline

1. Record OS, runtime versions, app build identifier, selected providers, requested models, and certification status.
2. Confirm the test checkout and worktree are the intended ones.
3. Confirm no real repository, credential, API key, token, or account identifier is used in fixtures or screenshots.
4. Capture baseline provider health and application persistence state.
5. Record the verifier command and raw output location.

## Gate 1: Provider acceptance

### Claude

- Confirm installed binary and version.
- Confirm authenticated status without printing credential values.
- Run certification and record `Installed`, `Authenticated`, `Certified`, and `Available` separately.
- Select an explicit Claude model and confirm the request reaches the provider.
- Confirm served-model verification when the provider reports it; otherwise show `PROVIDER_DOES_NOT_REPORT` and keep the limitation visible.
- Send a structured-output fixture and validate schema plus semantic fields.
- Simulate or use a safe quota-limit fixture and confirm the UI pauses with a repair/continue path.

### Antigravity

- Confirm the standalone authenticated CLI path, not an IDE window or GUI chat command.
- Confirm installed version, authentication, certification, and availability separately.
- Select an explicit requested model and retain the request in the artifact.
- Confirm the credit guard blocks unapproved fallback or reports the declared limitation.
- Feed malformed structured output through the repair path and confirm the repair policy is explicit, bounded, and independently validated.
- If the installed product cannot provide a headless structured route, mark the seat unavailable or limited. Do not simulate a passing CLI with an IDE command.

### Codex WSL

- Confirm the dedicated `CouncilCodexWSL` distro, Linux identity, `HOME`, `CODEX_HOME`, and Codex version.
- Confirm ChatGPT/subscription authentication without copying or printing credentials.
- Confirm no Windows mount, Windows interop, inherited Windows PATH, or Windows skill tree is reachable.
- Select an explicit requested model and report whether served identity is verified or provider-does-not-report.
- Confirm read-only evidence access and writable scratch separation.
- Run a safe WSL restart and recheck authentication, boundary settings, and absence of lingering processes.

### Provider acceptance evidence

Retain per-seat raw status, stdout/stderr, version, requested model, served-model state, limitation, and timing. A single “provider healthy” label is insufficient.

## Gate 2: Safety acceptance

Use a synthetic repository fixture and a harmless provider command where possible.

The detailed threat model, injection corpus, and safety cases are in `security/THREAT-MODEL.md`, `security/PROMPT-INJECTION-CORPUS.md`, and `security/SAFETY-TEST-CASES.md`.

### Repository and snapshot boundary

- Prove the real repository is unchanged before and after a complete debate attempt.
- Confirm no `.git` directory is present in the provider snapshot.
- Plant a junction and a symlink/reparse point in the fixture; confirm every path component is checked and the escape is blocked.
- Plant synthetic secrets by filename and content; confirm the secret scan blocks dispatch and creates a human-review event.
- Confirm `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, provider config, hooks, MCP config, and equivalent instruction surfaces are stripped or explicitly handled by contract.
- Confirm the Windows snapshot is read-only to the provider process, including temp-file and rename attempts.
- Confirm WSL snapshot copy hashes match the Windows manifest and bytes remain unchanged.
- Confirm provider scratch directories are separate from sealed evidence and packet directories.

### Codex isolation

- Confirm Codex cannot access the Windows skill tree.
- Confirm `/mnt/c` and Windows interop remain disabled in the dedicated distro.
- Confirm the clean-room call does not contain Windows user context, unrelated instructions, hooks, plugins, or MCP content.

### Process safety

- Start a harmless long-running provider fixture and cancel it.
- Confirm the process tree is terminated, no child remains, and scratch cleanup is bounded.
- Confirm a provider failure cannot trigger an unapproved alternate billing path.
- Confirm logs redact secrets and credential material.

## Gate 3: Debate semantics

### R1 independence

- Run the same R1 question, constraints, evidence packet, and candidate set for all seats.
- Prove no seat receives another seat's R1 output, claim IDs, or position.
- Prove R1 outputs have controller-assigned claim IDs.
- Confirm opening positions contain bounded load-bearing claims, evidence/assumptions, risks, flip condition, cost if wrong, reversibility, and commitment.

### R2 engagement

- Provide a peer packet containing at least one supported claim, one weak claim, and one claim outside available evidence.
- Confirm responses use only `CONCEDE`, `DISPUTE`, or `NO_BASIS_TO_JUDGE`.
- Confirm `NO_BASIS_TO_JUDGE` includes what evidence is missing and why judgment is not responsible.
- Confirm `CONCEDE` and `DISPUTE` address the actual claim rather than restating R1.
- Confirm provider failure is not interpreted as agreement, concession, or abstention.

### R3 final position

- Confirm R3 includes surviving claims, withdrawn claims, remaining disputes, strongest counterargument, acceptance criteria, implementation constraints, final recommendation, and commitment.
- Confirm every changed position has revision attribution or explicitly states why it did not change.
- Confirm dissent and minority reasoning remain visible after the human decision.
- Confirm flip conditions, cost-if-wrong, reversibility, and evidence quality remain in the Decision View.
- Confirm there is no vote count, winner-by-majority label, or automatic consensus authority.

### Human decision

- Confirm no final decision exists before a human action.
- Exercise `Approve Option`, `Approve Modified Decision`, `Continue Targeted Debate`, `Challenge Consensus`, and `Reject All` on separate fixtures.
- Confirm the action and rationale are persisted and attributable to the human.

## Gate 4: Master prompt acceptance

- Run the same approved state twice and compare deterministic prompt content and ordering.
- Confirm every important requirement in the prompt traces to a human-approved decision, Council claim, evidence reference, constraint, or explicitly labelled assumption.
- Confirm evidence paths and line ranges are preserved or normalized without changing meaning.
- Confirm the prompt contains dissent, risks, flip conditions, acceptance criteria, and implementation constraints where present.
- Confirm no implementation starts automatically.
- Confirm manual copy works from the desktop UI and produces the expected bytes.
- Confirm local save/export works and records path, hash, and source debate ID.
- Confirm no provider launch button exists anywhere in the final prompt flow.
- Confirm export failure does not present a partial artifact as final.

## Gate 5: Failure acceptance

Use `PROVIDER-FAILURE-MATRIX.md` as the authoritative detection, message, retry, state, degraded-mode, and audit contract.

Exercise each failure with a safe fixture or recorded provider simulation:

- Claude quota limit.
- Antigravity malformed structured output.
- Codex WSL unavailable.
- Provider timeout.
- Provider authentication expired.
- Unknown provider failure.
- Partial round completion.
- Application crash and restart.
- User cancellation.
- Secret discovered in candidate snapshot.
- Evidence verification failure.

For every failure confirm:

```text
failure is named
failure is attributable
partial output is not promoted to final
retry/repair policy is bounded
user can recover or cancel
no hidden provider switch occurs
audit evidence is retained
```

## Gate 6: Design acceptance

Use the visual smoke-test checklist at a supported Windows resolution and at 125% scaling. Test both light and dark themes if supported.

### Hierarchy and composition

- The question, current round, unresolved issue, and human decision state outrank provider transcripts.
- The app reads as a technical deliberation command center, not a generic AI dashboard.
- There is no full-screen gradient, glowing AI orb, marketing hero, or rainbow provider wall.
- Not every information group is a rounded card; dense comparisons use rows, dividers, or tables where appropriate.
- At least three deliberate typography levels are visible.
- Code and evidence have a readable monospaced treatment.

### Status and evidence

- Provider identity and limitation are clear without color alone.
- Requested model and served-model state are visibly separate.
- `VERIFIED_EXACT`, `VERIFIED_CONTENT_FOUND_ELSEWHERE`, and `UNVERIFIED` are distinct in text and visual treatment.
- Evidence line ranges, file paths, and claim relationships can be read without opening raw transcripts.

### Interaction and state polish

- Empty, loading, partial, failure, cancellation, disabled, and success states are specific to the operation.
- Loading does not replace the entire work surface with a generic spinner.
- Error actions are recoverable and do not imply silent retry.
- Decision actions are identifiable and cannot be confused with implementation commands.
- Keyboard focus, copy, close, navigation, and scaling work at 100% and 125%.
- Reduced motion and high-contrast-friendly treatments remain understandable.

### Design-taste review

The reviewer records `KEEP`, `REVISE`, `REMOVE`, or `CANNOT_DETERMINE` findings. “Looks good” is not a criterion. If visual evidence is missing, the reviewer says `CANNOT_DETERMINE` and names the missing state or screen.

## Gate 7: State, fixture, and artifact acceptance

- Exercise every state in `UI-STATE-CATALOG.md` with a deterministic mock or safe failure fixture.
- Render all ten files under `fixtures/mock-debates/` without provider calls.
- Confirm the mock debates show claims, evidence, disagreements, repairs/failures, decisions, degraded mode, and prompt readiness accurately.
- Validate the required sections and traceability rules in `MASTER-PROMPT-QUALITY-SPEC.md` against at least five gold examples.
- Create and reopen a `DECISION-RECORD-SPEC.md` record without provider availability.
- Walk through `ONBOARDING-SPEC.md` diagnostics, including Codex WSL isolation states.
- Replay `DEMO-SCENARIO.md` with live or clearly labelled synthetic inputs.
- Confirm UI copy follows `PRODUCT-LANGUAGE.md` and does not use execution or employee metaphors.

## Gate 8: Documentation and DOX

- Confirm product scope still says Council advises and the human decides.
- Confirm exactly five V1 reasoning packages are documented.
- Confirm benchmark corpus has at least 25 fixtures, at least five greenfield fixtures, and at least five repository-grounded fixture descriptions.
- Confirm evaluation distinguishes automatic metrics from human review and does not rely on a fake aggregate score.
- Confirm the final report names unverified items and provider limitations.
- Confirm the nearest `AGENTS.md` chain describes any durable new subtree.

## Release verdict

Use one of:

```text
V1 VERIFIED
V1 VERIFIED WITH DECLARED LIMITATIONS
V1 UNVERIFIED
V1 FAILED
```

`V1 VERIFIED` requires all blocking gates to pass and no material evidence gap. `V1 VERIFIED WITH DECLARED LIMITATIONS` requires all safety and governance gates to pass, with limitations explicitly visible and accepted. `UNVERIFIED` means implementation exists but a required test did not run. `FAILED` means a required invariant did not hold.

The final report must link each verdict to retained evidence. Never infer acceptance from a build passing, a screenshot looking polished, or a provider returning one successful response.
