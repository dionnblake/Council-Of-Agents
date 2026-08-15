# Council of Agents Desktop UI/UX Specification

## Experience direction

Council is a **technical deliberation command center**: a professional engineering workstation where a decision is assembled, challenged, evidenced, and approved.

It should feel closer to a developer tool, architecture review room, and evidence console than to a marketing site or generic AI SaaS dashboard.

Do not use a chat clone as the primary composition. Do not default to three giant provider columns. The primary object is the decision and its claim relationships; provider transcripts are supporting evidence.

## Information architecture

The desktop shell has three persistent regions:

1. **Rail:** Home, New Debate, active debate sections, Evidence, Decisions, Settings. The rail is narrow and text-led; icons are secondary.
2. **Work surface:** the current decision state. It owns the question header, round status, claims, evidence, disputes, or decision controls.
3. **Inspector:** contextual detail for the selected claim, provider, evidence item, failure, or export. The inspector can collapse without hiding the work surface.

The question and current decision state remain visible while moving between positions, evidence, and cross-examination. The user should not have to remember which debate a panel belongs to.

## Visual system

### Density and hierarchy

- Use a compact desktop rhythm with deliberate grouping and whitespace at transitions, not large empty hero areas.
- The question, round state, unresolved disputes, and human decision controls outrank provider transcripts.
- Use one dominant work surface per screen. Secondary information can be muted or placed in the inspector.
- A dense table or claim list is preferable to a wall of cards when the user is comparing structured information.
- A card is reserved for a meaningful boundary: provider, evidence item, decision block, or failure event. Do not wrap every label in a card.

### Typography

- `Display`: one compact title for the debate or decision, approximately 24–30 px, strong but not oversized.
- `Section`: 15–18 px semibold for the active work surface sections.
- `Body`: 13–15 px with 1.45–1.6 line height for explanatory text.
- `Label`: 11–12 px uppercase or small-caps only for stable metadata such as `R2`, `CLAIM`, or `PROVIDER`.
- `Code/evidence`: a dedicated monospaced face at 12–13 px, with line numbers and preserved whitespace.
- `Citation`: monospaced or tabular numerals for `path:line-line`, visually distinct from prose.
- `Status`: short, explicit words plus an icon or shape; never rely on color alone.

The hierarchy must remain legible at 100% Windows display scaling and retain clear focus at 125%.

### Spacing rhythm

Use a small set of spacing tokens rather than arbitrary padding:

```text
4 px  inline separation
8 px  control and metadata gap
12 px row and compact group gap
16 px section gap
24 px major section gap
32 px screen transition gap
```

Use a 1 px border or quiet surface change to separate dense groups. Avoid stacking multiple shadows and nested rounded containers.

### Containers and shape

- Prefer squared or softly rounded surfaces with one or two radii, not a different radius for every element.
- Use borders, dividers, indentation, and alignment to show structure.
- Reserve the strongest surface treatment for the active decision gate or a failure that requires action.
- Avoid glassmorphism, blurred panels, and floating tiles that reduce text contrast.

### Color

Use a restrained neutral foundation with one deliberate accent for active decision state. Provider identities may use subtle, low-saturation marks, but the application must not become a rainbow.

Every status has a redundant encoding:

```text
success / verified     icon + word + restrained green
warning / limitation   icon + word + amber or ochre
failure / blocked      icon + word + red or rust
unknown / abstention   icon + word + neutral gray
selected / active      border or surface + text treatment
```

Do not use a full-screen gradient, neon glow, AI orb, or purple/violet default palette.

### Motion

Motion communicates state change, provider activity, transition, or completion. It must not be decorative ambient shimmer.

- Opening a round: a short progress transition with explicit provider rows.
- Provider activity: a restrained activity marker and elapsed time, not an animated avatar.
- Claim revision: a brief highlight that settles into a revision badge.
- Completion: one calm state transition from running to reviewable.
- Failure: no auto-scrolling panic animation; show the failure at the relevant provider row.

Respect Windows reduced-motion settings and provide an immediate static equivalent.

## Shared interaction rules

- Every primary action has a text label. Color is supplementary.
- Destructive or irreversible-looking actions require an explicit confirmation or a clearly reversible cancel path.
- Long-running work exposes cancel. Cancel status is distinct from provider failure.
- Provider transcripts are collapsible and never auto-focus the user into a streaming wall of text.
- Links from claims to evidence preserve the current scroll position and return focus to the originating claim.
- A selected claim, provider, or evidence item is visible through border, text, and focus treatment.
- `CANNOT_DETERMINE` and `NO_BASIS_TO_JUDGE` are valid states, not blank placeholders.

## Screen specifications

### A. Home

**Purpose:** orient the user and make the next decision easy to start or resume.

**Primary information:** New Debate, incomplete debates, recent decisions, and current provider health.

