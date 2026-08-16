import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type Screen = "home" | "new" | "debate" | "decision" | "export" | "settings";
type SeatState = "READY" | "NOT_READY" | "LIMITED" | "CHECKING";

type ProviderStatus = {
  provider: string;
  label: string;
  model: string;
  certification: string;
  state: SeatState;
  detail: string;
  requested: string;
  served: string;
  auth: string;
};

type DebateSummary = {
  id: string;
  state: string;
  question: string;
  council_size: number;
  providers: string[];
  degraded: boolean;
  independent_only: boolean;
  discovery_required: boolean;
  discovery_complete: boolean;
  created_at: string;
};

type DiscoveryResult = {
  round: number;
  bounded: boolean;
  candidates: { id: string; label: string; source: string; status_quo: boolean; justifications: string[] }[];
};

type ProviderSetting = {
  provider: string;
  executable: string;
  model_default: string;
  enabled: boolean;
  timeout_ms: number;
  config_dir: string | null;
  safety_config_path: string | null;
  wsl_distribution: string | null;
  wsl_user: string | null;
  wsl_home: string | null;
  codex_home: string | null;
  extra_args: string[];
  certification: string;
  certification_evidence: string | null;
  safety_settings: Record<string, unknown>;
};

type SettingsView = { providers: ProviderSetting[]; export_directory: string };

type TurnSummary = {
  provider: string;
  state: string;
  attempts: number;
  failure_type: string | null;
  requested_model: string;
  reported_served_model: string | null;
  serving_identity_status: string;
};

type RunRoundSummary = {
  debate_id: string;
  round: number;
  state: string;
  packet_hashes: Record<string, string>;
  turns: TurnSummary[];
  valid_positions: number;
  evaluation: {
    citation_validity: string;
    schema_success_percent: number;
    repair_rate_percent: number;
    wall_time_ms_total: number;
    peer_response_quality_percent: number | null;
    revision_frequency_percent: number | null;
    decision_changed: boolean | null;
    new_considerations: number;
  };
  message: string;
};

type StoredPosition = {
  provider: string;
  round: number;
  requested_model: string;
  reported_served_model: string | null;
  serving_identity_status: string;
  position: {
    recommendation: string;
    commitment: string;
    claims: { id: string; text: string; evidence: string[] }[];
    risks: string[];
    peer_responses?: { peer_claim_reference: string; classification: string; reason: string; evidence: string[] }[];
    remaining_disputes?: string[];
    revision_reason?: string | null;
    flip_condition: string;
    cost_if_wrong: string;
    reversibility: string;
  };
};

type VerifiedEvidence = {
  file: string;
  requested_range: string;
  resolved_range: string | null;
  content: string;
  content_hash: string;
  file_hash: string | null;
  verdict: string;
};

type ExportSummary = {
  debate_id: string;
  directory: string;
  master_prompt_hash: string;
  decision_record_hash: string;
};

type SnapshotReviewExclusion = {
  relative_path: string;
  reason: string;
};

type SnapshotReviewStatus = {
  debate_id: string;
  snapshot_id: string;
  manifest_hash: string;
  exclusion_set_hash: string;
  secret_exclusion_count: number;
  exclusions: SnapshotReviewExclusion[];
  decision: "PENDING" | "APPROVED" | "REJECTED";
  reviewed_at: string | null;
};

const fallbackProviders: ProviderStatus[] = [
  {
    provider: "CLAUDE",
    label: "Claude Code",
    model: "claude-haiku-4-5-20251001",
    certification: "PASS",
    state: "READY",
    detail: "Dedicated local config",
    requested: "claude-haiku-4-5-20251001",
    served: "VERIFIED_MATCH",
    auth: "ISOLATED_CONFIG",
  },
  {
    provider: "ANTIGRAVITY",
    label: "Antigravity CLI",
    model: "gemini-3.7-flash-low",
    certification: "PASS_WITH_DECLARED_LIMITATION",
    state: "LIMITED",
    detail: "Served identity not reported",
    requested: "gemini-3.7-flash-low",
    served: "PROVIDER_DOES_NOT_REPORT",
    auth: "ISOLATED_PROVIDER_LOGIN",
  },
  {
    provider: "CODEX_WSL",
    label: "Codex WSL",
    model: "gpt-5.6-luna",
    certification: "PASS_WITH_DECLARED_LIMITATION",
    state: "READY",
    detail: "CouncilCodexWSL boundary",
    requested: "gpt-5.6-luna",
    served: "PROVIDER_DOES_NOT_REPORT",
    auth: "CHATGPT_SUBSCRIPTION",
  },
];

const navItems: { id: Screen; label: string; icon: string }[] = [
  { id: "home", label: "Command center", icon: "⌘" },
  { id: "new", label: "New debate", icon: "+" },
  { id: "debate", label: "Active debate", icon: "◈" },
  { id: "decision", label: "Decision record", icon: "✓" },
  { id: "export", label: "Export", icon: "↗" },
];

const UI_ZOOM_MIN = 0.75;
const UI_ZOOM_MAX = 1.5;
const UI_ZOOM_STEP = 0.1;

function statusClass(state: SeatState) {
  return state.toLowerCase().replace("_", "-");
}

function badgeClass(state: string) {
  return state.toLowerCase().replaceAll("_", "-");
}

