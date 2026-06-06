//! Service port ("interface assembly") for Wisp.
//!
//! Contains the [`TripService`] trait plus its request/response DTOs.
//! No implementations live here — both the UI and server depend on this crate,
//! while [`LocalTripService`] and [`RemoteTripService`] live in their respective
//! app crates.

mod trip_service;

pub use trip_service::{SyncReport, TrackQuery, TripService};
