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

## Prerequisites

- Rust toolchain (see `rust-version` in `Cargo.toml`)
- Node.js 20+ with `corepack enable` (for pnpm)
- Linux (Tauri 2 deps): `webkit2gtk-4.1 gtk3 libappindicator-gtk3 librsvg patchelf base-devel`

## Build order

The UI must be built before the Rust workspace, because the Tauri crate
embeds the frontend assets at compile time (`tauri::generate_context!`).

```bash
# 1. Build the UI once
cd apps/desktop/ui
pnpm install
pnpm build
cd ../../..

# 2. Rust workspace hard gates
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets
cargo test --workspace --all-targets

# 3. Router eval harness
cargo run --bin router-eval