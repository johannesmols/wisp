//! Smoke test: wire no-op store + source, call sync_source, assert zero counts.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use wisp_core::{Bbox, SourceKind, Trip};
use wisp_ingest::Ingest;
use wisp_sources::{SourceError, TripSource};
use wisp_store::{StoreError, TripStore};

// ── No-op implementations ──────────────────────────────────────────────────

struct NoopStore;

#[async_trait]
impl TripStore for NoopStore {
    async fn upsert_trips(&self, _trips: &[Trip]) -> Result<u64, StoreError> {
        Ok(0)
    }

    async fn trips_in_bbox(&self, _bbox: Bbox, _simplify_m: f64) -> Result<Vec<Trip>, StoreError> {
        Ok(vec![])
    }

    async fn latest_started_at(
        &self,
        _source: SourceKind,
    ) -> Result<Option<DateTime<Utc>>, StoreError> {
        Ok(None)
    }
}

struct NoopSource;

#[async_trait]
impl TripSource for NoopSource {
    fn kind(&self) -> SourceKind {
        SourceKind::Gpx
    }

    async fn fetch(&self, _since: Option<DateTime<Utc>>) -> Result<Vec<Trip>, SourceError> {
        Ok(vec![])
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[tokio::test]
async fn sync_source_with_noops_returns_zero_counts() {
    let store = Arc::new(NoopStore) as Arc<dyn TripStore>;
    let sources = vec![Arc::new(NoopSource) as Arc<dyn TripSource>];
    let ingest = Ingest::new(store, sources);

    let report = ingest.sync_source(SourceKind::Gpx).await.unwrap();

    assert_eq!(report.source, SourceKind::Gpx);
    assert_eq!(report.fetched, 0);
    assert_eq!(report.upserted, 0);
}

#[tokio::test]
async fn sync_source_errors_when_source_not_configured() {
    let store = Arc::new(NoopStore) as Arc<dyn TripStore>;
    let ingest = Ingest::new(store, vec![]); // no sources registered

    let err = ingest.sync_source(SourceKind::Strava).await.unwrap_err();
    assert!(err.to_string().contains("not configured"));
}