**Secondary information:** recent debate timestamps, seat count, mode, decision status, last export, and declared limitations.

**Primary controls:** `New Debate`, `Resume`, `Open Decision`, `Recheck Providers`.

**Navigation:** rail is active on Home. Recent items open their debate at the last meaningful state, not an arbitrary transcript position.

**Empty state:** explain the product in two sentences and show one `New Debate` action. Do not show a marketing hero or feature grid.

**Loading state:** skeleton rows for recent work and a compact provider health check indicator.

**Failure state:** if local persistence cannot load, show the exact local error and recovery action. Do not show an empty Home that could be mistaken for no data.

**Keyboard behavior:** `Ctrl+N` starts a new debate, `/` focuses Home search if present, `Enter` opens the focused item, and `Esc` exits search.

**Visual hierarchy:** New Debate first, then incomplete work, then decisions, with provider health as a calm side panel rather than the hero.

### B. New Debate

**Purpose:** collect the minimum context needed for a useful bounded council.

**Primary information and fields:**

```text
Question                         required, plain language
Mode                             Compare or Discovery
Product type                     desktop, web, mobile, game, service, library, other
Decision type                    architecture, stack, dependency, design, testing, operations, other
Candidate options                required in Compare; hidden or disabled in Discovery
Hard constraints                 platform, offline, cost, deadline, compliance, team, etc.
Primary decision priority       one explicit priority
Optional repository              local folder and snapshot summary
Optional initial leaning         optional, visibly labelled as a prior
Provider model selection         explicit model per selected seat
```

**Secondary information:** estimated debate shape, evidence implications, and a short explanation of Compare versus Discovery.

**Primary controls:** `Continue to Preflight`, `Save Draft`, `Cancel`.

**Navigation:** a compact step indicator shows Intake -> Preflight -> Debate. The user can go back without losing fields.

**Empty state:** the question field is focused with a prompt such as “What decision must be made before implementation?”

**Loading state:** repository inspection or local path validation is shown inline; unrelated fields remain usable.

**Failure state:** invalid path, inaccessible folder, unsupported option count, or missing required field is shown beside the field with a plain correction.

**Keyboard behavior:** tab order follows decision logic; `Ctrl+Enter` continues when valid; option rows support arrow keys; `Esc` cancels with draft-preservation choice.

**Visual hierarchy:** question and constraints dominate. Advanced provider details are below the decision context, not above it.

### C. Provider Preflight

**Purpose:** establish which seats can safely participate before dispatch.

**Primary information:** one row per selected provider: `Claude`, `Antigravity`, `Codex WSL`.

Each row exposes separate fields:

```text
Installed        yes/no/unknown
Authenticated    yes/no/expired/unknown
Certified        pass/limitation/not certified
Available        available/blocked/quota limited/unknown
Requested model  exact user selection
Served model     exact report, or PROVIDER_DOES_NOT_REPORT
Limitation       concise declared constraint
Last checked     local timestamp
```

**Secondary information:** authentication method category, isolation boundary, repair action, and whether the seat is required or optional for this debate.

**Primary controls:** `Run Preflight`, `Repair`, `Remove Seat`, `Continue with Available Seats`, `Cancel`.

**Navigation:** back to New Debate; forward to Debate only after required checks are resolved or explicitly waived.

**Empty state:** no providers selected is an actionable configuration error, not an empty council.

**Loading state:** each provider row progresses independently. Do not make a healthy provider wait visually for a slow check.

**Failure state:** show provider-specific failure, evidence timestamp, and safe next action. Unknown failures pause rather than auto-falling back.

**Keyboard behavior:** provider rows are navigable as a table; `R` rechecks the focused provider; `Space` toggles optional seat selection; focus moves to the unresolved row after a failed check.

**Visual hierarchy:** availability and safety first; model identity is visible but secondary. A limitation is calm and explicit, not a red alarm unless it blocks the chosen run.

### D. Debate View

**Purpose:** let the user understand the decision, claim relationships, evidence, and round state without reading three independent chat streams.

**Primary information:**

- question header and hard constraints;
- round indicator `R1`, `R2`, or `R3`;
- provider status strip with elapsed time and completion state;
- recommendation summary;
- load-bearing claim list;
- evidence status and citations;
- unresolved disputes and abstentions;
- position revisions.

**Secondary information:** expandable provider transcript excerpts, raw artifact links, token/time metadata, and audit events.

**Primary controls:** `Run R1`, `Run R2`, `Run R3`, `Cancel Round`, `Open Evidence`, `Compare Positions`, `Continue to Decision`.

**Navigation:** tabs or segmented work views for `Overview`, `Claims`, `Cross-Examination`, `Evidence`, `Positions`, and `Audit`. The question header remains fixed.

