/// Axis-aligned bounding box in WGS-84 coordinates (longitude/latitude).
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Bbox {
    pub min_lon: f64,
    pub min_lat: f64,
    pub max_lon: f64,
    pub max_lat: f64,
}

impl Bbox {
    /// Returns a Bbox that covers the entire globe.
    pub fn world() -> Self {
        Self {
            min_lon: -180.0,
            min_lat: -90.0,
            max_lon: 180.0,
            max_lat: 90.0,
        }
    }

    /// Returns `true` if `(lon, lat)` is within this bounding box (inclusive).
    pub fn contains(&self, lon: f64, lat: f64) -> bool {
        lon >= self.min_lon && lon <= self.max_lon && lat >= self.min_lat && lat <= self.max_lat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn europe() -> Bbox {
        Bbox {
            min_lon: -10.0,
            min_lat: 35.0,
            max_lon: 40.0,
            max_lat: 71.0,
        }
    }

    #[test]
    fn contains_interior_point() {
        assert!(europe().contains(10.0, 55.0));
    }

    #[test]
    fn rejects_exterior_point() {
        assert!(!europe().contains(50.0, 55.0)); // east of Europe
    }

    #[test]
    fn boundary_corners_are_included() {
        let b = europe();
        assert!(b.contains(b.min_lon, b.min_lat));
        assert!(b.contains(b.max_lon, b.max_lat));
    }

    #[test]
    fn world_contains_any_point() {
        assert!(Bbox::world().contains(0.0, 0.0));
        assert!(Bbox::world().contains(180.0, 90.0));
        assert!(Bbox::world().contains(-180.0, -90.0));
    }
}
