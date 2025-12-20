---
title: "FEAT: Unified Configuration System"
labels:
  - enhancement
  - cli
assignees: []
---

## Description
Implement a centralized configuration manager in `gestalt_core`.

## Requirements
1.  **Config File**: Read `gestalt.toml` from:
    - Linux: `~/.config/gestalt/gestalt.toml`
    - Windows: `%APPDATA%/gestalt/gestalt.toml`
    - macOS: `~/Library/Application Support/gestalt/gestalt.toml`
2.  **Env Vars**: Override config with `GESTALT_*` env vars (e.g., `GESTALT_GEMINI_MODEL`).
3.  **Defaults**: Fallback to hardcoded defaults if no config found.
4.  **CLI Command**: `gestalt config init` to generate a default file.

## Technical Details
- Use `config` crate or `toml` + `serde`.
- Create `gestalt_core/src/application/config.rs`.
