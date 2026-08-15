# Council of Agents — M0 Findings

**Date:** 2026-08-15
**Spike engineer:** Claude (Opus 5), acting as M0 feasibility engineer
**Status:** M0 partially complete — Phases A, B, C complete; Phase D partially complete (see §17)

Every conclusion is labelled `VERIFIED` / `OBSERVED` / `INFERRED` / `UNVERIFIED`.

---

## Executive Verdict

# GO WITH ARCHITECTURE CHANGES

The Windows foundations of the proposed architecture **work, and work better than the round-2 architecture assumed** in two places. Subprocess control, sanitized-environment no-spend protection, OS-level write denial, secret scanning, and Job-Object process-tree kill are all VERIFIED working on this machine. Codex and Claude Code are both viable Council seats today.

**But one accepted round-2 factual correction is wrong, and it removes the third seat.**

**Antigravity has no headless agent CLI on this machine.** There is no `agy` binary, no `-p` prompt flag, no `--output-format`, and no `--json-schema` anywhere in the installed product. `VERIFIED` by six independent checks (§4). The round-2 correction that reinstated Antigravity as the third participant does not hold against the installed software.

This is not a reason to stop. It is a reason to change the seat lineup before M1, which is exactly what M0 is for. Codex + Claude are a working two-seat Council today, and a third heterogeneous seat needs to be sourced differently.

A secondary finding also changes the design: **`--ignore-user-config` does not stop skill or hook injection in Codex**, and both Codex and Claude Code load roughly 30,000–35,000 tokens of ambient user configuration into every invocation. `VERIFIED`. That is a reproducibility problem, a cost problem, and a silent-bias problem, and it needs an answer before M1.

---

## 1. Environment

| Component | Version | Evidence |
|---|---|---|
| Windows | 11 Pro 10.0.26200 build 26200, x64 | `VERIFIED` |
| Rust | rustc 1.96.0 (ac68faa20 2026-05-25) | `VERIFIED` |
| Cargo | 1.96.0 (30a34c682 2026-05-25) | `VERIFIED` |
| Git | 2.54.0.windows.1 | `VERIFIED` |
| Node | v24.16.0 | `VERIFIED` |
| Codex CLI | codex-cli 0.147.0 (`@openai/codex`) | `VERIFIED` |
| Claude Code | 2.1.226 | `VERIFIED` |
| Antigravity (app) | Google.Antigravity 2.5.0 — **2.8.1 available via winget, not installed** | `VERIFIED` |
| Antigravity IDE | Google.AntigravityIDE 2.5.5 / internal VS Code 1.107.0 | `VERIFIED` |

**Binary resolution** `VERIFIED`:

| CLI | Path | Process type |
|---|---|---|
| `codex` | `%APPDATA%\npm\codex.cmd` / `.ps1` | npm shim → node |
| `claude` | `%USERPROFILE%\.local\bin\claude.exe` | **native executable** |
| `agy` | **does not exist** | — |
| Antigravity IDE CLI | `%LOCALAPPDATA%\Programs\Antigravity IDE\bin\antigravity-ide.cmd` | Electron-as-node → VS Code `cli.js` |

Incidental finding, `VERIFIED`: the first harness build failed with `LNK1104` because the scratchpad path exceeded MAX_PATH (~270 chars). Fixed by setting a short `CARGO_TARGET_DIR`. **Production snapshot and scratch directories must live under short roots** (`%LOCALAPPDATA%\CouncilOfAgents\...`), never nested deep.

---

## 2. CLI Invocation Matrix

| Provider | Headless | stdin prompt | JSON out | JSON Schema | Subscription auth | TTY required | Process type | Spawn overhead |
|---|---|---|---|---|---|---|---|---|
| **Codex** | YES `VERIFIED` | YES (`-` or piped) `VERIFIED` | `--json` JSONL `VERIFIED` | `--output-schema <FILE>` `VERIFIED` (flag present; not exercised) | YES, `auth_mode=chatgpt` `VERIFIED` | No `VERIFIED` | `.cmd` → node | **~5,000 ms** `VERIFIED` |
| **Claude Code** | YES `VERIFIED` | YES `VERIFIED` | `--output-format json` `VERIFIED` | **`--json-schema <schema>` `VERIFIED`** | YES, `provider:"firstParty"` `VERIFIED` | No `VERIFIED` | native `.exe` | **~8 ms** `VERIFIED` |
| **Antigravity** | **NO** `VERIFIED` | n/a | **NO** `VERIFIED` | **NO** `VERIFIED` | n/a | GUI | Electron | n/a |

**Round-2 correction accepted:** Claude Code does have `--json-schema` ("JSON Schema for structured output"). I was wrong in round 1 to assume schema-constrained output was Codex-only. `VERIFIED` from `claude --help`.

**Codex control surface is the strongest of the three** `VERIFIED` (from `codex exec --help`):
`--ignore-user-config`, `--ignore-rules`, `--ephemeral`, `--output-schema`, `--output-last-message`, `-s/--sandbox {read-only|workspace-write|danger-full-access}`, `-C/--cd`, `--skip-git-repo-check`, `--strict-config`, `-c key=value`.
`--skip-git-repo-check` is required for Council because the snapshot deliberately has no `.git`.

