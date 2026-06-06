//! Data-source port for Wisp.
//!
//! Defines [`TripSource`] and its error type. No concrete implementations
//! (GPX, Strava, etc.) live here — those arrive in slice 3.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use wisp_core::{SourceKind, Trip};

/// Errors that a [`TripSource`] implementation may return.
///
/// Marked `#[non_exhaustive]` so new variants can be added without breaking
/// existing error-handling code.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SourceError {
    #[error("network error: {0}")]
    Network(String),

    #[error("parse error: {0}")]
    Parse(String),

    #[error("auth error: {0}")]
    Auth(String),

    #[error("rate limited; retry after {0}s")]
    RateLimited(u64),
}

/// Data-source port. Each supported provider (GPX, Strava, Connected Cars,
/// Google Timeline) implements this trait, producing normalised [`Trip`]
/// values that the [`Ingest`](wisp_ingest::Ingest) orchestrator writes to the
/// store.
#[async_trait]
pub trait TripSource: Send + Sync {
    /// The provider this source represents. Used by [`Ingest`] to route
    /// sync requests to the correct implementation.
    fn kind(&self) -> SourceKind;

    /// Fetch trips from the source.
    ///
    /// `since = None` triggers a full backfill from the beginning of time.
    /// `since = Some(t)` fetches only trips that started after `t`.
    async fn fetch(&self, since: Option<DateTime<Utc>>) -> Result<Vec<Trip>, SourceError>;
}
