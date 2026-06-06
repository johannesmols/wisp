use async_trait::async_trait;
use chrono::{DateTime, Utc};
use wisp_core::{Bbox, SourceKind};

/// Query parameters for the tracks endpoint.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrackQuery {
    /// If `None`, return trips from the entire globe.
    pub bbox: Option<Bbox>,
    /// If `Some`, only return trips that started on or after this timestamp.
    pub since: Option<DateTime<Utc>>,
    /// Simplification tolerance in degrees. `0.0` disables simplification.
    pub simplify_m: f64,
}

impl Default for TrackQuery {
    fn default() -> Self {
        Self {
            bbox: None,
            since: None,
            simplify_m: 0.0,
        }
    }
}

/// Result of a sync operation for one data source.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SyncReport {
    pub source: SourceKind,
    /// Number of trips fetched from the source.
    pub fetched: u64,
    /// Number of trips written to the store (idempotent upsert).
    pub upserted: u64,
    pub duration_ms: u64,
}

/// The central service port.
///
/// The UI depends only on this trait and the DTOs above — it never knows
/// whether data is served locally (embedded SQLite) or over HTTP (NAS mode).
///
/// # .NET analogy
/// `Arc<dyn TripService>` is the Rust equivalent of `IServiceProvider
/// .GetRequiredService<ITripService>()` — one implementation is wired at
/// startup and injected everywhere via Dioxus context.
///
/// # Why `#[async_trait]`?
/// Native `async fn` in traits (stable since Rust 1.75) does not yet support
/// `dyn` dispatch. `#[async_trait]` boxes the futures, making
/// `Arc<dyn TripService>` work correctly. Drop it once AFIT supports `dyn`.
#[async_trait]
pub trait TripService: Send + Sync {
    /// Fetch trips matching the query, serialised as a GeoJSON FeatureCollection.
    async fn tracks(&self, q: TrackQuery) -> anyhow::Result<geojson::FeatureCollection>;

    /// Trigger an immediate sync for the given source and return a report.
    async fn sync_now(&self, source: SourceKind) -> anyhow::Result<SyncReport>;
}
