# React UI

Own the human-facing command center and navigation surfaces.

- Use Tauri invoke for controller actions.
- Keep provider limitations and preview state visible.
- Do not add implementation handoff or autonomous coding actions.
- Render `SNAPSHOT_REVIEW_REQUIRED` as a first-class human gate with the exact snapshot/manifest/exclusion hashes and safe exclusion metadata only. Approval and rejection use typed Tauri commands; the UI never displays excluded file contents.
- The command center supports UI zoom from 75% to 150% with Ctrl+mouse wheel or Ctrl+Plus/Minus; Ctrl+0 resets to 100%.
- Verify with npm run build from app/.
