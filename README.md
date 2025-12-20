# 🧬 Gestalt Timeline Orchestrator

> **CLI Meta-Agente para Coordinación Multi-Agente con Línea de Tiempo Universal**

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![SurrealDB](https://img.shields.io/badge/surrealdb-1.0+-purple.svg)](https://surrealdb.com/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## 🎯 ¿Qué es Gestalt Timeline?

Gestalt Timeline es un **sistema CLI** que permite a múltiples agentes de IA (VS Code Copilot, Antigravity, Gemini, etc.) coordinar proyectos y tareas en paralelo utilizando una **línea de tiempo universal** como memoria compartida.

### Características Principales

- 🕐 **Timeline Universal**: Cada acción se registra con timestamp UTC
- 🤖 **Multi-Agente**: Múltiples agentes pueden conectarse y coordinar
- ⚡ **Async/Paralelo**: Ejecución concurrente con Tokio
- 💾 **Persistencia**: SurrealDB para memoria y estado
- 🖥️ **Solo CLI**: Sin UI, máxima portabilidad para agentes

---

## 📦 Instalación

### Prerrequisitos

- Rust 1.75+
- SurrealDB 1.0+ (local o remoto)

### Build

```bash
# Clonar repositorio
git clone https://github.com/your-org/gestalt-rust.git
cd gestalt-rust

# Compilar
cargo build --release -p gestalt_timeline

# Instalar globalmente
cargo install --path gestalt_timeline
```

### Configuración

El sistema usa **SurrealDB embebido (en memoria)** por defecto, por lo que no necesitas instalar ni ejecutar un servidor externo para probarlo.

```bash
# Opcional: Para conectar a un servidor externo
export SURREAL_URL="ws://localhost:8000"
export SURREAL_USER="root"
export SURREAL_PASS="root"

# Identidad del Agente
export GESTALT_AGENT_ID="agent_copilot"
```

Si no configuras `SURREAL_URL`, se usará `mem://` (base de datos volátil en memoria). Para persistencia local sin servidor, puedes usar `file://timeline.db`.

---

## 🚀 Uso Rápido

```bash
# Crear proyecto
gestalt add-project mi-app

# Añadir tareas
gestalt add-task mi-app "Implementar autenticación"
gestalt add-task mi-app "Diseñar API REST"

# Ver proyectos
gestalt list-projects

# Ver tareas de un proyecto
gestalt list-tasks mi-app

# Ejecutar tarea
gestalt run-task task_abc123

# Ver estado del proyecto
gestalt status mi-app

# Ver línea de tiempo (últimas 2 horas)
gestalt timeline --since=2h

# Modo JSON para integración
gestalt list-projects --json
```

---

## 🗺️ Roadmap

### ✅ Fase 0: Documentación
- [x] PLANNING.md - Arquitectura y diseño
- [x] TASK.md - Gestión de tareas
- [x] README.md - Este archivo
- [x] CHANGELOG.md - Historial
- [x] RULES.md - Reglas para agentes
- [x] .gitignore - Configurado

### ✅ Fase 1: MVP Base
- [x] Crate `gestalt_timeline`
- [x] Conexión SurrealDB con schema auto-init
- [x] Modelos: TimelineEvent, Project, Task
- [x] Timeline Service (timestamp primario)
- [x] CLI: 7 comandos base

### ✅ Fase 2: Tiempo Real
- [x] Comando `watch` (proceso persistente)
- [x] Comando `broadcast` (mensaje a todos)
- [x] Comando `subscribe` (observar proyecto)
- [x] Manejo graceful de Ctrl+C

### ✅ Fase 3: Multi-Agente
- [x] AgentService: registro de agentes
- [x] Comando `list-agents`
- [x] 14 tests unitarios
- [x] 10 tests de integración CLI
- [x] Detección automática de tipo de agente

---

## 🖥️ Comandos Disponibles (13 total)

| Comando | Descripción |
|---------|-------------|
| `add-project <nombre>` | Crear proyecto |
| `add-task <proyecto> <desc>` | Añadir tarea |
| `run-task <task_id>` | Ejecutar tarea async |
| `list-projects` | Listar proyectos |
| `list-tasks [proyecto]` | Listar tareas |
| `status <proyecto>` | Ver progreso |
| `timeline [--since=1h]` | Ver línea de tiempo |
| `watch [--project=X]` | 🔭 Observar en tiempo real |
| `broadcast <msg>` | 📢 Enviar a todos |
| `subscribe <proyecto>` | 📡 Suscribirse a proyecto |
| `agent-connect [--name=X]` | 🤖 Registrar agente |
| `agent-disconnect` | 👋 Desconectar |
| `list-agents [--online]` | 📋 Listar agentes |

**Flag global:** `--json` para salida JSON (integración programática)

---

## 🏗️ Arquitectura

```
┌─────────────────────────────────────────────────────┐
│                  Agentes Externos                    │
│  ┌─────────┐  ┌─────────────┐  ┌──────────────┐    │
│  │ Copilot │  │ Antigravity │  │ Gemini CLI   │    │
│  └────┬────┘  └──────┬──────┘  └──────┬───────┘    │
│       │              │                │             │
│       └──────────────┼────────────────┘             │
│                      ▼                              │
│              ┌───────────────┐                      │
│              │  Gestalt CLI  │                      │
│              └───────┬───────┘                      │
│                      │                              │
│    ┌─────────────────┼─────────────────┐           │
│    ▼         ▼       ▼        ▼        ▼           │
│ ┌───────┐ ┌───────┐ ┌──────┐ ┌─────┐ ┌───────┐    │
│ │Timeline│ │Project│ │ Task │ │Watch│ │ Agent │    │
│ │Service│ │Service│ │Serv. │ │Serv.│ │Service│    │
│ └───┬───┘ └───┬───┘ └──┬───┘ └──┬──┘ └───┬───┘    │
│     │         │        │        │        │         │
│     └─────────┴────────┴────────┴────────┘         │
│                        ▼                            │
│              ┌───────────────┐                      │
│              │   SurrealDB   │                      │
│              └───────────────┘                      │
└─────────────────────────────────────────────────────┘
```

---

## 🤖 Para Agentes de IA

Este CLI está diseñado para ser invocado por otros agentes. Ejemplo de integración:

```bash
# Desde VS Code Copilot o similar
gestalt add-task proyecto "$(cat descripcion.txt)" --json

# Obtener timeline para contexto
CONTEXT=$(gestalt timeline --since=1h --json)

# Verificar estado antes de actuar
STATUS=$(gestalt status mi-proyecto --json)
```

### Identificación de Agente

```bash
export GESTALT_AGENT_ID="vscode_copilot_session_123"
gestalt add-task proyecto "tarea"  # Queda registrado con el agent_id
```

---

## 📚 Documentación

| Archivo | Contenido |
|---------|-----------|
| [PLANNING.md](PLANNING.md) | Arquitectura, stack técnico, diseño |
| [TASK.md](TASK.md) | Estado actual de tareas y progreso |
| [CHANGELOG.md](CHANGELOG.md) | Historial de cambios |

---

## 🤝 Contribución

1. Leer `PLANNING.md` para entender la arquitectura
2. Revisar `TASK.md` para tareas disponibles
3. Seguir las reglas en `.github/RULES.md`
4. Crear PR con descripción clara

---

## 📄 Licencia

MIT License - Ver [LICENSE](LICENSE) para detalles.
