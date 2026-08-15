# Council Decision Benchmark Corpus

## Purpose and use

These fixtures test whether Council reasons from constraints, evidence, and tradeoffs rather than repeating a fashionable answer. They are not answer keys. A strong result may choose different options when it identifies a different constraint interpretation, but it must surface the considerations and risks named here.

Repository fixtures are descriptions only. They do not create production repositories or authorize changes to a real checkout.

## Fixture 01: SQLite versus PostgreSQL for a local-first product

- **ID:** `ARCH-001`
- **Question:** Should a Windows-first local-first desktop product keep SQLite or migrate its primary state store to PostgreSQL?
- **Product Type:** Local-first desktop application
- **Decision Type:** Persistence architecture
- **Mode:** Compare
- **Candidate Options if Compare:** SQLite; PostgreSQL
- **Hard Constraints:** Offline operation must work; one active desktop user is expected; local data must remain inspectable; no mandatory hosted service.
- **Primary Priority:** Reliable local behavior with low operational burden.
- **Repository Required?:** yes
- **Repository fixture characteristics:** SQLite state layer, single-writer queue, background worker, migrations, no network database, export/import path, and evidence of current concurrency assumptions.
- **Important Considerations:** offline durability, backup, migration, concurrency, sync roadmap, security, deployment, and whether the problem is actually query scale.
- **Known Traps:** treating PostgreSQL maturity as proof of local fit; assuming future multi-user scale is a current requirement; ignoring sync conflict design.
- **What A Strong Council Should Surface:** SQLite is often the best local default, PostgreSQL becomes justified by a real multi-user or server requirement, and the exit path should be preserved through a storage boundary and migrations.

## Fixture 02: REST versus GraphQL for a mixed desktop/web API

- **ID:** `ARCH-002`
- **Question:** Should an existing product API remain REST or adopt GraphQL to serve a desktop client and a browser dashboard?
- **Product Type:** Multi-client application backend
- **Decision Type:** API architecture
- **Mode:** Compare
- **Candidate Options if Compare:** REST; GraphQL
- **Hard Constraints:** Small team; existing REST endpoints are in production; clients need predictable auth and caching; no dedicated platform team.
- **Primary Priority:** Maintainability and reliable delivery.
- **Repository Required?:** yes
- **Repository fixture characteristics:** REST routes, OpenAPI or route tests, two clients with overlapping but not identical payloads, auth middleware, and current performance logs.
- **Important Considerations:** endpoint evolution, overfetching, caching, authorization at field level, tooling, observability, migration cost, and client count.
- **Known Traps:** treating fewer network requests as automatic superiority; ignoring GraphQL resolver complexity and authorization; proposing a rewrite for a payload-shape problem.
- **What A Strong Council Should Surface:** improve REST composition or add a narrow aggregation layer when that solves the problem; GraphQL earns its cost when client query variability and a capable ownership model are real.

## Fixture 03: Tauri versus Electron for a Windows-first desktop tool

- **ID:** `DESKTOP-003`
- **Question:** Should a Windows-first local technical tool use Tauri or Electron?
- **Product Type:** Desktop application
- **Decision Type:** Desktop stack
- **Mode:** Compare
- **Candidate Options if Compare:** Tauri; Electron
- **Hard Constraints:** Windows is the launch platform; local filesystem integration matters; the UI is React-compatible; installer and update behavior must be supportable by a small team.
- **Primary Priority:** Safe local boundary with a maintainable desktop shell.
- **Repository Required?:** no
- **Important Considerations:** WebView2 dependency, Node-native ecosystem, memory footprint, IPC boundary, installer complexity, cross-platform optionality, debugging, and team skill.
- **Known Traps:** choosing by bundle size alone; assuming Tauri removes all native complexity; assuming Electron automatically makes cross-platform support cheap.
- **What A Strong Council Should Surface:** fit depends on native integration and platform horizon; isolate domain logic so the shell remains replaceable; name the operational cost of WebView2 or Electron updates.

