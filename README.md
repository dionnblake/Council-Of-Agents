# Council of Agents

Council of Agents is a Windows-first, local-first technical deliberation application. It lets Claude Code, Antigravity CLI, and Codex CLI through the dedicated CouncilCodexWSL boundary independently analyze a technical decision, respond to peer claims, preserve dissent, and produce a human-reviewed decision record.

It is not an autonomous coding agent.

The enforced product boundary is:

~~~text
QUESTION
  -> INDEPENDENT ANALYSIS
  -> CROSS-EXAMINATION
  -> FINAL POSITIONS
  -> HUMAN DECISION
  -> DETERMINISTIC MASTER PROMPT
  -> MANUAL COPY
  -> STOP
~~~

## Current build

The current implementation build is `bbf339df4942ad95a433f2b8e3cc978ac4ca34f6`. V1 remains a `RELEASE_CANDIDATE`, not production-certified. Automated verification, the exact model/level persistence path, and the NSIS install/relaunch/uninstall lifecycle pass. A fresh current-head snapshot review, authenticated three-seat R1/R2/R3 run, human decision, export, and live provider cancellation/recovery remain unverified. M1 has not started. See [the V1 production certification record](docs/evidence/V1-PRODUCTION-CERTIFICATION.md).

The V1 implementation now includes:

- a Rust controller core with deterministic state transitions and a human decision gate;
- strict output-position validation, controller-owned claim IDs, semantic checks, and per-seat repair policy;
- byte-preserving snapshot, secret/config exclusion, reparse-point rejection, native ACL sealing, and mechanical citation verification;
- persisted `SNAPSHOT_REVIEW_REQUIRED` state with exact snapshot/manifest/exclusion-set binding, safe exclusion metadata, source-change invalidation, and explicit approve/reject IPC;
- immutable packet hashing, WSL stdin bridging, Linux payload hash verification, and separate scratch paths;
- fresh-process provider command contracts for Claude, Antigravity, and Codex WSL;
- SQLite persistence for debates, turns, attempts, raw artifacts, positions, packets, snapshots, decisions, exports, safety events, and a hash-chained append-only audit log;
- a Tauri 2 command bridge and React command center for intake, R0 bounded stack discovery, explicit round dispatch, turn visibility, recovery/degraded-seat choices, evidence review, human decision, and local deterministic export;
- provider-specific model and reasoning-level dropdowns, exact requested provider/model/level persistence, fail-closed exact-configuration status, and visible requested-versus-served identity details;
- migration-safe reload of the latest persisted turn statuses, including failed or partial seats that must never be presented as final decision positions;
- independent-only evaluation mode with deterministic citation/schema/repair/wall-time/peer-response/revision metrics, without silently treating a reduced council as a production result;
- exactly five top-level reasoning-only V1 skill packages.

Provider execution is opt-in from the desktop round controls. A live three-seat call is not run during ordinary builds or tests.

The current NSIS candidate was built from the pushed source with Tauri CLI 2.5.0:

~~~text
PATH = C:\council-target\release\bundle\nsis\Council of Agents_0.1.0_x64-setup.exe
SIZE = 3,491,307 bytes
SHA256 = D20BCBD83AFC910829A55D7743559862C933B241123B8B951FD2D6248A0B7F56
~~~

## Safety boundaries

- The real repository is never passed directly to providers.
- Repository snapshots are copied, hashed, scanned, and sealed read-only.
- Provider configuration and instruction surfaces are excluded from snapshots.
- Provider environments use explicit allowlists.
- The runner clears inherited variables before applying each command spec; configured API-key, custom-base-URL, and alternate-routing arguments fail closed.
- Codex runs only inside CouncilCodexWSL with /mnt/c and Windows interop disabled.
- Subscription authentication is used. API-key billing is not a Council fallback.
- Council never edits code, creates branches, commits, pushes, deploys, or opens another coding harness.

## Run the headless foundation

~~~powershell
$env:CARGO_TARGET_DIR = 'C:\council-target'
rustup run 1.96.0-x86_64-pc-windows-msvc cargo test --workspace
rustup run 1.96.0-x86_64-pc-windows-msvc cargo run -p council-cli -- providers
rustup run 1.96.0-x86_64-pc-windows-msvc cargo run -p council-cli -- demo --output .\artifacts\demo
Push-Location app
npm ci
npm test
npm run build
Pop-Location
~~~

The desktop shell is under app and uses Tauri 2 with a React/TypeScript frontend. The Tauri command bridge is built with:

~~~powershell
$env:CARGO_TARGET_DIR = 'C:\council-target'
rustup run 1.96.0-x86_64-pc-windows-msvc cargo check -p council-desktop
~~~

Launch the shell with the checked-in Tauri 2 CLI:

~~~powershell
Push-Location app
npx tauri --version
npx tauri dev --no-watch
Pop-Location
~~~

The native smoke path requires the Windows WebView2 runtime. Provider dispatch remains opt-in. Ambient host credentials are not a dispatch blocker or an input: they are excluded from the effective provider environment, while prohibited values persisted in provider configuration fail closed.

## Evidence

The feasibility and certification records remain in this root directory. The final Codex WSL certification is CODEX-WSL-FINAL-CERTIFICATION.md. The boundary contract is docs/security/BOUNDARY-CONTRACT.md, the current verification index is docs/evidence/README.md, and the V1 gate record is docs/evidence/V1-PRODUCTION-CERTIFICATION.md.
