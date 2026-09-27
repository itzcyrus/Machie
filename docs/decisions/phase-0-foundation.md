# Phase 0 — Engineering Foundation

Status: In progress (Chunk 2)
Started: 2026-09-27

## Purpose

Phase 0 builds **no user-facing features.** Its only job is to remove
ambiguity before real work starts: pin conventions, set up CI, write the
initial ADRs, and establish the hard gates.

## Deliverables

### Repository scaffolding

- [x] Cargo workspace with the crates listed in the specification (Part K.7)
- [x] Tauri 2 shell under `apps/desktop`
- [x] Frontend (pnpm / React / TS / Tailwind) under `apps/desktop/ui`
- [x] `docs/{architecture,decisions,development,phases}/`
- [x] `evals/router/`

### Governance files (written verbatim from the specification)

- [x] `docs/decisions/ADR-0001-tech-stack.md` (spec Part B)
- [x] `docs/decisions/ADR-0002-gateway-independence.md` (spec Part D.3)
- [x] `docs/decisions/ADR-0003-product-naming.md` (spec Part A.1)
- [x] `docs/development/DEFINITION_OF_DONE.md` (spec Part Q.7)
- [x] `docs/development/REVIEW_GATES.md` (spec Part Q.8)

### Tooling and CI

- [x] `rustfmt.toml` and `clippy.toml`
- [x] CI running build / test / clippy / fmt-check on every push and PR
- [x] Frontend CI running typecheck / lint / test / build

### Config foundation

- [x] Initial TOML shape with `config_version = 1`
- [x] Migration registry stubbed, even with zero migrations yet

### Database foundation

- [x] SQLite connection handling in `crates/database`
- [x] Migration tool (`sqlx migrate`) with an initial empty migration

### Router eval skeleton

- [x] Empty `evals/router/*.jsonl` files with the schema documented
- [x] Stub eval runner (`cargo run --bin router-eval`) reporting 0/0

## Deviations from the specification

- **Frontend UI build order.** Because Tauri's `generate_context!()` macro
  embeds the built frontend at compile time, the UI must be built
  (`pnpm build`) before `cargo build` on a fresh checkout. This is noted
  in `README.md` and reflected in CI. Not a deviation — a build-order
  requirement that Tauri imposes.
- **Tauri crate lints.** The `apps/desktop` crate does not inherit the
  workspace's `unsafe_code = "forbid"` lint, because
  `tauri::generate_context!()` expands to code that trips it. Core crates
  still inherit it. This is a targeted, documented exception, not a
  relaxation of the rule for core code.
- **`bundle.active = false`.** Phase 0 does not produce bundled
  installers and therefore does not require icons. Icon generation and
  bundling are added in a later phase when they become relevant.

## Exit criteria

Every hard gate (`docs/development/REVIEW_GATES.md`) must pass, and the
Definition of Done must be satisfied for the Phase 0 deliverable set.
Once both hold, Phase 0 is closed and Phase 1 may begin.