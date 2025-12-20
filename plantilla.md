PLANNING.md
Purpose: High-level vision, architecture, constraints, tech stack, tools, etc.
Prompt to AI: “Use the structure and decisions outlined in PLANNING.md.”
Have the LLM reference this file at the beginning of any new conversation.


TASK.md
Purpose: Tracks current tasks, backlog, and sub-tasks.
Includes: Bullet list of active work, milestones, and anything discovered mid-process.
Prompt to AI: “Update TASK.md to mark XYZ as done and add ABC as a new task.”
Can prompt the LLM to automatically update and create tasks as well (through global rules). Ejemplo:
##TASK.md
Gestión de Tareas: [Nombre del Proyecto]
_Última actualización: YYYY-MM-DD_
## 🎯 Resumen Ejecutivo y Estado Actual
**Estado General:** [Ej: 85% - Enfocado en la implementación de la API de pagos]
Un breve resumen (1-2 frases) del estado actual, los logros recientes y el enfoque inmediato.
**Progreso por Componente:**
- [ ] 🏗️ Infraestructura: XX%
- [ ] 🔗 Backend API: XX%
- [ ] 🎨 Frontend UI: XX%
- [ ] 🧪 Testing: XX%
- [ ] 📚 Documentación: XX%
---
## 🚀 Fase Actual: [Nombre de la Fase, ej: "Integración de Pasarela de Pago"]
**Objetivo:** [Descripción del objetivo de la fase, ej: "Integrar Stripe para procesar pagos de suscripciones."]
| ID    | Tarea                                  | Prioridad | Estado      | Responsable |
|-------|----------------------------------------|-----------|-------------|-------------|
| F1-01 | Crear endpoint para iniciar pago       | ALTA      | ⬜ Pendiente | Cascade     |
| F1-02 | Diseñar modal de pago en el frontend   | ALTA      | ⚙️ En Progreso | Cascade     |
| F1-03 | Implementar webhook para confirmaciones| MEDIA     | ⬜ Pendiente | Cascade     |
| F1-04 | Añadir pruebas unitarias para el pago  | MEDIA     | ⬜ Pendiente | Cascade     |
**Leyenda de Estado:**
- `⬜ Pendiente`
- `⚙️ En Progreso`
- `✅ Completado`
- `❌ Bloqueado`
---
## ✅ Hitos Principales Completados
- Hito 1: Configuración inicial de la infraestructura.
- Hito 2: Implementación del sistema de autenticación de usuarios.
- Hito 3: Desarrollo del CRUD de productos.
---
## 👾 Deuda Técnica y Mejoras Pendientes
| ID    | Tarea                                  | Prioridad | Estado      | Responsable |
|-------|----------------------------------------|-----------|-------------|-------------|
| TD-01 | Refactorizar el servicio de usuarios   | MEDIA     | ⬜ Pendiente | Cascade     |
| TD-02 | Optimizar consultas a la base de datos | BAJA      | ⬜ Pendiente | Cascade     |
---
## 📝 Tareas Descubiertas Durante el Desarrollo
| ID    | Tarea                                        | Prioridad | Estado      | Responsable |
|-------|----------------------------------------------|-----------|-------------|-------------|
| AD-01 | Corregir bug visual en la barra de navegación| ALTA      | ⚙️ En Progreso | Cascade     |










3. ⚙️ Global Rules (For AI IDEs)
Global (or project level) rules are the best way to enforce the use of the golden rules for your AI coding assistants.
Global rules apply to all projects. Project rules apply to your current workspace. All AI IDEs support both.

Use the below example (for our Supabase MCP server) as a starting point to add global rules to your AI IDE system prompt to enforce consistency:
### 🔄 Project Awareness & Context
- **Always read `PLANNING.md`** at the start of a new conversation to understand the project's architecture, goals, style, and constraints.
- **Check `TASK.md`** before starting a new task. If the task isn’t listed, add it with a brief description and today's date.
- **Use consistent naming conventions, file structure, and architecture patterns** as described in `PLANNING.md`.

### 🧱 Code Structure & Modularity
- **Never create a file longer than 800 lines of code.** If a file approaches this limit, refactor by splitting it into modules or helper files.
- **Organize code into clearly separated modules**, grouped by feature or responsibility.
- **Use clear, consistent imports** (prefer relative imports within packages).

### 🧪 Testing & Reliability
- **Always create Pytest unit tests for new features** (functions, classes, routes, etc).
- **After updating any logic**, check whether existing unit tests need to be updated. If so, do it.
- **Tests should live in a `/tests` folder** mirroring the main app structure.
  - Include at least:
    - 1 test for expected use
    - 1 edge case
    - 1 failure case

### ✅ Task Completion
- **Mark completed tasks in `TASK.md`** immediately after finishing them.
- Add new sub-tasks or TODOs discovered during development to `TASK.md` under a “Discovered During Work” section.

### 📎 Style & Conventions
- **Use Python** as the primary language.
- **Follow PEP8**, use type hints, and format with `black`.
- **Use `pydantic` for data validation**.
- Use `FastAPI` for APIs and `SQLAlchemy` or `SQLModel` for ORM if applicable.
- Write **docstrings for every function** using the Google style:
  ```python
  def example():
      """
      Brief summary.

      Args:
          param1 (type): Description.

      Returns:
          type: Description.
      """
  ```

### 📚 Documentation & Explainability
- **Update `README.md`** when new features are added, dependencies change, or setup steps are modified.
- **Comment non-obvious code** and ensure everything is understandable to a mid-level developer.
- When writing complex logic, **add an inline `# Reason:` comment** explaining the why, not just the what.

### 🧠 AI Behavior Rules
- **Never assume missing context. Ask questions if uncertain.**
- **Never hallucinate libraries or functions** – only use known, verified Python packages.
- **Always confirm file paths and module names** exist before referencing them in code or tests.
- **Never delete or overwrite existing code** unless explicitly instructed to or if part of a task from `TASK.md`.