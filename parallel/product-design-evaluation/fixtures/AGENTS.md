# Parallel Synthetic Fixtures

## Purpose

This directory contains documentation-only synthetic fixtures that let the UI, evaluator, and acceptance workstreams exercise realistic Council states without provider usage.

## Ownership

This directory owns parallel mock inputs only. It does not own production fixture repositories, runtime schemas, provider artifacts, or application code.

## Local Contracts

- Every fixture must declare itself synthetic and provider-free.
- Evidence paths, hashes, provider names, and model labels are illustrative unless explicitly marked as observed project evidence.
- Fixtures may include failures, repairs, dissent, and decisions, but must not include real credentials or claim to be a live certification result.
- Keep fixture state internally coherent enough for UI rendering and QA assertions.

## Verification

Check that each mock debate has R1, R2, R3 or an explicit failure boundary, claim IDs, evidence status, provider attribution, human decision state, and a clear expected UI use.

## Child DOX Index

- `mock-debates/AGENTS.md` owns the ten complete synthetic Council debate fixtures.
