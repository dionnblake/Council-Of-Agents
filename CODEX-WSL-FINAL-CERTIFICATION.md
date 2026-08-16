# Council of Agents: Codex WSL Final Certification

Date: 2026-08-15  
Scope: Final Codex WSL recovery and certification only. No M1 implementation, Council scaffolding, UI, debate engine, provider adapters, Claude changes, or Antigravity changes.

The native-Windows Codex failure remains valid historical evidence. This document certifies the replacement Codex seat built on the dedicated CouncilCodexWSL boundary.

## Executive Verdict

CODEX_WSL_PASS_WITH_DECLARED_LIMITATION

The dedicated WSL seat passed live authenticated isolation. No uncontrolled the owner-specific Windows context was reachable or observed during the certification calls. The remaining declared limitation is that the Codex event stream does not report the provider-served model identity.

## 1. Runtime

| Field | Result |
|---|---|
| WSL version | 2.7.3.0 |
| WSL kernel | 6.6.114.1-microsoft-standard-WSL2 |
| Distribution | CouncilCodexWSL, Ubuntu 24.04 LTS |
| Linux version | Ubuntu 24.04 LTS, Noble Numbat |
| Linux user | council, UID 1001 |
| HOME | /home/council |
| CODEX_HOME | /home/council/.codex |
| Codex version | codex-cli 0.147.0 |
| Codex executable | /home/council/.local/bin/codex |
| Node/npm | Node v18.19.1, npm 9.2.0 |
| Sandbox implementation | Codex read-only, with approval_policy = "never" |

The distro was imported from the official Ubuntu 24.04 WSL rootfs. Rootfs SHA-256:

~~~text
2a790896740b14d637dbdc583cce1ba081ac53b9e9cdb46dc09a2f73abbd9934
~~~

The existing Ubuntu development distro was not reused or modified.

## 2. Isolation

The dedicated distro uses:

~~~ini
[automount]
enabled=false
mountFsTab=false

[interop]
enabled=false
appendWindowsPath=false

[user]
default=council
~~~

| Boundary check | Result |
|---|---|
| /mnt/c mounted | No. /mnt/c was absent and not a mountpoint. |
| Windows interop | Disabled. powershell.exe, cmd.exe, and wsl.exe were not found inside the distro. |
| Windows PATH | Not inherited. The authenticated calls used a Linux-only allowlist. |
| Windows skill trees reachable | No. The Windows filesystem was not mounted and no Windows path was reachable through the tested boundary. |
| Linux custom user skills | None. No /home/council/.agents directory and no SKILL.md files were present. |
| Hooks | Disabled in the Council config; no hooks.json or hook surface was present. |
| Plugins/apps | Disabled in the Council config; no remote plugin or app surface was enabled. |
| MCP | No MCP configuration or server was present. |
| AGENTS/rule surfaces | No AGENTS.md, CLAUDE.md, GEMINI.md, .mcp.json, or hooks.json was found under the Linux Council home. |
| Raw-event contamination scan | Zero references to C:\Users\<USER>, .agents, .codex, the owner, AGENTS, Claude, Gemini, MCP, or hooks in retained Windows-side raw logs. |

The 50 KB packet call emitted one provider/system diagnostic item saying that skill descriptions had been shortened to fit a context budget. It did not contain a the owner path, did not identify a Linux user skill file, and was not accompanied by any user-skill, hook, plugin, MCP, or AGENTS load. It is recorded as a system-managed diagnostic, not as uncontrolled user-context contamination.

Final:

~~~text
ISOLATION = PASS
~~~

## 3. Authentication

| Check | Result |
|---|---|
| ChatGPT login status | Logged in using ChatGPT |
| Auth method | ChatGPT device authentication |
| API key present | VARIABLE_ABSENT |
| Custom base URL | VARIABLE_ABSENT |
| Custom provider | Not configured |
| Platform API login | Not used |

Human device authentication was completed in the dedicated Linux CODEX_HOME. No Windows auth.json was copied, no token was inspected, and no API key was used.

## 4. Clean Ambient Baseline

The decisive clean-room inference ran in:

~~~text
HOME=/home/council
CODEX_HOME=/home/council/.codex
working directory=/home/council/council/minimal
repository=none
packet=none
custom skills=none
hooks=none
MCP=none
plugins/apps=none
requested model=gpt-5.6-luna
~~~