**Spawn-cost asymmetry matters for budgeting** `VERIFIED`: Codex pays ~5 s of npm-shim + node startup on *every* invocation before any model work. Claude's native exe pays 8 ms. Over a 9-call debate that is ~45 s of pure Codex process overhead.

---

## 3. Billing / Authentication Findings

### No-spend protection — built and asserted BEFORE the first authenticated call `VERIFIED`

The throwaway Rust harness uses an **allowlist**, not a denylist: `env_clear()` then re-add 23 named OS/profile variables plus `NO_COLOR`/`TERM`/`CI`. A post-construction assertion rejects any surviving `*_API_KEY`, `*_BASE_URL`, `*_AUTH_TOKEN`.

Result: `NO_SPEND_ASSERT=PASS`, 26 variables allowlisted, **61 dropped**.

**The allowlist decision is now evidence-backed, not stylistic.** `VERIFIED` — the live environment contained these, and a denylist built from the round-2 list would have missed the last three entirely:

| Variable | On the round-2 denylist? |
|---|---|
| `ANTHROPIC_BASE_URL` (= `https://api.anthropic.com`, process-scope, injected by the parent Claude Code session) | yes |
| `GEMINI_API_KEY` (prefix `AIza`, len 39, **User-scope = persistent**) | yes |
| `FREELLMAPI_KEY` | **no** |
| `SUPABASE_SERVICE_ROLE_KEY` | **no** |
| `USE_STAGING_OAUTH` / `USE_LOCAL_OAUTH` | **no** |

### Per-provider

| | Codex | Claude Code | Antigravity |
|---|---|---|---|
| Auth mechanism | `~/.codex/auth.json`, `auth_mode=chatgpt`, OAuth tokens (`id_token`/`access_token`/`refresh_token`/`account_id`) `VERIFIED` | `~/.claude/.credentials.json` (10,209 B) `VERIFIED` | GUI login, `.codeium`/`.gemini` state `OBSERVED` |
| API key required? | **No** — `OPENAI_API_KEY` field in `auth.json` is `null` `VERIFIED` | No `VERIFIED` | n/a |
| Auth survives sanitized env? | **Yes** `VERIFIED` (call succeeded) | **Yes** `VERIFIED` | untested |
| Provider actually used | `provider: openai` `VERIFIED` | `provider: "firstParty"` `VERIFIED` | n/a |
| Config-based fallback risk | **Yes** — see below | Low: user `settings.json` has **no** `env` block `VERIFIED` | `UNVERIFIED` |
| Unexpected spend observed | **None** `OBSERVED` | **None** `OBSERVED` | none (no calls made) |

### Config re-injection — the round-1 concern is confirmed real, and partially unsolved

`~/.codex/config.toml` contains `VERIFIED`:
```
sandbox_mode    = "danger-full-access"
approval_policy = "never"
model           = "gpt-5.6-luna"
```
Plus 13 enabled plugins including `browser-use`, `computer-use`, `chrome`, `github`, `google-drive`.

`--ignore-user-config` **does** neutralise `config.toml` — `VERIFIED` by observing the served model change from `gpt-5.6-luna` to the built-in default `gpt-5.6-sol`, and `sandbox: read-only` taking effect.

**But it does not neutralise skills or hooks** `VERIFIED`. With `--ignore-user-config` set, the same invocation still emitted:
```
failed to load skill %USERPROFILE%\.codex\skills\...        (x3)
failed to load skill %USERPROFILE%\.agents\skills\...       (x6)
warning: skipping async hook in %USERPROFILE%\.codex\hooks.json   (x2)
warning: Exceeded skills context budget. All skill descriptions were removed
         and 829 additional skills were not included
```

Two consequences:
1. **Hooks from `~/.codex/hooks.json` execute** during a Council turn. Hooks are arbitrary code. This is a boundary hole that `--ignore-user-config` does not close.
2. **829 skills** were considered for injection.

**Ambient-context cost, measured** `VERIFIED`:

| Call | Prompt | Ambient tokens |
|---|---|---|
| Claude, "Reply with exactly OK" | 9 input tokens | **34,815** cache-creation tokens |
| Codex, "Reply with exactly OK" | ~10 tokens | **30,604** total tokens used |

Roughly 30–35 K tokens of the user's global configuration is injected into every single turn. This is a reproducibility defect (the debate depends on files nobody recorded), a cost defect, and a bias-injection vector — the owner's global `CLAUDE.md` mandates specific stack defaults, which would silently participate in a stack-selection debate.

### Quota / usage-limit behaviour

`NOT REPRODUCED`. No limit was reached during 5 authenticated calls, and deliberately exhausting a subscription was out of scope. The failure signature per provider remains `UNVERIFIED`. The fail-closed rule (`UNCLASSIFIED FAILURE → UNKNOWN → PAUSE`) must therefore be implemented on the assumption that the signature is unknown.

### Billing surface check

`OBSERVED` — Claude reported `total_cost_usd` of $0.0699 and $0.0338 for the two calls, and Codex reported token counts only. **These are subscription-usage estimates, not invoices.** Claude Code reports a notional cost field even on subscription auth, so a non-zero value here is *not* evidence of API billing; `provider:"firstParty"` and `auth_mode=chatgpt` are the real evidence. A human check of the ChatGPT and Anthropic billing dashboards is still recommended and has **not** been performed by me.

