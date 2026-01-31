# Specification: Gestalt Cyber-Terminal

## Vision
A "Luxury" terminal emulator that bypasses traditional TUI limitations to provide a fluid, GPU-accelerated interface for autonomous agents and complex data visualization.

## Technical Architecture

### 1. Backend (Rust)
- **Engine**: `gestalt_core` (Agentic).
- **Bridge**: `flutter_rust_bridge` (v2).
- **Protocol**: Event-driven streaming of agent thoughts and action results.

### 2. Frontend (Flutter)
- **Renderer**: Impeller/Skia (Direct Canvas access).
- **Performance**: Locked 120fps targeting desktop (Windows).
- **UI Components**:
    - **Thought Stream**: A real-time log of agent reasoning.
    - **Action Canvas**: Visualization of file edits, command outputs, and network state.
    - **Command Input**: A smart, context-aware prompt with multi-line support.

## Aesthetics: Cyber-Minimalism
- **Palette**: Deep Black (#000000), Cyber Green (#00FF00), Neon Amber (#FFB000).
- **Typography**: JetBrains Mono / Inter.
- **Effects**:
    - CRT-scanline shaders (optional toggle).
    - Glitch micro-animations on state change.
    - Smooth, eased transitions for all movement.

## Core Features
1. **Autonomous Loop Visualization**: Observe the agent's thought process as a graph.
2. **Infinite Scroll Logs**: GPU-accelerated list that handles millions of lines without lag.
3. **Internal Shell**: Ability to launch system commands (PowerShell) and pipe output back to the agent.
