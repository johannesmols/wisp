//! Orchestration layer for Wisp.
//!
//! [`Ingest`] wires a set of [`TripSource`] adapters to a [`TripStore`],
//! driving the fetch → normalise → persist pipeline.
//!
//! In Mode B (standalone), [`Ingest`] will implement [`TripService`] as
//! `LocalTripService`. That implementation arrives in slice 2/3 once the
//! store and sources have concrete implementations.

use std::sync::Arc;

use anyhow::Context;
use wisp_contract::SyncReport;
use wisp_core::SourceKind;
use wisp_sources::TripSource;
use wisp_store::TripStore;

/// Orchestrates data sources → store. The entry point for all sync operations.
pub struct Ingest {
    store: Arc<dyn TripStore>,
    sources: Vec<Arc<dyn TripSource>>,
}

impl Ingest {
    /// Create an orchestrator with the given store and sources.
    ///
    /// # .NET analogy
    /// This is the explicit constructor injection that replaces the DI
    /// container. In `apps/server/main.rs` you'll call:
    /// ```ignore
    /// let ingest = Ingest::new(Arc::new(SqliteTripStore::new(...)), vec![
    ///     Arc::new(GpxSource::new(...)),
    ///     Arc::new(StravaSource::new(...)),
    /// ]);
    /// ```
    pub fn new(store: Arc<dyn TripStore>, sources: Vec<Arc<dyn TripSource>>) -> Self {
        Self { store, sources }
    }

    /// Sync a single source: fetch since the last known timestamp, then upsert.
    ///
    /// Returns a [`SyncReport`] with counts for observability. The duration
    /// field is 0 in this skeleton — wall-clock timing will be added with
    /// `tracing` in slice 4.
    pub async fn sync_source(&self, source: SourceKind) -> anyhow::Result<SyncReport> {
        let src = self
            .sources
            .iter()
            .find(|s| s.kind() == source)
            .with_context(|| format!("source not configured: {source:?}"))?;

        let since = self
            .store
            .latest_started_at(source)
            .await
            .with_context(|| format!("failed to query latest timestamp for {source:?}"))?;

        let trips = src
            .fetch(since)
            .await
            .map_err(|e| anyhow::anyhow!("fetch failed for {source:?}: {e}"))?;

        let fetched = trips.len() as u64;
        let upserted = self
            .store
            .upsert_trips(&trips)
            .await
            .with_context(|| format!("upsert failed for {source:?}"))?;

        Ok(SyncReport {
            source,
            fetched,
            upserted,
            duration_ms: 0,
        })
    }
}
