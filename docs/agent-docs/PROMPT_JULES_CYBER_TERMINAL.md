---
title: "Mission: Cyber-Terminal & Agentic UI Refactor"
type: PROMPT
id: "prompt-jules-cyber-terminal"
created: 2026-01-30
updated: 2026-01-30
agent: protocol-architect
model: o3-mini-high
requested_by: antigravity
summary: |
  Instructions for Jules to refactor Gestalt's CLI and Flutter interface
  into a premium, high-performance agentic terminal ecosystem.
keywords: [jules, ui, ux, flutter, rust, terminal, agentic]
tags: ["#jules", "#ui", "#ux", "#cyberpunk"]
priority: high
status: draft
---

# 🚀 Mission: The Cyber-Terminal Evolution

**Jules**, your objective is to elevate the **Gestalt Ecosystem** by implementing a premium, high-performance user interface for both the CLI and the Flutter Desktop App. We are moving from a "Simple Tool" to a "Sovereign Agent Dashboard".

## 🛠️ Project 1: `gestalt_cli` (The Pro Terminal)

Existing state: Minimalist text-based I/O.
**Your Goal**: Implement a "Rich Terminal" experience.

1.  **Visuals**: Use `ratatui` or `indicatif` for progress bars while the agent "thinks".
2.  **Streaming**: Implement smooth word-by-word streaming for the final answer.
3.  **Context Visualization**: Show a collapsible tree or a list of "Observed Files" before the response.
4.  **Branding**: Deep Matrix Green / Cyberpunk Yellow accents.

## 🎨 Project 2: `gestalt_terminal` (The 120fps Dashboard)

Existing state: Basic Flutter setup with Rust Bridge.
**Your Goal**: Create a "Luxury" Desktop experience.

1.  **The Canvas**: Leverage Flutter's GPU-accelerated canvas to draw **Real-time Node Graphs** of the Agent's reasoning path.
2.  **Aesthetics**:
    - Glassmorphism / Frosted Glass effects.
    - Custom Shaders (Glitch effects on state transitions).
    - Monospaced typography (Inter or JetBrains Mono).
3.  **Performance**: Must maintain **120fps** during smooth scrolling and animations.
4.  **Agentic View**: A dedicated panel showing the "Thought Process" (Observe -> Think -> Act) with micro-animations for each state.

## 🧠 Integration

Both interfaces must consume the new `GestaltAgent` (based on `synapse-agentic`).
- Handle asynchronous events from the Agent (EventBus).
- Display telemetric data (Tokens used, cost, time).

**Protocol**:
- Follow `Git-Core Protocol` for all commits.
- No placeholders. Implement working interaction loops.

> **"Intelligent, sophisticated yet minimalist in complexity"**
