//! Machie Core: routing, task orchestration, and security.
//!
//! This crate must never contain provider-specific logic (no `if provider == "x"`)
//! and must never contain UI-specific logic. Everything here is
//! provider-agnostic and presentation-agnostic.
//!
//! See the project specification, Parts C, E, M, and P.

pub mod config;