function App() {
  const [screen, setScreen] = useState<Screen>("home");
  const [providers, setProviders] = useState<ProviderStatus[]>(fallbackProviders);
  const [debate, setDebate] = useState<DebateSummary | null>(null);
  const [recentDebates, setRecentDebates] = useState<DebateSummary[]>([]);
  const [runSummary, setRunSummary] = useState<RunRoundSummary | null>(null);
  const [discovery, setDiscovery] = useState<DiscoveryResult | null>(null);
  const [positions, setPositions] = useState<StoredPosition[]>([]);
  const [evidence, setEvidence] = useState<VerifiedEvidence[]>([]);
  const [decisionRecord, setDecisionRecord] = useState<unknown | null>(null);
  const [decisionRationale, setDecisionRationale] = useState("");
  const [decisionKind, setDecisionKind] = useState("APPROVE_OPTION");
  const [selectedOption, setSelectedOption] = useState("");
  const [modifiedDecision, setModifiedDecision] = useState("");
  const [degradedRationale, setDegradedRationale] = useState("");
  const [exportSummary, setExportSummary] = useState<ExportSummary | null>(null);
  const [snapshotReview, setSnapshotReview] = useState<SnapshotReviewStatus | null>(null);
  const [reviewResumeRound, setReviewResumeRound] = useState<number | null>(null);
  const [question, setQuestion] = useState("");
  const [mode, setMode] = useState("DISCOVERY");
  const [independentOnly, setIndependentOnly] = useState(false);
  const [productType, setProductType] = useState("DESKTOP");
  const [decisionType, setDecisionType] = useState("ARCHITECTURE");
  const [priority, setPriority] = useState("Correctness over speed");
  const [constraints, setConstraints] = useState("Local-first\nNo autonomous implementation");
  const [optionA, setOptionA] = useState("");
  const [optionB, setOptionB] = useState("");
  const [repository, setRepository] = useState("");
  const [currentLeaning, setCurrentLeaning] = useState("");
  const [currentLeaningReason, setCurrentLeaningReason] = useState("");
  const [enabledProviders, setEnabledProviders] = useState(["claude", "antigravity", "codex-wsl"]);
  const [models, setModels] = useState({
    claude: "claude-haiku-4-5-20251001",
    antigravity: "gemini-3.7-flash-low",
    "codex-wsl": "gpt-5.6-luna",
  });
  const [notice, setNotice] = useState("Runtime is in preview mode until the Tauri shell is launched.");
  const [settingsView, setSettingsView] = useState<SettingsView | null>(null);
  const [uiZoom, setUiZoom] = useState(1);

  useEffect(() => {
    document.documentElement.style.setProperty("zoom", String(uiZoom));
  }, [uiZoom]);

  useEffect(() => {
    const adjustZoom = (direction: 1 | -1) => {
      setUiZoom((current) => Math.min(UI_ZOOM_MAX, Math.max(UI_ZOOM_MIN, Number((current + direction * UI_ZOOM_STEP).toFixed(2)))));
    };

    const handleWheel = (event: WheelEvent) => {
      if (!event.ctrlKey || event.deltaY === 0) return;
      event.preventDefault();
      adjustZoom(event.deltaY < 0 ? 1 : -1);
    };

    const handleKeyDown = (event: KeyboardEvent) => {
      if (!event.ctrlKey || event.altKey || event.metaKey) return;
      const zoomIn = event.key === "+" || event.key === "=" || event.code === "NumpadAdd";
      const zoomOut = event.key === "-" || event.key === "_" || event.code === "NumpadSubtract";
      const reset = event.key === "0" || event.code === "Numpad0";
      if (!zoomIn && !zoomOut && !reset) return;
      event.preventDefault();
      if (reset) {
        setUiZoom(1);
      } else {
        adjustZoom(zoomIn ? 1 : -1);
      }
    };

    window.addEventListener("wheel", handleWheel, { capture: true, passive: false });
    window.addEventListener("keydown", handleKeyDown, true);
    return () => {
      window.removeEventListener("wheel", handleWheel, true);
      window.removeEventListener("keydown", handleKeyDown, true);
    };
  }, []);

  useEffect(() => {
    invoke<ProviderStatus[]>("provider_statuses")
      .then((result) => {
        setProviders(result);
        setNotice("Provider boundary status checked by the local controller.");
      })
      .catch(() => setNotice("Preview data loaded. Tauri provider checks will run in the desktop shell."));
    invoke<DebateSummary[]>("recent_debates")
      .then((result) => {
        setRecentDebates(result);
        if (result.length > 0) {
          setDebate(result[0]);
          void refreshPositions(result[0].id);
          void refreshDiscovery(result[0].id);
          void refreshSnapshotReview(result[0].id).then(() => refreshDebateSummary(result[0].id));
        }
      })
      .catch(() => undefined);
    invoke<SettingsView>("settings")
      .then((result) => {
        setSettingsView(result);
        setModels((current) => {
          const next = { ...current };
          result.providers.forEach((provider) => {
            const key = (provider.provider === "CODEX_WSL" ? "codex-wsl" : provider.provider.toLowerCase()) as keyof typeof next;
            if (key in next) next[key] = provider.model_default;
          });
          return next;
        });
      })
      .catch(() => undefined);
  }, []);

  async function refreshPositions(debateId: string) {
    if (debateId === "preview-debate") return;
    try {
      setPositions(await invoke<StoredPosition[]>("debate_positions", { debateId }));
      setEvidence(await invoke<VerifiedEvidence[]>("debate_evidence", { debateId }));
    } catch {
      setPositions([]);
      setEvidence([]);
    }
  }

  async function refreshDiscovery(debateId: string) {
    if (debateId === "preview-debate") return;
    try {
      setDiscovery(await invoke<DiscoveryResult | null>("debate_discovery", { debateId }));
    } catch {
      setDiscovery(null);
    }
  }

  async function refreshSnapshotReview(debateId: string) {
    if (debateId === "preview-debate") {
      setSnapshotReview(null);
      return null;
    }
    try {
      const result = await invoke<SnapshotReviewStatus | null>("snapshot_review_status", { debateId });
      setSnapshotReview(result);
      return result;
    } catch {
      setSnapshotReview(null);
      return null;
    }
  }

  async function refreshDebateSummary(debateId: string) {
    try {
      const result = await invoke<DebateSummary[]>("recent_debates");
      setRecentDebates(result);
      const latest = result.find((item) => item.id === debateId);
      if (latest) setDebate(latest);
      return latest ?? null;
    } catch {
      return null;
    }
  }

  async function openDebate(item: DebateSummary) {
    setDebate(item);
    setScreen("debate");
    setRunSummary(null);
    setReviewResumeRound(null);
    await refreshPositions(item.id);
    await refreshDiscovery(item.id);
    await refreshSnapshotReview(item.id);
    await refreshDebateSummary(item.id);
  }

  const readyCount = useMemo(() => providers.filter((provider) => provider.state === "READY").length, [providers]);

  const nextRound = useMemo(() => {
    if (!debate) return null;
    if (debate.state === "DRAFT") return debate.discovery_required && !debate.discovery_complete ? 0 : 1;
    if (debate.state === "READY" && debate.discovery_required && !debate.discovery_complete) return 0;
    if (debate.state === "READY") return reviewResumeRound ?? runSummary?.round ?? 1;
    if (debate.state === "CROSS_EXAMINATION") return runSummary?.round === 4 ? 5 : 2;
    if (debate.state === "FINAL_POSITIONS") return runSummary?.round === 4 ? 5 : 3;
    if (debate.state === "AWAITING_HUMAN_DECISION" && !debate.independent_only) return 4;
    return null;
  }, [debate, runSummary, reviewResumeRound]);

  async function startDebate() {
    if (question.trim().length < 12) {
      setNotice("Add a decision question with enough context for independent positions.");
      return;
    }
    const intake = {
      question: question.trim(),
      mode,
      options: mode === "COMPARE" ? [optionA.trim(), optionB.trim()].filter(Boolean) : [],
      product_type: productType,
      decision_type: decisionType,
      hard_constraints: constraints.split("\n").map((value) => value.trim()).filter(Boolean),
      priority: priority.trim() || "Best overall",
      current_leaning: currentLeaning.trim() || null,
      current_leaning_reason: currentLeaningReason.trim() || null,
      repository: repository.trim() || null,
    };
    if (enabledProviders.length < 2 || enabledProviders.length > 3) {
      setNotice("Select two or three provider seats explicitly before opening the debate.");
      return;
    }
    try {
      const created = await invoke<DebateSummary>("create_debate", { intake, modelOverrides: models, independentOnly, enabledProviders });
      setDebate(created);
      setRecentDebates((current) => [created, ...current.filter((item) => item.id !== created.id)]);
      setDiscovery(null);
      setSnapshotReview(null);
      setNotice("Debate created. Provider turns remain human-visible and auditable.");
    } catch {
      setDebate({ id: "preview-debate", state: "DRAFT", question: intake.question, council_size: enabledProviders.length, providers: enabledProviders, degraded: enabledProviders.length < 3, independent_only: independentOnly, discovery_required: decisionType === "STACK" && mode === "DISCOVERY" && intake.options.length === 0, discovery_complete: false, created_at: new Date().toISOString() });
      setNotice("Preview debate created. Launch the Tauri shell to persist it to SQLite.");
    }
    setScreen("debate");
  }

  async function dispatchRound(round: number, retry = false) {
    if (!debate || debate.id === "preview-debate") {
      setNotice("Launch the Tauri desktop shell to dispatch a persisted provider round.");
      return;
    }
    setNotice("Dispatching round " + round + ". Each seat gets a fresh process.");
    try {
      const result = await invoke<RunRoundSummary>("run_round", {
        debateId: debate.id,
        round,
        retryToken: retry ? crypto.randomUUID() : null,
      });
      setRunSummary(result);
      setReviewResumeRound(null);
      await refreshPositions(debate.id);
      await refreshDiscovery(debate.id);
      setDebate((current) => current ? { ...current, state: result.state, discovery_complete: round === 0 ? result.valid_positions > 0 && result.state === "READY" : current.discovery_complete } : current);
      setNotice(result.message);
      if (result.state === "AWAITING_HUMAN_DECISION") {
        setScreen("decision");
      }
    } catch (error) {
      const review = await refreshSnapshotReview(debate.id);
      await refreshDebateSummary(debate.id);
      if (review?.decision === "PENDING") setReviewResumeRound(round);
      setNotice(review?.decision === "PENDING"
        ? "Provider dispatch paused. Review the exact sanitized snapshot before continuing."
        : "Round " + round + " did not dispatch: " + String(error));
    }
  }

  async function decideSnapshotReview(decision: "approve" | "reject") {
    if (!debate || !snapshotReview || snapshotReview.decision !== "PENDING") {
      setNotice("No pending snapshot review is available.");
      return;
    }
    const command = decision === "approve" ? "approve_snapshot_review" : "reject_snapshot_review";
    try {
      const result = await invoke<SnapshotReviewStatus>(command, {
        input: {
          debateId: snapshotReview.debate_id,
          snapshotId: snapshotReview.snapshot_id,
          manifestHash: snapshotReview.manifest_hash,
          exclusionSetHash: snapshotReview.exclusion_set_hash,
        },
      });
      setSnapshotReview(result);
      setDebate((current) => current ? { ...current, state: result.decision === "APPROVED" ? "READY" : "SAFETY_ABORT" } : current);
      setNotice(result.decision === "APPROVED"
        ? "Snapshot approved. Dispatch remains a separate human-visible action."
        : "Snapshot rejected. Provider dispatch is blocked for this debate.");
    } catch (error) {
      const refreshed = await refreshSnapshotReview(debate.id);
      await refreshDebateSummary(debate.id);
      setNotice(refreshed?.decision === "PENDING"
        ? "The snapshot changed or could not be verified. Review the refreshed evidence before deciding."
        : "Snapshot review was not recorded: " + String(error));
    }
  }

  async function recordDecision() {
    if (!debate || !decisionRationale.trim()) {
      setNotice("Write the human rationale before approving a direction.");
      return;
    }
    try {
      const record = await invoke<unknown>("record_decision", {
        input: {
          debateId: debate.id,
          kind: decisionKind,
          selectedOption: selectedOption || null,
          modifiedDecision: modifiedDecision || null,
          rationale: decisionRationale.trim(),
        },
      });
      setDecisionRecord(record);
      setDebate((current) => current ? { ...current, state: "DECIDED" } : current);
      setNotice("Human decision recorded. Compile the deterministic export when ready.");
    } catch (error) {
      setNotice("Decision was not recorded: " + String(error));
    }
  }

  async function compileExport() {
    if (!decisionRecord) {
      setNotice("Record the human decision before compiling an export.");
      return;
    }
    try {
      const result = await invoke<ExportSummary>("compile_export", { debateId: debate?.id });
      setExportSummary(result);
      setDebate((current) => current ? { ...current, state: "EXPORTED" } : current);
      setScreen("export");
      setNotice("Export compiled into Council application data. No provider handoff was performed.");
    } catch (error) {
      setNotice("Export was not compiled: " + String(error));
    }
  }

  async function resumeDebate() {
    if (!debate || debate.id === "preview-debate") return;
    try {
      const state = await invoke<string>("resume_debate", { debateId: debate.id });
      setDebate((current) => current ? { ...current, state } : current);
      setNotice("Debate resumed. Re-dispatching the selected round with a new immutable call id.");
      if (runSummary?.round !== undefined) await dispatchRound(runSummary.round, true);
    } catch (error) {
      setNotice("Resume was not allowed: " + String(error));
    }
  }

  async function proceedDegraded() {
    if (!debate || !degradedRationale.trim()) {
      setNotice("Explain why the remaining seats are sufficient before proceeding degraded.");
      return;
    }
    const failedProviders = (runSummary?.turns ?? [])
      .filter((turn) => turn.failure_type || turn.state !== "VALID")
      .map((turn) => turn.provider.toLowerCase().replace("_", "-"));
    const excludedProviders = failedProviders.length > 0
      ? failedProviders
      : providers
        .filter((provider) => provider.state === "NOT_READY")
        .map((provider) => provider.provider.toLowerCase().replace("_", "-"));
    try {
      const updated = await invoke<DebateSummary>("proceed_degraded", {
        input: {
          debateId: debate.id,
          excludedProviders,
          rationale: degradedRationale.trim(),
        },
      });
      setDebate(updated);
      setNotice("Degraded council mode recorded. Excluded seats remain visible in the audit.");
      const retryRound = runSummary?.round ?? (updated.discovery_required && !updated.discovery_complete ? 0 : 1);
      await dispatchRound(retryRound, true);
    } catch (error) {
      setNotice("Degraded mode was not recorded: " + String(error));
    }
  }

  async function cancelDebate() {
    if (!debate) return;
    try {
      const state = await invoke<string>("cancel_debate", { debateId: debate.id });
      setDebate((current) => current ? { ...current, state } : current);
      setNotice("Debate cancelled. No provider handoff or implementation action occurred.");
    } catch (error) {
      setNotice("Cancellation failed: " + String(error));
    }
  }

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="brand-lockup">
          <div className="brand-mark"><span /><span /><span /></div>
          <div>
            <div className="brand-name">COUNCIL</div>
            <div className="brand-subtitle">OF AGENTS / LOCAL</div>
          </div>
        </div>

        <div className="sidebar-label">WORKSPACE</div>
        <nav className="primary-nav" aria-label="Primary navigation">
          {navItems.map((item) => (
            <button key={item.id} className={`nav-item ${screen === item.id ? "active" : ""}`} onClick={() => setScreen(item.id)}>
              <span className="nav-icon">{item.icon}</span>
              <span>{item.label}</span>
              {item.id === "debate" && debate ? <span className="nav-live">LIVE</span> : null}
            </button>
          ))}
        </nav>

        <div className="sidebar-spacer" />
        <div className="boundary-card">
          <div className="boundary-topline"><span className="status-dot good" />BOUNDARY STATUS</div>
          <div className="boundary-title">LOCAL / SEALED</div>
          <p>Packets are immutable. Evidence is cited by file and line range.</p>
          <div className="boundary-foot"><span>3 seats</span><span>{readyCount}/3 ready</span></div>
        </div>
        <button className={`nav-item settings-link ${screen === "settings" ? "active" : ""}`} onClick={() => setScreen("settings")}>
          <span className="nav-icon">⚙</span><span>Settings</span>
        </button>
        <div className="version-stamp">MVP BUILD 0.1.0 · OFFLINE-FIRST</div>
      </aside>

      <main className="main-pane">
        <header className="topbar">
          <div className="breadcrumb"><span>COUNCIL</span><b>/</b><strong>{screen === "home" ? "COMMAND CENTER" : screen.replace("-", " ").toUpperCase()}</strong></div>
          <div className="topbar-actions"><span className="local-pill"><span className="status-dot good" />LOCAL RUNTIME</span><button className="icon-button" aria-label="Notifications">◌</button><button className="avatar" aria-label="the owner">B</button></div>
        </header>

        <div className="content-scroll">
          {screen === "home" && <HomeScreen providers={providers} setScreen={setScreen} debate={debate} recentDebates={recentDebates} openDebate={openDebate} />}
          {screen === "new" && <LiveNewDebateScreen question={question} setQuestion={setQuestion} mode={mode} setMode={setMode} independentOnly={independentOnly} setIndependentOnly={setIndependentOnly} productType={productType} setProductType={setProductType} decisionType={decisionType} setDecisionType={setDecisionType} priority={priority} setPriority={setPriority} constraints={constraints} setConstraints={setConstraints} optionA={optionA} setOptionA={setOptionA} optionB={optionB} setOptionB={setOptionB} repository={repository} setRepository={setRepository} currentLeaning={currentLeaning} setCurrentLeaning={setCurrentLeaning} currentLeaningReason={currentLeaningReason} setCurrentLeaningReason={setCurrentLeaningReason} enabledProviders={enabledProviders} setEnabledProviders={setEnabledProviders} models={models} setModels={setModels} startDebate={startDebate} />}
          {screen === "debate" && <LiveDebateScreen providers={providers} debate={debate} runSummary={runSummary} discovery={discovery} positions={positions} nextRound={nextRound} snapshotReview={snapshotReview} decideSnapshotReview={decideSnapshotReview} degradedRationale={degradedRationale} setDegradedRationale={setDegradedRationale} dispatchRound={dispatchRound} resumeDebate={resumeDebate} proceedDegraded={proceedDegraded} cancelDebate={cancelDebate} setScreen={setScreen} />}
          {screen === "decision" && <LiveDecisionScreen debate={debate} runSummary={runSummary} positions={positions} evidence={evidence} decisionRationale={decisionRationale} setDecisionRationale={setDecisionRationale} decisionKind={decisionKind} setDecisionKind={setDecisionKind} selectedOption={selectedOption} setSelectedOption={setSelectedOption} modifiedDecision={modifiedDecision} setModifiedDecision={setModifiedDecision} decisionRecord={decisionRecord} exportSummary={exportSummary} recordDecision={recordDecision} compileExport={compileExport} dispatchRound={dispatchRound} setScreen={setScreen} />}
          {screen === "export" && <ExportScreen exportSummary={exportSummary} debate={debate} />}
          {screen === "settings" && <SettingsScreen providers={providers} settingsView={settingsView} setSettingsView={setSettingsView} />}
        </div>
        <div className="notice-bar"><span className="notice-symbol">i</span><span>{notice}</span><span className="notice-right">No provider state is hidden</span></div>
      </main>
    </div>
  );
}