## Fixture 04: Godot versus Unity for a 2D educational game

- **ID:** `GAME-004`
- **Question:** Should a small team build a 2D educational game in Godot or Unity?
- **Product Type:** 2D educational game
- **Decision Type:** Game engine selection
- **Mode:** Compare
- **Candidate Options if Compare:** Godot; Unity
- **Hard Constraints:** Small team; desktop and Android targets; limited budget; simple 2D scenes; accessibility and fast iteration matter.
- **Primary Priority:** Iteration speed and long-term ownership.
- **Repository Required?:** no
- **Important Considerations:** export targets, asset pipeline, licensing, plugin ecosystem, tooling, educator updates, performance, and team familiarity.
- **Known Traps:** equating larger ecosystem with better fit; ignoring Unity policy or version cost; ignoring Godot's gaps for a required plugin.
- **What A Strong Council Should Surface:** the answer turns on required tooling, export reliability, team experience, and licensing risk; a boring engine with a proven target path may beat a feature-rich one.

## Fixture 05: Unreal versus Unity for a higher-end 3D game

- **ID:** `GAME-005`
- **Question:** Should a small studio use Unreal or Unity for a higher-end 3D action game?
- **Product Type:** Higher-end 3D game
- **Decision Type:** Game engine selection
- **Mode:** Compare
- **Candidate Options if Compare:** Unreal Engine; Unity
- **Hard Constraints:** High visual fidelity; console-like rendering target; small but experienced team; first playable in 12 months; no custom engine.
- **Primary Priority:** Visual quality and production feasibility.
- **Repository Required?:** no
- **Important Considerations:** renderer strength, asset workflow, performance budget, team language skills, licensing, build automation, source access, and multiplayer needs.
- **Known Traps:** choosing based on screenshots; ignoring content production capacity; assuming engine rendering solves art direction; overlooking licensing and build costs.
- **What A Strong Council Should Surface:** the winner depends on visual target, team pipeline, and delivery horizon; engine fit cannot compensate for a missing content or optimization plan.

## Fixture 06: Native Android versus Flutter for a field app

- **ID:** `MOBILE-006`
- **Question:** Should a field-service Android app use native Android or Flutter?
- **Product Type:** Android field-service app
- **Decision Type:** Mobile stack
- **Mode:** Compare
- **Candidate Options if Compare:** Native Android; Flutter
- **Hard Constraints:** Android-only launch; offline mode; camera, Bluetooth, background sync, rugged devices, and long maintenance horizon.
- **Primary Priority:** Reliable device integration and offline operation.
- **Repository Required?:** no
- **Important Considerations:** hardware APIs, background restrictions, UI consistency, team staffing, test devices, update cadence, and future iOS possibility.
- **Known Traps:** choosing Flutter only for shared UI when there is no second platform; choosing native without a plan for iteration speed and design consistency.
- **What A Strong Council Should Surface:** Android-only and hardware-heavy constraints favor native unless a proven Flutter integration path exists; future iOS is not free optionality.

## Fixture 07: Native Android versus React Native for a consumer app

- **ID:** `MOBILE-007`
- **Question:** Should a consumer app launch Android-native or React Native when iOS is planned but not staffed yet?
- **Product Type:** Consumer mobile application
- **Decision Type:** Mobile stack
- **Mode:** Compare
- **Candidate Options if Compare:** Native Android; React Native
- **Hard Constraints:** Android launch in six months; iOS may follow; push notifications, payments, deep links, and accessibility are required.
- **Primary Priority:** Shipping a reliable Android launch without blocking future platform work.
- **Repository Required?:** no
- **Important Considerations:** native module quality, platform parity, startup performance, team skill, release process, accessibility, and eventual migration risk.
- **Known Traps:** treating “write once” as zero platform work; assuming native guarantees a faster launch; ignoring the cost of maintaining platform-specific modules.
- **What A Strong Council Should Surface:** platform horizon and team composition matter more than a generic cross-platform slogan; define which capabilities must remain native.

