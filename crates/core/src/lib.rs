//! Core domain types for Wisp. Pure Rust — no I/O, no async, no network.
//! Compiles to `wasm32-unknown-unknown` for client-side use.

mod bbox;
mod source_kind;
mod trip;

pub use bbox::Bbox;
pub use source_kind::SourceKind;
pub use trip::{Trip, TripId};