function HomeScreen({ providers, setScreen, debate, recentDebates, openDebate }: { providers: ProviderStatus[]; setScreen: (screen: Screen) => void; debate: DebateSummary | null; recentDebates: DebateSummary[]; openDebate: (debate: DebateSummary) => void }) {
  return (
    <section className="page page-home">
      <div className="eyebrow"><span className="eyebrow-line" />DECISION INTELLIGENCE / 01</div>
      <div className="hero-row">
        <div className="hero-copy"><h1>Make the decision<br /><em>worth defending.</em></h1><p>Three independent seats. One controlled packet. A human-owned final call.</p><button className="primary-button" onClick={() => setScreen("new")}><span>Start a debate</span><b>↗</b></button></div>
        <div className="hero-orbit" aria-label="Three certified provider seats"><div className="orbit-ring ring-one" /><div className="orbit-ring ring-two" /><div className="orbit-core"><span>3</span><small>SEATS</small></div><div className="orbit-label orbit-label-one">CLAUDE <i /></div><div className="orbit-label orbit-label-two">ANTIGRAVITY <i /></div><div className="orbit-label orbit-label-three">CODEX WSL <i /></div></div>
      </div>

      <div className="section-heading"><div><span className="section-kicker">01 / PROVIDERS</span><h2>Certified seats</h2></div><button className="text-button" onClick={() => setScreen("settings")}>View certification →</button></div>
      <div className="provider-grid">{providers.map((provider) => <ProviderCard key={provider.provider} provider={provider} />)}</div>

      <div className="lower-grid">
        <div className="recent-panel"><div className="panel-heading"><div><span className="section-kicker">02 / RECENT</span><h2>Debate log</h2></div><span className="count-badge">{String(recentDebates.length).padStart(2, "0")}</span></div>{recentDebates.length > 0 ? <div className="recent-list">{recentDebates.slice(0, 8).map((item) => <button className="recent-row" key={item.id} onClick={() => openDebate(item)}><div className="recent-icon">◈</div><div className="recent-main"><strong>{item.question}</strong><span>{item.id} · {item.state} · {item.council_size} seats</span></div><span className="recent-arrow">→</span></button>)}</div> : debate ? <button className="recent-row" onClick={() => openDebate(debate)}><div className="recent-icon">◈</div><div className="recent-main"><strong>{debate.question}</strong><span>{debate.id} · {debate.state}</span></div><span className="recent-arrow">→</span></button> : <div className="empty-state"><div className="empty-mark">＋</div><p>Your first decision packet is waiting.</p><button className="text-button" onClick={() => setScreen("new")}>Create debate →</button></div>}</div>
        <div className="principles-panel"><span className="section-kicker">03 / OPERATING CONTRACT</span><h2>Evidence over<br /><em>confidence.</em></h2><div className="principle-list"><div><span>01</span><p>Agents draft positions.<br /><b>the owner decides.</b></p></div><div><span>02</span><p>Packets are hashed.<br /><b>Sessions are fresh.</b></p></div><div><span>03</span><p>Dissent is preserved.<br /><b>Majority is not authority.</b></p></div></div></div>
      </div>
    </section>
  );
}