## Fixture 08: Monolith versus microservices for a growing product

- **ID:** `ARCH-008`
- **Question:** Should an existing modular monolith be split into microservices before the next growth phase?
- **Product Type:** SaaS product backend
- **Decision Type:** Deployment architecture
- **Mode:** Compare
- **Candidate Options if Compare:** Keep modular monolith; split into microservices
- **Hard Constraints:** Team of four; current deployment is reliable; one module has scaling pain; on-call capacity is limited.
- **Primary Priority:** Reduce the actual bottleneck without multiplying operational burden.
- **Repository Required?:** yes
- **Repository fixture characteristics:** modular boundaries, shared database, deployment scripts, one high-load module, request traces, and incident history.
- **Important Considerations:** independent scaling, data ownership, release coupling, observability, queues, failure modes, and organizational ownership.
- **Known Traps:** using microservices as a synonym for scale; splitting by folder rather than capability; ignoring distributed transactions and on-call cost.
- **What A Strong Council Should Surface:** isolate the measured bottleneck first, possibly with a worker or bounded extraction; a full split needs ownership, observability, and a data migration plan.

## Fixture 09: Local-first versus cloud-first for a casework tool

- **ID:** `PRODUCT-009`
- **Question:** Should a casework tool be local-first with later sync or cloud-first with a thin offline cache?
- **Product Type:** Professional casework application
- **Decision Type:** Data and product architecture
- **Mode:** Compare
- **Candidate Options if Compare:** Local-first; cloud-first
- **Hard Constraints:** Field workers lose connectivity; records are sensitive; multiple staff eventually need shared access; audit history is required.
- **Primary Priority:** Safe offline work without losing accountable synchronization.
- **Repository Required?:** no
- **Important Considerations:** conflict resolution, encryption, backup, identity, audit, device loss, sync UX, and support burden.
- **Known Traps:** calling a cache local-first; postponing conflict policy; treating cloud access as a substitute for offline data design.
- **What A Strong Council Should Surface:** the decision is about conflict and authority models, not storage branding; a staged local-first design can be valid if sync rules are specified before implementation.

## Fixture 10: Authentication architecture for a desktop and API product

- **ID:** `SEC-010`
- **Question:** Should a Windows desktop app use a hosted identity provider, device codes, or a custom token flow for its API?
- **Product Type:** Desktop client with backend API
- **Decision Type:** Authentication architecture
- **Mode:** Compare
- **Candidate Options if Compare:** Hosted OIDC provider; device-code flow; custom auth service
- **Hard Constraints:** Desktop secret storage is limited; refresh tokens must be protected; enterprise users may require SSO; no custom password database desired.
- **Primary Priority:** Secure, supportable authentication with minimal credential handling.
- **Repository Required?:** yes
- **Repository fixture characteristics:** current login flow, token storage, API middleware, redirect URIs, local config, and any existing account or tenant boundaries.
- **Important Considerations:** phishing resistance, token lifetime, revocation, SSO, device registration, offline behavior, support, and vendor exit.
- **Known Traps:** storing client secrets in a desktop binary; treating authentication as authorization; trusting a provider without checking local token handling.
- **What A Strong Council Should Surface:** use established standards and separate authentication from authorization; document the device boundary and recovery path; do not invent a credential store without a compelling requirement.

## Fixture 11: SQL versus document database for a catalog

- **ID:** `DATA-011`
- **Question:** Should a product catalog use a relational SQL database or a document database?
- **Product Type:** Catalog and search service
- **Decision Type:** Data model selection
- **Mode:** Compare
- **Candidate Options if Compare:** SQL database; document database
- **Hard Constraints:** Product attributes vary by category; orders and inventory need consistency; reporting is expected; a small team owns operations.
- **Primary Priority:** Correctness and useful reporting with manageable evolution.
- **Repository Required?:** no
- **Important Considerations:** schema variability, transactions, search, analytics, migrations, indexes, tooling, and operational maturity.
- **Known Traps:** equating flexible schema with easy schema; ignoring relational reporting and inventory invariants; adding a second database for a search problem.
- **What A Strong Council Should Surface:** model stable transactional facts relationally and isolate true document-shaped content; choose a document store only with a clear access and consistency model.

