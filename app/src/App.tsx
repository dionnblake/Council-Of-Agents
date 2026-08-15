import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type Screen = "home" | "new" | "debate" | "decision" | "settings";
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
};

type DebateSummary = {
  id: string;
  state: string;
  question: string;
  created_at: string;
};

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
    flip_condition: string;
    cost_if_wrong: string;
    reversibility: string;
  };
};

type ExportSummary = {
  debate_id: string;
  directory: string;
  master_prompt_hash: string;
  decision_record_hash: string;
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
  },
];

const navItems: { id: Screen; label: string; icon: string }[] = [
  { id: "home", label: "Command center", icon: "⌘" },
  { id: "new", label: "New debate", icon: "+" },
  { id: "debate", label: "Active debate", icon: "◈" },
  { id: "decision", label: "Decision record", icon: "✓" },
];

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
  const [runSummary, setRunSummary] = useState<RunRoundSummary | null>(null);
  const [positions, setPositions] = useState<StoredPosition[]>([]);
  const [decisionRecord, setDecisionRecord] = useState<unknown | null>(null);
  const [decisionRationale, setDecisionRationale] = useState("");
  const [exportSummary, setExportSummary] = useState<ExportSummary | null>(null);
  const [question, setQuestion] = useState("");
  const [mode, setMode] = useState("DISCOVERY");
  const [productType, setProductType] = useState("DESKTOP");
  const [decisionType, setDecisionType] = useState("ARCHITECTURE");
  const [priority, setPriority] = useState("Correctness over speed");
  const [constraints, setConstraints] = useState("Local-first\nNo autonomous implementation");
  const [optionA, setOptionA] = useState("");
  const [optionB, setOptionB] = useState("");
  const [repository, setRepository] = useState("");
  const [models, setModels] = useState({
    claude: "claude-haiku-4-5-20251001",
    antigravity: "gemini-3.7-flash-low",
    "codex-wsl": "gpt-5.6-luna",
  });
  const [notice, setNotice] = useState("Runtime is in preview mode until the Tauri shell is launched.");

  useEffect(() => {
    invoke<ProviderStatus[]>("provider_statuses")
      .then((result) => {
        setProviders(result);
        setNotice("Provider boundary status checked by the local controller.");
      })
      .catch(() => setNotice("Preview data loaded. Tauri provider checks will run in the desktop shell."));
    invoke<DebateSummary[]>("recent_debates")
      .then((result) => {
        if (result.length > 0) {
          setDebate(result[0]);
          void refreshPositions(result[0].id);
        }
      })
      .catch(() => undefined);
  }, []);

  async function refreshPositions(debateId: string) {
    if (debateId === "preview-debate") return;
    try {
      setPositions(await invoke<StoredPosition[]>("debate_positions", { debateId }));
    } catch {
      setPositions([]);
    }
  }

  const readyCount = useMemo(() => providers.filter((provider) => provider.state === "READY").length, [providers]);

  const nextRound = useMemo(() => {
    if (!debate) return null;
    if (debate.state === "DRAFT") return 1;
    if (debate.state === "CROSS_EXAMINATION") return runSummary?.round === 4 ? 5 : 2;
    if (debate.state === "FINAL_POSITIONS") return runSummary?.round === 4 ? 5 : 3;
    if (debate.state === "AWAITING_HUMAN_DECISION") return 4;
    return null;
  }, [debate, runSummary]);

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
      current_leaning: null,
      current_leaning_reason: null,
      repository: repository.trim() || null,
    };
    try {
      const created = await invoke<DebateSummary>("create_debate", { intake, modelOverrides: models });
      setDebate(created);
      setNotice("Debate created. Provider turns remain human-visible and auditable.");
    } catch {
      setDebate({ id: "preview-debate", state: "DRAFT", question: intake.question, created_at: new Date().toISOString() });
      setNotice("Preview debate created. Launch the Tauri shell to persist it to SQLite.");
    }
    setScreen("debate");
  }

  async function dispatchRound(round: number) {
    if (!debate || debate.id === "preview-debate") {
      setNotice("Launch the Tauri desktop shell to dispatch a persisted provider round.");
      return;
    }
    setNotice("Dispatching round " + round + ". Each seat gets a fresh process.");
    try {
      const result = await invoke<RunRoundSummary>("run_round", {
        debateId: debate.id,
        round,
      });
      setRunSummary(result);
      await refreshPositions(debate.id);
      setDebate((current) => current ? { ...current, state: result.state } : current);
      setNotice(result.message);
      if (result.state === "AWAITING_HUMAN_DECISION") {
        setScreen("decision");
      }
    } catch (error) {
      setNotice("Round " + round + " did not dispatch: " + String(error));
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
          kind: "APPROVE_OPTION",
          selectedOption: null,
          modifiedDecision: null,
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
      const result = await invoke<ExportSummary>("compile_export", { record: decisionRecord });
      setExportSummary(result);
      setDebate((current) => current ? { ...current, state: "EXPORTED" } : current);
      setNotice("Export compiled into Council application data. No provider handoff was performed.");
    } catch (error) {
      setNotice("Export was not compiled: " + String(error));
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
          {screen === "home" && <HomeScreen providers={providers} setScreen={setScreen} debate={debate} />}
          {screen === "new" && <LiveNewDebateScreen question={question} setQuestion={setQuestion} mode={mode} setMode={setMode} productType={productType} setProductType={setProductType} decisionType={decisionType} setDecisionType={setDecisionType} priority={priority} setPriority={setPriority} constraints={constraints} setConstraints={setConstraints} optionA={optionA} setOptionA={setOptionA} optionB={optionB} setOptionB={setOptionB} repository={repository} setRepository={setRepository} models={models} setModels={setModels} startDebate={startDebate} />}
          {screen === "debate" && <LiveDebateScreen providers={providers} debate={debate} runSummary={runSummary} positions={positions} nextRound={nextRound} dispatchRound={dispatchRound} setScreen={setScreen} />}
          {screen === "decision" && <LiveDecisionScreen debate={debate} runSummary={runSummary} positions={positions} decisionRationale={decisionRationale} setDecisionRationale={setDecisionRationale} decisionRecord={decisionRecord} exportSummary={exportSummary} recordDecision={recordDecision} compileExport={compileExport} dispatchRound={dispatchRound} setScreen={setScreen} />}
          {screen === "settings" && <SettingsScreen providers={providers} />}
        </div>
        <div className="notice-bar"><span className="notice-symbol">i</span><span>{notice}</span><span className="notice-right">No provider state is hidden</span></div>
      </main>
    </div>
  );
}