function ProviderCard({ provider }: { provider: ProviderStatus }) {
  return <article className="provider-card"><div className="provider-head"><div className={`provider-glyph glyph-${provider.provider.toLowerCase()}`}>{provider.provider === "CLAUDE" ? "C" : provider.provider === "ANTIGRAVITY" ? "A" : "✦"}</div><div className="provider-name"><strong>{provider.label}</strong><span>{provider.model}</span></div><span className={`state-badge ${statusClass(provider.state)}`}><i />{provider.state.replace("_", " ")}</span></div><div className="provider-rule" /><div className="provider-meta"><span>REQUESTED</span><strong>{provider.requested}</strong></div><div className="provider-meta"><span>SERVED IDENTITY</span><strong className={provider.served === "VERIFIED_MATCH" ? "green-text" : "amber-text"}>{provider.served.replaceAll("_", " ")}</strong></div><div className="provider-meta"><span>AUTH</span><strong className={provider.auth === "CHATGPT_SUBSCRIPTION" ? "green-text" : "amber-text"}>{provider.auth.replaceAll("_", " ")}</strong></div><div className="provider-foot"><span className="cert-stamp">{provider.certification.replaceAll("_", " ")}</span><span className="provider-detail">{provider.detail}</span></div></article>;
}

function NewDebateScreen(props: { question: string; setQuestion: (value: string) => void; mode: string; setMode: (value: string) => void; productType: string; setProductType: (value: string) => void; decisionType: string; setDecisionType: (value: string) => void; priority: string; setPriority: (value: string) => void; constraints: string; setConstraints: (value: string) => void; startDebate: () => void }) {
  return <section className="page page-form"><div className="eyebrow"><span className="eyebrow-line" />NEW DEBATE / INTAKE</div><div className="form-header"><div><h1>Frame the decision.</h1><p>Give every seat the same clean question. Context comes later through the controlled packet.</p></div><div className="form-step"><span>STEP</span><strong>01</strong><small>/ 03</small></div></div><div className="form-layout"><div className="form-main"><label className="field-label">DECISION QUESTION <span>REQUIRED</span></label><textarea className="question-input" value={props.question} onChange={(event) => props.setQuestion(event.target.value)} placeholder="What decision are you trying to make, and what makes it consequential?" rows={5} /><div className="field-hint">Be specific enough that a stranger could evaluate the decision without a follow-up call.</div><div className="form-two-up"><div><label className="field-label">MODE</label><div className="segmented-control">{[["DISCOVERY", "Open question"], ["COMPARE", "Compare options"]].map(([value, label]) => <button className={props.mode === value ? "selected" : ""} key={value} onClick={() => props.setMode(value)}>{label}</button>)}</div></div><div><label className="field-label">PRIORITY</label><input className="text-input" value={props.priority} onChange={(event) => props.setPriority(event.target.value)} /></div></div><div className="form-two-up"><div><label className="field-label">PRODUCT SURFACE</label><select className="text-input" value={props.productType} onChange={(event) => props.setProductType(event.target.value)}><option value="DESKTOP">Desktop app</option><option value="WEB">Web product</option><option value="ANDROID">Android app</option><option value="AI_SYSTEM">AI system</option><option value="OTHER">Other</option></select></div><div><label className="field-label">DECISION TYPE</label><select className="text-input" value={props.decisionType} onChange={(event) => props.setDecisionType(event.target.value)}><option value="ARCHITECTURE">Architecture</option><option value="STACK">Stack</option><option value="DESIGN">Design</option><option value="SECURITY">Security</option><option value="PERFORMANCE">Performance</option><option value="GENERAL">General</option></select></div></div><label className="field-label">HARD CONSTRAINTS</label><textarea className="text-input constraints-input" value={props.constraints} onChange={(event) => props.setConstraints(event.target.value)} rows={3} /><div className="form-actions"><button className="secondary-button" onClick={() => props.setQuestion("")}>Clear</button><button className="primary-button" onClick={props.startDebate}><span>Open controlled debate</span><b>↗</b></button></div></div><aside className="form-aside"><div className="aside-number">01</div><h3>Same packet.<br /><em>Independent reads.</em></h3><p>Each provider receives a fresh process, the same immutable evidence, and the same response contract.</p><div className="aside-divider" /><div className="aside-check"><span>✓</span><p>Claude Code<br /><small>isolated local config</small></p></div><div className="aside-check"><span>✓</span><p>Antigravity CLI<br /><small>credit guard enforced</small></p></div><div className="aside-check"><span>✓</span><p>Codex WSL<br /><small>dedicated Linux boundary</small></p></div></aside></div></section>;
}