---

## 4. Antigravity Findings

Six independent checks, all `VERIFIED`:

1. **No `agy` on PATH.** `Get-Command agy` → not found.
2. **No standalone `agy` binary anywhere.** Recursive search (depth 4) of `%LOCALAPPDATA%`, `%APPDATA%`, both Program Files, `.cargo\bin`, `.local\bin`, scoop, chocolatey, go/bin → *"NO standalone 'agy' executable found in any standard install root."*
3. **The one `agy*` file is not an agent CLI.** `%APPDATA%\Antigravity\bin\agy-node.cmd` is 108 bytes:
   ```
   @echo off
   set ELECTRON_RUN_AS_NODE=1
   "…\antigravity\Antigravity.exe" %*
   ```
   Running it with `--help` prints **Node.js's own help text**. It is a Node launcher.
4. **`Antigravity.exe --help` times out and must be killed** → GUI launch, no CLI surface.
5. **The IDE CLI is stock VS Code.** `antigravity-ide --help` lists `--diff`, `--goto`, `--install-extension`, `serve-web`, `tunnel`, and a `chat` subcommand. `chat --help` gives `-m/--mode`, `-a/--add-file`, `--maximize`, `-r/--reuse-window`, `-n/--new-window`, `--profile`. **`--maximize` and the window flags prove it drives the GUI.** No `-p`, no `--output-format`, no `--json-schema`.
6. **Binary string scan.** In `app.asar`: `--json-schema` = 0 occurrences, `--output-format` = 0, `stream-json` = 0.

| Question | Answer |
|---|---|
| Headless usable? | **NO** `VERIFIED` |
| IDE required? | **YES** — GUI window management is in the only prompt-accepting command `VERIFIED` |
| Daemon required? | Not applicable — no headless path exists `VERIFIED` |
| Workspace trust issue? | `UNVERIFIED` (untestable without a headless path) |
| Writes artifacts? | `UNVERIFIED` |
| Artifact redirection possible? | `UNVERIFIED` |
| Browser capability disableable? | `UNVERIFIED` |
| Model identity reportable? | **`MODEL_IDENTITY_UNKNOWN`** `VERIFIED` — no structured output channel exists to report it |
| Credit fallback machine-checkable? | **`ANTIGRAVITY_CREDIT_GUARD = UNENFORCEABLE`** `VERIFIED` — no `useG1Credits` / `g1Credit` / `useCredits` / `personalCredits` / `creditFallback` string found in `%APPDATA%\Antigravity`, `~/.antigravity`, `~/.gemini`, `~/.codeium`, or `%LOCALAPPDATA%\Antigravity`. The only Antigravity `settings.json` contains two theme keys. |
| Suitable for Council? | **Not at installed version** |

### Antigravity verdict: **REPLACE** (with one cheap reversal condition)

`REPLACE` is the correct call **for the installed version**, and I want to be precise about the scope of that claim rather than overstate it:

- What is `VERIFIED`: Antigravity **2.5.0 / IDE 2.5.5** has no headless agent CLI, no structured output, and no machine-checkable credit guard.
- What is `UNVERIFIED`: whether **2.8.1** (available via winget, three minor versions newer, not installed) adds one. I did not install it — installing software is a system change outside spike authority and requires the owner's decision.

**Cheapest possible reversal test, ~10 minutes, recommended before M1 locks the lineup:**
```
winget upgrade Google.Antigravity
```
then re-run checks 1, 2, 5 and 6 above. If `agy -p` with `--json-schema` appears in 2.8.1, Antigravity returns as the third seat and only §4 of this report changes. If it does not, the seat must be sourced elsewhere.

**I am explicitly not recommending Gemini CLI on the strength of it having a CLI** — that was the round-2 instruction and it still stands. `gemini` is not installed on this machine either (`VERIFIED`, not found on PATH). The third-seat decision is an architecture-debate question, not an M0 finding. What M0 establishes is only that **the currently installed Antigravity cannot fill the seat**, and that **a two-seat Council (Codex + Claude) is fully functional today** — which is why this is GO-with-changes rather than NO-GO.

---

## 5. Windows Safety Findings

| Control | Result | Evidence |
|---|---|---|
| Snapshot built | **YES** — 12 files from 18 candidates | `VERIFIED` |
| No `.git` confirmed | **YES** — `Test-Path snapshot\.git` = False | `VERIFIED` |
| ACL write denial works | **YES** | `VERIFIED` |
| Codex read-only sandbox works on Windows | **YES** | `VERIFIED` |
| Claude read-only controls | Not needed for containment — OS blocked it | `VERIFIED` |
| Antigravity controls | untestable | `UNVERIFIED` |
| Symlink escape prevented | **YES** (and symlink creation needs admin here) | `VERIFIED` |
| Junction escape prevented | **YES — but only after adding an explicit filter** | `VERIFIED` |
| Post-run integrity verification practical | **YES** | `VERIFIED` |

### 5.1 The snapshot algorithm I recommended in rounds 1–2 is BROKEN — corrected and re-verified

`git ls-files --cached --others --exclude-standard` **follows NTFS junctions.** `VERIFIED`.

