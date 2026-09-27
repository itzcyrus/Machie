# Review Gates

Two layers. Only the first one is a gate.

## Layer 1 — Hard Gates (objective, automated, mandatory)

A failed hard gate blocks the milestone — no exceptions, no override.

For every change:

- [ ] `cargo build --workspace --all-targets` succeeds.
- [ ] `cargo test --workspace --all-targets` passes.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes.
- [ ] `cargo fmt --all -- --check` passes.
- [ ] If the frontend was touched: `pnpm typecheck`, `pnpm lint`,
      `pnpm test`, `pnpm build` all pass inside `apps/desktop/ui`.
- [ ] Security tests pass, where applicable.
- [ ] No regressions against existing tests.
- [ ] The full Definition of Done is satisfied
      (`docs/development/DEFINITION_OF_DONE.md`).
- [ ] For router changes: no accuracy regression on the golden eval set in
      `evals/router/`.

## Layer 2 — Self-Review (diagnostic only, never a gate)

After the hard gates pass, score the change 0–10 across:

- architecture correctness
- maintainability
- security
- UX
- requirement coverage
- code quality

This score **has no authority to mark something done** and **cannot
compensate for a failed hard gate.** It is used only to prioritize
follow-up work. Do not inflate the score. If it is low but the hard gates
pass, the feature still ships and the score becomes a backlog item — it is
not a blocker and it is not a reason to fabricate a higher number.

## Why this split exists

A coding agent working incrementally, across many sessions, will drift.
The hard gates make drift loud: the build fails, the test fails, or the
eval accuracy drops. Self-review cannot do that, because it is a
judgment, and judgments can be rationalized. Keeping the two layers
distinct — one mechanical and binding, one diagnostic and non-binding —
is what keeps the process honest.