function DebateScreen({ providers, debate, setScreen }: { providers: ProviderStatus[]; debate: DebateSummary | null; setScreen: (screen: Screen) => void }) {
  const phases = ["PREFLIGHT", "SNAPSHOT", "OPENING", "CROSS-EXAM", "FINAL POSITIONS"];
  return <section className="page page-debate"><div className="eyebrow"><span className="eyebrow-line" />ACTIVE DEBATE / {debate?.id ?? "PREVIEW"}</div><div className="debate-heading"><div><h1>{debate?.question ?? "No active debate yet."}</h1><div className="heading-tags"><span>DISCOVERY</span><span>ARCHITECTURE</span><span>LOCAL-FIRST</span></div></div><span className="live-status"><i />CONTROLLER READY</span></div><div className="phase-track">{phases.map((phase, index) => <div className={`phase ${index < 2 ? "complete" : index === 2 ? "current" : ""}`} key={phase}><span>{index < 2 ? "✓" : String(index + 1).padStart(2, "0")}</span><small>{phase}</small></div>)}</div><div className="debate-layout"><div className="turns-panel"><div className="panel-heading"><div><span className="section-kicker">TURN MONITOR</span><h2>Independent positions</h2></div><span className="count-badge">{providers.length} SEATS</span></div>{providers.map((provider, index) => <div className={`turn-row ${index === 0 ? "turn-active" : ""}`} key={provider.provider}><div className={`turn-number ${index === 0 ? "on" : ""}`}>0{index + 1}</div><div className="turn-provider"><strong>{provider.label}</strong><span>{provider.model}</span></div><div className="turn-state"><span className={`state-badge ${statusClass(index === 0 ? "CHECKING" : provider.state)}`}><i />{index === 0 ? "PENDING" : provider.state.replace("_", " ")}</span><small>{index === 0 ? "fresh process · awaiting dispatch" : provider.detail}</small></div><div className="turn-chevron">{index === 0 ? "⋮" : "›"}</div></div>)}</div><aside className="packet-panel"><div className="packet-art"><div className="packet-lines"><span /><span /><span /><span /></div><div className="packet-seal">SHA<br /><b>256</b></div></div><span className="section-kicker">CONTROLLED PACKET</span><h3>Evidence snapshot</h3><p>Read-only context is sealed separately from writable scratch.</p><div className="packet-stat"><span>PACKET HASH</span><strong>awaiting run</strong></div><div className="packet-stat"><span>SESSION MODE</span><strong>FRESH / NO RESUME</strong></div><button className="text-button muted-button" onClick={() => setScreen("decision")}>Preview decision surface →</button></aside></div></section>;
}

function DecisionScreen({ setScreen }: { setScreen: (screen: Screen) => void }) {
  return <section className="page page-decision"><div className="eyebrow"><span className="eyebrow-line" />DECISION RECORD / HUMAN GATE</div><div className="decision-header"><div><h1>Make the call<br /><em>with the dissent visible.</em></h1><p>The council can structure the disagreement. Only you can commit the decision.</p></div><div className="decision-stamp"><span>STATUS</span><strong>AWAITING</strong><small>HUMAN DECISION</small></div></div><div className="decision-grid"><div className="recommendation-panel"><div className="panel-heading"><div><span className="section-kicker">COUNCIL SYNTHESIS</span><h2>Recommendation</h2></div><span className="confidence-tag">3 POSITIONS</span></div><div className="recommendation-copy"><span className="recommendation-label">PROVISIONAL DIRECTION</span><h3>Choose the smallest reversible path that can answer the highest-risk unknown.</h3><p>The council converges on a narrow first move, while preserving a credible exit if the assumption about usage or integration proves wrong.</p></div><div className="evidence-strip"><span>↗</span><div><strong>4 verified citations</strong><small>2 open risks · 1 minority position</small></div><button className="text-button">Inspect evidence</button></div></div><aside className="dissent-panel"><span className="section-kicker">MINORITY REPORT</span><h2>What could change the call?</h2><div className="dissent-quote">“The cheaper path is only cheaper if the migration trigger is observable before the next commitment.”</div><div className="dissent-source"><span className="provider-glyph glyph-antigravity">A</span><div><strong>Antigravity CLI</strong><small>CONDITIONAL / COSTLY</small></div></div></aside></div><div className="decision-actions"><button className="secondary-button" onClick={() => setScreen("debate")}>Return to debate</button><div><button className="outline-button" onClick={() => setScreen("new")}>Continue targeted debate</button><button className="primary-button" onClick={() => setScreen("home")}><span>Approve direction</span><b>✓</b></button></div></div></section>;
}

function SettingsScreen({ providers, settingsView, setSettingsView }: { providers: ProviderStatus[]; settingsView: SettingsView | null; setSettingsView: (value: SettingsView) => void }) {
  const [draft, setDraft] = useState<SettingsView | null>(settingsView);
  const [message, setMessage] = useState("");
  useEffect(() => setDraft(settingsView), [settingsView]);
  if (!draft) return <section className="page page-settings"><div className="empty-state"><p>Settings are available in the Tauri desktop shell.</p></div></section>;
  const activeDraft = draft;
  const updateProvider = (index: number, patch: Partial<ProviderSetting>) => setDraft({ ...activeDraft, providers: activeDraft.providers.map((provider, providerIndex) => providerIndex === index ? { ...provider, ...patch } : provider) });
  async function save() {
    try {
      const saved = await invoke<SettingsView>("save_settings", { input: { providers: activeDraft.providers, exportDirectory: activeDraft.export_directory } });
      setSettingsView(saved);
      setDraft(saved);
      setMessage("Settings saved locally. Safety invariants were revalidated by the controller.");
    } catch (error) {
      setMessage("Settings were not saved: " + String(error));
    }
  }
  return <section className="page page-settings"><div className="eyebrow"><span className="eyebrow-line" />SETTINGS / CERTIFICATION</div><div className="settings-header"><div><h1>Runtime boundaries.</h1><p>Provider locations, models, timeouts, certification, and export routing are persisted locally. Codex's isolation boundary is fixed.</p></div><span className="settings-lock">⌁ LOCAL ONLY</span></div><div className="settings-list">{providers.map((provider) => <div className="settings-row" key={provider.provider}><div className={`provider-glyph glyph-${provider.provider.toLowerCase()}`}>{provider.provider[0]}</div><div className="settings-provider"><strong>{provider.label}</strong><span>{provider.provider === "CODEX_WSL" ? "CouncilCodexWSL / council / read-only" : provider.detail}</span></div><div className="settings-model"><span>REQUESTED MODEL</span><strong>{provider.requested}</strong><small>{provider.auth.replaceAll("_", " ")}</small></div><div className="settings-cert"><span className={`state-badge ${statusClass(provider.state)}`}><i />{provider.state.replace("_", " ")}</span><small>{provider.certification.replaceAll("_", " ")}</small></div></div>)}</div><div className="settings-editor">{draft.providers.map((provider, index) => <div className="settings-editor-card" key={provider.provider}><div className="panel-heading"><div><span className="section-kicker">{provider.provider.replaceAll("_", " ")}</span><h2>{provider.provider === "CODEX_WSL" ? "Dedicated Linux seat" : "Provider runtime"}</h2></div><label className="toggle-line"><input type="checkbox" checked={provider.enabled} onChange={(event) => updateProvider(index, { enabled: event.target.checked })} /><span>Available for new debates</span></label></div><div className="form-two-up"><label className="field-label">EXECUTABLE<input className="text-input" value={provider.executable} disabled={provider.provider === "CODEX_WSL"} onChange={(event) => updateProvider(index, { executable: event.target.value })} /></label><label className="field-label">DEFAULT MODEL<input className="text-input" value={provider.model_default} onChange={(event) => updateProvider(index, { model_default: event.target.value })} /></label></div><div className="form-two-up"><label className="field-label">TIMEOUT MS<input className="text-input" type="number" min={1000} max={900000} value={provider.timeout_ms} onChange={(event) => updateProvider(index, { timeout_ms: Number(event.target.value) })} /></label><label className="field-label">ISOLATION / CONFIG PATH<input className="text-input" value={provider.provider === "CODEX_WSL" ? (provider.codex_home ?? "") : (provider.config_dir ?? "")} disabled={provider.provider === "CODEX_WSL"} onChange={(event) => updateProvider(index, provider.provider === "CODEX_WSL" ? {} : { config_dir: event.target.value || null })} /></label></div>{provider.provider === "ANTIGRAVITY" ? <label className="field-label">CREDIT GUARD SETTINGS PATH<input className="text-input" value={provider.safety_config_path ?? ""} onChange={(event) => updateProvider(index, { safety_config_path: event.target.value || null })} placeholder="Optional settings.json path" /></label> : null}{provider.provider === "CODEX_WSL" ? <div className="field-hint">Fixed certified boundary: CouncilCodexWSL, Linux user council, HOME /home/council, CODEX_HOME /home/council/.codex, no Windows mounts or inherited PATH.</div> : null}</div>)}</div><label className="field-label export-setting">EXPORT DIRECTORY<input className="text-input" value={draft.export_directory} onChange={(event) => setDraft({ ...draft, export_directory: event.target.value })} /></label><div className="form-actions"><button className="primary-button" onClick={save}><span>Save local settings</span><b>✓</b></button></div>{message ? <div className="settings-note"><span>i</span><p>{message}</p></div> : null}<div className="settings-note"><span>!</span><p><strong>Declared limitation</strong> means the provider is usable under a known boundary, but the product will keep that limitation visible in every debate record.</p></div></section>;
}

