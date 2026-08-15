# Mock Debate Fixtures

## Purpose

These files are complete synthetic Council debates for UI rendering, evaluator development, and acceptance rehearsal. They are not provider transcripts and do not call any provider.

## Ownership

This directory owns the named mock-debate markdown fixtures and their internal state narratives.

## Local Contracts

- Use the Council vocabulary: seats, positions, claims, evidence, rounds, decisions, declared limitations, and degraded council.
- Preserve the difference between a provider response failure and a reasoning position.
- Make dissent and evidence quality visible.
- Use synthetic paths such as `src/state/store.ts` and synthetic hashes; never include secrets.

## Verification

Each fixture should contain enough structured content to render claims, disagreements, evidence, repairs or failures where relevant, the final decision state, and the intended UI/QA purpose.

## Child DOX Index

This directory has no child `AGENTS.md` files.