function HomeScreen({ providers, setScreen, debate }: { providers: ProviderStatus[]; setScreen: (screen: Screen) => void; debate: DebateSummary | null }) {
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
        <div className="recent-panel"><div className="panel-heading"><div><span className="section-kicker">02 / RECENT</span><h2>Debate log</h2></div><span className="count-badge">{debate ? "01" : "00"}</span></div>{debate ? <button className="recent-row" onClick={() => setScreen("debate")}><div className="recent-icon">◈</div><div className="recent-main"><strong>{debate.question}</strong><span>{debate.id} · {debate.state}</span></div><span className="recent-arrow">→</span></button> : <div className="empty-state"><div className="empty-mark">＋</div><p>Your first decision packet is waiting.</p><button className="text-button" onClick={() => setScreen("new")}>Create debate →</button></div>}</div>
        <div className="principles-panel"><span className="section-kicker">03 / OPERATING CONTRACT</span><h2>Evidence over<br /><em>confidence.</em></h2><div className="principle-list"><div><span>01</span><p>Agents draft positions.<br /><b>the owner decides.</b></p></div><div><span>02</span><p>Packets are hashed.<br /><b>Sessions are fresh.</b></p></div><div><span>03</span><p>Dissent is preserved.<br /><b>Majority is not authority.</b></p></div></div></div>
      </div>
    </section>
  );
}

