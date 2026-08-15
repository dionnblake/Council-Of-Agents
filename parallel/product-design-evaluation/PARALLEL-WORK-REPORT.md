# Parallel Work Report

## Work completed

Created the product, design, reasoning-skill, stack-selection, benchmark, security, mock-debate, evaluation, acceptance, accessibility, performance, prompt-quality, decision-record, onboarding, demo, portfolio, and engineering handoff package for Council of Agents in a separate worktree and dedicated branch. The package was subsequently merged into `main` under `parallel/product-design-evaluation/`.

No production runtime code, schemas, provider adapters, UI source, persistence files, or primary-agent files were edited.

## Verification status

- **PASS:** all 12 required deliverable files exist.
- **PASS:** expanded package contains security threat/injection/safety documents, provider failure matrix, state catalog, accessibility/keyboard specification, performance budgets, master-prompt quality specification, decision-record specification, onboarding, demo, product language, and release gate.
- **PASS:** ten complete synthetic mock debates exist under `fixtures/mock-debates/` and are provider-free.
- **PASS:** all nine specified UI screens contain purpose, information, controls, navigation, empty, loading, failure, keyboard, and hierarchy sections.
- **PASS:** benchmark corpus contains 25 complete fixtures, 11 repository-grounded fixtures, and 14 no-repository fixtures; all repository fixtures describe evidence characteristics.
- **PASS:** every required benchmark topic and both Compare/Discovery paths are represented.
- **PASS:** all staged paths are inside `parallel/product-design-evaluation/`.
- **PASS:** `git diff --cached --check` reported no whitespace errors.
- **UNAVAILABLE:** `markdownlint` is not installed in the isolated worktree.
- **UNVERIFIED:** the global project verifier found no `package.json`, `Cargo.toml`, or `pyproject.toml` in this documentation-only orphan worktree, so no application runtime tests ran. This is not a claim about the primary production checkout.

## Files created

```text
parallel/product-design-evaluation/AGENTS.md
parallel/product-design-evaluation/PRODUCT-SPEC.md
parallel/product-design-evaluation/UI-UX-SPEC.md
parallel/product-design-evaluation/DESIGN-TASTE-SPEC.md
parallel/product-design-evaluation/SKILLS-SPEC.md
parallel/product-design-evaluation/STACK-SELECTION-SPEC.md
parallel/product-design-evaluation/BENCHMARK-CORPUS.md
parallel/product-design-evaluation/EVALUATION-FRAMEWORK.md
parallel/product-design-evaluation/ACCEPTANCE-GAUNTLET.md
parallel/product-design-evaluation/PORTFOLIO-STORY.md
parallel/product-design-evaluation/ENGINEERING-HANDOFF.md
parallel/product-design-evaluation/PARALLEL-WORK-REPORT.md
parallel/product-design-evaluation/security/AGENTS.md
parallel/product-design-evaluation/security/THREAT-MODEL.md
parallel/product-design-evaluation/security/PROMPT-INJECTION-CORPUS.md
parallel/product-design-evaluation/security/SAFETY-TEST-CASES.md
parallel/product-design-evaluation/PROVIDER-FAILURE-MATRIX.md
parallel/product-design-evaluation/UI-STATE-CATALOG.md
parallel/product-design-evaluation/ACCESSIBILITY-KEYBOARD-SPEC.md
parallel/product-design-evaluation/PERFORMANCE-BUDGETS.md
parallel/product-design-evaluation/MASTER-PROMPT-QUALITY-SPEC.md
parallel/product-design-evaluation/DECISION-RECORD-SPEC.md
parallel/product-design-evaluation/ONBOARDING-SPEC.md
parallel/product-design-evaluation/DEMO-SCENARIO.md
parallel/product-design-evaluation/PRODUCT-LANGUAGE.md
parallel/product-design-evaluation/RELEASE-GATE.md
parallel/product-design-evaluation/fixtures/AGENTS.md
parallel/product-design-evaluation/fixtures/mock-debates/AGENTS.md
parallel/product-design-evaluation/fixtures/mock-debates/unanimous-but-correct.md
parallel/product-design-evaluation/fixtures/mock-debates/unanimous-but-suspicious.md
parallel/product-design-evaluation/fixtures/mock-debates/strong-minority.md
parallel/product-design-evaluation/fixtures/mock-debates/evidence-changes-position.md
parallel/product-design-evaluation/fixtures/mock-debates/fake-citation.md
parallel/product-design-evaluation/fixtures/mock-debates/no-basis.md
parallel/product-design-evaluation/fixtures/mock-debates/provider-timeout.md
parallel/product-design-evaluation/fixtures/mock-debates/stack-discovery.md
parallel/product-design-evaluation/fixtures/mock-debates/design-debate.md
parallel/product-design-evaluation/fixtures/mock-debates/degraded-two-seat.md
```

## Major product decisions

- Council is a local-first technical deliberation product, not a coding or execution agent.
- Security is fail-closed: untrusted content may be evidence but never controller authority, and provider failure never becomes agreement.
- Provider failure behavior is explicit across detection, user message, retry, state, degraded continuation, and audit event.
- Mock debates are first-class UI/QA inputs and are clearly synthetic rather than live certification evidence.
- The human remains the final authority; no majority vote or model moderator exists.
- The product flow ends at a deterministic master prompt and manual copy.
- R1 is independent, R2 is explicit cross-examination, and R3 preserves revision and dissent.
- Compare mode uses a human-supplied bounded candidate set. Discovery mode uses a thin R0 and a bounded controller-formed union.
- Two-seat operation is explicit and limitation-aware; it is never disguised as a three-seat consensus.
- Requested and served model identity are separate facts.