## Fixture 12: Whether to introduce Redis

- **ID:** `OPS-012`
- **Question:** Should an existing web service introduce Redis for performance and background coordination?
- **Product Type:** Web service
- **Decision Type:** Dependency and infrastructure adoption
- **Mode:** Compare
- **Candidate Options if Compare:** Keep current database/cache; introduce Redis
- **Hard Constraints:** Current database is reliable; one endpoint is slow; team has no Redis operational experience; data loss is unacceptable for core records.
- **Primary Priority:** Solve measured latency without creating a second source of truth.
- **Repository Required?:** yes
- **Repository fixture characteristics:** slow query traces, current cache headers, database schema/indexes, deployment topology, and background worker behavior.
- **Important Considerations:** cache invalidation, eviction, persistence, failure behavior, cost, observability, and whether an index or query change is enough.
- **Known Traps:** adding Redis because it is common; using it for durable state without recovery proof; measuring only cache hits.
- **What A Strong Council Should Surface:** first optimize the measured bottleneck; if Redis is introduced, define ownership, stale-data policy, outage behavior, and a non-Redis fallback.

## Fixture 13: Adopting a dependency for PDF generation

- **ID:** `DEP-013`
- **Question:** Should a product adopt a third-party PDF library or keep its current minimal export implementation?
- **Product Type:** Desktop and web reporting product
- **Decision Type:** Dependency adoption
- **Mode:** Compare
- **Candidate Options if Compare:** Existing implementation; library A; library B
- **Hard Constraints:** Legal documents need stable layout; exports must work offline; license must be compatible; the team cannot maintain a rendering engine.
- **Primary Priority:** Correct output with a defensible maintenance and licensing story.
- **Repository Required?:** yes
- **Repository fixture characteristics:** current exporter, sample documents, build scripts, lockfile, license policy, and known layout failures.
- **Important Considerations:** license, binary size, fonts, determinism, security updates, platform support, and migration rollback.
- **Known Traps:** choosing the most popular library; ignoring transitive licenses; assuming a larger library guarantees deterministic output.
- **What A Strong Council Should Surface:** compare a small set with actual sample outputs and license evidence; the winner must include a pin/update policy and an exit plan.

## Fixture 14: Whether to introduce a message queue

- **ID:** `ARCH-014`
- **Question:** Should an application add a message queue for email, imports, and webhook processing?
- **Product Type:** Web application with integrations
- **Decision Type:** Background architecture
- **Mode:** Compare
- **Candidate Options if Compare:** In-process jobs; database-backed jobs; managed message queue
- **Hard Constraints:** At-least-once delivery is acceptable; idempotency is not yet consistent; current volume is moderate; operators are few.
- **Primary Priority:** Reliable work completion and debuggability.
- **Repository Required?:** yes
- **Repository fixture characteristics:** current job runner, retry logic, webhook handlers, database transactions, deployment process, and incident examples.
- **Important Considerations:** retries, idempotency, visibility, ordering, poison messages, costs, local development, and operational ownership.
- **Known Traps:** queueing before defining idempotency; confusing asynchronous execution with reliability; adding a hosted service to hide a transaction problem.
- **What A Strong Council Should Surface:** start with the simplest durable mechanism that meets volume and failure needs; state when a queue becomes justified and what semantics it must provide.

## Fixture 15: Testing strategy for a multi-provider controller

