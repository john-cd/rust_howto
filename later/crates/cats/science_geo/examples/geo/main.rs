//! Runs the geo examples.
//!
//! TODO Reviewed ??-??-2026.

mod geo;
mod geocode;
mod geojson_polygon;
mod geos;
mod haversine;
mod spatial_index;
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
