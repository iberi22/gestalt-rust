# 🚶 Gestalt System Walkthrough

This guide provides a step-by-step walkthrough for running the complete Gestalt system, including the Rust backend server and the Flutter frontend application.

## 🏗️ Architecture Overview

The system consists of two main components:
1. **Gestalt Timeline (Backend):** A Rust application that manages the autonomous agent runtime, database (SurrealDB), and LLM orchestration (AWS Bedrock). It exposes an HTTP API.
2. **Gestalt App (Frontend):** A Flutter application that provides a UI for interacting with the agent, visualizing the timeline, and setting goals.

## 🚀 Prerequisites

- **Rust:** Installed via `rustup` (stable toolchain).
- **Flutter:** SDK installed and configured (`flutter doctor`).
- **SurrealDB:** Running locally or accessible via URL.
- **AWS Credentials:** Configured for Bedrock access (e.g., `~/.aws/credentials`).

## 🛠️ Step 1: Start the Backend Server

The backend runs the autonomous agent and exposes endpoints for the UI.

1. Navigate to the `gestalt_timeline` directory:
   ```bash
   cd gestalt_timeline
   ```

2. Ensure SurrealDB is running:
   ```bash
   surreal start --user root --pass root file://gestalt.db
   ```
   *(Or just rely on the app connecting to your existing instance)*

3. Run the Server:
   ```bash
   cargo run -- server --port 3000
   ```
   You should see:
   ```
   🚀 Starting Agent Server on port 3000
   🚀 Agent Server listening on 0.0.0.0:3000
   ```

## 📱 Step 2: Run the Flutter App

The frontend connects to the local server to control the agent.

1. Navigate to the `gestalt_app` directory:
   ```bash
   cd gestalt_app
   ```

2. Install dependencies:
   ```bash
   flutter pub get
   ```

3. Run the app (Desktop or Mobile):
   ```bash
   flutter run -d windows
   # or
   flutter run -d macos
   # or
   flutter run -d chrome
   ```

## 🔄 Step 3: End-to-End Workflow

1. **Set a Goal:**
   - In the Flutter app, type a goal in the input field (e.g., "Create a project 'Omega' and add a task 'Init'").
   - Click **Send**.

2. **Autonomous Loop:**
   - The Flutter app sends the goal to `POST /orchestrate`.
   - The Rust Server triggers the `AgentRuntime`.
   - The Agent "Thinks" (via Bedrock), "Acts" (creates DB records), and "Observes" (logs results).

3. **Real-time Feedback:**
   - The App polls `GET /timeline`.
   - You will see "Thoughts", "Actions", and "Observations" appear in the chat interface as the agent works.
   - The Project/Task creation events will also appear in the feed.

## 🧪 Testing

To verify the system integration logic without running the full UI:

```bash
cd gestalt_timeline
cargo test --test e2e_runtime
```

This runs a mocked simulation of the autonomous loop against an in-memory database to ensure the core logic is sound.