- **ID:** `QA-015`
- **Question:** What testing strategy should protect a controller that validates provider outputs, persists rounds, and handles partial failure?
- **Product Type:** Local-first multi-provider desktop application
- **Decision Type:** Testing architecture
- **Mode:** Compare
- **Candidate Options if Compare:** Unit-heavy contract tests; integration-heavy provider fixtures; end-to-end-first strategy
- **Hard Constraints:** Real provider calls are expensive and nondeterministic; safety boundaries must be proven; UI and CLI share core behavior.
- **Primary Priority:** Catch boundary and state-machine regressions with reproducible evidence.
- **Repository Required?:** yes
- **Repository fixture characteristics:** state transitions, provider command contracts, schema validators, snapshot code, fixture outputs, and existing build/test commands.
- **Important Considerations:** deterministic fixtures, property tests, process cancellation, citation verification, UI smoke tests, and live certification gates.
- **Known Traps:** treating passing unit tests as proof of provider isolation; snapshotting implementation details; omitting failure paths.
- **What A Strong Council Should Surface:** layer tests by risk: deterministic core, contract fixtures, boundary/safety checks, and a small set of live certification and UI acceptance tests.

## Fixture 16: Desktop-app architecture for a local technical workstation

- **ID:** `DESKTOP-016`
- **Question:** What architecture should a Windows desktop workstation use for local state, native commands, and a React interface?
- **Product Type:** Windows desktop developer tool
- **Decision Type:** Desktop application architecture
- **Mode:** Discovery
- **Candidate Options if Compare:** N/A; discovery should nominate a bounded set
- **Hard Constraints:** Local-first; filesystem and subprocess control; auditable state; no automatic external publishing; small team.
- **Primary Priority:** Safe local integration with inspectable state.
- **Repository Required?:** no
- **Important Considerations:** shell boundary, IPC, persistence, crash recovery, installer, permissions, UI testability, and upgrade path.
- **Known Traps:** starting from a web dashboard template; putting process control in the renderer; hiding state in provider sessions; using a cloud backend by default.
- **What A Strong Council Should Surface:** a small local shell plus deterministic core is likely; the candidate set must include a boring established desktop path and define what stays outside the UI process.

## Fixture 17: Persistence for an AI-agent workflow

- **ID:** `AGENT-017`
- **Question:** Should an AI-agent workflow persist conversational sessions, immutable packets, or both?
- **Product Type:** AI-assisted workflow tool
- **Decision Type:** Agent persistence architecture
- **Mode:** Compare
- **Candidate Options if Compare:** Session resume; stateless immutable packets; hybrid with explicit reconstruction
- **Hard Constraints:** Reproducibility and crash recovery matter; providers differ; hidden context must not affect a decision record.
- **Primary Priority:** Auditability and deterministic reconstruction.
- **Repository Required?:** yes
- **Repository fixture characteristics:** debate/turn persistence, packet or snapshot files, raw provider outputs, resume logic, and audit records.
- **Important Considerations:** context transmission cost, provider session semantics, raw artifacts, hashes, privacy, and replay.
- **Known Traps:** assuming provider sessions are portable; treating a transcript as enough provenance; optimizing token count before proving state reconstruction.
- **What A Strong Council Should Surface:** immutable packets and explicit state reconstruction are the trustworthy baseline; a hybrid may optimize cost only if the visible packet remains authoritative.

## Fixture 18: Vector database decision for document retrieval

- **ID:** `DATA-018`
- **Question:** Does a support knowledge tool need a vector database, or will full-text search plus metadata filters suffice?
- **Product Type:** Internal knowledge tool
- **Decision Type:** Retrieval architecture
- **Mode:** Compare
- **Candidate Options if Compare:** Full-text search; relational full-text plus embeddings; dedicated vector database
- **Hard Constraints:** Corpus is under 100,000 documents; citations must be exact; data is sensitive; one team owns the system.
- **Primary Priority:** Relevant, auditable retrieval with low operational burden.
- **Repository Required?:** no
- **Important Considerations:** corpus size, update frequency, exact citations, hybrid search, embedding drift, cost, deletion, and evaluation data.
- **Known Traps:** adding a vector database because the product is AI; confusing semantic similarity with answer correctness; ignoring deletion and re-indexing.
- **What A Strong Council Should Surface:** measure retrieval quality and citation needs first; a dedicated vector service is justified only by access patterns and operational value that simpler options cannot meet.

