use crate::SourceKind;
use chrono::{DateTime, Utc};
use geo::{LineString, Simplify};
use uuid::Uuid;

/// Opaque, strongly-typed trip identifier.
///
/// Wraps a UUID so that `TripId` and other future ID types (`TrackId`, etc.)
/// can't be accidentally mixed at compile time.
///
/// `#[serde(transparent)]` serializes as the raw UUID string, not `{"0":"..."}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TripId(Uuid);

impl TripId {
    /// Creates a new random TripId.
    ///
    /// Only available on non-WASM targets; in the browser, IDs always arrive
    /// from the server response and are never generated client-side.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Returns the underlying UUID.
    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for TripId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl std::fmt::Display for TripId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A normalized trip record, regardless of which data source it came from.
///
/// All sources (GPX, Strava, Connected Cars, Google Timeline) produce `Trip`
/// values. Downstream code never knows which source a trip came from —
/// only `source` carries that provenance.
#[derive(Debug, Clone)]
pub struct Trip {
    pub id: TripId,
    pub source: SourceKind,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
    /// Total driven distance in metres.
    pub distance_m: f64,
    /// WGS-84 lon/lat polyline.
    pub geometry: LineString,
}

impl Trip {
    /// Returns a new `Trip` with a simplified geometry using Ramer–Douglas–Peucker.
    ///
    /// `tolerance` is in the same units as the coordinates (degrees for WGS-84).
    /// All other fields are copied unchanged. The original trip is not modified.
    pub fn simplify(&self, tolerance: f64) -> Self {
        Self {
            id: self.id,
            source: self.source,
            started_at: self.started_at,
            ended_at: self.ended_at,
            distance_m: self.distance_m,
            geometry: self.geometry.simplify(&tolerance),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use geo::coord;

    fn fixed_ts() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap()
    }

    fn make_trip(geometry: LineString) -> Trip {
        Trip {
            id: TripId(Uuid::nil()),
            source: SourceKind::Gpx,
            started_at: fixed_ts(),
            ended_at: fixed_ts(),
            distance_m: 0.0,
            geometry,
        }
    }

    #[test]
    fn trip_id_from_uuid_roundtrips() {
        let uuid = Uuid::nil();
        let id = TripId::from(uuid);
        assert_eq!(id.as_uuid(), uuid);
    }

    #[test]
    fn trip_id_displays_as_uuid_string() {
        let id = TripId(Uuid::nil());
        assert_eq!(id.to_string(), "00000000-0000-0000-0000-000000000000");
    }

    #[test]
    fn trip_id_serializes_as_plain_string() {
        let id = TripId(Uuid::nil());
        let json = serde_json::to_string(&id).unwrap();
        // transparent serde: no wrapping object
        assert_eq!(json, "\"00000000-0000-0000-0000-000000000000\"");
    }

    #[test]
    fn simplify_preserves_endpoints() {
        let ls = LineString::new(vec![
            coord! { x: 0.0, y: 0.0 },
            coord! { x: 0.5, y: 0.0001 }, // nearly collinear — removed at high tolerance
            coord! { x: 1.0, y: 0.0 },
        ]);
        let trip = make_trip(ls);
        let simplified = trip.simplify(0.01);
        let first = simplified.geometry.coords().next().unwrap();
        let last = simplified.geometry.coords().last().unwrap();
        assert!((first.x - 0.0).abs() < f64::EPSILON);
        assert!((last.x - 1.0).abs() < f64::EPSILON);
    }
}