Prompt:

~~~text
Return exactly CLEANROOM_OK.
~~~

Observed result:

| Metric | Result |
|---|---:|
| Exit code | 0 |
| Wall time | 5,652 ms |
| Output | CLEANROOM_OK |
| Input tokens | 7,276 |
| Cached input tokens | 0 |
| Cache-write input tokens | 0 |
| Output tokens | 8 |
| Reasoning output tokens | 0 |
| Reported served model | Not present in the event stream |

The event stream contained no references to the owner-specific Windows context. This is a valid clean ambient baseline. It is lower than the native-Windows contaminated observation of 11,344 input tokens and is not comparable to the earlier 30K+ contaminated observations except as evidence of the isolation improvement.

Retained raw files:

~~~text
%LOCALAPPDATA%\Temp\council-codex-wsl-cert\20260815-084548-minimal.stdout.jsonl
%LOCALAPPDATA%\Temp\council-codex-wsl-cert\20260815-084548-minimal.stderr.log
~~~

## 5. Sandbox

The test used a synthetic fixture at /home/council/council/test-snapshot, not a real repository.

| Test | Result |
|---|---|
| Read synthetic fixture | Pass. Returned READ_MARKER_TEST_SNAPSHOT. |
| Attempt to create PWNED.txt in read-only evidence | Write rejected. |
| Sandbox diagnostic | writing is blocked by read-only sandbox; rejected by user approval settings |
| PWNED.txt after the call | Absent |
| Normal reads | Pass |

Final:

~~~text
SANDBOX = PASS
~~~

## 6. Snapshot Bridge

The bridge used a controlled synthetic Windows-side snapshot. The real Windows repository was never mounted in WSL.

Final Linux snapshot:

~~~text
/home/council/council/snap/bridge-20260815-085218-fixed
~~~

The fixture included five files, including UTF-8 LF text, UTF-8 CRLF text, and binary bytes. The final source tree was sealed with directory mode 555 and file mode 444.

| Check | Result |
|---|---|
| File count | 5 |
| Windows manifest SHA-256 | 8ced118af4fa5561ace9544fa0b5bf9c909aa79cef7687dabea9e323ce8b4e2f |
| Linux manifest SHA-256 | Same value |
| Hash mismatches | 0 |
| Bad snapshot file modes | 0 |
| Bad snapshot directory modes | 0 |
| Byte preservation | Pass |

Final file hashes:

~~~text
README.md|41|d9dc1c1539337c49ca2725872bd97d327294a92e121ab95b13b56e03c8471695
data/blob.bin|10|bd8d89913e550baa17e3890f4649b1e1c5e1b86a2a8c1ceb4764e1b24c502282
docs/citation.md|100|12a56123949b0db40a0c3b5698ef59f2872e63e5ec48db3a8d238cbe0539ed04
src/core/manager.ts|99|6a0dbc95a3d1b242d8b705e41084361591a022f3fc8343310999bd1794c3d78f
src/workers/scheduler.ts|88|d3b9d064fd55708ba0ab939bad27ddd7ad52e33702193a842b651dc7208e2a21
~~~

Two early transfer/manifest attempts exposed mechanical test-harness defects: one did not create a nested data directory, and one PowerShell manifest helper used an invalid Replace overload. Neither attempt reached Codex, changed source bytes, or crossed the boundary. The final transfer used a binary-safe .NET WSL stdin bridge and passed the manifest comparison.

Final:

~~~text
SNAPSHOT_BRIDGE = PASS
~~~

## 7. Packet Delivery

Packets were exact-byte files under the sealed directory:

~~~text
/home/council/council/packet
~~~

The directory is mode 555; packet files are mode 444. Packet hashes:

~~~text
packet-50kb.md  cbc834ede2510aea49e4effc3575e8644260f005d5c661170d65017d0e923570
packet-200kb.md 297b7ccaee809fec600185717b22faf6065f605c0c005073e1087f5b0af59513
packet-500kb.md 5199030b1c75d45719f144a793b3ef408f0556feb5af6a7843792c6a542ca2ab
~~~

