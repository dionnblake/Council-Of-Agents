# Council of Agents V1 Release Gate

## Purpose

This is the final evidence-backed release checklist. It is broader than a build check and narrower than a promise that every provider or environment is perfect.

## Verdict vocabulary

```text
V1 VERIFIED
V1 VERIFIED WITH DECLARED LIMITATIONS
V1 UNVERIFIED
V1 FAILED
```

Every gate is `PASS`, `FAIL`, `UNVERIFIED`, or `NOT APPLICABLE` with evidence. A missing test is not a pass.

## Required release packet

```text
build identifier / commit
environment and tool versions
provider certification snapshots
test command outputs
raw safety/debate/failure artifacts
UI screenshots or recorded walkthroughs
accessibility evidence
performance measurements
fresh-machine installation evidence
README verification
demo recording or deterministic replay
known limitations and owner decisions
```

## 1. Security pass

- [ ] Live repository unchanged after provider and failure tests.
- [ ] Snapshot excludes `.git`, provider configuration, hooks, MCP, and instruction surfaces.
- [ ] Secret filename/content scan blocks synthetic markers and redacts logs.
- [ ] Junction, symlink, and reparse-point escapes are rejected.
- [ ] Snapshot and packet are sealed/read-only; provider writes fail.
- [ ] WSL Codex boundary blocks Windows mount, interop, and skill-tree access.
- [ ] Provider network/browser/tool attempts are blocked or unavailable.
- [ ] Malformed output and prompt-injection corpus produce expected fail-closed states.
- [ ] No credential values appear in source, fixtures, logs, exports, or screenshots.

Evidence: `security/THREAT-MODEL.md`, `security/PROMPT-INJECTION-CORPUS.md`, `security/SAFETY-TEST-CASES.md`, raw test artifacts.

## 2. Provider pass

- [ ] Claude installation, auth, certification, schema behavior, model request, served identity, and quota path checked.
- [ ] Codex WSL distro, auth, isolation, packet bridge, restart, cancellation, and model limitation checked.
- [ ] Antigravity standalone CLI route, auth, credit guard, malformed-output repair, and model limitation checked.
- [ ] Requested and served model are separate everywhere.
- [ ] Unknown provider failures pause rather than become consensus.
- [ ] No API billing or silent paid fallback is used.

Evidence: provider certification records, failure matrix execution, preflight screenshots, raw status output.

## 3. Debate pass

- [ ] R1 independence proven with packet inspection.
- [ ] R1 claims are bounded and attributable.
- [ ] R2 responses use `CONCEDE`, `DISPUTE`, or explained `NO_BASIS_TO_JUDGE`.
- [ ] R3 preserves revisions, withdrawn claims, dissent, flip conditions, and cost if wrong.
- [ ] Partial rounds and provider failures are distinct from abstention.
- [ ] Human decision is required; no vote authority exists.
- [ ] Two-seat degraded mode is explicit and policy-controlled.
- [ ] R1-only versus full-council comparison can be run on a benchmark fixture.

Evidence: mock debates, benchmark results, state transitions, human decision records.

## 4. Visual pass

- [ ] Question, round, unresolved issue, evidence, and human decision outrank transcripts.
- [ ] The interface reads as a technical deliberation command center.
- [ ] No generic AI dashboard defaults dominate: no gradient hero, orb, excessive cards/pills, or rainbow providers.
- [ ] Evidence statuses and provider limitations are readable without color alone.
- [ ] Loading, empty, failure, partial, cancellation, and export states are designed and usable.
- [ ] Dark mode and 125% scaling retain hierarchy and contrast.

Evidence: visual smoke checklist, screenshots or recorded walkthrough, design review findings.

## 5. Accessibility pass

- [ ] Keyboard-only path completes intake, preflight, debate review, decision, and export.
- [ ] Focus is visible and restored after inspector/dialog actions.
- [ ] Screen-reader labels identify claim, evidence, provider, model, status, and decision action.
- [ ] Reduced motion and high-contrast-friendly states work.
- [ ] Copy shortcuts and evidence navigation are consistent.

Evidence: keyboard recording, accessibility tree inspection, contrast/scaling notes.

## 6. Performance pass

- [ ] Startup, debate list, evidence lookup, SQLite query, cancellation acknowledgement, and UI responsiveness meet documented budgets.
- [ ] Provider calls do not block the UI thread.
- [ ] Large repository behavior shows progress, remains cancellable, and never silently truncates evidence.
- [ ] Mock debates render without provider calls or visible stalls.

Evidence: `PERFORMANCE-BUDGETS.md` measurements with environment and fixture size.

## 7. Crash/restart pass

- [ ] App restart reconstructs durable debate state without hidden provider resume.
- [ ] Provider cancellation leaves no lingering child process.
- [ ] Snapshot/packet/export hash mismatch quarantines the artifact.
- [ ] Partial round recovery is explicit.
- [ ] Audit-write failure blocks false completion.

Evidence: restart logs, process list, state hashes, recovery screenshots.

## 8. No API billing pass

- [ ] Existing subscription routes are identified for Claude, Codex/ChatGPT, and Antigravity.
- [ ] Environment allowlist removes API keys and custom base URLs from provider execution.
- [ ] No fallback silently changes billing mode.
- [ ] Any provider usage estimate is not presented as an invoice.

Evidence: sanitized environment report, provider auth category, billing-boundary test.

## 9. Air-gap/local-control pass

- [ ] Normal app operation does not require a Council cloud service.
- [ ] Questions, packets, snapshots, artifacts, decisions, and exports are inspectable locally.
- [ ] Network/tool access is explicit and blocked where certification requires it.
- [ ] Export does not publish or send automatically.

Evidence: local run log, process/network observation, export inspection.

## 10. Packaging pass

- [ ] Installer/package is built from the intended commit.
- [ ] Provider paths and safe defaults survive install.
- [ ] Export folder permissions and local data migration are checked.
- [ ] Uninstall/reinstall behavior does not silently delete decision records.
- [ ] Windows 100%/125% display scaling is checked.

Evidence: package hash, install log, clean-machine screenshots, migration result.

## 11. Fresh-machine install test

On a clean or disposable Windows environment:

1. Install the application.
2. Launch it directly through the intended desktop path.
3. Complete onboarding diagnostics without hidden developer tools.
4. Load a synthetic mock debate.
5. Run a no-provider UI review.
6. Export a decision record.
7. Confirm no credential or production repository is required for the local demo.

If provider certification is not possible on the fresh machine, mark provider gates `UNVERIFIED`; do not imply full release verification.

## 12. README pass

- [ ] README states local-first boundary and no automatic implementation.
- [ ] README names provider limitations and subscription/no-API-billing constraint.
- [ ] README launch/build commands are current and tested or marked as prerequisites.
- [ ] README links to evidence and acceptance records.

## 13. Demo pass

- [ ] `DEMO-SCENARIO.md` can be replayed with live or clearly labelled synthetic inputs.
- [ ] Demo shows independent opinions, challenge, evidence, revision, minority concern, human decision, and manual copy stop.
- [ ] No staged output is described as live provider behavior.
- [ ] Demo does not trigger implementation or external publication.

## Final release decision

The release owner records:

```text
Verdict
Blocking failures
Declared limitations
Unverified checks
Evidence locations
Human approval/date
Next re-certification trigger
```

`V1 VERIFIED WITH DECLARED LIMITATIONS` is appropriate when safety, governance, and core product gates pass but a provider cannot prove a capability such as served-model identity. `V1 UNVERIFIED` is required when a material gate did not run. `V1 FAILED` is required when a safety or human-authority invariant does not hold.
