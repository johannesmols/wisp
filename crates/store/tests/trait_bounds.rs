//! Compile-time proof that Arc<dyn TripStore> is Send + Sync.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use wisp_core::{Bbox, SourceKind, Trip};
use wisp_store::{StoreError, TripStore};

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

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn arc_dyn_trip_store_is_send_sync() {
    assert_send_sync::<Arc<dyn TripStore>>();
}

#[tokio::test]
async fn noop_store_upsert_returns_zero() {
    let store = NoopStore;
    let result = store.upsert_trips(&[]).await.unwrap();
    assert_eq!(result, 0);
}