| Packet | Size | Marker recovered | Elapsed | Result |
|---|---:|---|---:|---|
| packet-50kb.md | 51,200 bytes | PACKET_END_50KB | 10,992 ms | PASS |
| packet-200kb.md | 204,800 bytes | PACKET_END_200KB | 7,110 ms | PASS |
| packet-500kb.md | 512,000 bytes | PACKET_END_500KB | 6,377 ms | PASS |

Each call was a fresh process with the same explicit requested model, read-only sandbox, sanitized environment, and no resumed provider session. No truncation or invalid marker was observed.

Final:

~~~text
PACKET = PASS
~~~

## 8. Schema Conformance

The final representative schema used the current Council commitment vocabulary:

~~~text
WOULD_STAKE
WOULD_NOT_STAKE
CONDITIONAL
~~~

Required fields were:

~~~text
recommendation
commitment
claims[]
risks[]
flip_condition
cost_if_wrong
reversibility
~~~

Claims required a strict path:startLine-endLine evidence value. Additional properties were rejected.

Schema file SHA-256:

~~~text
be7cc9a7c2becd924bfa97b1dc5d33e0b68df4dcd6ec43bbd093b33866be7295
~~~

Exactly 20 fresh Codex processes ran against the same read-only synthetic packet and schema.

| Category | Count |
|---|---:|
| Attempts | 20 |
| USABLE | 20 |
| SCHEMA_INVALID | 0 |
| SEMANTIC_INVALID | 0 |
| NO_STRUCTURED_OUTPUT | 0 |
| TRUNCATED | 0 |
| REFUSAL | 0 |
| TIMEOUT | 0 |
| MODEL_MISMATCH | 0 |
| PROCESS_FAILURE | 0 |
| Other failures | 0 |
| Usable percentage | 100% |

All outputs contained the required fields, legal commitment values, non-empty claims and risks, meaningful flip conditions, meaningful cost/reversibility fields, and syntactically valid evidence references.

Precommitted threshold application:

~~~text
100% usable >= 95%
Policy: NO_AUTOMATIC_REPAIR
~~~

Raw per-call JSONL, stderr, exit status, and wall-time files were retained at:

~~~text
/home/council/council/scratch/schema-cert-20260815-1
~~~

## 9. Stateless Reconstruction

The two turns used separate fresh Codex processes with --ephemeral. No session resume was used.

### Turn 1

| Field | Result |
|---|---|
| Exit code | 0 |
| Wall time | 19,259 ms |
| Commitment | WOULD_STAKE |
| Position | Open M1 with a small evidence-backed structured-position core; defer the interactive debate engine until validation. |
| Output usage | 25,161 input, 14,848 cached input, 676 output, 84 reasoning |

The exact Turn 1 output was stored before the process ended:

~~~text
/home/council/council/scratch/stateless-cert-20260815-1/turn1-agent-output.json
~~~

### Turn 2

Turn 2 received a packet containing the original question, the exact prior output, prior claims/evidence, and a peer claim requiring an interactive debate engine before M1.

| Requirement | Result |
|---|---|
| Prior position recovered | Yes |
| Prior commitment recovered | Yes, WOULD_STAKE |
| Peer claim understood | Yes |
| Peer challenge answered directly | Yes |
| Position preserved or revised coherently | Preserved coherently |
| Evidence cited | Yes |
| Hidden session resume used | No |
| Exit code | 0 |
| Wall time | 37,024 ms |
| Output usage | 36,862 input, 24,832 cached input, 1,606 output, 584 reasoning |

Packet and prior-output hashes:

~~~text
TURN1_PACKET_SHA256=4af01d5ee6ab53a3918cf9a60cec03b18f460e5bdd2b30efcc0be516ee9f8e36
TURN2_PACKET_SHA256=e18c92ac6bc0ab1e7f6ea655ce93765a75a82a0b0325f858f6c6785acd67058d
TURN1_OUTPUT_SHA256=9d0a700f1a540082f25b8a338f0f561c90d15cd3a015edb982b8fa10e5bc935a
~~~

Final:

~~~text
STATELESS = PASS
~~~

## 10. Citation Verification

The genuine citation call returned four model citations. The model used absolute Linux snapshot paths. The verifier normalized the known snapshot prefix to the relative manifest key without changing the cited file or line range.

