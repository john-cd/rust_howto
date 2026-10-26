#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Prints the great-circle distance between Paris and London.

/// Calculates the distance between two points on the Earth's surface using the
/// Haversine formula.
///
/// This example demonstrates how to calculate the distance between Paris and
/// London based on their latitude and longitude coordinates.
fn haversine_distance_kilometers(
    latitude1_degrees: f64,
    longitude1_degrees: f64,
    latitude2_degrees: f64,
    longitude2_degrees: f64,
) -> f64 {
    /// Mean radius of Earth in kilometers.
    const EARTH_RADIUS_KILOMETERS: f64 = 6371.0;

    let latitude1 = latitude1_degrees.to_radians();
    let latitude2 = latitude2_degrees.to_radians();
    let delta_latitude = (latitude1_degrees - latitude2_degrees).to_radians();
    let delta_longitude =
        (longitude1_degrees - longitude2_degrees).to_radians();

    // Haversine formula:
    // a = sin²(Δφ/2) + cos φ1 ⋅ cos φ2 ⋅ sin²(Δλ/2)
    let a = (delta_latitude / 2.0).sin().powi(2)
        + latitude1.cos()
            * latitude2.cos()
            * (delta_longitude / 2.0).sin().powi(2);
    // Clamp rounding noise at coincident and antipodal points into the
    // mathematical domain of the square roots.
    let a = a.clamp(0.0, 1.0);
    let central_angle = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_KILOMETERS * central_angle
}

/// Prints the great-circle distance between Paris and London.
fn main() {
    // Latitude and longitude of Paris in degrees.
    let (paris_latitude_degrees, paris_longitude_degrees) =
        (48.85341_f64, 2.34880_f64);
    // Latitude and longitude of London in degrees.
    let (london_latitude_degrees, london_longitude_degrees) =
        (51.50853_f64, -0.12574_f64);

    let distance = haversine_distance_kilometers(
        paris_latitude_degrees,
        paris_longitude_degrees,
        london_latitude_degrees,
        london_longitude_degrees,
    );

    println!(
        "Distance between Paris and London on the surface of Earth is {distance:.1} kilometers"
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    /// Antipodal points have a finite distance of half Earth's circumference.
    #[test]
    fn distance_is_finite_for_antipodal_points() {
        let distance = haversine_distance_kilometers(0.0, 0.0, 0.0, 180.0);

        assert!(distance.is_finite());
        assert!((distance - std::f64::consts::PI * 6371.0).abs() < 1e-9);
    }

    /// Verifies zero distance for coincident points and about 344 km from Paris
    /// to London.
    #[test]
    fn distance_matches_coincident_points_and_paris_to_london() {
        assert_eq!(haversine_distance_kilometers(48.0, 2.0, 48.0, 2.0), 0.0);
        let distance = haversine_distance_kilometers(
            48.85341, 2.34880, 51.50853, -0.12574,
        );
        assert!((distance - 344.0).abs() < 2.0);
    }
}
// review
