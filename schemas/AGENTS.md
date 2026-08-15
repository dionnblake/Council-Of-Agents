# Schemas

## Purpose

Own versioned JSON contracts used by the controller and provider seats.

## Local Contracts

- Schema files are source-of-truth contracts, not generated examples.
- Versions are immutable once used by a persisted debate.
- Keep syntax requirements separate from semantic validation in Rust.
- Every schema change requires fixture and validation coverage.

## Verification

- Validate representative valid and invalid payloads through council-cli or core tests.

## Child DOX Index

No child directories are present.