function LiveNewDebateScreen(props: {
  question: string;
  setQuestion: (value: string) => void;
  mode: string;
  setMode: (value: string) => void;
  independentOnly: boolean;
  setIndependentOnly: (value: boolean) => void;
  productType: string;
  setProductType: (value: string) => void;
  decisionType: string;
  setDecisionType: (value: string) => void;
  priority: string;
  setPriority: (value: string) => void;
  constraints: string;
  setConstraints: (value: string) => void;
  optionA: string;
  setOptionA: (value: string) => void;
  optionB: string;
  setOptionB: (value: string) => void;
  repository: string;
  setRepository: (value: string) => void;
  currentLeaning: string;
  setCurrentLeaning: (value: string) => void;
  currentLeaningReason: string;
  setCurrentLeaningReason: (value: string) => void;
  enabledProviders: string[];
  setEnabledProviders: (value: string[]) => void;
  models: { claude: string; antigravity: string; "codex-wsl": string };
  setModels: (value: { claude: string; antigravity: string; "codex-wsl": string }) => void;
  startDebate: () => void;
}) {
  const updateModel = (key: "claude" | "antigravity" | "codex-wsl", value: string) => {
    props.setModels({ ...props.models, [key]: value });
  };
  return (
    <section className="page page-form">
      <div className="eyebrow"><span className="eyebrow-line" />NEW DEBATE / INTAKE</div>
      <div className="form-header"><div><h1>Frame the decision.</h1><p>Give every seat the same clean question. Context comes later through the controlled packet.</p></div><div className="form-step"><span>STEP</span><strong>01</strong><small>/ 03</small></div></div>
      <div className="form-layout">
        <div className="form-main">
          <label className="field-label">DECISION QUESTION <span>REQUIRED</span></label>
          <textarea className="question-input" value={props.question} onChange={(event) => props.setQuestion(event.target.value)} placeholder="What decision are you trying to make, and what makes it consequential?" rows={5} />
          <div className="field-hint">Be specific enough that a stranger could evaluate the decision without a follow-up call.</div>
          <div className="form-two-up"><div><label className="field-label">MODE</label><div className="segmented-control">{[["DISCOVERY", "Open question"], ["COMPARE", "Compare options"]].map(([value, label]) => <button className={props.mode === value ? "selected" : ""} key={value} onClick={() => props.setMode(value)}>{label}</button>)}</div></div><div><label className="field-label">PRIORITY</label><input className="text-input" value={props.priority} onChange={(event) => props.setPriority(event.target.value)} /></div></div>
          <label className="toggle-line"><input type="checkbox" checked={props.independentOnly} onChange={(event) => props.setIndependentOnly(event.target.checked)} /><span>Independent-only evaluation</span><small>Run fresh opening positions and stop before peer exposure.</small></label>
          <label className="field-label">YOUR CURRENT LEANING <span>OPTIONAL / HIDDEN FROM AGENTS</span></label><input className="text-input" value={props.currentLeaning} onChange={(event) => props.setCurrentLeaning(event.target.value)} placeholder="Keep this private preregistration context, if useful" /><textarea className="text-input constraints-input" value={props.currentLeaningReason} onChange={(event) => props.setCurrentLeaningReason(event.target.value)} placeholder="Why are you leaning that way? This stays controller-side." rows={2} />
          {props.mode === "COMPARE" ? <div className="form-two-up"><div><label className="field-label">OPTION A <span>REQUIRED</span></label><input className="text-input" value={props.optionA} onChange={(event) => props.setOptionA(event.target.value)} placeholder="First candidate" /></div><div><label className="field-label">OPTION B <span>REQUIRED</span></label><input className="text-input" value={props.optionB} onChange={(event) => props.setOptionB(event.target.value)} placeholder="Second candidate" /></div></div> : null}
          <div className="form-two-up"><div><label className="field-label">PRODUCT SURFACE</label><select className="text-input" value={props.productType} onChange={(event) => props.setProductType(event.target.value)}><option value="DESKTOP">Desktop app</option><option value="WEB">Web product</option><option value="ANDROID">Android app</option><option value="WINDOWS">Windows</option><option value="GAME">Game</option><option value="BACKEND">Backend</option><option value="AI_SYSTEM">AI system</option><option value="OTHER">Other</option></select></div><div><label className="field-label">DECISION TYPE</label><select className="text-input" value={props.decisionType} onChange={(event) => props.setDecisionType(event.target.value)}><option value="ARCHITECTURE">Architecture</option><option value="STACK">Stack</option><option value="DESIGN">Design</option><option value="SECURITY">Security</option><option value="PERFORMANCE">Performance</option><option value="DATABASE">Database</option><option value="DEPENDENCY">Dependency</option><option value="TESTING">Testing</option><option value="GENERAL">General</option></select></div></div>
          <label className="field-label">HARD CONSTRAINTS</label><textarea className="text-input constraints-input" value={props.constraints} onChange={(event) => props.setConstraints(event.target.value)} rows={3} />
          <label className="field-label">OPTIONAL REPOSITORY PATH <span>SNAPSHOT REQUIRED</span></label><input className="text-input" value={props.repository} onChange={(event) => props.setRepository(event.target.value)} placeholder="C:\path\to\project, or leave blank for greenfield" /><div className="field-hint">The controller copies bytes into a sealed snapshot, excludes instructions and secrets, verifies hashes, and dispatches only from that copy.</div>
          <label className="field-label">COUNCIL SEATS <span>EXPLICIT / NO SILENT DEGRADATION</span></label><div className="seat-selector">{[["claude", "Claude Code"], ["antigravity", "Antigravity CLI"], ["codex-wsl", "Codex WSL"]].map(([value, label]) => <label className="toggle-line" key={value}><input type="checkbox" checked={props.enabledProviders.includes(value)} onChange={(event) => props.setEnabledProviders(event.target.checked ? [...props.enabledProviders, value] : props.enabledProviders.filter((provider) => provider !== value))} /><span>{label}</span></label>)}</div><div className="field-hint">Choose exactly two or three seats. A two-seat council is labeled degraded in the record and is never inferred from a failed provider.</div>
          <div className="form-actions"><button className="secondary-button" onClick={() => props.setQuestion("")}>Clear</button><button className="primary-button" onClick={props.startDebate}><span>Open controlled debate</span><b>↗</b></button></div>
        </div>
        <aside className="form-aside"><div className="aside-number">01</div><h3>Same packet.<br /><em>Independent reads.</em></h3><p>Each provider receives a fresh process, the same immutable evidence, and the same response contract.</p><div className="aside-divider" /><div className="aside-check"><span>✓</span><p>Claude Code<br /><small>isolated local config</small></p></div><div className="aside-check"><span>✓</span><p>Antigravity CLI<br /><small>credit guard enforced</small></p></div><div className="aside-check"><span>✓</span><p>Codex WSL<br /><small>dedicated Linux boundary</small></p></div><div className="aside-divider" /><label className="field-label">REQUESTED MODELS</label><div className="model-stack"><label>CLAUDE<input className="text-input" value={props.models.claude} onChange={(event) => updateModel("claude", event.target.value)} /></label><label>ANTIGRAVITY<input className="text-input" value={props.models.antigravity} onChange={(event) => updateModel("antigravity", event.target.value)} /></label><label>CODEX WSL<input className="text-input" value={props.models["codex-wsl"]} onChange={(event) => updateModel("codex-wsl", event.target.value)} /></label></div></aside>
      </div>
    </section>
  );
}

