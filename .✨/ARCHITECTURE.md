---
title: "Gestalt System Architecture"
type: ARCHITECTURE
id: "arch-gestalt"
created: 2025-12-20
updated: 2025-12-20
agent: copilot
model: gemini-3-pro
requested_by: user
summary: |
  Architecture for Gestalt: A multi-model, context-aware AI assistant system.
keywords: [architecture, rust, flutter, mcp, hexagonal, context-engine]
tags: ["#architecture", "#design", "#gestalt"]
project: Gestalt
---

# 🏗️ Gestalt Architecture

## 🚨 CRITICAL DECISIONS - READ FIRST

> ⚠️ **STOP!** Before implementing ANY feature, verify against this table.

| # | Category | Decision | Rationale | ❌ NEVER Use |
|---|----------|----------|-----------|--------------|
| 1 | Core Logic | `gestalt_core` (Rust) | Shared logic between CLI and UI, Hexagonal Arch | Logic in UI/CLI directly |
| 2 | Architecture | Hexagonal (Ports & Adapters) | Decouple domain from tools/UI | Spaghetti code |
| 3 | Tools | Model Context Protocol (MCP) | Standardized tool interface | Custom tool protocols |
| 4 | State | Local Files + SQLite (Timeline) | User privacy, offline first | Cloud-only state |
| 5 | Context | Context Engine (Auto-discovery) | Intelligent context gathering over manual input | Hardcoded paths |

---

## Stack
- **Core Language:** Rust
- **UI Framework:** Flutter (Dart)
- **CLI Framework:** Rust (`clap`, `ratatui`)
- **Database:** SQLite (via `rusqlite` or `sqlx`)
- **Communication:** FFI (Flutter Rust Bridge) for UI, Direct linking for CLI

## Project Structure
```
/
├── gestalt_core/       # 🧠 The Brain (Hexagonal Arch)
│   ├── domain/         # Pure business logic
│   ├── ports/          # Interfaces (Traits)
│   ├── adapters/       # Implementations (FS, Http, MCP)
│   └── application/    # Use Cases / Services
├── gestalt_cli/        # 💻 Terminal Interface
├── gestalt_app/        # 📱 Graphical Interface (Flutter)
├── gestalt_timeline/   # ⏳ Memory & History Service
└── .✨/                # 📋 Protocol Metadata
```

## Key Decisions

### Decision 1: Hexagonal Core
- **Context:** We need to support both a CLI and a Flutter App.
- **Decision:** All business logic resides in `gestalt_core`. The CLI and App are just "adapters" (driving side) that plug into the core.
- **Consequences:** Changes in core propagate to both. Strict separation of concerns required.

### Decision 2: Context Engine over Consensus
- **Date:** 2025-12-20
- **Context:** Consensus is slow and expensive. Better context yields better results from a single model.
- **Decision:** Shift focus to a "Context Engine" that intelligently gathers project context (files, structure, configs) to feed a single smart model. Consensus becomes opt-in.

### Decision 3: Git-Core Protocol Alignment
- **Date:** 2025-12-20
- **Context:** Need a standardized way to manage the project and agent interactions.
- **Decision:** Adopt Git-Core Protocol (Issues as state, Atomic commits, Architecture-first).

## Integration Points
- [ ] MCP Servers (Local & Remote)
- [ ] LLM Providers (Gemini, OpenAI, Ollama)
- [ ] Git-Core Protocol (reading .✨/ files)

## Security Considerations
- API Keys must be stored in OS Keychain or encrypted local config.
- No telemetry sent without explicit opt-in.
