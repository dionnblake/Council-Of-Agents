# Council of Agents V1 Certification Handoff

Paused for later continuation on 2026-08-16/17. The certification copy of the desktop app has been stopped. No other application was stopped.

## Current stop point

The workflow is paused at the mandatory human snapshot-review gate. No provider call has started for the current debate.

```text
DEBATE_ID = debate-1d7c0221-3910-4531-88aa-b2a55b54d6bc
SNAPSHOT_ID = snapshot-debate-1d7c0221-3910-4531-88aa-b2a55b54d6bc
SNAPSHOT_REVIEW = PENDING
TURNS = 0
```

Current snapshot evidence:

```text
MANIFEST_SHA256 = 743b15bcb89c0646d2b6cb4e5eb08c74a6c6b241ff79f05fb162040e7ba4f4b4
EXCLUSION_SET_SHA256 = f3e266280ff7a33d222af4a6032348c4c70b7c607ba859c8dd421403af628bc8
SOURCE_TREE_HASH = 9f35fc8dc796bdbf38e102e05cf7ccb3a7b6ecc448d05dc53ade4225df298415
SECRET_EXCLUSION_COUNT = 3
SNAPSHOT_ROOT = %APPDATA%\com.councilofagents.desktop\runtime\snapshots\snapshot-debate-1d7c0221-3910-4531-88aa-b2a55b54d6bc
```

The manifest records 108 repository files. The snapshot root contains 109 read-only files including the manifest itself. No reparse points were found. The safe excluded paths and reasons are persisted in the snapshot review record. Secret-excluded contents were not displayed or transmitted.

The exact human action still required is:

```text
APPROVE EXACT SANITIZED SNAPSHOT
```

Do not approve on the owner's behalf. Do not reuse the earlier approved snapshot from debate `debate-6ac0505d-bb73-4b19-af1f-855f7c438a0a`; it predates the timeout correction.

## Source state

```text
GIT_HEAD = 9642c0d1495f032241cf0dc54d55485884145e62
OLD_RUNTIME_CHECKPOINT = bbf339df4942ad95a433f2b8e3cc978ac4ca34f6
COMMITTED = NO
PUSHED = NO
```

Uncommitted files:

- `app/src/App.tsx`: clear retained per-debate UI state when creating a new debate.
- `crates/council-core/src/runner.rs`: run the Codex WSL cancellation fallback before joining output-reader threads, preventing the WSL pipe deadlock observed during timeout.
- `crates/council-core/src/model.rs`: raise only the Codex WSL default timeout from `180000` to `300000` ms. M0.8 evidence documented large-packet timings above the old budget.

The local persisted provider setting was also updated and verified:

```text
CODEX_WSL timeout_ms = 300000
model = gpt-5.6-luna
reasoning level = max
distribution = CouncilCodexWSL
user = council
CODEX_HOME = /home/council/.codex
```

## Verification already completed

```text
Rust format                 PASS
Rust workspace tests        49 passed, 0 failed
Rust workspace check        PASS
Frontend policy tests       9/9 passed
Frontend production build   PASS
Current privacy audit       PASS, 0 live-secret matches
History privacy audit       FAIL in this local checkout; 0 live-secret matches, 2 old metadata warnings, and one hidden Codex turn-diff ref retains an older handoff blob with personal paths
Pinned global verifier      VERIFIED with Rust toolchain 1.96.0-x86_64-pc-windows-msvc

## Sandbox-only verification completed

The current local state was checked without launching the app, taking focus, using screen automation, or dispatching any provider:

```text
CURRENT_FILES_SCANNED = 137
SNAPSHOT_MANIFEST_FILES = 108
SNAPSHOT_EXCLUSIONS = 35
SNAPSHOT_FILE_HASHES = PASS
SNAPSHOT_MANIFEST_IDENTITY = PASS
SNAPSHOT_EXCLUSION_IDENTITY = PASS
SNAPSHOT_SEALED_WRITE_TEST = PASS
CURRENT_DEBATE_STATE = SNAPSHOT_REVIEW_REQUIRED
CURRENT_REVIEW_DECISION = PENDING
CURRENT_TURNS = 0
CURRENT_DISPATCH_INTENTS = 0
```

The history-audit failure is confined to this local checkout's hidden `refs/codex/turn-diffs/...` capture, which retains an earlier version of this handoff containing personal Windows paths. The current handoff is sanitized, and the audit found no live-secret matches. Do not claim a public-history privacy pass until the final commit is checked in a clean public clone or the internal capture is handled under explicit authorization. No internal ref was deleted or rewritten.
```

## Exact rebuilt artifact

```text
INSTALLER = C:\council-target\release\bundle\nsis\Council of Agents_0.1.0_x64-setup.exe
INSTALLER_SIZE = 3492490 bytes
INSTALLER_SHA256 = 65529AFDEBD52C40B6BA37A559A4D08966B4BB7EF7068ED2462354CA43E1349A
APP_SHA256 = 2CE1F1A40172BAA69757AEDFE84A32D101172FA5A7C11629E7171BE0B7AFD01D
INSTALLED_PATH = %LOCALAPPDATA%\CouncilV1CertificationTimeout-65529AFD
CODE_SIGNING = NOT_SIGNED
```

The exact installed app was used to create the current debate and snapshot, then stopped. The app data remains in the normal local data directory.

## Prior live result that caused the correction

On the prior approved snapshot, Claude and Antigravity produced valid R1 positions. Codex WSL timed out at `180786 ms` while reading the sealed repository snapshot. The corrected runner fallback ran, no orphan WSL process remained, and the controller correctly left the debate failed rather than accepting a fake position.

## Resume sequence

1. When the owner is available, manually approve the current exact snapshot in the installed app.
2. Verify the persisted approval matches debate ID, snapshot ID, manifest SHA, exclusion-set SHA, and source-tree hash.
3. Dispatch R1 from the installed/native controller path with all three seats. Do not proceed degraded.
4. If all three R1 positions are valid, continue R2, R3, citation verification, WSL boundary checks, sealed-write denial, and the human decision gate.
5. Run the live Claude, Antigravity, and Codex cancellation tests plus interrupted-dispatch and restart tests.
6. Update the durable evidence files, rerun verification, commit, push, and verify GitHub Actions on the final commit.
7. Keep `V1_STATUS = RELEASE_CANDIDATE` until every mandatory gate has evidence. Keep `M1 = NOT_STARTED`.

## Sandbox boundary

The sandbox is appropriate for Rust tests, frontend tests/builds, privacy audits, database inspection, snapshot integrity checks, and controlled runner/provider-command diagnostics. It cannot replace the required live installed/native controller path for R1/R2/R3, subscription routing, provider cancellation, or the human snapshot and decision gates. Those steps should wait until the owner is free to operate the app manually.

## Screen-control rule for the next session

Do not use mouse, keyboard, focus, taskbar, or screenshot automation while the owner is working on another task. Start the installed app only after the owner says the live certification window is available. Read this handoff first and preserve the current pending snapshot rather than creating another one unnecessarily.