function LiveDebateScreen(props: {
  providers: ProviderStatus[];
  debate: DebateSummary | null;
  runSummary: RunRoundSummary | null;
  discovery: DiscoveryResult | null;
  positions: StoredPosition[];
  nextRound: number | null;
  snapshotReview: SnapshotReviewStatus | null;
  decideSnapshotReview: (decision: "approve" | "reject") => void;
  degradedRationale: string;
  setDegradedRationale: (value: string) => void;
  dispatchRound: (round: number, retry?: boolean) => void;
  resumeDebate: () => void;
  proceedDegraded: () => void;
  cancelDebate: () => void;
  setScreen: (screen: Screen) => void;
}) {
  const phases = ["PREFLIGHT", "SNAPSHOT", "OPENING", "CROSS-EXAM", "FINAL POSITIONS"];
  const phaseIndex = props.debate?.state === "DRAFT" ? 0
    : props.debate?.state === "SNAPSHOT_REVIEW_REQUIRED" ? 1
    : props.debate?.state === "OPENING" ? 2
    : props.debate?.state === "CROSS_EXAMINATION" ? 3
    : props.debate?.state === "FINAL_POSITIONS" ? 4
    : props.debate?.state === "AWAITING_HUMAN_DECISION" ? 5
    : 0;
  const actionLabel = props.nextRound === 0 ? "Discover candidate stacks"
    : props.nextRound === 1 ? "Dispatch opening round"
    : props.nextRound === 2 ? "Dispatch cross-examination"
    : props.nextRound === 3 || props.nextRound === 5 ? "Dispatch final positions"
    : props.nextRound === 4 ? "Open targeted round"
    : null;
  return (
    <section className="page page-debate">
      <div className="eyebrow"><span className="eyebrow-line" />ACTIVE DEBATE / {props.debate?.id ?? "PREVIEW"}</div>
      <div className="debate-heading">
        <div><h1>{props.debate?.question ?? "No active debate yet."}</h1><div className="heading-tags"><span>CONTROLLED PACKET</span><span>FRESH PROCESSES</span><span>HUMAN GATE</span></div></div>
        <span className={"live-status " + (props.debate?.state === "FAILED" ? "failed-status" : "")}><i />{props.debate?.state ?? "DRAFT"}</span>
      </div>
      <div className="phase-track">
        {phases.map((phase, index) => <div className={"phase " + (index < phaseIndex ? "complete" : index === phaseIndex ? "current" : "")} key={phase}><span>{index < phaseIndex ? "✓" : String(index + 1).padStart(2, "0")}</span><small>{phase}</small></div>)}
      </div>
      {props.snapshotReview?.decision === "PENDING" && props.debate?.state === "SNAPSHOT_REVIEW_REQUIRED" ? <div className="snapshot-review-panel">
        <div className="panel-heading"><div><span className="section-kicker">HUMAN SAFETY REVIEW</span><h2>Approve this exact sanitized snapshot</h2></div><span className="count-badge">{props.snapshotReview.secret_exclusion_count} SECRET FLAGS</span></div>
        <p className="review-intro">Provider dispatch is paused. Review the exclusion metadata and hashes below. No excluded file contents, secret values, or provider context are shown or persisted in this review record.</p>
        <div className="review-hash-grid"><div><span>SNAPSHOT ID</span><strong>{props.snapshotReview.snapshot_id}</strong></div><div><span>MANIFEST SHA-256</span><strong>{props.snapshotReview.manifest_hash}</strong></div><div><span>EXCLUSION SET SHA-256</span><strong>{props.snapshotReview.exclusion_set_hash}</strong></div></div>
        <div className="review-exclusion-list"><div className="section-kicker">EXCLUDED PATH METADATA</div>{props.snapshotReview.exclusions.map((item) => <div className="review-exclusion-row" key={item.relative_path + item.reason}><code>{item.relative_path}</code><span>{item.reason.replaceAll("_", " ")}</span></div>)}</div>
        <div className="form-actions"><button className="primary-button" onClick={() => props.decideSnapshotReview("approve")}><span>Approve exact snapshot</span><b>✓</b></button><button className="secondary-button" onClick={() => props.decideSnapshotReview("reject")}>Reject and abort</button><button className="text-button muted-button" onClick={props.cancelDebate}>Cancel review</button></div>
      </div> : null}
      <div className="debate-layout">
        <div className="turns-panel">
          <div className="panel-heading"><div><span className="section-kicker">TURN MONITOR</span><h2>Independent positions</h2></div><span className="count-badge">{props.runSummary ? props.runSummary.valid_positions + " / " + props.providers.length + " VALID" : props.providers.length + " SEATS"}</span></div>
          {props.providers.map((provider, index) => {
            const turn = props.runSummary?.turns.find((item) => item.provider === provider.provider || item.provider === provider.provider.replace("_", "-"));
            const state = turn?.state ?? "PENDING";
            const detail = turn ? (turn.failure_type ?? (turn.serving_identity_status === "PROVIDER_DOES_NOT_REPORT" ? "served identity not reported" : "fresh process complete")) : "fresh process · awaiting dispatch";
            return <div className={"turn-row " + (turn ? "turn-active" : "")} key={provider.provider}><div className={"turn-number " + (turn ? "on" : "")}>0{index + 1}</div><div className="turn-provider"><strong>{provider.label}</strong><span>{turn?.requested_model ?? provider.model}</span></div><div className="turn-state"><span className={"state-badge " + badgeClass(state)}><i />{state.replaceAll("_", " ")}</span><small>{detail}</small></div><div className="turn-chevron">{turn ? "✓" : "·"}</div></div>;
          })}
        </div>
        <aside className="packet-panel">
          <div className="packet-art"><div className="packet-lines"><span /><span /><span /><span /></div><div className="packet-seal">SHA<br /><b>256</b></div></div>
          <span className="section-kicker">CONTROLLED PACKET</span><h3>Evidence snapshot</h3><p>Packets are written once, hashed, and bridged to Codex without exposing the Windows filesystem.</p>
          <div className="packet-stat"><span>PACKET HASH</span><strong>{props.runSummary ? Object.values(props.runSummary.packet_hashes)[0]?.slice(0, 18) + "…" : "awaiting run"}</strong></div>
          <div className="packet-stat"><span>SESSION MODE</span><strong>FRESH / NO RESUME</strong></div>
          {actionLabel && props.nextRound !== null ? <button className="primary-button packet-action" onClick={() => props.dispatchRound(props.nextRound as number)}><span>{actionLabel}</span><b>↗</b></button> : props.debate?.state === "AWAITING_HUMAN_DECISION" ? <button className="text-button muted-button" onClick={() => props.setScreen("decision")}>Open human decision gate →</button> : null}
        </aside>
      </div>
      {props.discovery ? <div className="positions-strip discovery-strip"><div className="panel-heading"><div><span className="section-kicker">R0 / BOUNDED CANDIDATE UNION</span><h2>Review before independent opening</h2></div><span className="count-badge">{props.discovery.candidates.length} CANDIDATES</span></div><div className="candidate-grid">{props.discovery.candidates.map((candidate) => <article className="candidate-card" key={candidate.id}><strong>{candidate.label}</strong><span>{candidate.status_quo ? "STATUS QUO" : candidate.source}</span><p>{candidate.justifications[0] ?? "Controller-bounded candidate."}</p></article>)}</div></div> : null}
      {(props.debate?.state === "PAUSED" || (props.debate?.state === "DRAFT" && props.providers.some((provider) => provider.state === "NOT_READY"))) ? <div className="run-note degraded-note"><strong>Seat availability requires a human choice.</strong><span>Retry when ready, cancel, or explicitly continue with at least two seats.</span><div className="form-actions"><button className="outline-button" onClick={props.resumeDebate}>Retry later / resume</button><button className="secondary-button" onClick={props.cancelDebate}>Cancel debate</button></div><textarea className="text-input" value={props.degradedRationale} onChange={(event) => props.setDegradedRationale(event.target.value)} placeholder="Why is a degraded council acceptable for this decision?" rows={2} /><button className="outline-button" onClick={props.proceedDegraded}>Proceed with selected seats</button></div> : null}
      {props.positions.length > 0 ? <div className="positions-strip"><div className="panel-heading"><div><span className="section-kicker">STORED POSITIONS</span><h2>Recommendations and risks</h2></div><span className="count-badge">{props.positions.length} PROVIDERS</span></div><div className="position-cards">{props.positions.map((position) => <article className="position-card" key={position.provider}><div className="position-card-top"><strong>{position.provider.replace("-", " ").toUpperCase()}</strong><span>{position.position.commitment.replaceAll("_", " ")}</span></div><h3>{position.position.recommendation}</h3><p>{position.position.risks[0] ?? "No risk recorded."}</p><small>{position.position.flip_condition}</small></article>)}</div></div> : null}
      {props.runSummary ? <div className="run-note"><strong>{props.runSummary.message}</strong><span>round {props.runSummary.round} · {props.runSummary.valid_positions} usable positions · schema {props.runSummary.evaluation.schema_success_percent}% · citations {props.runSummary.evaluation.citation_validity} · state {props.runSummary.state}</span></div> : null}
    </section>
  );
}

