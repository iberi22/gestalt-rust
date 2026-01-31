---
title: "Refactor: Transition Gestalt Core to Agentic Framework"
labels:
  - enhancement
  - ai-plan
  - architecture
assignees: []
---

## Description
Evolve the current `AgentOrchestrator` from a stateless query wrapper into a fully autonomous system by adopting the patterns and components of the `synapse-agentic` framework.

## Tasks
- [ ] Add `synapse-agentic` as a local dependency.
- [ ] Implement a `GestaltAgent` using the `synapse_agentic::Agent` trait.
- [ ] Transition from single-shot LLM requests to a stateful autonomous loop.
- [ ] Integrate the existing `LlmProvider` and `RepoManager` into the `Tool` system.
- [ ] Implement structured state persistence in SurrealDB.
