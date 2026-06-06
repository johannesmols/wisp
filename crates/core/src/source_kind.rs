/// The data provider a trip was imported from.
///
/// Marked `#[non_exhaustive]` so adding new sources (e.g. `Garmin`) in a
/// future release doesn't break existing `match` arms in caller code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum SourceKind {
    Gpx,
    Strava,
    ConnectedCars,
    GoogleTimeline,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpx_serializes_to_expected_string() {
        let json = serde_json::to_string(&SourceKind::Gpx).unwrap();
        assert_eq!(json, "\"Gpx\"");
    }

    #[test]
    fn gpx_round_trips_through_json() {
        let json = serde_json::to_string(&SourceKind::Gpx).unwrap();
        let back: SourceKind = serde_json::from_str(&json).unwrap();
        assert_eq!(SourceKind::Gpx, back);
    }
}