function ProviderCard({ provider }: { provider: ProviderStatus }) {
  return <article className="provider-card"><div className="provider-head"><div className={`provider-glyph glyph-${provider.provider.toLowerCase()}`}>{provider.provider === "CLAUDE" ? "C" : provider.provider === "ANTIGRAVITY" ? "A" : "✦"}</div><div className="provider-name"><strong>{provider.label}</strong><span>{provider.model}</span></div><span className={`state-badge ${statusClass(provider.state)}`}><i />{provider.state.replace("_", " ")}</span></div><div className="provider-rule" /><div className="provider-meta"><span>REQUESTED</span><strong>{provider.requested}</strong></div><div className="provider-meta"><span>SERVED IDENTITY</span><strong className={provider.served === "VERIFIED_MATCH" ? "green-text" : "amber-text"}>{provider.served.replaceAll("_", " ")}</strong></div><div className="provider-foot"><span className="cert-stamp">{provider.certification.replaceAll("_", " ")}</span><span className="provider-detail">{provider.detail}</span></div></article>;
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

function SettingsScreen({ providers }: { providers: ProviderStatus[] }) {
  return <section className="page page-settings"><div className="eyebrow"><span className="eyebrow-line" />SETTINGS / CERTIFICATION</div><div className="settings-header"><div><h1>Runtime boundaries.</h1><p>Every provider is explicit about what it can see, what it reports, and where it is limited.</p></div><span className="settings-lock">⌁ LOCAL ONLY</span></div><div className="settings-list">{providers.map((provider) => <div className="settings-row" key={provider.provider}><div className={`provider-glyph glyph-${provider.provider.toLowerCase()}`}>{provider.provider[0]}</div><div className="settings-provider"><strong>{provider.label}</strong><span>{provider.provider === "CODEX_WSL" ? "CouncilCodexWSL / council / read-only" : provider.detail}</span></div><div className="settings-model"><span>REQUESTED MODEL</span><strong>{provider.requested}</strong></div><div className="settings-cert"><span className={`state-badge ${statusClass(provider.state)}`}><i />{provider.state.replace("_", " ")}</span><small>{provider.certification.replaceAll("_", " ")}</small></div></div>)}</div><div className="settings-note"><span>!</span><p><strong>Declared limitation</strong> means the provider is usable under a known boundary, but the product will keep that limitation visible in every debate record.</p></div></section>;
}

function LiveNewDebateScreen(props: {
  question: string;
  setQuestion: (value: string) => void;
  mode: string;
  setMode: (value: string) => void;
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
          {props.mode === "COMPARE" ? <div className="form-two-up"><div><label className="field-label">OPTION A <span>REQUIRED</span></label><input className="text-input" value={props.optionA} onChange={(event) => props.setOptionA(event.target.value)} placeholder="First candidate" /></div><div><label className="field-label">OPTION B <span>REQUIRED</span></label><input className="text-input" value={props.optionB} onChange={(event) => props.setOptionB(event.target.value)} placeholder="Second candidate" /></div></div> : null}
          <div className="form-two-up"><div><label className="field-label">PRODUCT SURFACE</label><select className="text-input" value={props.productType} onChange={(event) => props.setProductType(event.target.value)}><option value="DESKTOP">Desktop app</option><option value="WEB">Web product</option><option value="ANDROID">Android app</option><option value="WINDOWS">Windows</option><option value="GAME">Game</option><option value="BACKEND">Backend</option><option value="AI_SYSTEM">AI system</option><option value="OTHER">Other</option></select></div><div><label className="field-label">DECISION TYPE</label><select className="text-input" value={props.decisionType} onChange={(event) => props.setDecisionType(event.target.value)}><option value="ARCHITECTURE">Architecture</option><option value="STACK">Stack</option><option value="DESIGN">Design</option><option value="SECURITY">Security</option><option value="PERFORMANCE">Performance</option><option value="DATABASE">Database</option><option value="DEPENDENCY">Dependency</option><option value="TESTING">Testing</option><option value="GENERAL">General</option></select></div></div>
          <label className="field-label">HARD CONSTRAINTS</label><textarea className="text-input constraints-input" value={props.constraints} onChange={(event) => props.setConstraints(event.target.value)} rows={3} />
          <label className="field-label">OPTIONAL REPOSITORY PATH <span>SNAPSHOT REQUIRED</span></label><input className="text-input" value={props.repository} onChange={(event) => props.setRepository(event.target.value)} placeholder="C:\path\to\project, or leave blank for greenfield" /><div className="field-hint">A repository path is recorded as intake only. The controller must create and verify a sanitized snapshot before dispatch.</div>
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
  positions: StoredPosition[];
  nextRound: number | null;
  dispatchRound: (round: number) => void;
  setScreen: (screen: Screen) => void;
}) {
  const phases = ["PREFLIGHT", "SNAPSHOT", "OPENING", "CROSS-EXAM", "FINAL POSITIONS"];
  const phaseIndex = props.debate?.state === "DRAFT" ? 0
    : props.debate?.state === "OPENING" ? 2
    : props.debate?.state === "CROSS_EXAMINATION" ? 3
    : props.debate?.state === "FINAL_POSITIONS" ? 4
    : props.debate?.state === "AWAITING_HUMAN_DECISION" ? 5
    : 0;
  const actionLabel = props.nextRound === 1 ? "Dispatch opening round"
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
          {actionLabel && props.nextRound ? <button className="primary-button packet-action" onClick={() => props.dispatchRound(props.nextRound as number)}><span>{actionLabel}</span><b>↗</b></button> : props.debate?.state === "AWAITING_HUMAN_DECISION" ? <button className="text-button muted-button" onClick={() => props.setScreen("decision")}>Open human decision gate →</button> : null}
        </aside>
      </div>
      {props.positions.length > 0 ? <div className="positions-strip"><div className="panel-heading"><div><span className="section-kicker">STORED POSITIONS</span><h2>Recommendations and risks</h2></div><span className="count-badge">{props.positions.length} PROVIDERS</span></div><div className="position-cards">{props.positions.map((position) => <article className="position-card" key={position.provider}><div className="position-card-top"><strong>{position.provider.replace("-", " ").toUpperCase()}</strong><span>{position.position.commitment.replaceAll("_", " ")}</span></div><h3>{position.position.recommendation}</h3><p>{position.position.risks[0] ?? "No risk recorded."}</p><small>{position.position.flip_condition}</small></article>)}</div></div> : null}
      {props.runSummary ? <div className="run-note"><strong>{props.runSummary.message}</strong><span>round {props.runSummary.round} · {props.runSummary.valid_positions} usable positions · state {props.runSummary.state}</span></div> : null}
    </section>
  );
}