| Genuine model citation | Normalized path/range | Verdict | File SHA-256 |
|---|---|---|---|
| .../README.md:1-2 | README.md:1-2 | VERIFIED_EXACT | d9dc1c1539337c49ca2725872bd97d327294a92e121ab95b13b56e03c8471695 |
| .../src/core/manager.ts:1-5 | src/core/manager.ts:1-5 | VERIFIED_EXACT | 6a0dbc95a3d1b242d8b705e41084361591a022f3fc8343310999bd1794c3d78f |
| .../src/workers/scheduler.ts:1-3 | src/workers/scheduler.ts:1-3 | VERIFIED_EXACT | d3b9d064fd55708ba0ab939bad27ddd7ad52e33702193a842b651dc7208e2a21 |
| .../docs/citation.md:3-3 | docs/citation.md:3-3 | VERIFIED_EXACT | 12a56123949b0db40a0c3b5698ef59f2872e63e5ec48db3a8d238cbe0539ed04 |

The range content hashes were also recorded. For example, the exact docs/citation.md:3-3 range hash was:

~~~text
c518810ccbf0928640ceb4b4ac071e0e11c1d87c6844b0bf2d43fc32207b2347
~~~

Planted controls:

| Control | Verdict |
|---|---|
| Exact real citation docs/citation.md:3-3 | VERIFIED_EXACT |
| Nearby/off-by-small-range docs/citation.md:4-4 containing content found at line 3 | VERIFIED_CONTENT_FOUND_ELSEWHERE |
| Nonexistent file missing/not-real.md:1-1 | UNVERIFIED |
| Out-of-range docs/citation.md:99-99 | UNVERIFIED |

Summary:

~~~text
Genuine exact citations: 4
Genuine shifted valid citations: 0
Genuine invalid citations: 0
Planted exact citations: 1
Planted shifted valid citations: 1
Planted invalid citations: 2
False accusations of hallucination: 0
~~~

The verifier did not classify the shifted-but-valid control as hallucinated.

## 11. Model Identity

Every certification call explicitly requested:

~~~text
REQUESTED_MODEL = gpt-5.6-luna
~~~

The available Codex JSONL event stream did not report a served-model field for the clean-room, packet, schema, stateless, or citation calls.

~~~text
REPORTED_SERVED_MODEL = NOT_REPORTED
SERVING_IDENTITY_STATUS = PROVIDER_DOES_NOT_REPORT
~~~

No verified model mismatch was observed. This is a declared limitation, not an isolation failure, because the provider and requested model were explicit and the certification rules allow this status when the provider does not expose served identity.

## 12. Process Cancellation

The cancellation test used harmless Linux sleep processes inside the dedicated distro. No real repository process and no Codex inference process was used for the cancellation fixture.

| Level | Result |
|---|---|
| Linux-side termination | PID 8 received graceful TERM; no hard kill was needed; no lingering process remained. |
| Windows fallback | PID 22 was running, then wsl --terminate CouncilCodexWSL completed in 97 ms. |
| Distro state after fallback | CouncilCodexWSL was not listed as running. |
| Restart check | No PID 22, no council-cancel-level2 process, and no lingering child remained. |
| Scope | Only the dedicated Codex distro was targeted. |

Final:

~~~text
PROCESS_CONTROL = PASS
~~~

## 13. Restart Persistence

After wsl --terminate CouncilCodexWSL, the distro was reopened and the following persisted:

| Check | Result |
|---|---|
| Linux user | council |
| HOME | /home/council |
| CODEX_HOME | /home/council/.codex |
| Codex executable | /home/council/.local/bin/codex |
| Codex version | codex-cli 0.147.0 |
| ChatGPT authentication | Persisted: Logged in using ChatGPT |
| Council config | Persisted and accepted by --strict-config |
| /mnt/c | Absent and not mounted |
| Windows interop | Still disabled |
| Windows PATH | Still absent |
| User skills | Still absent |
| Instruction/config surfaces | Still absent |

The authenticated status command wrote its status line to stderr in this CLI version, but exited successfully and reported Logged in using ChatGPT. No credential value was displayed or inspected.

## 14. Repair Policy

NO AUTOMATIC REPAIR

Codex produced 20/20 = 100% usable structured responses. The immutable >=95% threshold therefore selects validation and quarantine only. No automatic repair call was made.

## 15. Final Codex Certification