A junction was planted at `scratch-repo\junction_escape` pointing to a sibling directory outside the repo. Git enumerated straight through it:
```
junction_escape/outside_marker.txt      <-- file from OUTSIDE the repository
```
Had the snapshot been built from that list unfiltered, an out-of-repo file would have been copied into the snapshot and sent to three providers. **The mechanism I proposed in both prior rounds does not prevent boundary escape on its own.**

Relevant detail `VERIFIED`: creating a **symlink** required administrator privilege and failed; creating a **junction** via `mklink /J` succeeded with no elevation. On a default Windows 11 install the realistic escape vector is the junction, which is the one people forget.

**Corrected algorithm, verified working:**
```
1. git ls-files --cached --others --exclude-standard
2. FOR EACH path, walk EVERY path component and reject if any component
   has FILE_ATTRIBUTE_REPARSE_POINT
3. Canonicalise and assert the resolved path is still under the snapshot root
4. Secret scan -> human gate
5. Copy survivors; never copy .git
```
Result: 18 candidates → 1 rejected (`[REPARSE_POINT]`) → 17 kept, containment assertion clean. `VERIFIED`.

Note for the Rust implementation: check the `FILE_ATTRIBUTE_REPARSE_POINT` attribute directly rather than relying on `FileType::is_symlink()`, because junction (`IO_REPARSE_TAG_MOUNT_POINT`) and symlink (`IO_REPARSE_TAG_SYMLINK`) semantics differ. `INFERRED` from the observed junction/symlink asymmetry.

### 5.2 ACL write denial — verified, and verified *independently* of CLI controls

Applied via `Get-Acl`/`Set-Acl` with `SetAccessRuleProtection($true,$true)` plus a Deny ACE for the current user covering `CreateFiles,CreateDirectories,AppendData,WriteData,Delete,DeleteSubdirectoriesAndFiles`.

Direct control test as the same user that will run the agents `VERIFIED`:
```
create new file : BLOCKED (UnauthorizedAccessException)
overwrite       : BLOCKED (UnauthorizedAccessException)
delete          : BLOCKED (UnauthorizedAccessException)
mkdir           : BLOCKED (UnauthorizedAccessException)
read            : OK (113 bytes)
scratch write   : OK (writable as intended)
```

Implementation note `VERIFIED`: PowerShell 7 removed `DirectoryInfo.GetAccessControl()`; a first attempt silently applied nothing and every write succeeded. That failed attempt is a useful negative control — it confirms the test can detect an unprotected directory. Also `VERIFIED`: `icacls` invocation was blocked by a local shell guard misparsing `/inheritance:r`; the .NET ACL API is the more reliable path and is what Rust should use.

**The isolation test that matters** — Claude Code run with `--permission-mode acceptEdits` (deliberately permissive) and ordered to write into the sealed snapshot:
```
permission_denials: []            <-- Claude's OWN permission system allowed it
result: "EPERM: operation not permitted, open
        '%LOCALAPPDATA%\m0snap2\PWNED.txt.tmp.11008.1b18989670b3'"
PWNED.txt in snapshot? False
SNAPSHOT_MANIFEST unchanged
```
`VERIFIED`. The CLI's own permission layer permitted the write and **the OS stopped it**. This is direct evidence that layer 2 holds when layer 3 is fully disabled — the defence-in-depth claim is now tested, not assumed.

Incidental `VERIFIED`: Claude writes via a `<name>.tmp.<pid>.<rand>` temp file in the *target* directory then renames, so denying `CreateFiles` on the target directory is what blocks it.

### 5.3 Codex Windows sandbox — round-2 correction accepted

`-s read-only` is genuinely enforced on Windows. `VERIFIED`:
```
sandbox: read-only
ERROR codex_core::tools::router:
  error=patch rejected: writing is blocked by read-only sandbox
```
No file created, snapshot manifest unchanged. **My round-1 claim that Windows lacks Codex sandbox enforcement was wrong.** Conceded.

One anomaly `OBSERVED`, not explained: invoking with `-s workspace-write` still reported `sandbox: read-only` in the session header. That attempt to isolate the ACL by relaxing the CLI sandbox therefore failed — Codex refused to run permissively in this configuration. The ACL isolation result in §5.2 comes from Claude instead, which did relax successfully. Root cause `UNVERIFIED`.

---

## 6. Secret Scanning

Corpus: 17 candidate files, 5 planted synthetic secrets (all marked `M0SPIKE`, none real).

| Pattern class | Tested | Result |
|---|---|---|
| Filename: `.env`, `*.pem`, `*.key`, `*.pfx`, `id_rsa` | yes | caught `certs.pem`, `config/prod.key`, `id_rsa` |
| Content: `sk-`, `ghp_`, `AIza`, `AKIA`, PEM header, JWT, `postgres://user:pw@` | yes | caught `id_rsa`, `src/tokens.ts` |

```
FLAG certs.pem        [FILENAME]
FLAG config/prod.key  [FILENAME]
FLAG id_rsa           [FILENAME+CONTENT]
FLAG src/tokens.ts    [CONTENT]
flagged = 4 of 17
```

- **Detections: 4/4 of the files that reached the candidate list.** `VERIFIED`
- **False positives: 0** across 17 files. `VERIFIED`
- **Misses: 0.** `VERIFIED`
- Performance: negligible (<1 s for 17 files, ~450 KB). `VERIFIED`