function LiveDecisionScreen(props: {
  debate: DebateSummary | null;
  runSummary: RunRoundSummary | null;
  positions: StoredPosition[];
  evidence: VerifiedEvidence[];
  decisionRationale: string;
  setDecisionRationale: (value: string) => void;
  decisionKind: string;
  setDecisionKind: (value: string) => void;
  selectedOption: string;
  setSelectedOption: (value: string) => void;
  modifiedDecision: string;
  setModifiedDecision: (value: string) => void;
  decisionRecord: unknown | null;
  exportSummary: ExportSummary | null;
  recordDecision: () => void;
  compileExport: () => void;
  dispatchRound: (round: number, retry?: boolean) => void;
  setScreen: (screen: Screen) => void;
}) {
  const awaiting = props.debate?.state === "AWAITING_HUMAN_DECISION";
  const decided = Boolean(props.decisionRecord) || props.debate?.state === "DECIDED" || props.debate?.state === "EXPORTED";
  return (
    <section className="page page-decision">
      <div className="eyebrow"><span className="eyebrow-line" />DECISION RECORD / HUMAN GATE</div>
      <div className="decision-header"><div><h1>Make the call<br /><em>with the dissent visible.</em></h1><p>The council can structure disagreement. Only the owner can commit the decision.</p></div><div className="decision-stamp"><span>STATUS</span><strong>{decided ? "RECORDED" : awaiting ? "AWAITING" : "NOT READY"}</strong><small>HUMAN DECISION</small></div></div>
      <div className="decision-grid">
        <div className="recommendation-panel">
          <div className="panel-heading"><div><span className="section-kicker">COUNCIL SYNTHESIS</span><h2>Recommendation surface</h2></div><span className="confidence-tag">{props.runSummary?.valid_positions ?? 0} POSITIONS</span></div>
          <div className="recommendation-copy"><span className="recommendation-label">PERSISTED EVIDENCE</span><h3>{props.positions[0]?.position.recommendation ?? (props.runSummary ? "The final positions are stored in the local audit record." : "No live positions are available yet.")}</h3><p>{props.positions.length > 0 ? "The controller retains each seat's recommendation, commitment, risks, flip condition, and claim citations. Review the full audit record before deciding." : props.runSummary ? "Review provider rows, risks, commitments, and citations in the audit trail before making a human decision. This screen does not grant majority authority to any seat." : "Complete the three controlled rounds before opening the decision gate."}</p></div>
          <div className="evidence-strip"><span>↗</span><div><strong>{props.evidence.filter((item) => item.verdict !== "UNVERIFIED").length} verified citations</strong><small>{props.evidence.filter((item) => item.verdict === "UNVERIFIED").length} unverified · {props.runSummary?.valid_positions ?? 0} usable positions</small></div></div>
          {props.evidence.length > 0 ? <div className="evidence-list">{props.evidence.slice(-8).map((item, index) => <details key={`${item.requested_range}-${index}`}><summary>{item.requested_range} · {item.verdict.replaceAll("_", " ")}</summary><code>{item.resolved_range ?? "No valid range resolved"}</code><pre>{item.content || "No excerpt stored."}</pre></details>)}</div> : <p className="field-hint">No citation verification records are stored yet. Greenfield claims remain unverified until a snapshot exists.</p>}
        </div>
        <aside className="dissent-panel"><span className="section-kicker">MINORITY REPORT</span><h2>Dissent stays in the record.</h2><div className="dissent-quote">{props.positions[1]?.position.recommendation ?? "No recommendation becomes authority by being the majority. The final record keeps every usable seat and its stated limits."}</div><div className="dissent-source"><span className="provider-glyph glyph-antigravity">A</span><div><strong>{props.positions[1]?.provider.replace("-", " ").toUpperCase() ?? "ALL SEATS REMAIN ATTRIBUTABLE"}</strong><small>REQUESTED / SERVED STATUS VISIBLE</small></div></div></aside>
      </div>
      <div className="human-gate-panel">
          <div><span className="section-kicker">YOUR DECISION</span><h2>What are you committing to?</h2><select className="text-input" value={props.decisionKind} onChange={(event) => props.setDecisionKind(event.target.value)}><option value="APPROVE_OPTION">Approve an option</option><option value="APPROVE_MODIFIED_DECISION">Approve a modified decision</option><option value="CHALLENGE_CONSENSUS">Challenge the consensus</option><option value="REJECT_ALL">Reject all positions</option></select>{props.decisionKind === "APPROVE_OPTION" ? <input className="text-input decision-option" value={props.selectedOption} onChange={(event) => props.setSelectedOption(event.target.value)} placeholder="Selected option (required)" /> : null}{props.decisionKind === "APPROVE_MODIFIED_DECISION" ? <input className="text-input decision-option" value={props.modifiedDecision} onChange={(event) => props.setModifiedDecision(event.target.value)} placeholder="Modified decision (required)" /> : null}<textarea className="text-input decision-rationale" value={props.decisionRationale} onChange={(event) => props.setDecisionRationale(event.target.value)} placeholder="State the decision, the rationale, and what evidence would make you revisit it." rows={4} /></div>
        <div className="decision-gate-actions">{!decided ? <button className="primary-button" onClick={props.recordDecision} disabled={!awaiting}><span>{awaiting ? "Record human decision" : "Await final positions"}</span><b>✓</b></button> : <button className="primary-button" onClick={props.compileExport} disabled={Boolean(props.exportSummary)}><span>{props.exportSummary ? "Export compiled" : "Compile deterministic export"}</span><b>↗</b></button>}{props.exportSummary ? <div className="export-hash"><span>MASTER PROMPT</span><strong>{props.exportSummary.master_prompt_hash.slice(0, 18)}…</strong><small>Manual copy only. No handoff performed.</small></div> : null}</div>
      </div>
      <div className="decision-actions"><button className="secondary-button" onClick={() => props.setScreen("debate")}>Return to debate</button><div>{awaiting && !props.debate?.independent_only ? <button className="outline-button" onClick={() => { props.setScreen("debate"); props.dispatchRound(4); }}>Continue targeted debate</button> : null}{props.debate?.state === "EXPORTED" ? <span className="export-complete">EXPORTED TO LOCAL APP DATA</span> : null}</div></div>
    </section>
  );
}

function ExportScreen({ exportSummary, debate }: { exportSummary: ExportSummary | null; debate: DebateSummary | null }) {
  return <section className="page page-settings"><div className="eyebrow"><span className="eyebrow-line" />EXPORT / MANUAL BOUNDARY</div><div className="settings-header"><div><h1>Decision package.</h1><p>The controller produced deterministic local files. Copy or attach them manually when you choose.</p></div><span className="settings-lock">NO HANDOFF</span></div>{exportSummary ? <div className="settings-list"><div className="settings-row"><div className="settings-provider"><strong>MASTER PROMPT</strong><span>{debate?.id ?? exportSummary.debate_id}</span></div><div className="settings-model"><span>PATH</span><strong>{exportSummary.directory}\\master-prompt.md</strong></div><div className="settings-cert"><small>{exportSummary.master_prompt_hash}</small></div></div><div className="settings-row"><div className="settings-provider"><strong>DECISION RECORD</strong><span>human-owned output</span></div><div className="settings-model"><span>PATH</span><strong>{exportSummary.directory}\\decision-record.md</strong></div><div className="settings-cert"><small>{exportSummary.decision_record_hash}</small></div></div></div> : <div className="empty-state"><div className="empty-mark">＋</div><p>Record a human decision and compile the export first.</p></div>}<div className="settings-note"><span>!</span><p><strong>Manual boundary.</strong> Council never opens a browser, sends a message, starts implementation, creates a branch, commits, pushes, or hands off work automatically.</p></div></section>;
}

export default App;
