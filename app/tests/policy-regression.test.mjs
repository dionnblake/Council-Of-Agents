import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const appRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const appSource = fs.readFileSync(path.join(appRoot, "src", "App.tsx"), "utf8");

function section(startMarker, endMarker) {
  const start = appSource.indexOf(startMarker);
  const end = appSource.indexOf(endMarker, start + startMarker.length);
  assert.notEqual(start, -1, `missing source marker: ${startMarker}`);
  assert.notEqual(end, -1, `missing source marker: ${endMarker}`);
  return appSource.slice(start, end);
}

const startDebate = section("  async function startDebate()", "  async function dispatchRound");
const dispatchRound = section("  async function dispatchRound", "  async function recordDecision");
const recordDecision = section("  async function recordDecision", "  async function compileExport");
const compileExport = section("  async function compileExport", "  async function resumeDebate");
const proceedDegraded = section("  async function proceedDegraded", "  async function cancelDebate");
const liveDebateScreen = section("function LiveDebateScreen", "function LiveDecisionScreen");
const liveDecisionScreen = section("function LiveDecisionScreen", "function ExportScreen");
const exportScreen = appSource.slice(appSource.indexOf("function ExportScreen"));

test("human decision is mandatory before a decision record can be sent", () => {
  assert.match(recordDecision, /if \(!debate \|\| !decisionRationale\.trim\(\)\)/);
  assert.match(recordDecision, /Write the human rationale before approving a direction/);
  assert.match(recordDecision, /invoke<unknown>\("record_decision"/);
});

test("preview debates cannot dispatch or masquerade as persisted runtime state", () => {
  assert.match(startDebate, /invoke<DebateSummary>\("create_debate"/);
  assert.match(startDebate, /id: "preview-debate"/);
  assert.match(startDebate, /Preview debate created\. Launch the Tauri shell to persist it to SQLite/);
  assert.match(dispatchRound, /debate\.id === "preview-debate"/);
  assert.match(dispatchRound, /Launch the Tauri desktop shell to dispatch a persisted provider round/);
});

test("incomplete or failed runs remain visibly blocked at the human gate", () => {
  assert.match(dispatchRound, /Round " \+ round \+ " did not dispatch/);
  assert.match(liveDecisionScreen, /const awaiting = props\.debate\?\.state === "AWAITING_HUMAN_DECISION"/);
  assert.match(liveDecisionScreen, /disabled=\{!awaiting\}/);
  assert.match(liveDecisionScreen, /Await final positions/);
  assert.match(liveDecisionScreen, /unverified/);
});

test("degraded mode requires explicit human action and exposes recovery controls", () => {
  assert.match(liveDebateScreen, /Seat availability requires a human choice/);
  assert.match(liveDebateScreen, /Retry later \/ resume/);
  assert.match(liveDebateScreen, /Cancel debate/);
  assert.match(liveDebateScreen, /Proceed with selected seats/);
  assert.match(proceedDegraded, /!degradedRationale\.trim\(\)/);
  assert.match(proceedDegraded, /invoke<DebateSummary>\("proceed_degraded"/);
  assert.match(proceedDegraded, /excludedProviders/);
  assert.match(appSource, /invoke<string>\("resume_debate"/);
  assert.match(appSource, /invoke<string>\("cancel_debate"/);
});

test("export is gated by the persisted decision and keeps the manual boundary visible", () => {
  const guard = compileExport.indexOf("if (!decisionRecord)");
  const invoke = compileExport.indexOf('invoke<ExportSummary>("compile_export"');
  assert.notEqual(guard, -1);
  assert.notEqual(invoke, -1);
  assert.ok(guard < invoke, "compile_export must remain after the decision guard");
  assert.match(compileExport, /Record the human decision before compiling an export/);
  assert.match(exportScreen, /NO HANDOFF/);
  assert.match(exportScreen, /starts implementation, creates a branch, commits, pushes, or hands off work automatically/);
  assert.match(appSource, /No provider handoff or implementation action occurred/);
});

test("provider limitations and requested/served status remain visible", () => {
  assert.match(liveDebateScreen, /served identity not reported/);
  assert.match(liveDecisionScreen, /REQUESTED \/ SERVED STATUS VISIBLE/);
  assert.match(appSource, /No provider state is hidden/);
  assert.match(appSource, /certification/i);
});

test("secret snapshot review is persisted, exact, and human-controlled", () => {
  assert.match(appSource, /invoke<SnapshotReviewStatus \| null>\("snapshot_review_status"/);
  assert.match(appSource, /invoke<SnapshotReviewStatus>\(command/);
  assert.match(appSource, /snapshotId: snapshotReview\.snapshot_id/);
  assert.match(appSource, /manifestHash: snapshotReview\.manifest_hash/);
  assert.match(appSource, /exclusionSetHash: snapshotReview\.exclusion_set_hash/);
  assert.match(appSource, /Provider dispatch paused\. Review the exact sanitized snapshot/);
  assert.match(liveDebateScreen, /Approve this exact sanitized snapshot/);
  assert.match(liveDebateScreen, /No excluded file contents, secret values, or provider context/);
  assert.match(liveDebateScreen, /Approve exact snapshot/);
  assert.match(liveDebateScreen, /Reject and abort/);
  assert.match(liveDebateScreen, /snapshotReview\?\.decision === "PENDING"/);
  assert.match(appSource, /SNAPSHOT_REVIEW_REQUIRED/);
});
