# `design-taste.v1` Specification

## Purpose

`design-taste.v1` evaluates whether a proposed product experience has intentional visual and interaction decisions that fit its users, platform, content, and identity. It is a reasoning package, not a universal beauty judge and not a numerical “premium score.”

The package is meant to prevent obvious AI-generated design patterns while allowing deliberate use of any visual language when the product gives a clear reason for it.

## Core stance

The question is not “does this use a gradient?” or “is this modern?” The question is:

> Does this choice make the product more legible, useful, distinctive, and coherent for this audience, or does it appear because a template usually does it?

A visually quiet tool can be excellent. A colorful game can be excellent. A rounded interface can be excellent. The failure is unreasoned imitation.

## Inputs

The skill accepts as much of the following as exists:

- product purpose and audience;
- platform conventions and target devices;
- content hierarchy and density;
- screenshots, mockups, implementation, or a declared lack of visual evidence;
- typography and color choices;
- interaction states and motion examples;
- asset inventory and art direction for games;
- brand constraints and examples the user intentionally wants to reference;
- accessibility and localization requirements.

When no visual evidence exists, the skill may reason about design intent and acceptance criteria, but it must mark visual conclusions `CANNOT_DETERMINE`.

## Evaluation dimensions

The skill produces qualitative findings for each applicable dimension. It does not collapse them into one score.

### Visual hierarchy

Identify the first, second, and third things a user should notice. Check whether scale, contrast, placement, and grouping support the product's actual priorities. Call out when a decorative hero or provider transcript outranks the decision.

### Typography

Review typeface choice, size hierarchy, weight, line length, code treatment, metadata legibility, numeric alignment, and whether typography carries meaning without excessive color or decoration.

### Spacing and rhythm

Look for a deliberate spacing system, useful grouping, and meaningful breathing room. Flag both cramped dense content and empty premium-looking space that displaces useful information.

### Composition

Review layout, alignment, visual balance, reading path, responsive or desktop behavior, and whether the composition fits the task. A dashboard is not automatically the right composition for a deliberation tool.

### Information density

Check whether dense information is structured through hierarchy, tables, progressive disclosure, and grouping. Dense is acceptable; chaotic is not. Sparse is acceptable; under-informative is not.

### Color discipline

Check semantic use, contrast, saturation, theme consistency, and provider differentiation. Color must not substitute for hierarchy or accessibility. A palette may be expressive when tied to product identity, state, or content.

### Component philosophy

Check whether containers, cards, tables, dividers, tabs, and controls have a reason to exist. Flag a design where every object is independently wrapped, rounded, elevated, or pill-shaped.

### Interaction philosophy

Review whether controls express the user's mental model, whether state transitions are clear, whether motion has purpose, and whether the interaction supports review rather than performance theater.

### Platform conventions

Check Windows desktop expectations: predictable menus and focus, keyboard operation, scaling, contrast, dialogs, copy/save behavior, and non-browser-like resizing. Break conventions only when the product benefit is visible.

### Originality and distinctive identity

Look for one or more product-specific choices that could not have been copied from a generic AI dashboard: claim relationships, evidence rails, decision gates, editorial tone, or a domain-specific visual metaphor. Originality is not novelty for its own sake.

### Asset coherence

Check whether icons, illustrations, screenshots, avatars, and generated assets share a coherent style, scale, lighting, stroke, and semantic role. Inconsistent assets are a common form of generated slop.

### Intentionality and polish

Ask whether each conspicuous choice has a reason, whether edge states received equal care, and whether the final interface feels finished at the points where users make decisions. Polish includes empty, loading, error, disabled, and focus states.

### Game-specific dimensions

For games, also consider art direction, silhouette quality, environment consistency, HUD coherence, asset style coherence, lighting, animation direction, effects, and scene composition. Evaluate the whole visual system, not isolated concept art.

## AI-slop analysis

These are signals, not automatic prohibitions:

