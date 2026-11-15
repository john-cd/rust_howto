//! Runs the geo examples.
//!
//! TODO Reviewed ??-??-2026.

/// Measure a unit square and compare two city coordinates with `geo`.
mod geo;
/// Turns an address into coordinates with the `geocoding` crate.
mod geocode;
/// Reads a GeoJSON feature and checks its planar geometry.
mod geojson_polygon;
/// Runs GEOS spatial predicates and operations through the `geos` crate.
mod geos;
/// Computes the great-circle distance between two places with the
mod haversine;
/// Uses an R-tree to find a nearby point without a network lookup.
mod spatial_index;
/// Recognizes US states by name or by abbreviation with `us-state-info`.
mod us_states;

/// Runs the geo examples. The online geocoding query is not run.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    geo::run();
    haversine::run();
    us_states::run();
    geocode::run()?;
    geojson_polygon::run()?;
    geos::run()?;
    spatial_index::run();
    Ok(())
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
