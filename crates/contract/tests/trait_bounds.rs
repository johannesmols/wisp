//! Compile-time proof that Arc<dyn TripService> is Send + Sync,
//! and that a minimal implementation compiles correctly.

use async_trait::async_trait;
use std::sync::Arc;
use wisp_contract::{SyncReport, TrackQuery, TripService};
use wisp_core::SourceKind;

struct NullService;

#[async_trait]
impl TripService for NullService {
    async fn tracks(&self, _q: TrackQuery) -> anyhow::Result<geojson::FeatureCollection> {
        Ok(geojson::FeatureCollection {
            bbox: None,
            features: vec![],
            foreign_members: None,
        })
    }

    async fn sync_now(&self, source: SourceKind) -> anyhow::Result<SyncReport> {
        Ok(SyncReport {
            source,
            fetched: 0,
            upserted: 0,
            duration_ms: 0,
        })
    }
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn arc_dyn_trip_service_is_send_sync() {
    assert_send_sync::<Arc<dyn TripService>>();
}

#[tokio::test]
async fn null_service_tracks_returns_empty_collection() {
    let svc = NullService;
    let result = svc.tracks(TrackQuery::default()).await.unwrap();
    assert!(result.features.is_empty());
}
