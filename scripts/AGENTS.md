# Public Repository Checks

## Purpose

Own deterministic, local-only checks that prevent private workstation data, credentials, or unsafe release artifacts from entering the public repository.

## Ownership

- `public-repo-audit.cjs` owns current-tree, reachable-history, filename, content, and commit-metadata privacy checks.
- Ordinary Git author/committer metadata is reported as identity metadata warning only. It is not a credential or secret finding. Private paths, credentials, tokens, key material, and other content findings remain release-blocking.

## Local Contracts

- Never print matched secret values, credential prefixes, email addresses, or private path contents.
- Keep checks dependency-free and based on the repository checkout and Git object database.
- Allow only explicit public placeholders and intentional Council architecture paths.

## Verification

- Run `node scripts/public-repo-audit.cjs --history` from the repository root.
- A release pass requires `PUBLIC_REPO_AUDIT=PASS` and `CONFIRMED_LIVE_SECRET_MATCHES=0`. `IDENTITY_METADATA_WARNINGS` may be nonzero and must be reported without printing email values.

## Child DOX Index

No child directories are present.
