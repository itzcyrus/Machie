# Machie

A local-first intelligent workspace.

Machie is an orchestration layer: it routes user intent to the safest,
smallest, most capable mechanism that can complete it — deterministic
tools, local AI, or (only when explicitly permitted) network AI.

- Full engineering specification: `machie-engineering-spec.md`
- Architecture decisions: `docs/decisions/`
- Development process and gates: `docs/development/`
- Phase-by-phase plan and progress: `docs/phases/`

## Repository layout
```
apps/desktop Tauri 2 desktop shell (Rust) + React/TS/Tailwind UI
crates/ Rust workspace — all core logic lives here
integrations/ Optional integrations (e.g. OpenCode)
docs/ Architecture, ADRs, dev process, phase notes
evals/ Router golden test sets + harness
tests/ Cross-crate integration tests
```

## Building

Rust workspace:
```
cargo build
cargo test
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

The desktop app and UI are set up during Phase 0 Chunk 2.

## Status

Phase 0 — Engineering Foundation. No user-facing features are implemented yet.