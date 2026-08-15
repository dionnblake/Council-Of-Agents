# First-Run and Onboarding Specification

## Purpose

First launch should explain Council's boundary and get the local provider seats to an honest usable state without overwhelming the user with architecture details.

## First-run promise

The first-run sequence says, in plain language:

1. Council does not code.
2. It checks three local agent seats.
3. You choose the requested model for each seat.
4. You ask a technical question.
5. The seats analyze and challenge one another.
6. You decide.
7. You manually copy the final prompt elsewhere.

The user can skip optional setup, but the boundary explanation and safety acknowledgment cannot be skipped before the first provider dispatch.

## First-run flow

### Step 1: Welcome and boundary

Show a compact diagram of the product flow and a sentence: “Council advises. You decide. Nothing is implemented or sent automatically.”

Actions: `Start Setup`, `View Safety Boundary`, `Exit`.

### Step 2: Provider seats

Show three seat rows:

```text
Claude
Antigravity
Codex WSL
```

Each row has `Check`, status, requested model selector, and a short limitation field. Do not ask the user to configure providers that are not selected for the first debate.

### Step 3: Diagnostics

Run checks independently and show:

```text
Installed
Authenticated
Certified
Available
Requested model
Served model or PROVIDER_DOES_NOT_REPORT
Isolation status
Declared limitation
```

The user can open details but should not need to understand CLI paths to know whether a seat is ready.

### Step 4: Safe local settings

Ask only for provider locations, Codex WSL distro, export folder, timeouts, and explicit model requests. Do not ask for API keys when the certified route uses an existing subscription. Do not expose secret values in the form.

### Step 5: First question

Offer `Ask a Technical Question` with the same fast intake as New Debate. A sample question is visible but never auto-submitted.

### Step 6: Preflight summary

Before the first real dispatch, show the selected seats, repository/no-repository state, snapshot or evidence scope, model status, and what will happen next. The user must explicitly continue.

## Codex WSL diagnostic detail

The Codex row has an expandable diagnostic card:

```text
WSL distro detected: CouncilCodexWSL / NOT DETECTED
Codex executable detected: yes/no
ChatGPT authenticated: yes/no/unknown
Linux identity: council / unknown
HOME and CODEX_HOME: verified/not verified
/mnt/c available: no/yes (yes blocks certification)
Windows interop: disabled/enabled (enabled blocks certification)
Windows skill tree reachable: no/yes/unknown
Read-only evidence boundary: verified/not verified
Ready: yes/no
```

The card explains that this is an isolation check, not a request to copy Windows credentials into WSL.

## Setup failure paths

- **Not installed:** explain what local install is missing; no automatic download or paid fallback.
- **Not authenticated:** show the provider's local repair action category; never display credentials.
- **Not certified:** block or mark optional according to debate policy.
- **Quota limited:** explain that existing access is limited; do not switch to API billing.
- **Codex WSL unavailable:** offer recheck/restart diagnostics and keep the Windows boundary visible.
- **Antigravity route unavailable:** distinguish IDE presence from standalone headless certification.
- **Unknown failure:** pause and offer diagnostics; do not tell the user the provider is ready.

## Returning user experience

On later launches, show a compact health summary and “last checked” timestamp. Do not repeat onboarding unless certification changed, the user opens diagnostics, or a required safety state is stale.

## Onboarding copy rules

- Say “seat,” “provider,” “position,” and “human decision.”
- Avoid “the AI will build,” “agents collaborate automatically,” or “consensus guarantees correctness.”
- Use concrete next actions: `Check`, `Repair`, `Remove Seat`, `Continue with Available Seats`.
- Explain a limitation in one sentence before linking to technical details.

## Onboarding acceptance

1. A new user can explain what Council does and does not do after the first screen.
2. A user can identify which seats are ready without reading logs.
3. Codex WSL diagnostics distinguish authentication from isolation.
4. A missing provider does not appear healthy or silently disappear.
5. No API key field, provider-send action, or automatic setup command appears.
6. The first debate cannot start until required safety/preflight state is understood and accepted.
