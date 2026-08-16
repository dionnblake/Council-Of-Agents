# Public Repository Sanitation Report

Date: 2026-08-15

## Scope

This pass reviewed the current repository tree, all reachable Git blobs and commits, local branches and tags, generated artifacts, feasibility reports, source defaults, fixtures, and parallel product/evaluation material for unnecessary personal information, workstation identifiers, private paths, credentials, and secret-like data.

## Changes

- Generalized workstation-specific evidence while preserving the architecture, experiments, results, limitations, and certification conclusions.
- Replaced private home paths with public placeholders such as `C:\Users\<USER>`, `%LOCALAPPDATA%`, `%APPDATA%`, `%USERPROFILE%`, and `/home/<USER>`.
- Generalized personal identity references and synthetic fixture labels where they were not part of the product contract.
- Added `.gitignore` protection for environment files, key material, local credential files, databases, and logs.
- Added `scripts/public-repo-audit.cjs`, a dependency-free local checker that scans the current tree, reachable Git history, filenames, content patterns, and commit metadata without printing matched values.
- Added `docs/security/PUBLIC-REPOSITORY-DATA-POLICY.md` and the owning DOX entry.

## Privacy and history gates

| Gate | Result |
| --- | --- |
| Current-tree privacy | PASS |
| Reachable-history privacy | PASS |
| Machine identifiers removed | PASS |
| Private paths removed | PASS |
| Unnecessary personal identity removed | PASS |
| Commit metadata | SANITIZED to public noreply metadata |
| History rewrite | YES |
| Credential-pattern scan | PASS; 0 confirmed live-secret matches |
| Credential rotation | No rotation triggered; no actual credential value was found by the configured scans |
| Local branches and tags reviewed | PASS; no tags and no legacy public backup ref retained |

The intentional Council architecture paths, including `/home/council`, remain documented because they are product boundaries rather than personal workstation paths.

## Verification

- `node scripts/public-repo-audit.cjs --history`: PASS; current tree and 163 reachable blobs scanned in the remote verification clone.
- `rustup run 1.96.0-x86_64-pc-windows-msvc cargo fmt --all -- --check`: PASS.
- `rustup run 1.96.0-x86_64-pc-windows-msvc cargo test --workspace`: PASS; 29 tests passed.
- `cargo check --workspace` with the pinned toolchain: PASS.
- `npm ci --ignore-scripts` and `npm run build` from `app/`: PASS.
- `C:\Users\<USER>\.context\scripts\verify.js` with the pinned toolchain: `VERIFIED`.
- No live provider calls, billing changes, authentication changes, or credential changes were performed.

## Remote status

- The repository was private during sanitation and history rewriting.
- Sanitized `main` was force-pushed with a lease after local gates passed.
- Independent remote clone verification passed: the remote exposed only `main`, the privacy checker passed, all known private-path indicators were absent from reachable history, and commit metadata was public-safe.
- Final visibility at this report revision: PRIVATE. Visibility restoration is the remaining owner-controlled step after this report is published.

## Known limitations

The deterministic checker covers common credential formats, private-key headers, bearer/JWT patterns, private paths, workstation hostnames, emails, unsafe filenames, and commit metadata. It is a release guard, not a substitute for reviewing newly added domain-specific identifiers or external service dashboards.
