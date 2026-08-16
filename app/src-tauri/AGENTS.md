# Tauri Shell

Own typed IPC commands and desktop process integration.

- Call council-core for policy and persistence.
- Never accept raw shell commands from the React layer.
- Keep filesystem and provider execution under explicit controller policy.
- Persist provider-specific model level defaults and forward per-debate level overrides through typed IPC to council-core.
- Expose snapshot review through typed IPC only. Review responses may contain exclusion paths, reasons, and hashes, but never excluded file contents or user-entered free-form rationale. Provider dispatch must remain blocked until the exact persisted review is approved.
- Verify with cargo check -p council-desktop.