**Empty state:** before R1, show the question, selected seats, evidence scope, and a concise “ready to run independent positions” state.

**Loading state:** provider rows show state and elapsed time; the claims surface shows a stable placeholder explaining that no claims are accepted until validation completes.

**Failure state:** failed seats remain in the provider strip with the failure artifact. Partial output is marked partial and cannot silently populate a final position.

**Keyboard behavior:** `Ctrl+1/2/3` selects the round view, `J/K` moves between claims, `Ctrl+E` opens evidence for the selected claim, `R` runs the next permitted round, and `Esc` closes the inspector. The detailed keyboard and assistive-technology contract is authoritative in `ACCESSIBILITY-KEYBOARD-SPEC.md`.

**Visual hierarchy:** the question and unresolved decision issue sit above claims; claims sit above transcript. Do not make provider prose the dominant visual object.

**Intentional deliberation presentation:** use a claim relationship board or dense comparison table. Each claim row shows claim ID, short statement, supporting seats, challenging seats, evidence quality, and revision state. A provider filter changes attribution but does not create separate universes of content.

### E. Evidence Viewer

**Purpose:** make evidence provenance inspectable and distinguish exact support from nearby or unsupported content.

**Primary information:** source file, exact line range, excerpt, claim using it, and verification status.

**Required statuses:**

```text
VERIFIED_EXACT
VERIFIED_CONTENT_FOUND_ELSEWHERE
UNVERIFIED
```

**Secondary information:** snapshot identifier, file hash, normalized path, line count, provider attribution, and whether the evidence is repository-grounded or an external/product assumption.

**Primary controls:** `Open Claim`, `Copy Citation`, `Show Full File`, `Show Verification Details`, `Close`.

**Navigation:** opened from a claim or citation; back returns to the same claim and scroll position.

**Empty state:** when a position has no repository evidence, state `No repository supplied` or `No citation provided`; never show a blank code panel.

**Loading state:** line range and verification status load independently, with the source identity shown immediately.

**Failure state:** missing snapshot, invalid range, hash mismatch, or unreadable file is explicit and blocks a verified label.

**Keyboard behavior:** `Ctrl+C` copies the citation when the viewer is focused; `Home/End` move within the excerpt; `Esc` returns to the claim.

**Visual hierarchy:** status and source identity appear above excerpt text. `UNVERIFIED` must be legible without relying on red color.

### F. Cross-Examination View

**Purpose:** show whether peer interaction changed the reasoning, without presenting disagreement as social conflict.

**Primary information:** claim-by-claim response matrix showing who `CONCEDED`, `DISPUTED`, or returned `NO_BASIS_TO_JUDGE`, plus the explanation.

**Secondary information:** original claim, evidence references, response timestamp, and whether the response caused a later revision.

**Primary controls:** `Filter by Claim`, `Filter by Provider`, `Open Evidence`, `Show Revision`, `Continue to R3`.

**Navigation:** return to Debate Overview or open the affected claim.

**Empty state:** “No cross-examination has run yet” with the permitted next round.

**Loading state:** response rows appear only after validation; no fake typing transcript is needed.

**Failure state:** a provider timeout or malformed response is shown as an unavailable response, not as agreement or silence.

**Keyboard behavior:** arrow keys move through the matrix; `Enter` opens the selected claim; `C`, `D`, and `N` are labels only when a response is being reviewed, never shortcut keys that mutate state without confirmation.

**Visual hierarchy:** the claim is the row anchor; provider identity and response type are columns. Avoid avatars, speech bubbles, likes, or chat-style reaction affordances.

### G. Decision View

**Purpose:** give the human enough structure to make an informed decision and record it.

**Primary information:**

```text
agent final positions
commitment per seat
agreements
disagreements
minority position
highest-impact unresolved issue
risks
flip conditions
reversibility
evidence quality
```

**Secondary information:** round history, provider limitations, withdrawn claims, raw evidence links, and decision provenance.

**Primary controls:**

```text
Approve Option
Approve Modified Decision
Continue Targeted Debate
Challenge Consensus
Reject All
```

**Navigation:** back to claims and cross-examination; forward to Master Prompt only after a human action is recorded.

**Empty state:** if R3 is incomplete, show the missing prerequisites and do not render disabled-looking final controls as if the decision were ready.

**Loading state:** compile the decision summary only from validated positions; show which seat is still pending.

**Failure state:** if the decision cannot be persisted, preserve the proposed choice locally in the current session and show that it is not yet recorded.

**Keyboard behavior:** decision actions are reachable in a predictable order, require a focused confirmation step, and never activate from a single accidental keypress. `Ctrl+Enter` confirms only after the user has selected an action and reviewed the summary.

**Visual hierarchy:** the highest-impact unresolved issue and minority position are visible before the action bar. Approval is prominent but not coercive.

### H. Master Prompt View

