//! Compile-time proof that Arc<dyn TripSource> is Send + Sync.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use wisp_core::{SourceKind, Trip};
use wisp_sources::{SourceError, TripSource};

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

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn arc_dyn_trip_source_is_send_sync() {
    assert_send_sync::<Arc<dyn TripSource>>();
}

#[tokio::test]
async fn noop_source_fetch_returns_empty() {
    let src = NoopSource;
    let trips = src.fetch(None).await.unwrap();
    assert!(trips.is_empty());
}
