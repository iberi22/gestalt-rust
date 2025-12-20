# 📜 Changelog - Gestalt Timeline Orchestrator

Todos los cambios notables en este proyecto serán documentados en este archivo.

El formato está basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.0.0/),
y este proyecto adhiere a [Semantic Versioning](https://semver.org/lang/es/).

---

## [Unreleased]

### 🎉 Added
- **PLANNING.md**: Documento de planificación con arquitectura, stack tecnológico y diseño del sistema
- **TASK.md**: Sistema de gestión de tareas con fases y tracking de progreso
- **README.md**: Documentación principal con roadmap y guía de uso
- **CHANGELOG.md**: Este archivo de historial de cambios
- **.gitignore**: Configuración para ignorar archivos de compilación y temporales
- **RULES.md**: Reglas para agentes de IA que trabajen en el proyecto
- **gestalt_timeline crate**: MVP completo del CLI con:
  - Modelos: `TimelineEvent`, `Project`, `Task` con timestamp UTC como variable primaria
  - Cliente SurrealDB con schema auto-inicializado
  - Servicios: `TimelineService`, `ProjectService`, `TaskService`
  - Comandos CLI: `add-project`, `add-task`, `run-task`, `list-projects`, `list-tasks`, `status`, `timeline`
  - Flag `--json` para integración programática
- **Fase 2 - Modo Watch en Tiempo Real**:
  - Comando `watch`: proceso persistente que no termina
  - Comando `broadcast`: enviar mensajes a todos los agentes
  - Comando `subscribe`: suscribirse a eventos de un proyecto
  - Comandos `agent-connect` / `agent-disconnect`: registro de agentes
  - Manejo graceful de Ctrl+C
- **Fase 3 - Multi-Agente y Tests**:
  - `AgentService`: registro y tracking de agentes conectados
  - Comando `list-agents`: listar agentes (con flag `--online`)
  - 14 tests unitarios para modelos y serialización
  - Detección automática de tipo de agente (copilot, antigravity, gemini, etc.)

---

## [0.1.0] - 2025-12-19 (Planificado)

### 🎉 Added
- MVP inicial del CLI
- Comandos base funcionales
- Persistencia en SurrealDB
- Timeline con registro de eventos

---

## [0.2.0] - TBD

### 🎉 Added
- Comando `watch` para modo observador persistente
- Suscripciones live a eventos
- Flag `--json` para salida estructurada

---

## [0.3.0] - TBD

### 🎉 Added
- Soporte multi-agente
- Registro de agentes conectados
- Protocolo de comunicación inter-agente

---

## Tipos de Cambios

- 🎉 **Added**: Nuevas características
- 🔄 **Changed**: Cambios en funcionalidad existente
- ⚠️ **Deprecated**: Características próximas a eliminar
- 🗑️ **Removed**: Características eliminadas
- 🐛 **Fixed**: Corrección de bugs
- 🔒 **Security**: Correcciones de seguridad