~~~text
CODEX
ISOLATION: PASS
AUTH: PASS
AMBIENT: PASS
SANDBOX: PASS
SNAPSHOT_BRIDGE: PASS
PACKET: PASS
SCHEMA: 20/20 = 100%
STATELESS: PASS
CITATIONS: PASS
PROCESS_CONTROL: PASS
MODEL_IDENTITY: PROVIDER_DOES_NOT_REPORT
REPAIR_POLICY: NO AUTOMATIC REPAIR
CERTIFICATION: PASS_WITH_DECLARED_LIMITATION
~~~

## 16. Declared Limitations and Blocker Status

1. Codex does not expose the served-model identity in the available event stream. The requested model is explicit, but the served-model status remains PROVIDER_DOES_NOT_REPORT.
2. One 50 KB packet event contained a system/provider skill-context-budget diagnostic. No the owner-specific path, user skill file, hook, MCP, plugin, or instruction surface was observed.
3. The bridge used a synthetic Windows-side snapshot by design. The real Windows repository was not mounted or exposed.
4. The native-Windows Codex route remains closed because its authenticated execution loaded real files from %USERPROFILE%\.agents\skills despite the tested 1,080 disable entries. The certified seat is the dedicated WSL architecture, not the native route.

There is no remaining P0 blocker for the dedicated Codex WSL seat or for the evidence-backed three-seat product under the declared limitations.

## 17. Final Three-Seat Decision

Claude evidence carried forward from the completed isolated certification:

~~~text
CLAUDE
ISOLATION: PASS
AUTH: PASS
AMBIENT: PASS
SCHEMA: 20/20 = 100%
PACKET: PASS
STATELESS: PASS
MODEL_IDENTITY: VERIFIED_MATCH
REPAIR_POLICY: NO AUTOMATIC REPAIR
CERTIFICATION: PASS
~~~

Antigravity evidence carried forward from the completed certification:

~~~text
ANTIGRAVITY
ISOLATION: PASS
AUTH: PASS
AMBIENT: PASS
SCHEMA: 17/20 = 85%
PACKET: PASS
STATELESS: PASS
MODEL_IDENTITY: PROVIDER_DOES_NOT_REPORT
REPAIR_POLICY: ONE REPAIR ATTEMPT
CERTIFICATION: PASS_WITH_DECLARED_LIMITATION
~~~

Certified Codex WSL result:

~~~text
CODEX
ISOLATION: PASS
AUTH: PASS
AMBIENT: PASS
SCHEMA: 20/20 = 100%
PACKET: PASS
STATELESS: PASS
MODEL_IDENTITY: PROVIDER_DOES_NOT_REPORT
REPAIR_POLICY: NO AUTOMATIC REPAIR
CERTIFICATION: PASS_WITH_DECLARED_LIMITATION
~~~

ALL THREE INDIVIDUAL SEAT RECORDS CERTIFIED

This record certifies individual seat boundaries only. It does not certify the V1 Tauri product, does not establish a current three-seat R1/R2/R3 debate, and does not open product M1.

Product M1 was not started by this pass.

## Verification and Safety Record

- Dedicated CouncilCodexWSL was the only new runtime boundary used.
- Existing Ubuntu, Claude, Antigravity, native Windows Codex, Windows skill trees, and Claude configuration were not modified.
- No real repository was mounted in WSL.
- No API key, token, password, or credential value was printed, copied, rotated, or inspected.
- Snapshot and packet bytes were hashed and compared.
- Evidence snapshot and packet files were sealed read-only.
- Raw certification outputs remain in the dedicated WSL scratch directories and the controlled Windows temporary evidence directory.
- The project verifier is not applicable to this findings-only workspace because it has no package.json, Cargo.toml, or pyproject.toml; no application source was created.

## Sources

- [OpenAI Codex configuration reference](https://developers.openai.com/codex/config-reference/)
- [OpenAI Codex sandbox and approvals](https://developers.openai.com/codex/sandbox/)
- [OpenAI Codex CLI](https://developers.openai.com/codex/cli/)
- [Microsoft WSL basic commands](https://learn.microsoft.com/en-us/windows/wsl/basic-commands)
- M0.8-FINDINGS.md for the closed native-Windows Codex result and carried-forward Claude/Antigravity evidence
- CODEX-WSL-CERTIFICATION.md for the pre-authenticated WSL recovery checkpoint