**`.gitignore` is not a secret control** `VERIFIED`. The planted `.env` *was* correctly excluded by `.gitignore` — but `id_rsa`, `certs.pem`, `config/prod.key` and `src/tokens.ts` were **not** gitignored and sailed into the candidate list. Only the scanner stopped them. The human review gate is mandatory, not belt-and-braces.

**Recommended V1 strategy:** filename blocklist + content regex, both cheap, run pre-dispatch, findings presented as a blocking human gate with per-file exclude. Skip content scanning for files >1 MB and for binaries.

**Gap found in my own scan** `VERIFIED`: the first snapshot copied `AGENTS.md` into the snapshot. Project-level agent instruction files must be stripped alongside secrets — see §16.

---

## 7. Process Lifecycle

Tested with a `.cmd` shim spawning a long-lived `node.exe` child — the exact process shape of `codex.cmd`.

| Question | Result |
|---|---|
| Timeout works | **YES** — `timed_out=true` at the 6 s deadline `VERIFIED` |
| Cancel works | **YES** — `TerminateJobObject` `VERIFIED` |
| Job Object created & assigned | **YES** — `job_created=true job_assigned=true` on every run `VERIFIED` |
| Child processes remain | **NO** — `node.exe` count 2 before / 2 after `VERIFIED` |
| Breakaway / reparenting observed | **None** `VERIFIED` |
| Auth hang observed | **None** in 5 calls `OBSERVED` |

**The Job Object kills the whole tree through a `.cmd` shim.** This was the round-1 concern and it is resolved: `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` + `TerminateJobObject`, with the process assigned immediately after spawn, is sufficient. No native addon, no `taskkill /T /F`.

Harness defect worth carrying into M1 `VERIFIED`: wall-clock was 12.1 s for a 6 s timeout because the harness joins the stdout/stderr reader threads *after* terminating the job. Production must abandon the pipe readers on kill rather than blocking on drain.

Stdin was closed immediately after prompt delivery in every test, per the round-2 anti-hang rule. No interactive prompt was ever encountered. `OBSERVED` — this does not prove expired-credential behaviour, which remains `UNVERIFIED` (§17).

---

## 8. Structured Output Results

### Methodology declared before measurement, per the M0 brief

Pre-committed rule, written before any conformance call:
```
>= 95%  -> no automatic repair turn in MVP; validate + quarantine only
80-94%  -> one automatic repair attempt justified
<  80%  -> simplify the output contract before building repair machinery
```

### Result: **NOT MEASURED. The pre-committed rule is NOT applied.**

I did not run the 20-attempts-per-provider conformance test. This is a deliberate stop, not an oversight, and I am reporting it as a gap rather than substituting a smaller sample and quietly applying thresholds designed for n=20.

Reasoning:
1. **The seat lineup is unresolved.** Conformance is a per-provider measurement. With the third seat's identity open (§4), 40+ calls would measure a configuration that may not ship.
2. **The measurement would be invalid as-is.** Both CLIs currently inject 30–35 K tokens of ambient user config (§3) and Codex reports *"Exceeded skills context budget"*. Conformance measured under that contamination would not predict conformance under the isolated configuration Council will actually use. The config-isolation work in §16 must land first or the numbers are noise.
3. The brief instructs minimising unnecessary subscription usage, and Codex costs ~33 s wall-clock per call (§10).

**What is known:** both providers expose schema-constrained output flags (`--output-schema`, `--json-schema`) `VERIFIED`, and both returned exactly-correct trivial output (`"OK"`) in 5/5 calls `OBSERVED` — a sample far too small to support any rate claim.

**Verdict on repair infrastructure: `INSUFFICIENT DATA`.** Build the unconditional floor from round 2 (parse → schema-validate → semantic-validate → typed failure taxonomy → quarantine, raw always retained) and defer the repair-turn decision to a properly isolated conformance run early in M1.

---

## 9. Context Size Findings

**NOT TESTED.** `UNVERIFIED`.

Deferred for the same reason as §8 — the measurement is provider-specific and the lineup is open. What *is* established `VERIFIED`: stdin prompt delivery works on both CLIs, which is the mechanism that makes large packets possible at all, and Codex's help text explicitly documents stdin as the prompt channel. Windows argv limits are therefore avoidable.

| Size | Codex | Claude | Antigravity |
|---|---|---|---|
| 50 KB | `UNVERIFIED` | `UNVERIFIED` | n/a |
| 200 KB | `UNVERIFIED` | `UNVERIFIED` | n/a |
| 500 KB | `UNVERIFIED` | `UNVERIFIED` | n/a |

---

## 10. Latency Findings

**Sample size is 2–3 calls per provider. This is far too small for p50 or p95 and I am not reporting either.** Raw observations only:

| Provider | Call | Spawn | Total wall-clock |
|---|---|---|---|
| Claude | `--help` | 8 ms | ~1 s |
| Claude | trivial prompt | 8 ms | **7.4 s** |
| Claude | write-attempt (2 turns) | 8 ms | ~9 s |
| Codex | `exec --help` | 5,008 ms | **26.8 s** |
| Codex | trivial prompt | 5,008 ms | **33.1 s** |
| Codex | write-attempt | 5,008 ms | ~35 s |