function LiveDecisionScreen(props: {
  debate: DebateSummary | null;
  runSummary: RunRoundSummary | null;
  positions: StoredPosition[];
  decisionRationale: string;
  setDecisionRationale: (value: string) => void;
  decisionRecord: unknown | null;
  exportSummary: ExportSummary | null;
  recordDecision: () => void;
  compileExport: () => void;
  dispatchRound: (round: number) => void;
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
          <div className="evidence-strip"><span>↗</span><div><strong>{props.runSummary?.valid_positions ?? 0} usable positions</strong><small>{props.runSummary ? "raw artifacts and attempts persisted" : "awaiting controlled dispatch"}</small></div><button className="text-button">Inspect evidence</button></div>
        </div>
        <aside className="dissent-panel"><span className="section-kicker">MINORITY REPORT</span><h2>Dissent stays in the record.</h2><div className="dissent-quote">{props.positions[1]?.position.recommendation ?? "No recommendation becomes authority by being the majority. The final record keeps every usable seat and its stated limits."}</div><div className="dissent-source"><span className="provider-glyph glyph-antigravity">A</span><div><strong>{props.positions[1]?.provider.replace("-", " ").toUpperCase() ?? "ALL SEATS REMAIN ATTRIBUTABLE"}</strong><small>REQUESTED / SERVED STATUS VISIBLE</small></div></div></aside>
      </div>
      <div className="human-gate-panel">
        <div><span className="section-kicker">YOUR DECISION</span><h2>What are you committing to?</h2><textarea className="text-input decision-rationale" value={props.decisionRationale} onChange={(event) => props.setDecisionRationale(event.target.value)} placeholder="State the decision, the rationale, and what evidence would make you revisit it." rows={4} /></div>
        <div className="decision-gate-actions">{!decided ? <button className="primary-button" onClick={props.recordDecision} disabled={!awaiting}><span>{awaiting ? "Record human decision" : "Await final positions"}</span><b>✓</b></button> : <button className="primary-button" onClick={props.compileExport} disabled={Boolean(props.exportSummary)}><span>{props.exportSummary ? "Export compiled" : "Compile deterministic export"}</span><b>↗</b></button>}{props.exportSummary ? <div className="export-hash"><span>MASTER PROMPT</span><strong>{props.exportSummary.master_prompt_hash.slice(0, 18)}…</strong><small>Manual copy only. No handoff performed.</small></div> : null}</div>
      </div>
      <div className="decision-actions"><button className="secondary-button" onClick={() => props.setScreen("debate")}>Return to debate</button><div>{awaiting ? <button className="outline-button" onClick={() => { props.setScreen("debate"); props.dispatchRound(4); }}>Continue targeted debate</button> : null}{props.debate?.state === "EXPORTED" ? <span className="export-complete">EXPORTED TO LOCAL APP DATA</span> : null}</div></div>
    </section>
  );
}

export default App;