```text
every section is a rounded card
gratuitous gradients
purple/violet AI palette by default
glassmorphism without purpose
giant hero layouts inside applications
weak typography hierarchy
generic icons
random shadows
excessive pill controls
centered-everything layouts
identical spacing everywhere
default dashboard compositions
generic stock imagery
inconsistent generated assets
fake premium minimalism
```

For each signal, the skill asks:

1. Is the pattern present?
2. What product-specific reason is offered for it?
3. Does that reason improve comprehension, identity, or interaction?
4. Is the pattern repeated because it is useful or because the template repeated it?
5. What is the smallest change that would restore hierarchy or intent?

The finding should state the observed consequence, not merely label the design “AI slop.” For example: “The same elevated rounded container is used for question, provider status, evidence, and decision controls, so the approval gate has no visual priority. Reserve the strongest container treatment for the decision gate and use rows for evidence.”

## Deliberate versus default choices

A choice is deliberate when the design can explain:

- the user need it serves;
- the information or interaction it makes clearer;
- the platform or product constraint it respects;
- the cost it introduces;
- the evidence that the choice works in the relevant state.

A choice is probably default when it is justified only by “modern,” “premium,” “clean,” “AI,” or a reference to a trend without a product-specific benefit. The skill should ask for a reason once, then make a concrete recommendation rather than debate taste indefinitely.

## Output shape

The conceptual result contains:

```text
observed_strengths
observed_risks
dimension_findings[]
ai_slop_signals[]
deliberate_choices[]
missing_evidence[]
testable_acceptance_criteria[]
cannot_determine[]
```

Each finding includes the dimension, observed evidence, user consequence, and a concrete recommendation. A recommendation may be `KEEP`, `REVISE`, `REMOVE`, or `CANNOT_DETERMINE` with a reason.

## Testable design acceptance criteria

The skill must turn taste into observable requirements. Good criteria look like:

- No full-screen gradient background is used in the application shell.
- The question, current round, and primary decision state remain visible without scrolling past provider transcripts.
- Not every information group is placed in a rounded container; at least one dense comparison surface uses rows, dividers, or a table.
- At least three deliberate typography levels are visible: title, section/body, and metadata/code.
- Code and evidence use a dedicated monospaced treatment with readable line numbers.
- Primary actions remain identifiable in grayscale or high contrast and do not rely only on color.
- `VERIFIED_EXACT`, `VERIFIED_CONTENT_FOUND_ELSEWHERE`, and `UNVERIFIED` are distinguishable through text and shape as well as color.
- Loading, failure, empty, disabled, and focus states preserve the same hierarchy as the success state.
- Provider differentiation uses restrained identity cues and does not turn the screen into a rainbow of branded panels.
- Motion has a documented relation to state change, provider activity, transition, or completion; decorative loops are absent.
- Windows 125% scaling does not clip or hide question, decision, or export controls.
- A visual reviewer can identify one product-specific composition or interaction choice that would not appear in a generic AI dashboard.

For a game, add criteria for silhouette readability, consistent asset style, HUD hierarchy, lighting intent, and effect restraint. For a web application, add criteria for responsive breakpoints, content order, and keyboard/focus behavior.

## Review language

Use direct language:

```text
KEEP: the evidence line treatment makes provenance scannable.
REVISE: the provider cards overpower the decision; convert them to a status rail.
REMOVE: the decorative glow does not communicate state and reduces contrast.
CANNOT_DETERMINE: no loading or failure evidence was provided.
```

Do not use “make it modern,” “make it premium,” “give it personality,” or “make it clean” as an acceptance criterion. If those words appear in a brief, translate them into hierarchy, rhythm, color, interaction, and asset requirements.

## Limits

`design-taste.v1` does not approve a product, replace accessibility testing, choose a brand without owner input, or claim that an unseen implementation is polished. It identifies design decisions and the evidence needed to evaluate them.
