//! Machie providers: the `Provider` trait and its implementations.
//!
//! The core depends on the trait, never on any specific gateway or model
//! (spec Parts C.1, D.3, P). No file outside a gateway's own module may
//! reference `omniroute` or `openrouter` by name.
//!
//! Phase 0: crate scaffold only. Real implementations begin in Phases 1
//! (`LocalInferenceProvider`) and 7 (`GatewayProvider`).

pub mod direct;
pub mod gateways;
pub mod local;