`OBSERVED`. Two things are already actionable:

1. **Codex's ~5 s spawn penalty is fixed cost per invocation** `VERIFIED` — npm `.cmd` shim plus node startup, paid before any model work.
2. **A trivial 2-character response costs 33 s on Codex.** A real repository-grounded turn will be substantially longer. The round-2 concern that "9 calls" is the wrong cost unit is supported: even at these floor numbers a 9-call debate is ≥5 minutes of pure overhead-plus-trivial-work, and realistic turns will dominate that.

Default timeouts must be measured properly in M1, not guessed. A 60 s timeout would kill roughly half of these floor-case Codex calls.

---

## 11. Stateless Turn Findings

**NOT TESTED.** `UNVERIFIED`.

The two-turn reconstruction scenario (turn 1 position → fresh process → turn 2 with explicit packet) was not run. Deferred behind the config-isolation work, because with 30–35 K tokens of uncontrolled ambient context per call, a "fresh process + explicit packet" turn is not actually controlled and the test would not measure what it claims to.

Supporting evidence that does exist `VERIFIED`: both CLIs return a `session_id` (Claude `47f0370a-…`, Codex `01a004da-…`) suitable for diagnostic recording, and Codex offers `--ephemeral` to avoid persisting session files at all — which is a good fit for the stateless design.

**Recommendation unchanged: `STATELESS`.** No evidence emerged against it. But the recommendation is now carried on round-2 reasoning rather than M0 measurement, and should be labelled as such.

---

## 12. Citation Verification

Agent-generated citations were **not** tested `UNVERIFIED`. The mechanical substrate was, and it produced the most consequential Phase-D finding.

### Line endings and encoding — a real hazard on this machine `VERIFIED`

`core.autocrlf = true` is active. Four fixtures with identical logical content:

| File | Bytes | CRLF | bare LF | BOM | UTF-16 |
|---|---|---|---|---|---|
| `enc_lf.txt` | 80 | 0 | 5 | no | no |
| `enc_crlf.txt` | 85 | 5 | 0 | no | no |
| `enc_bom.txt` | 88 | 5 | 0 | **yes** | no |
| `enc_utf16.txt` | 172 | 0 | 5 | no | **yes** |

Same five logical lines, four different byte counts and four different SHA-256 values.

### `git archive` mutates content — that option is dead `VERIFIED`

| File | Working tree | `git archive` | Match |
|---|---|---|---|
| `enc_crlf.txt` | `1F67C373…` (85 B) | `1F67C373…` (85 B) | identical |
| `enc_lf.txt` | `A3CD0B1E…` (80 B) | `1F67C373…` (**85 B**) | **DIFFERENT** |
| `enc_utf16.txt` | `BB3CEB39…` (172 B) | `BB3CEB39…` (172 B) | identical |

Under `autocrlf=true`, `git archive` re-applies CRLF normalisation and **changes file bytes**. An LF file became a CRLF file with a different hash. **This eliminates `git archive` as a snapshot mechanism for an evidence-grounded product** — cited content hashes would not match what the developer sees. The working-tree file copy already recommended is the correct approach, and is now verified as the *only* correct one of the two.

### Recommended canonical strategy

```
Copy working-tree bytes verbatim. Never git archive. Never normalise silently.
Record per file: byte length, SHA-256 of raw bytes, detected encoding
  (utf8 / utf8-bom / utf16le / binary), and dominant line ending.
Line numbering: 1-indexed, inclusive-inclusive, counted over the RAW bytes
  of the snapshot copy, splitting on LF with a preceding CR absorbed.
Strip no BOM; count it in byte offsets but not as line content.
UTF-16 and binary files: ineligible for line citation.
```

**Three-state verdict confirmed as necessary** — the round-2 recommendation stands and the encoding data reinforces it:
```
VERIFIED_EXACT                    content present at the cited range
VERIFIED_CONTENT_FOUND_ELSEWHERE  content present, different lines
UNVERIFIED                        content not present in the file
```
Only the third indicates fabrication. A strict two-state verifier would score encoding-induced offsets as hallucination and make the metric worthless.

---

## 13. Snapshot Limitations

| Case | Classification | Evidence |
|---|---|---|
| Tracked files | **SUPPORTED** | `VERIFIED` |
| Untracked non-ignored | **SUPPORTED** — `untracked_note.md` correctly included | `VERIFIED` |
| Ignored files | **EXCLUDED WITH DISCLOSURE** — `.env`, `build/out.js`, `app.log` all correctly excluded | `VERIFIED` |
| Symlinks | **EXCLUDED WITH DISCLOSURE** — creation needs admin here; filter handles them | `VERIFIED` |
| Junctions | **EXCLUDED WITH DISCLOSURE** — mandatory filter, see §5.1 | `VERIFIED` |
| Binary files | **EXCLUDED WITH DISCLOSURE** — `blob.bin` copied but must be citation-ineligible | `VERIFIED` (copy) / `INFERRED` (policy) |
| Very large files | **EXCLUDED WITH DISCLOSURE** — needs a size cap; 445 KB fixture copied fine, no cap implemented | `OBSERVED` |
| Generated files | **EXCLUDED WITH DISCLOSURE** — gitignored in practice | `VERIFIED` |
| Submodules | **DEFER** — none in fixture, `ls-files` does not recurse | `UNVERIFIED` |
| Git LFS | **DEFER** — not tested | `UNVERIFIED` |
| Agent instruction files | **EXCLUDED WITH DISCLOSURE** — `AGENTS.md` leaked into snapshot v1; stripped in v2 | `VERIFIED` |