**Purpose:** present the deterministic handoff artifact clearly after human approval.

**Primary information:** the generated prompt, decision status, approved option or modified decision, constraints, acceptance criteria, evidence references, risks, dissent, and explicit stop boundary.

**Secondary information:** packet/hash metadata, generation timestamp, source debate ID, and export format.

**Primary controls:** `Copy`, `Save`, `Export`, `Back to Decision`.

**Absolutely absent controls:** `Implement`, `Run`, `Send to Codex`, `Send to Claude`, `Open Hermes`, or any provider-launch equivalent.

**Navigation:** return to Decision; export confirmation returns focus to the relevant control.

**Empty state:** no approved human decision means “Master prompt is not available yet,” with a link to Decision View.

**Loading state:** deterministic compilation shows the source debate ID and waits for completion; it does not stream provider text.

**Failure state:** missing state, non-deterministic compilation, or export write failure is explicit. No partial prompt is presented as final.

**Keyboard behavior:** `Ctrl+Shift+C` copies only when the prompt surface is focused or the Copy button is focused; `Ctrl+S` saves through the app's local export flow; `Esc` returns to Decision. The detailed keyboard and assistive-technology contract is authoritative in `ACCESSIBILITY-KEYBOARD-SPEC.md`.

**Visual hierarchy:** the prompt is readable as a document, while status and metadata remain visible in a narrow header. The stop boundary is visually explicit.

### I. Settings

**Purpose:** expose the few operational choices required to keep provider execution safe and predictable.

**Primary information and controls:**

```text
provider locations
provider models
timeouts
export folder
Codex WSL distro
Antigravity credit guard
provider certification
```

**Secondary information:** last verification, declared limitations, local paths, and reset/recheck actions.

**Primary controls:** `Save`, `Recheck Certification`, `Restore Safe Defaults`, `Open Export Folder`.

**Navigation:** settings are grouped by Provider, Safety, and Export. Avoid a long undifferentiated settings list.

**Empty state:** missing provider configuration has a guided field-level explanation; no setup wizard is required for unrelated features.

**Loading state:** provider checks run per row and preserve editable fields.

**Failure state:** invalid path or certification failure names the exact field and blocks unsafe use; saving unrelated safe settings remains possible.

**Keyboard behavior:** standard form navigation, clear focus rings, `Ctrl+S` saves, and `Esc` cancels unsaved changes with confirmation only when changes exist.

**Visual hierarchy:** safety and certification are more visible than cosmetic settings. Provider model selectors never collapse the requested-versus-served distinction.

## Provider model-selection UX

Each seat has an explicit requested model selector:

```text
CODEX
Requested model:      [gpt-5.6-luna       v]

CLAUDE
Requested model:      [claude-haiku-4-5   v]

ANTIGRAVITY
Requested model:      [gemini-3.7-flash-low v]
```

The selected value is stored as a request. A separate audit block reports:

```text
Claude
Requested: claude-haiku-4-5
Served:    claude-haiku-4-5
Status:    VERIFIED

Codex
Requested: gpt-5.6-luna
Served:    Provider does not report
Status:    REQUESTED ONLY

Antigravity
Requested: gemini-3.7-flash-low
Served:    Provider does not report
Status:    REQUESTED ONLY
```

The audit state is calm: a small status label and explanatory sentence, not a large error banner. A provider limitation becomes blocking only when the chosen debate requires verified served identity.

## Accessibility and Windows behavior

- Full keyboard operation is required for intake, round review, evidence navigation, decision actions, and export.
- Focus is always visible with a non-color-only treatment.
- Tables and claim relationships expose headers and labels to screen readers.
- Status icons have text alternatives. Color never carries the only meaning.
- Text remains readable at Windows 125% scaling without clipping primary controls.
- The app supports a high-contrast-friendly neutral mode and respects reduced motion.
- Minimum interactive target size is approximately 32 px for dense desktop controls, with larger targets for primary decisions.
- Dialogs trap focus, announce their title and purpose, and restore focus on close.
- Long code/evidence regions support horizontal scrolling without causing the whole work surface to scroll sideways.

## Visual smoke-test questions

Acceptance must answer specific questions, not “looks good”:

- Is the current decision and round visible before provider transcript detail?
- Can a reviewer locate an unresolved dispute within two actions from Debate View?
- Can a user distinguish `VERIFIED_EXACT`, `VERIFIED_CONTENT_FOUND_ELSEWHERE`, and `UNVERIFIED` without color?
- Does the interface still look like engineering software if all provider labels are removed?
- Are cards used for meaningful boundaries rather than every row?
- Is the human decision control clear without feeling like an automatic execution button?
- Does the dark mode preserve code contrast, dividers, focus, and warning legibility?
- Do loading and failure states retain layout hierarchy instead of replacing the work surface with a generic spinner?
