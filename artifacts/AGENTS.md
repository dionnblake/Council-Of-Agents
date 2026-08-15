# Artifacts

## Purpose

Own generated demo outputs and inspectable run evidence.

## Local Contracts

- Artifacts are outputs, not source-of-truth configuration.
- Generated databases and logs must not be treated as current runtime state without a fresh run marker.
- Do not store secrets or authenticated provider output containing credentials.
- Keep synthetic fixtures clearly labeled as synthetic.

## Verification

- Each retained artifact should identify the command or fixture that produced it.

## Child DOX Index

No child directories are present.