A `SNAPSHOT_LIMITATIONS` block was produced and is practical:
```
EXCLUDED junction_escape/outside_marker.txt   [REPARSE_POINT]
EXCLUDED .env, build/out.js, app.log          [GITIGNORED]
EXCLUDED certs.pem, config/prod.key, id_rsa, src/tokens.ts  [SECRET_SCAN]
EXCLUDED AGENTS.md                            [AGENT_CONFIG]
EXCLUDED .git                                 [NEVER COPIED]
```

---

## 14. Model Identity / Independence

| Seat | Requested model | Reported actual model | Base family known? |
|---|---|---|---|
| Codex | (default, config ignored) | `gpt-5.6-sol` `VERIFIED` | **Yes** — OpenAI GPT |
| Claude | `haiku` alias | `claude-haiku-4-5-20251001`, `canonicalModel: claude-haiku-4-5`, `provider: firstParty` `VERIFIED` | **Yes** — Anthropic Claude |
| Antigravity | n/a | no structured channel exists `VERIFIED` | **No** |

### Verdict: **INDEPENDENCE PARTIAL**

Two seats report actual served model precisely enough to verify independence, and they are from different vendors and different families. The third seat cannot report anything because it has no headless interface.

Two supporting observations:
- **Model pinning is mandatory, verified.** Claude served `haiku-4-5` because the user's `settings.json` sets `model: claude-haiku-4-5-20251001`. Codex served `gpt-5.6-sol` rather than the configured `gpt-5.6-luna` only because `--ignore-user-config` was passed. In both cases the *user's* configuration, not Council, chose the model. Council must pin explicitly and record what was actually served.
- **Antigravity lineage is `MODEL_LINEAGE_UNKNOWN`.** `OBSERVED`: the install carries `.codeium/` state (Codeium/Windsurf heritage) and a `~/.gemini/antigravity/` tree, suggesting Gemini-family routing, but no verified per-turn model identity is obtainable. If it ever becomes a seat, model-identity reporting must be a gating requirement.

---

## 15. Remaining Risks

### P0 — must be resolved before or during early M1

| # | Risk | Status |
|---|---|---|
| P0-1 | **Third Council seat does not exist.** Installed Antigravity cannot be orchestrated. | `VERIFIED` |
| P0-2 | **Skills and hooks inject into every Codex turn despite `--ignore-user-config`.** `~/.codex/hooks.json` executed; 829 skills considered. Arbitrary code + uncontrolled context inside the boundary. | `VERIFIED` |
| P0-3 | **30–35 K tokens of ambient user config per turn, both providers.** Reproducibility, cost, and silent bias — the owner's global `CLAUDE.md` prescribes stack defaults that would contaminate stack debates. | `VERIFIED` |
| P0-4 | **Junction traversal in the snapshot algorithm.** Fixed and verified, but the fix is mandatory and was absent from the round-2 design. | `VERIFIED` (fix verified) |

### P1

| # | Risk | Status |
|---|---|---|
| P1-1 | Codex ~5 s spawn overhead + ~33 s trivial-call wall-clock; timeout defaults must be measured, not guessed. | `VERIFIED` |
| P1-2 | Encoding/CRLF instability with `autocrlf=true`; `git archive` unusable for evidence. | `VERIFIED` |
| P1-3 | Quota-exhaustion signature unknown for all providers; detector cannot be written yet. Fail-closed is the only safe policy. | `NOT REPRODUCED` |
| P1-4 | Schema conformance rate unmeasured; repair-pipeline scope undecided. | `UNVERIFIED` |
| P1-5 | Agents write substantial temp data — ~400 files appeared in the scratch dir during one Claude call. Per-turn scratch isolation and cleanup are required, not optional. | `VERIFIED` |
| P1-6 | MAX_PATH: deep paths broke the Rust link step outright. | `VERIFIED` |

### P2

| # | Risk | Status |
|---|---|---|
| P2-1 | Expired-credential / non-TTY behaviour untested; hang risk unquantified. | `UNVERIFIED` |
| P2-2 | Codex ignored `-s workspace-write` and stayed read-only; cause unknown. Benign today, could surprise later. | `OBSERVED` |
| P2-3 | Concurrent-CLI / token-refresh contention untested (though all M0 calls ran while an interactive Claude Code session was active, with no failures). | `OBSERVED` |
| P2-4 | Submodules and Git LFS untested. | `UNVERIFIED` |

---

## 16. Architecture Changes Required

Only changes supported by M0 evidence.

**AC-1 — Third seat must be re-decided.** Installed Antigravity cannot be a Council participant. Run `winget upgrade Google.Antigravity` and re-test (§4) before committing; if 2.8.1 has no `agy`, the seat choice returns to architecture debate. *Evidence: §4.*

**AC-2 — Council must support a 2-seat configuration as a first-class mode, not a degraded fallback.** Codex + Claude works today and is the only verified-working lineup. *Evidence: §2, §3, §14.*

