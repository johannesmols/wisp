//! Persistence port for Wisp.
//!
//! Defines [`TripStore`] and its error type. No SQLite implementation lives
//! here — that arrives in slice 2. Having the trait separate means the rest
//! of the codebase (including tests via [`NoopStore`]) can compile and run
//! without any database dependency.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use wisp_core::{Bbox, SourceKind, Trip, TripId};

/// Errors that a [`TripStore`] implementation may return.
///
/// Marked `#[non_exhaustive]` so adding new variants (e.g. `Conflict`,
/// `Timeout`) in a future release doesn't break existing `match` arms.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum StoreError {
    #[error("trip not found: {0}")]
    NotFound(TripId),

    /// The storage backend is unavailable. The `String` carries a
    /// human-readable reason; concrete implementations convert their
    /// specific error types (e.g. `sqlx::Error`) into this variant.
    #[error("storage unavailable: {0}")]
    Unavailable(String),

    #[error("serialization error: {0}")]
    Serialization(String),
}

/// Persistence port.
///
/// # .NET analogy
/// `Arc<dyn TripStore>` is the Rust equivalent of injecting
/// `IRepository<Trip>` — the interface is defined here; `SqliteTripStore`
/// implements it in slice 2.
#[async_trait]
pub trait TripStore: Send + Sync {
    /// Insert or ignore on conflict (idempotent). Returns the number of rows
    /// actually written (0 if all were already present).
    async fn upsert_trips(&self, trips: &[Trip]) -> Result<u64, StoreError>;

    /// Return all trips whose geometry intersects `bbox`, simplified to
    /// `simplify_m` degrees of tolerance (`0.0` = no simplification).
    async fn trips_in_bbox(&self, bbox: Bbox, simplify_m: f64) -> Result<Vec<Trip>, StoreError>;

    /// Return the `started_at` timestamp of the most recent trip from the
    /// given source, or `None` if none have been imported yet (triggers a
    /// full backfill on the next sync).
    async fn latest_started_at(
        &self,
        source: SourceKind,
    ) -> Result<Option<DateTime<Utc>>, StoreError>;
}
