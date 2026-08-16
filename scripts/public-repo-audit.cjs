#!/usr/bin/env node

const fs = require("node:fs");
const path = require("node:path");
const { execFileSync } = require("node:child_process");

const repoRoot = execFileSync("git", ["rev-parse", "--show-toplevel"], {
  encoding: "utf8",
}).trim();
const includeHistory = process.argv.includes("--history") || process.argv.includes("--all");

const findings = [];
const seenFindings = new Set();
const secretKinds = new Set([
  "OPENAI_KEY",
  "ANTHROPIC_KEY",
  "GOOGLE_KEY",
  "GITHUB_TOKEN",
  "AWS_ACCESS_KEY",
  "PRIVATE_KEY_BLOCK",
  "JWT",
  "BEARER_TOKEN",
  "SECRET_ASSIGNMENT",
  "URL_CREDENTIALS",
]);
const identityMetadataKinds = new Set([
  "COMMIT_AUTHOR_EMAIL",
  "COMMIT_COMMITTER_EMAIL",
]);

const contentChecks = [
  { kind: "MACHINE_HOSTNAME", pattern: /\b(?:DESKTOP|LAPTOP|WIN)-[A-Z0-9]{5,}\b/gi },
  {
    kind: "PRIVATE_WINDOWS_PATH",
    pattern: /[A-Za-z]:\\Users\\(?!<USER>|Public(?=\\|$)|Default(?: User)?(?=\\|$)|All Users(?=\\|$))[A-Za-z0-9._-]{2,}(?=\\|\/|$)/g,
  },
  {
    kind: "PRIVATE_LINUX_HOME",
    pattern: /\/home\/(?!<USER>(?=\/|$)|council(?=\/|$))[A-Za-z][A-Za-z0-9._-]{1,}(?=\/|$)/g,
  },
  {
    kind: "LOCAL_TIMEZONE",
    pattern: /\b(?:America|Europe|Asia|Australia|Pacific)\/[A-Za-z_]+(?:\/[A-Za-z_]+)?\b/g,
  },
  { kind: "EMAIL", pattern: /\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b/gi },
  { kind: "OPENAI_KEY", pattern: /\bsk-(?:proj-)?[A-Za-z0-9_-]{20,}\b/g },
  { kind: "ANTHROPIC_KEY", pattern: /\bsk-ant-[A-Za-z0-9_-]{20,}\b/g },
  { kind: "GOOGLE_KEY", pattern: /\bAIza[0-9A-Za-z_-]{30,}\b/g },
  { kind: "GITHUB_TOKEN", pattern: /\b(?:gh[pousr]_[A-Za-z0-9_]{20,}|github_pat_[A-Za-z0-9_]{20,})\b/g },
  { kind: "AWS_ACCESS_KEY", pattern: /\bAKIA[0-9A-Z]{16}\b/g },
  { kind: "PRIVATE_KEY_BLOCK", pattern: /-----BEGIN (?:[A-Z0-9]+ )?PRIVATE KEY-----/g },
  { kind: "JWT", pattern: /\beyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\b/g },
  { kind: "BEARER_TOKEN", pattern: /\bBearer\s+[A-Za-z0-9._~+/-]{20,}/gi },
  {
    kind: "SECRET_ASSIGNMENT",
    pattern:
      /\b(?:OPENAI_API_KEY|ANTHROPIC_API_KEY|GEMINI_API_KEY|GOOGLE_API_KEY|SUPABASE_SERVICE_ROLE_KEY|SUPABASE_ANON_KEY|GITHUB_TOKEN|GH_TOKEN|ACCESS_TOKEN|REFRESH_TOKEN|CLIENT_SECRET|SECRET_KEY|PRIVATE_KEY|PASSWORD|DATABASE_URL)\s*[:=]\s*["']?[A-Za-z0-9_./+=-]{16,}/gi,
  },
  { kind: "URL_CREDENTIALS", pattern: /\b[A-Za-z][A-Za-z0-9+.-]{1,}:\/\/[^/\s:@]+:[^/\s@]{8,}@/g },
];

function runGit(args, options = {}) {
  try {
    return execFileSync("git", args, {
      cwd: repoRoot,
      encoding: options.encoding || "utf8",
      maxBuffer: 128 * 1024 * 1024,
      stdio: ["ignore", "pipe", options.captureStderr ? "pipe" : "ignore"],
    });
  } catch (error) {
    if (options.allowFailure && error.stdout) return error.stdout;
    throw error;
  }
}

function relativePath(filePath) {
  return path.relative(repoRoot, filePath).split(path.sep).join("/");
}

function lineNumber(source, index) {
  let line = 1;
  for (let cursor = 0; cursor < index; cursor += 1) {
    if (source.charCodeAt(cursor) === 10) line += 1;
  }
  return line;
}

function allowedEmail(value) {
  const domain = value.slice(value.lastIndexOf("@")).toLowerCase();
  return domain === "@users.noreply.github.com" || domain === "@noreply.github.com" || domain === "@github.com" || domain === "@example.com" || domain === "@example.org" || domain === "@example.net";
}

function allowedMatch(kind, value) {
  if (kind !== "EMAIL") return false;
  return allowedEmail(value);
}

function addFinding(scope, kind, file, line, extra = "") {
  const key = [scope, kind, file, line || "", extra].join("|");
  if (seenFindings.has(key)) return;
  seenFindings.add(key);
  findings.push({ scope, kind, file, line, extra });
}

function scanText(source, file, scope) {
  if (source.includes("\u0000")) return;
  for (const check of contentChecks) {
    check.pattern.lastIndex = 0;
    for (const match of source.matchAll(check.pattern)) {
      const value = match[0];
      if (allowedMatch(check.kind, value)) continue;
      addFinding(scope, check.kind, file, lineNumber(source, match.index));
    }
  }
}

function suspiciousFilename(file) {
  const base = path.basename(file).toLowerCase();
  if (/^\.env(?:\..+)?$/.test(base) && !/^\.env\.(?:example|sample|template)$/.test(base)) return "ENV_FILE";
  if (/^(?:credentials?|secrets?)(?:[._-].*)?$/.test(base)) return "CREDENTIAL_FILENAME";
  if (/^(?:auth|token)\.json$/.test(base)) return "CREDENTIAL_FILENAME";
  if (/^(?:id_rsa|id_ed25519|id_ecdsa)$/.test(base)) return "PRIVATE_KEY_FILENAME";
  if (/\.(?:pem|key|pfx|p12)$/i.test(base)) return "PRIVATE_KEY_FILENAME";
  return null;
}

function currentFiles() {
  const output = runGit(["ls-files", "-co", "--exclude-standard", "-z"]);
  return output
    .split("\u0000")
    .filter(Boolean)
    .map((file) => path.join(repoRoot, file));
}

function scanCurrentTree() {
  let scanned = 0;
  for (const filePath of currentFiles()) {
    const file = relativePath(filePath);
    const filenameFinding = suspiciousFilename(file);
    if (filenameFinding) addFinding("CURRENT", filenameFinding, file, null);
    let stat;
    try {
      stat = fs.lstatSync(filePath);
    } catch {
      continue;
    }
    if (!stat.isFile() || stat.size > 32 * 1024 * 1024) continue;
    const data = fs.readFileSync(filePath);
    scanText(data.toString("utf8"), file, "CURRENT");
    scanned += 1;
  }
  return scanned;
}

function reachableObjects() {
  const output = runGit(["rev-list", "--objects", "--all"]);
  const objects = [];
  const seen = new Set();
  for (const line of output.split(/\r?\n/)) {
    if (!line) continue;
    const separator = line.indexOf(" ");
    const objectId = separator === -1 ? line : line.slice(0, separator);
    const objectPath = separator === -1 ? "" : line.slice(separator + 1);
    if (seen.has(objectId)) continue;
    seen.add(objectId);
    objects.push({ objectId, objectPath });
  }
  return objects;
}

function scanHistory() {
  let scanned = 0;
  const objects = reachableObjects();
  const objectIds = objects.map((object) => object.objectId);
  const objectPaths = new Map(objects.map((object) => [object.objectId, object.objectPath]));
  const batch = execFileSync("git", ["cat-file", "--batch"], {
    cwd: repoRoot,
    input: `${objectIds.join("\n")}\n`,
    encoding: null,
    maxBuffer: 256 * 1024 * 1024,
    stdio: ["pipe", "pipe", "ignore"],
  });

  let offset = 0;
  while (offset < batch.length) {
    const headerEnd = batch.indexOf(10, offset);
    if (headerEnd === -1) break;
    const header = batch.toString("utf8", offset, headerEnd);
    const [objectId, type, sizeText] = header.split(" ");
    offset = headerEnd + 1;
    if (!objectId || !type || !sizeText) break;
    const size = Number(sizeText);
    const dataEnd = offset + size;
    if (dataEnd > batch.length) break;
    if (type === "blob") {
      const data = batch.subarray(offset, dataEnd);
      const file = objectPaths.get(objectId) || `blob:${objectId.slice(0, 12)}`;
      const before = findings.length;
      scanText(data.toString("utf8"), file, "HISTORY");
      if (findings.length > before) {
        for (let index = before; index < findings.length; index += 1) {
          findings[index].extra = `blob:${objectId.slice(0, 12)}`;
        }
      }
      scanned += 1;
    }
    offset = dataEnd + 1;
  }

  const metadata = runGit(["log", "--all", "--format=%H%x09%aE%x09%cE"]);
  for (const row of metadata.split(/\r?\n/).filter(Boolean)) {
    const [commit, authorEmail, committerEmail] = row.split("\t");
    if (authorEmail && !allowedEmail(authorEmail)) addFinding("HISTORY", "COMMIT_AUTHOR_EMAIL", "<commit metadata>", null, `commit:${commit.slice(0, 12)}`);
    if (committerEmail && !allowedEmail(committerEmail)) addFinding("HISTORY", "COMMIT_COMMITTER_EMAIL", "<commit metadata>", null, `commit:${commit.slice(0, 12)}`);
  }
  return scanned;
}

const currentScanned = scanCurrentTree();
const historyScanned = includeHistory ? scanHistory() : 0;
const historyState = includeHistory ? "SCANNED" : "SKIPPED";
const secretFindings = findings.filter((finding) => secretKinds.has(finding.kind));
const identityMetadataFindings = findings.filter((finding) =>
  identityMetadataKinds.has(finding.kind),
);
const releaseFindings = findings.filter(
  (finding) => !identityMetadataKinds.has(finding.kind),
);

console.log(`CURRENT_FILES_SCANNED=${currentScanned}`);
console.log(`HISTORY_BLOBS_SCANNED=${historyScanned}`);
console.log(`GIT_HISTORY=${historyState}`);
console.log(`CONFIRMED_LIVE_SECRET_MATCHES=${secretFindings.length}`);
console.log(`IDENTITY_METADATA_WARNINGS=${identityMetadataFindings.length}`);
console.log(`PUBLIC_REPO_AUDIT=${releaseFindings.length === 0 ? "PASS" : "FAIL"}`);

for (const finding of findings.sort((left, right) => {
  const a = `${left.scope}|${left.kind}|${left.file}|${left.line || ""}|${left.extra}`;
  const b = `${right.scope}|${right.kind}|${right.file}|${right.line || ""}|${right.extra}`;
  return a.localeCompare(b);
})) {
  const location = finding.line ? `${finding.file}:${finding.line}` : finding.file;
  const suffix = finding.extra ? ` ${finding.extra}` : "";
  const severity = identityMetadataKinds.has(finding.kind) ? "WARNING" : "FAIL";
  console.log(`${severity} ${finding.scope} ${finding.kind} ${location}${suffix}`);
}

process.exitCode = releaseFindings.length === 0 ? 0 : 1;
