# Public Repository Checks

## Purpose

Own deterministic, local-only checks that prevent private workstation data, credentials, or unsafe release artifacts from entering the public repository.

## Ownership

- `public-repo-audit.cjs` owns current-tree, reachable-history, filename, content, and commit-metadata privacy checks.

## Local Contracts

- Never print matched secret values, credential prefixes, email addresses, or private path contents.
- Keep checks dependency-free and based on the repository checkout and Git object database.
- Allow only explicit public placeholders and intentional Council architecture paths.

## Verification

- Run `node scripts/public-repo-audit.cjs --history` from the repository root.
- A release pass requires `PUBLIC_REPO_AUDIT=PASS` and `CONFIRMED_LIVE_SECRET_MATCHES=0`.

## Child DOX Index

No child directories are present.
