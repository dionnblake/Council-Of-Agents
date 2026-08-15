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

The V1 implementation now includes:

- a Rust controller core with deterministic state transitions and a human decision gate;
- strict output-position validation, controller-owned claim IDs, semantic checks, and per-seat repair policy;
- byte-preserving snapshot, secret/config exclusion, reparse-point rejection, native ACL sealing, and mechanical citation verification;
- immutable packet hashing, WSL stdin bridging, Linux payload hash verification, and separate scratch paths;
- fresh-process provider command contracts for Claude, Antigravity, and Codex WSL;
- SQLite persistence for debates, turns, attempts, raw artifacts, positions, packets, snapshots, decisions, exports, safety events, and a hash-chained append-only audit log;
- a Tauri 2 command bridge and React command center for intake, explicit round dispatch, turn visibility, human decision, and local deterministic export;
- exactly five top-level reasoning-only V1 skill packages.

Provider execution is opt-in from the desktop round controls. A live three-seat call is not run during ordinary builds or tests.

## Safety boundaries

- The real repository is never passed directly to providers.
- Repository snapshots are copied, hashed, scanned, and sealed read-only.
- Provider configuration and instruction surfaces are excluded from snapshots.
- Provider environments use explicit allowlists.
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
npm install
npm run build
Pop-Location
~~~

The desktop shell is under app and uses Tauri 2 with a React/TypeScript frontend. The Tauri command bridge is built with:

~~~powershell
$env:CARGO_TARGET_DIR = 'C:\council-target'
rustup run 1.96.0-x86_64-pc-windows-msvc cargo check -p council-desktop
~~~

Launch the shell with the Tauri 2 CLI when it is installed. The current repository verifies the frontend through Vite; the Tauri CLI itself is an environment prerequisite.

## Evidence

The feasibility and certification records remain in this root directory. The final Codex WSL certification is CODEX-WSL-FINAL-CERTIFICATION.md. The boundary contract is docs/security/BOUNDARY-CONTRACT.md, and the current verification index is docs/evidence/README.md.
