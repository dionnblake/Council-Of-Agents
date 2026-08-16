# Desktop App

## Purpose

Own the Tauri 2 shell, React command-center UI, and the typed IPC boundary for Council of Agents.

## Ownership

- src/ owns presentation, navigation, form state, and human decision surfaces.
- src-tauri/ owns Tauri commands and is the only desktop layer allowed to call council-core or provider processes.

## Local Contracts

- React must never spawn provider commands directly.
- The UI must show requested model, served-model status, certification limitations, and runtime state.
- No implementation handoff button, automatic coding action, or hidden provider session may be added.
- Preview/fallback data must be visually and textually distinguishable from persisted runtime state.
- Keep the interface local-first and usable without a network font or remote asset.

## Verification

- Run npm run build from app/.
- Run cargo check -p council-desktop with the workspace target directory.
- For Windows release certification, run `npx tauri build --bundles nsis` from app/ and verify the generated installer artifact.
- Perform a desktop/browser smoke check when the Tauri runtime is available; report it separately from compile verification.

## Child DOX Index

- src/ owns React UI implementation under this contract.
- src-tauri/ owns Rust shell commands under this contract.
