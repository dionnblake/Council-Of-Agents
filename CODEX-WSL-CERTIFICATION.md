# Council of Agents: Codex WSL Isolation Recovery

**Date:** 2026-08-15  
**Scope:** Codex WSL isolation recovery and certification gate only. No M1 implementation, Council redesign, UI work, debate-engine work, or changes to Claude or Antigravity configuration.

## Decision

# CODEX STILL BLOCKS THREE-SEAT COUNCIL

The dedicated WSL2 boundary was created and hardened successfully. The Linux Codex CLI was installed from the Linux-side npm runtime. Certification cannot proceed because the clean WSL Codex home is not authenticated. The required human ChatGPT device-auth step has not been run, so no authenticated live inference was attempted.

The M1 opening decision remains:

```text
DO NOT START M1
```

This is a blocked recovery gate, not evidence that the WSL boundary failed. It is also not evidence that Codex WSL isolation passed during live inference. That decisive test remains unrun.

## Final certification fields

```text
CODEX_WSL
WSL2: PASS
LINUX_HOME: PASS
CODEX_HOME: PASS
ISOLATION: PASS_WITH_DECLARED_LIMITATION
AUTH: BLOCKED
SANDBOX: BLOCKED
AMBIENT: BLOCKED
SCHEMA: BLOCKED
PACKET: BLOCKED
STATELESS: BLOCKED
CITATIONS: BLOCKED
PROCESS_CONTROL: BLOCKED
MODEL_IDENTITY: BLOCKED
REPAIR_POLICY: NOT_APPLIED; existing thresholds unchanged
CERTIFICATION: BLOCKED
```

`ISOLATION` is marked `PASS_WITH_DECLARED_LIMITATION` only for the pre-authenticated WSL boundary checks. It must not be read as a live inference certification. The recovery rules require a zero-contamination authenticated inference before the seat can be certified.

## WSL2 boundary evidence

The pre-existing Ubuntu distro was not reused. It is a development distro with a populated `/home/<USER>`, Windows mounts, and `codex` resolving to the Windows npm shim. It remains unchanged.

The dedicated distro was imported as `CouncilCodexWSL` from the official Ubuntu 24.04 WSL rootfs:

```text
WSL version: 2.7.3.0
Kernel: 6.6.114.1-microsoft-standard-WSL2
Distro: Ubuntu 24.04 LTS
Distro version: 2
Import path: %LOCALAPPDATA%\council\wsl\CouncilCodexWSL
Rootfs SHA-256: 2a790896740b14d637dbdc583cce1ba081ac53b9e9cdb46dc09a2f73abbd9934
```

The distro was configured with:

```ini
[automount]
enabled=false
mountFsTab=false

[interop]
enabled=false
appendWindowsPath=false

[user]
default=council
```

After distro restart, the live shell checks returned:

```text
ID=council
HOME=/home/council
PWD=/home/council
PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/usr/lib/wsl/lib
/mnt/c: NOT_MOUNTED
powershell.exe: not found
Windows codex shim: not found before Linux installation
```

The new Linux home contained only the normal shell files and the empty Council Codex home. There was no `/home/council/.agents` tree and no Windows skill tree was reachable from the shell.

## Linux Codex installation evidence

Node and npm were installed inside `CouncilCodexWSL`, not inherited from Windows:

```text
node: v18.19.1
npm: 9.2.0
Codex package: @openai/codex@0.147.0
Codex executable: /home/council/.local/bin/codex
Codex version: codex-cli 0.147.0
CODEX_HOME: /home/council/.codex
```

The installation and pre-auth checks used a Linux-only environment allowlist. No Windows `CODEX_HOME`, Windows skills, Windows PATH entries, API key, access token, or Claude credential was copied into the distro.

## Authentication gate

The clean WSL Codex home is not authenticated:

```text
codex login status -> Not logged in
```

The installed CLI help confirms that the required device-auth option is available. Human browser/device interaction is required. Run this exact command from PowerShell, complete the ChatGPT device-auth flow, and do not provide an API key:

```powershell
wsl.exe -d CouncilCodexWSL --user council -- bash -lc '/usr/bin/env -i HOME=/home/council USER=council LOGNAME=council SHELL=/bin/bash PATH=/home/council/.local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin CODEX_HOME=/home/council/.codex /bin/bash --noprofile --norc -c "exec codex login --device-auth"'
```

After the flow completes, the required verification command is:

```powershell
wsl.exe -d CouncilCodexWSL --user council -- bash -lc '/usr/bin/env -i HOME=/home/council USER=council LOGNAME=council SHELL=/bin/bash PATH=/home/council/.local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin CODEX_HOME=/home/council/.codex /bin/bash --noprofile --norc -c "codex login status"'
```

Do not copy or inspect the token file. Report only the resulting status text. The WSL home must remain the only authentication source for the next pass.

## Tests intentionally not run

Because authenticated live inference is a prerequisite, the following tests were not run and are not passes:

| Gate | Result | Reason |
|---|---|---|
| Zero-contamination live inference | `BLOCKED` | No ChatGPT device auth in WSL |
| Codex ambient baseline | `BLOCKED` | No live inference usage envelope |
| Sandbox behavior | `BLOCKED` | No authenticated Codex execution |
| 20-call schema conformance | `BLOCKED` | Isolation gate precedes schema gate |
| File packet delivery, 50/200/500 KB | `BLOCKED` | No authenticated Codex execution |
| Stateless fresh-process pair | `BLOCKED` | No authenticated Codex execution |
| Citation verification | `BLOCKED` | No Codex output to verify |
| WSL cancellation and descendant control | `BLOCKED` | Process-control probe not started before auth |
| Network/billing behavior | `BLOCKED` | No authenticated inference |
| Snapshot bridge | `BLOCKED` | No reason to copy even a sanitized snapshot before auth |

No real repository was mounted or copied into the distro. The existing Windows snapshot, Windows Codex home, Windows skill trees, and Claude configuration were not modified.

## Carried-forward seat evidence

These values are carried forward from `M0.8-FINDINGS.md`; they were not re-run during this blocked WSL gate:

```text
CLAUDE
ISOLATION: PASS
AUTH: PASS
AMBIENT: PASS
SCHEMA: 20/20 = 100%
PACKET: PASS
STATELESS: PASS
MODEL_IDENTITY: VERIFIED_MATCH
REPAIR_POLICY: VALIDATION AND QUARANTINE ONLY
CERTIFICATION: PASS

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
```

The prior native-Windows Codex result remains a separate failure record: authenticated live inference loaded six real files from `%USERPROFILE%\.agents\skills` despite 1,080 explicit disable entries, producing a contaminated observation of 11,344 input tokens. This WSL pass does not erase that result and does not yet replace it with a certified result.

## Sources and prior evidence

- [Microsoft WSL basic commands](https://learn.microsoft.com/en-us/windows/wsl/basic-commands)
- [Microsoft install and import guidance for WSL](https://learn.microsoft.com/en-us/windows/wsl/install)
- [Official Ubuntu 24.04 WSL image index](https://cloud-images.ubuntu.com/wsl/releases/24.04/current/)
- [OpenAI Codex CLI getting started](https://help.openai.com/en/articles/11096431)
- [OpenAI Codex CLI ChatGPT sign-in guidance](https://help.openai.com/en/articles/11381614-api-codex-cli-and-sign-in-with-chatgpt)
- `M0.8-FINDINGS.md` in this workspace for the carried-forward Claude, Antigravity, and native-Windows Codex evidence

## Final line

CODEX STILL BLOCKS THREE-SEAT COUNCIL