## Major design decisions

- The primary Debate View is a claim/evidence relationship surface with a provider status rail, not three giant chat columns.
- The question, round, unresolved issue, and human decision outrank transcript detail.
- Dense technical information uses rows, tables, dividers, and progressive disclosure; cards have a bounded purpose.
- Typography, code treatment, evidence status, keyboard behavior, and Windows scaling are specified as acceptance concerns.
- Loading, partial, failure, cancellation, empty, disabled, and success states are part of the design contract.
- AI-slop signals are treated as diagnostic evidence, not absolute visual prohibitions.

## Skill specification decisions

- V1 contains exactly five packages: `protocol.v1`, `architecture.v1`, `stack-selection.v1`, `design-taste.v1`, and `output-position.v1`.
- Protocol limits opening positions to approximately 5–7 load-bearing claims and requires fact/inference/assumption discipline, steelmanning, abstention, revision, cost-if-wrong, reversibility, and flip conditions.
- Peer responses use exactly `CONCEDE`, `DISPUTE`, and `NO_BASIS_TO_JUDGE`, with an explanation required for abstention.
- Commitment uses exactly `WOULD_STAKE`, `CONDITIONAL`, and `WOULD_NOT_STAKE`; numeric confidence is excluded.
- Stack selection requires winner weakness, runner-up, disqualifiers, cost to leave, migration path, operational complexity, and a boring established alternative.
- Design taste outputs concrete criteria and `CANNOT_DETERMINE` when visual evidence is missing; it does not emit a fake universal beauty score.

## Benchmark corpus summary

- 25 realistic fixtures across architecture, data, desktop, mobile, games, design, security, testing, operations, hosting, and greenfield discovery.
- More than five greenfield/no-repository cases.
- More than five repository-grounded cases with described fixture characteristics.
- Compare mode and Discovery mode are both exercised.
- Fixtures intentionally include contextual decisions without a universal answer key.

## Evaluation framework summary

- Automatic metrics are individually defined for schema conformance, citation validity, repair, provider failures, timing, claim count, peer response, response types, revision, recommendation change, duplicate arguments, packet size, availability, and deterministic export.
- Human review asks whether Council surfaced useful considerations, changed the decision or rationale, preserved useful dissent, and produced a usable handoff.
- R1-only and full-council checkpoints measure whether debate adds value beyond independent answers.
- Negative controls catch malformed output, citation shifts, hidden peer visibility, unsafe snapshots, and nondeterministic exports.
- No single aggregate Council quality score is used.
- Security evaluation includes 18 threat classes, 20 prompt-injection corpus cases, and 30 safety test cases.
- Master-prompt quality requires traceability, preserved dissent, explicit conditions, and a manual-copy stop boundary.

## Acceptance-test summary

The Acceptance Gauntlet now covers provider certification, model selection, served-model limitations, WSL isolation, snapshot and packet safety, secret/reparse controls, prompt injection, debate independence, peer response semantics, human decision authority, deterministic master prompt output, failure recovery, complete UI state coverage, mock-debate rendering, keyboard/accessibility behavior, performance budgets, onboarding, packaging, fresh-machine, and design-specific smoke tests.

## Engineering handoff items

The handoff separates `MUST IMPLEMENT`, `SHOULD IMPLEMENT`, `DESIGN REQUIREMENT`, `EVALUATION REQUIREMENT`, `OPEN QUESTION`, and `DO NOT IMPLEMENT`. The most important open item is the current Antigravity route/version evidence conflict; the installed route must be recertified rather than inferred from an IDE command.

## Potential merge conflicts

- The primary agent may add or change root documentation, `skills/`, `schemas/`, `fixtures/`, `app/`, or `docs/` while consuming this package. This branch intentionally does not touch those paths.
- The root `AGENTS.md` Child DOX Index was updated when the parallel package was merged.
- Production skill files and schemas may already encode parts of the conceptual fields here. Align them with the handoff during merge; do not create competing copies.
- UI implementation may have stable layout contracts that require translating the UX spec into incremental changes rather than replacing the shell.

## DOX updates completed at merge

- Added the durable `parallel/product-design-evaluation/` boundary to the parent DOX index.
- Preserved the local `AGENTS.md` and child indexes.
- No package move or split occurred.

## Open questions

- Current Antigravity standalone headless CLI availability and certification state.
- Whether high-stakes debate types require three certified seats.
- Exact production schema alignment for the conceptual R1/R2/R3 fields.
- Whether existing UI IPC contracts can support a claim relationship view incrementally.
- Which model identity statuses are provider-verifiable in the current installed versions.

## Recommended merge order

1. Review `ENGINEERING-HANDOFF.md` against the primary architecture and resolve contradictions.
2. Merge the documentation subtree without touching production paths.
3. Align existing production skill and schema contracts with the five-package and output-position requirements.
4. Add executable benchmark/safety harnesses using the synthetic fixtures and prompt corpus.
5. Implement or refine UI states, accessibility, onboarding, and mock-debate rendering against the specifications.
6. Measure application performance budgets and run the provider failure matrix.
7. Replay the demo scenario and run the `RELEASE-GATE.md`/`ACCEPTANCE-GAUNTLET.md`, recording declared limitations.
8. Perform the required root DOX index update after concurrent work is stable.
