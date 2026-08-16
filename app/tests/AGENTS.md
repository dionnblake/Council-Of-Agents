# Frontend Regression Tests

## Purpose

Protect the visible React policy boundary during V1 maintenance.

## Ownership

- `policy-regression.test.mjs` checks critical frontend guard and limitation contracts.

## Local Contracts

- Tests remain dependency-free and must not call live providers.
- Tests protect frontend presentation of policy; Rust remains the policy authority.
- Preview, incomplete, degraded, decision, export, recovery, and no-handoff states must remain explicit.

## Work Guidance

Run the suite from `app/` with `npm test`.

## Verification

The suite must pass before frontend changes are committed. It runs in the Windows GitHub Actions workflow and does not replace Rust, build, installer, or desktop smoke verification.

## Child DOX Index

No child directories are present.
