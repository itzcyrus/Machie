//! Machie Router: intent/task classification and capability-based model
//! selection.
//!
//! The router is the most consequential component in Machie (spec Part Q.6).
//! A misclassification silently breaks the deterministic-tools philosophy.
//! Every router change must be validated against the golden eval sets in
//! `evals/router/` before merging.
//!
//! Phase 0: crate scaffold only. Real implementation begins in Phase 1.