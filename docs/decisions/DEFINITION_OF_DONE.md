# Definition of Done

No feature, tool, or provider is complete unless every item below is true.
Treat an unchecked box as "not done," regardless of how finished the code
looks.
```
[ ] Implementation complete against its spec (specification / relevant ADR / phase doc)
[ ] Unit tests added
[ ] Integration test added where the feature crosses a module boundary
[ ] At least one error-path test
[ ] Permission behavior tested, if applicable
[ ] Network behavior tested, if applicable
[ ] No provider-specific logic leaked into crates/core
[ ] No UI-side business logic
[ ] Documentation updated (doc comments or docs/)
[ ] ADR written/updated if an architectural decision changed
[ ] Migration added if DB or config schema changed
[ ] Help content added/updated for any new user-facing capability
[ ] Feature-flag/capability entry added for any new gated capability — no
hardcoded phase checks in the UI
[ ] Accessibility basics preserved for any new UI surface
[ ] cargo build succeeds with no warnings
[ ] cargo test passes (full workspace)
[ ] cargo clippy -- -D warnings passes
[ ] cargo fmt --check passes
[ ] Frontend: pnpm typecheck / lint / test pass, if touched
[ ] No TODO/placeholder/stub presented as finished
[ ] No fabricated success state
```

## How this is enforced

- Every phase ends with a hard-gate run (`docs/development/REVIEW_GATES.md`).
- A failed hard gate blocks the milestone — no exceptions, no override.
- This document is a living checklist. Adding a rule to it is a change to
  the project's development process; treat it with the same care as an
  architectural decision.