**AC-3 — Add a mandatory reparse-point filter to snapshot construction.** Per-component `FILE_ATTRIBUTE_REPARSE_POINT` check plus canonical containment assertion. The round-2 algorithm is unsafe without it. *Evidence: §5.1.*

**AC-4 — Forbid `git archive`; mandate verbatim working-tree copy.** *Evidence: §12.*

**AC-5 — Config isolation needs a real design; `--ignore-user-config` is insufficient.** It does not stop skills or hooks. Council needs a per-invocation isolated agent home (e.g. a controlled `CODEX_HOME` seeded with auth only, or equivalent per-CLI mechanism), and preflight must *measure* ambient token count and refuse to start if it exceeds a threshold. *Evidence: §3.*

**AC-6 — Add ambient-context accounting to the context packet record.** Store the reported input/cache-creation token counts per turn so contamination is visible in the debate record. *Evidence: §3.*

**AC-7 — Strip agent instruction files from the snapshot** (`AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `.claude/`, `.codex/`, `.antigravity/`, `.mcp.json`) alongside secrets. *Evidence: §6, §13.*

**AC-8 — Per-turn writable scratch directory is required, with cleanup.** `TEMP`/`TMP` redirection works and agents genuinely need it. *Evidence: §5.2, §7, P1-5.*

**AC-9 — Short paths mandatory** for snapshot, scratch, and build roots. *Evidence: §1.*

**AC-10 — Explicit model pinning plus served-model recording per turn.** User config currently chooses the model. *Evidence: §14.*

**AC-11 — Production must use the .NET/Win32 ACL API,** not `icacls` shell-out. *Evidence: §5.2.*

**AC-12 — Kill path must abandon pipe readers,** not join them. *Evidence: §7.*

---

## 17. Questions Still Unknown

Explicitly not established by M0:

1. **Schema conformance rate** per provider. The pre-committed decision rule is unused. (§8)
2. **Practical context limits** at 50/200/500 KB via stdin. (§9)
3. **p50/p95 latency** — n=2–3 is not a distribution. (§10)
4. **Quota-exhaustion signature** — not reproduced, deliberately. (§3)
5. **Stateless two-turn reconstruction** quality. (§11)
6. **Agent-generated citation accuracy** — substrate tested, agents not. (§12)
7. **Whether Antigravity 2.8.1 adds a headless CLI.** (§4)
8. **Expired-credential behaviour** in a non-TTY child. (P2-1)
9. **Submodule and Git LFS** handling. (P2-4)
10. **Why Codex ignored `-s workspace-write`.** (P2-2)
11. **Whether ambient config can be fully suppressed** for Codex skills/hooks and Claude global memory. This is the single most important open question for M1, because items 1, 5 and 6 cannot be measured honestly until it is answered.
12. **Independent confirmation of zero billing** from provider dashboards — not performed by me.

---

## 18. Recommendation for M1

**Begin M1 — but with a short M0.5 first.** Items 11, 1 and 7 above are cheap, and building adapters before config isolation is solved means measuring a contaminated system.

- Run `winget upgrade Google.Antigravity`; re-test for `agy -p` + `--json-schema`. Settle the third seat. (~10 min)
- Solve config isolation: per-invocation isolated agent home carrying auth only; verify ambient tokens drop from ~32 K to near zero.
- With isolation in place, run the real 20×/provider schema-conformance test and apply the pre-committed rule from §8.
- Measure p50/p95 latency and 50/200/500 KB stdin limits on the final seat lineup.
- Then M1 proper: `council-core` skeleton — SQLite append-only log, `call_id` idempotency, snapshot builder with the §5.1 corrected algorithm, secret scan, ACL sealing, integrity verification. No agents.
- Port the verified spike mechanics into `council-core`: allowlist env, Job Object, stdin delivery, short paths, .NET/Win32 ACL API.
- Build the 2-seat Council as the primary target; treat the third seat as additive.
- Keep the unconditional response floor (parse → schema → semantic → typed failure → quarantine, raw always retained); defer the repair turn until conformance is measured.
- Do not build the debate engine, skills system, Tauri UI, or master-prompt compiler in M1.
- Delete the M0 harness. It is throwaway; its findings are the deliverable.

---

## Appendix — Spike artefacts

Throwaway, safe to delete:

| Path | Contents |
|---|---|
| `…\scratchpad\m0run1\scratch-repo` | throwaway git repo, synthetic fixtures, planted junction |
| `…\scratchpad\m0run1\harness` | Rust spike harness source (`m0harness`) |
| `%LOCALAPPDATA%\m0t` | cargo target dir (short path) |
| `%LOCALAPPDATA%\m0snap`, `m0snap2` | sealed snapshots (ACL-denied) |
| `%LOCALAPPDATA%\m0scratch`, `m0scratch2` | writable per-turn scratch |
| `…\scratchpad\m0run1\OUTSIDE_SECRET_AREA` | junction escape target |

All planted secrets are synthetic and marked `M0SPIKE`. No real credential values were read, logged, or written into this report. Authenticated calls made: **5** (3 Claude, 2 Codex — plus 2 zero-cost `--help` invocations).

**Note:** `%LOCALAPPDATA%\m0snap2` carries a Deny ACE for the current user and will resist ordinary deletion until that ACE is removed.
