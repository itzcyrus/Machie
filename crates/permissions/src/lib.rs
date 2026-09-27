//! Machie permissions: scope enforcement, high-risk operation gating, and
//! user confirmation flow.
//!
//! Defaults are conservative (spec Part E.8). Every privilege escalation is
//! an explicit, auditable step — never a side effect of a model output.
//!
//! Phase 0: crate scaffold only. Real implementation begins in Phase 5.
