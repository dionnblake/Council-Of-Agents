# Accessibility and Keyboard Specification

## Purpose

Council is a dense desktop engineering tool. Keyboard use, scaling, focus, and screen-reader semantics are part of the product contract, not later polish.

## Accessibility baseline

- All primary workflows are keyboard-completable: intake, preflight, rounds, evidence, decision, and export.
- Every status has text plus a non-color visual cue; color is never the only meaning.
- Focus is visible at 100% and 125% Windows scaling and remains visible in dark mode.
- Text and controls meet a documented contrast target equivalent to WCAG AA for normal text; critical status text uses stronger contrast where possible.
- Interactive targets are at least 32 px in dense desktop surfaces and larger for decision actions.
- Tables expose headers, row/column relationships, and selected state to assistive technology.
- Dialogs have a labelled title, described purpose, trapped focus, Escape behavior, and focus restoration.
- Live provider activity is announced without flooding the screen reader with token-by-token output.
- Reduced-motion settings remove decorative animation while preserving state changes through text and layout.
- Text, code, and evidence remain usable at Windows 125% scaling and with high-contrast settings.

## Global tab order

The default order is:

```text
window/menu
-> rail navigation
-> question/debate header
-> primary work-surface action
-> work-surface rows or claims
-> inspector trigger
-> secondary actions
-> status/details
```

The DOM/accessibility order follows reading and decision order, not visual absolute positioning. A collapsed inspector removes its controls from tab order.

## Screen-specific keyboard flow

### Home

`New Debate` -> incomplete debates -> recent decisions -> provider health -> settings.

`Enter` opens the focused debate. `Ctrl+N` starts a new debate. Search, if present, uses `/` and Escape returns focus to the list.

### New Debate

Question -> mode -> product type -> decision type -> candidates/constraints -> priority -> repository -> initial leaning -> provider selections/models -> actions.

`Ctrl+Enter` starts preflight only when required fields are valid. Invalid fields receive focus in the first unresolved order.

### Provider Preflight

Provider table rows are navigated with Up/Down. `Space` selects an optional seat. `R` rechecks the focused provider. `Enter` opens diagnostics. The action bar follows the table.

### Debate View

`Ctrl+1` opens Opening Positions, `Ctrl+2` opens Cross-Examination, and `Ctrl+3` opens Final Positions. Within a claim list, Up/Down or `J/K` moves selection. `Ctrl+E` opens evidence for the selected claim. `Ctrl+D` opens Decision View when permitted. `Escape` closes the inspector and returns focus to the originating claim.

### Evidence Viewer

`Ctrl+E` focuses the evidence viewer, `Home/End` move within the excerpt, and `Ctrl+C` copies the citation when the citation control or excerpt is focused. The copied text includes source path and range, not hidden metadata.

### Decision View

Tab order is summary -> highest-impact issue -> minority position -> evidence quality -> decision action group. The actions are not triggered by a single letter key. `Ctrl+Enter` confirms only after a choice and rationale are focused/valid.

### Master Prompt View

`Ctrl+Shift+C` copies the master prompt. `Ctrl+S` opens the local save/export flow. Copy never launches a provider. Focus returns to the Copy or Export control after completion.

## Focus indicators

- Use a 2 px or stronger outline with sufficient contrast against both surface and background.
- Focus must survive dark mode, warning surfaces, selected rows, and code panels.
- Selection and focus are distinct: selection can persist while focus moves to the inspector.
- Do not use an animated glow as the only focus indication.

## Screen-reader semantics

Use explicit labels such as:

```text
“Round 2, cross-examination, complete”
“Claim C3, disputed by Claude, evidence unverified”
“Codex WSL, ready, requested model gpt-5.6-luna, served model not reported”
“Antigravity, unavailable, standalone headless route not verified”
“Decision action, Approve Modified Decision, requires rationale”
```

Provider streaming announces state changes (`started`, `completed`, `failed`, `cancelled`) and elapsed time at sensible intervals. It does not read every token.

## Evidence navigation

- Every citation link has an accessible name containing claim ID, file, and line range.
- The selected evidence excerpt announces source, range, and verification status before code text.
- `VERIFIED_EXACT`, `VERIFIED_CONTENT_FOUND_ELSEWHERE`, and `UNVERIFIED` are literal words in accessible text.
- Returning from evidence restores focus to the originating claim.
- Missing evidence explains why no source can be opened.

## Provider-status semantics

Provider status is a table or labelled group with separate fields for installed, authenticated, certified, available, requested model, served model, and limitation. A color dot alone is invalid. `PROVIDER_DOES_NOT_REPORT` is an informative limitation, not a failure icon without explanation.

## Copy and clipboard behavior

- Copy buttons expose what will be copied and confirm success without stealing focus.
- Copy errors name the local failure and leave the prompt visible for manual selection.
- The app does not copy secrets or hidden provider configuration into the clipboard.
- Clipboard confirmation includes artifact identity/hash when available.

## Reduced motion and cognitive load

- Replace pulsing provider indicators with static state words when reduced motion is enabled.
- Preserve progress through text, percentage only when meaningful, elapsed time, and completed count.
- Avoid auto-scrolling transcripts and unexpected focus movement.
- Keep the question and decision state anchored while details change.

## Accessibility acceptance

The reviewer must demonstrate:

1. Complete a mock debate using keyboard only.
2. Open an evidence item and return to its claim.
3. Identify a provider limitation without color.
4. Reach and confirm a human decision without accidental activation.
5. Copy the master prompt with `Ctrl+Shift+C`.
6. Use 125% scaling, dark mode, and reduced motion.
7. Inspect screen-reader names for a claim, provider, evidence status, and decision action.
