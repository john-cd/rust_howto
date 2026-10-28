// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Computes the great-circle distance between two places with the
//! `haversine` crate and with `geo`.

use geo::Distance as _;
use geo::Haversine;
use geo::Point;
use haversine::Location;
use haversine::Units;

/// Computes the distance in kilometers from London to New York two ways.
fn main() {
    // `haversine` takes (latitude, longitude) in degrees.
    let london = Location {
        latitude: 51.5074,
        longitude: -0.1278,
    };
    let new_york = Location {
        latitude: 40.7128,
        longitude: -74.0060,
    };
    let km = haversine::distance(london, new_york, Units::Kilometers);

    // `geo` points are (longitude, latitude), that is (x, y).
    // `Haversine` returns meters.
    let london = Point::new(-0.1278, 51.5074);
    let new_york = Point::new(-74.0060, 40.7128);
    let geo_km = Haversine.distance(london, new_york) / 1_000.0;

    println!("haversine: {km:.0} km, geo: {geo_km:.0} km");

    // The known distance is about 5 570 km.
    assert!((km - 5_570.0).abs() < 30.0);
    // The two crates use slightly different Earth radii.
    assert!((km - geo_km).abs() < 5.0);
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the haversine example through its entry point.
    #[test]
    fn test_haversine_example_entry_point() {
        main();
    }

    #[test]
    fn distance_to_self_is_zero() {
        let here = Location {
            latitude: 48.8566,
            longitude: 2.3522,
        };
        let there = Location {
            latitude: 48.8566,
            longitude: 2.3522,
        };
        assert!(haversine::distance(here, there, Units::Kilometers) < 1e-9);
    }
}
