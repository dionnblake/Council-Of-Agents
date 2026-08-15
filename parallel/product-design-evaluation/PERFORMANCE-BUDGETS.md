# Application Performance Budgets

## Scope

These budgets cover application-side responsiveness, persistence, snapshot preparation, evidence lookup, and cancellation. They do not set provider inference speed, model latency, quota behavior, or external service SLOs.

Budgets are targets for a normal supported Windows machine. Every measurement records hardware class, fixture size, build, cold/warm state, and whether a provider process is active.

## Interactive budgets

| Operation | Target | Measurement | Failure behavior |
|---|---:|---|---|
| Cold desktop startup to usable Home | < 2 s | Process start to interactive Home | Show staged loading; do not block on provider calls |
| Warm desktop startup | < 1 s | Process start to interactive Home | Persist local state and defer noncritical checks |
| Debate list render | < 200 ms | Query completion to visible rows for 1,000 debates | Paginate/virtualize; never freeze shell |
| Debate open | < 300 ms | Selection to question/header visible | Load transcript/evidence progressively |
| Claim list render | < 200 ms | Validated position to visible claims for 500 claims | Virtualize rows and preserve selection |
| Evidence lookup | < 100 ms | Claim click to source/range metadata | Show excerpt loading separately |
| SQLite normal query | < 100 ms | Query start to result for normal indexed operation | Log slow query and keep UI responsive |
| Local decision save | < 200 ms | Human confirmation to durable acknowledgement | Do not show recorded decision before durable write |
| Copy master prompt | < 100 ms after ready | Copy action to clipboard confirmation | Keep prompt visible for manual selection |
| Cancellation acknowledgement | < 250 ms | User cancel to UI state change | Process termination may take longer; show cleanup separately |

## Provider orchestration budgets

- No provider call runs on the UI thread.
- Starting, waiting, parsing, repairing, and persisting a provider attempt are independently observable stages.
- Provider output streaming may update a status surface, but it cannot block navigation, evidence review, or cancellation.
- A provider deadline is configurable per seat and displayed before dispatch.
- Retry and repair work must yield to the UI and expose current attempt number.
- A provider process that exceeds its deadline is cancelled and moved to `TIMEOUT`; the app does not wait indefinitely.

## Snapshot and evidence budgets

For a normal repository fixture:

```text
snapshot progress visible within 250 ms of start
file scan progress updates at least once per 500 ms during long work
evidence lookup remains under 100 ms after manifest load
hashing work runs off the UI thread
secret scan does not log file contents
```

The user sees counts and stage names, not a false precision percentage when total work is unknown.

## Large-repository behavior

The app must remain usable when a repository exceeds normal size:

- Show a candidate count and size estimate before copy.
- Stream or batch manifest processing rather than loading every file into UI memory.
- Skip or quarantine unsupported large/binary content according to policy and show the decision.
- Keep the snapshot builder cancellable and clean up partial output.
- Prefer short local temp roots to avoid Windows path-length failures.
- Do not silently truncate evidence. A truncated snapshot is `SNAPSHOT_FAILED` or explicitly partial, never ready.
- Keep UI memory bounded by virtualizing file lists and provider artifacts.

Suggested stress fixtures:

```text
10,000 small files
1,000 files with long paths
100 MB text corpus
500 MB mixed binary/text corpus
one junction and one symlink/reparse path
multiple synthetic secret markers
```

## Persistence and recovery budgets

- State writes are serialized through the existing persistence contract.
- A crash during a round must leave a recoverable state record within one restart cycle.
- Raw artifacts may be large, but the Home and Debate Overview surfaces load summaries first.
- Audit writes must not be silently dropped. If the audit path is unavailable, the affected operation pauses and reports `AUDIT_WRITE_FAILED`.

## Measurement protocol

1. Run cold and warm variants.
2. Use mock debates and synthetic snapshots for deterministic application measurements.
3. Run with a provider fixture active to prove the UI remains responsive.
4. Capture start/end monotonic timestamps, memory high-water mark, CPU saturation, and state transitions.
5. Report median and worst observed sample for each fixture size.
6. Mark a target `UNVERIFIED` if the environment cannot measure it; do not substitute provider timing.

## Performance acceptance

The product passes the application performance gate when normal startup, list render, evidence lookup, query, cancellation acknowledgement, and UI responsiveness targets are met, and large repositories degrade through visible progress and bounded memory rather than freezing or silently truncating evidence.