## Fixture 19: Background job architecture for imports

- **ID:** `OPS-019`
- **Question:** Should long-running imports run in a worker process, a thread pool, or a separate service?
- **Product Type:** Data-import desktop/web hybrid
- **Decision Type:** Background execution
- **Mode:** Compare
- **Candidate Options if Compare:** In-process worker; separate worker process; hosted job service
- **Hard Constraints:** Imports must survive UI restart; users need progress and cancel; files are local; no required cloud control plane.
- **Primary Priority:** Reliable, cancellable local work with clear progress.
- **Repository Required?:** yes
- **Repository fixture characteristics:** current import code, UI bridge, progress events, cancellation behavior, persisted job records, and sample large files.
- **Important Considerations:** process ownership, cancellation, resume, memory, crash recovery, file locks, and state persistence.
- **Known Traps:** using threads as a substitute for persistence; reporting progress without a durable job identity; killing the whole app to cancel one job.
- **What A Strong Council Should Surface:** separate process or durable worker boundary may fit when restart/cancel matters, but the decision must include job state and safe cleanup.

## Fixture 20: Hosting strategy for a small public API

- **ID:** `DEPLOY-020`
- **Question:** Should a small public API use a managed platform, a container on a virtual machine, or serverless functions?
- **Product Type:** Public web API
- **Decision Type:** Hosting strategy
- **Mode:** Compare
- **Candidate Options if Compare:** Managed application platform; VM/container; serverless functions
- **Hard Constraints:** Small team; uneven traffic; predictable monthly budget; background work and database access required.
- **Primary Priority:** Low operational burden with a clear cost ceiling.
- **Repository Required?:** no
- **Important Considerations:** cold starts, networking, logs, database, deployment rollback, vendor lock-in, scaling, and support.
- **Known Traps:** optimizing for hypothetical scale; comparing sticker prices without operations; forgetting scheduled jobs and migrations.
- **What A Strong Council Should Surface:** choose the simplest platform that satisfies workload and rollback needs; name the point at which a VM or serverless tradeoff changes.

## Fixture 21: Windows-only versus cross-platform desktop

- **ID:** `PRODUCT-021`
- **Question:** Should a technical workstation launch Windows-only or support macOS and Linux in V1?
- **Product Type:** Desktop developer tool
- **Decision Type:** Platform scope
- **Mode:** Compare
- **Candidate Options if Compare:** Windows-only; cross-platform desktop
- **Hard Constraints:** Founding user is on Windows; provider boundaries differ by OS; local security controls are important; small team.
- **Primary Priority:** Prove a safe, useful product before widening the platform promise.
- **Repository Required?:** no
- **Important Considerations:** market reach, process control, filesystem semantics, installer support, provider availability, testing matrix, and architecture portability.
- **Known Traps:** treating cross-platform as a checkbox; hiding OS-specific safety assumptions; promising parity without provider certification.
- **What A Strong Council Should Surface:** Windows-only can be a deliberate safety and proof boundary if the architecture isolates platform concerns and records a credible expansion path.

## Fixture 22: Game visual-direction decision

- **ID:** `DESIGN-022`
- **Question:** Should a small narrative game use stylized low-poly 3D, hand-painted 2D, or limited monochrome illustration?
- **Product Type:** Narrative game
- **Decision Type:** Visual direction
- **Mode:** Compare
- **Candidate Options if Compare:** Stylized low-poly 3D; hand-painted 2D; limited monochrome illustration
- **Hard Constraints:** Small art team; readable characters on handheld screens; emotional tone is quiet and strange; content must be produced consistently.
- **Primary Priority:** Distinctive identity with sustainable asset production.
- **Repository Required?:** no
- **Important Considerations:** silhouette, animation, environment consistency, lighting, UI/HUD, asset throughput, and audience expectations.
- **Known Traps:** choosing concept-art appeal over production capacity; mixing asset styles; using visual effects to hide weak composition.
- **What A Strong Council Should Surface:** visual direction is a production system; the winner must include a repeatable asset grammar, not just a mood board.

