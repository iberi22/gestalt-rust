# 🏗️ Architecture

## Stack
- **Language:** Rust (Edition 2021)
- **Framework:**
  - `gestalt_core`: Generic Logic (No IO)
  - `gestalt_timeline`: Tokio + SurrealDB (Orchestration)
  - `gestalt_cli`: Clap + Rustyline (Interface)
- **Database:** SurrealDB (Embedded / WebSocket)
- **Messaging:** Git-Core Protocol v3.5.0 (JSON over Stdio/HTTP)

## Key Decisions

### 1. Asynchronous Autonomy
**Decision:** All long-running agent actions must be non-blocking.
**Rationale:** The agent cannot "Think" if it is waiting for `cargo build`.
**Implementation:**
- `AgentRuntime` spawns `tokio::task` for `StartJob` / `ExecuteShell`.
- Returns `JobId` immediately.
- Agent explicitly polls or awaits via `AwaitJob`.

### 2. Protocol-First Tooling
**Decision:** Tools are effectively just other Agents.
**Rationale:** To avoid hardcoding every tool (gh, aws, docker), we treat them as "specialized agents" invoked via CLI.
**Implementation:** `CallAgent { tool: "gh", args: [...] }`.

### 3. Context Injection
**Decision:** Context is gathered *before* the prompt is built, never during.
**Rationale:** LLMs need the full picture immediately.
**Implementation:** `ContextEngine` scans `.gitcore` and source map.

## Component Diagram
```mermaid
graph TD
    CLI[gestalt_cli] -->|Command| Runtime[AgentRuntime]
    Runtime -->|Prompt| LLM[Bedrock/Gemini/Ollama]
    Runtime -->|Action| Tools
    Runtime -->|Async| Jobs[JobManager]
    Tools -->|Git-Core| ExtAgents[External Agents (gh, aws)]
    Jobs -->|Status| Runtime
```
