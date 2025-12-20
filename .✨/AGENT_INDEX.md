# 🎭 Agent Index

| Role | Description | Capabilities |
|------|-------------|--------------|
| **@architect** | System Design | `.✨/ARCHITECTURE.md`, High-level decisions |
| **@fixer** | Bug Resolution | Debugging, Testing, Hotfixes |
| **@feature** | Feature Dev | Implementation, Rust, Flutter |
| **@release** | CI/CD & Ops | GitHub Actions, Release builds |

## Routing Rules
- **Infrastructure/Stack** -> @architect
- **Panic/Crash** -> @fixer
- **New Command/UI** -> @feature
- **Build Failure** -> @release
