//! Runs the geo examples.
//!
//! TODO Reviewed ??-??-2026.

mod geo;
mod geocode;
mod haversine;
mod us_states;

/// Runs the geo examples. The online geocoding query is not run.
fn main() -> Result<(), geocoding::GeocodingError> {
    geo::run();
    haversine::run();
    us_states::run();
    geocode::run()
}