## Fixture 23: Web-application visual direction

- **ID:** `DESIGN-023`
- **Question:** Should a financial operations web app use dense utilitarian tables, editorial dashboard composition, or a calm card-based workflow?
- **Product Type:** Web operations application
- **Decision Type:** Product visual direction
- **Mode:** Compare
- **Candidate Options if Compare:** Dense utilitarian tables; editorial dashboard; calm card-based workflow
- **Hard Constraints:** Users monitor exceptions quickly; keyboard use is common; data density is high; auditability matters.
- **Primary Priority:** Fast comprehension and trustworthy action.
- **Repository Required?:** no
- **Important Considerations:** hierarchy, density, keyboard navigation, table behavior, status semantics, accessibility, and responsive degradation.
- **Known Traps:** treating “friendly” cards as inherently clearer; hiding exceptions in decoration; confusing visual polish with trust.
- **What A Strong Council Should Surface:** choose composition by task frequency and decision urgency; preserve dense evidence while using selective emphasis for exceptions.

## Fixture 24: AI-generated UI redesign

- **ID:** `DESIGN-024`
- **Question:** Should an existing generic AI dashboard be redesigned around a technical deliberation command center, and what should change first?
- **Product Type:** Desktop technical product
- **Decision Type:** UI redesign
- **Mode:** Compare
- **Candidate Options if Compare:** Incremental card polish; claim/evidence command center; transcript-first redesign
- **Hard Constraints:** Existing users need continuity; decision state must dominate; evidence and dissent must remain accessible; no decorative redesign without functional gain.
- **Primary Priority:** Make deliberation legible and intentional.
- **Repository Required?:** yes
- **Repository fixture characteristics:** current screens with repeated cards, provider columns, loading/error states, typography tokens, and user task recordings or annotated screenshots.
- **Important Considerations:** hierarchy, information density, provider differentiation, evidence readability, keyboard flow, state coverage, and migration of existing users.
- **Known Traps:** merely changing colors and radii; adding more cards; removing useful density in pursuit of minimalism; judging unseen states.
- **What A Strong Council Should Surface:** redesign the information architecture first, then component styling; use explicit acceptance criteria and `CANNOT_DETERMINE` where visual evidence is missing.

## Fixture 25: Greenfield stack discovery

- **ID:** `GREEN-025`
- **Question:** What stack should a small team choose for a local-first Windows technical workstation that may later support a web companion?
- **Product Type:** Greenfield local-first desktop product
- **Decision Type:** Full stack discovery
- **Mode:** Discovery
- **Candidate Options if Compare:** N/A; candidates must be nominated in R0 and bounded before R1
- **Hard Constraints:** Windows-first; local state; native process control; React-capable UI; no normal API billing; human-controlled export; small team.
- **Primary Priority:** Safe local delivery with a credible path to maintainable growth.
- **Repository Required?:** no
- **Important Considerations:** desktop shell, core language, persistence, IPC, provider isolation, packaging, testing, future web boundary, and cost to leave.
- **Known Traps:** comparing every current framework; selecting a cloud-first stack; treating AI scaffolding speed as architecture proof; skipping the status quo and boring alternative.
- **What A Strong Council Should Surface:** R0 should nominate a small set such as Tauri plus a native core, Electron, and a deliberately constrained alternative; R1 must evaluate the same set and make a bounded commitment.

## Corpus coverage notes

- Greenfield/no-repository fixtures include 3–7, 9, 11, 16, 18, 20–23, and 25.
- Repository-grounded fixtures include 1, 2, 8, 10, 12–15, 17, 19, and 24.
- Compare fixtures provide the candidate set up front. Fixture 25 tests Discovery mode and bounded candidate-union behavior.
- Several fixtures intentionally have no universal correct answer. The evaluator should reward constraint alignment, evidence handling, explicit tradeoffs, and useful uncertainty rather than a predetermined winner.
