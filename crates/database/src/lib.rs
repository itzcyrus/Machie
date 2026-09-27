//! Machie database layer: SQLite connection handling and schema migrations.
//!
//! SQLite is the primary application database and is accessed **only** from
//! Rust. The UI never talks to SQLite directly (spec Parts B, K.5, P).
//!
//! Large source documents live in the workspace filesystem, not in SQLite;
//! this crate stores structured metadata and relationships only.

pub mod pool;

pub use pool::{connect, DatabaseError};