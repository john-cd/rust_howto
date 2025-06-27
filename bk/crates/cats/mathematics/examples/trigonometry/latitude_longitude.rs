#![allow(dead_code)]
// ANCHOR: example
/// Calculates the distance between two points on the Earth's surface using the
/// Haversine formula.
///
/// This example demonstrates how to calculate the distance between Paris and
/// London based on their latitude and longitude coordinates.
fn main() {
    // Earth's radius in kilometers.
    let earth_radius_kilometer = 6371.0_f64;
    // Latitude and longitude of Paris in degrees.
    let (paris_latitude_degrees, paris_longitude_degrees) =
        (48.85341_f64, -2.34880_f64);
    // Latitude and longitude of London in degrees.
    let (london_latitude_degrees, london_longitude_degrees) =
        (51.50853_f64, -0.12574_f64);

    // Convert latitude from degrees to radians.
    let paris_latitude = paris_latitude_degrees.to_radians();
    let london_latitude = london_latitude_degrees.to_radians();

    let delta_latitude =
        (paris_latitude_degrees - london_latitude_degrees).to_radians();
    let delta_longitude =
        (paris_longitude_degrees - london_longitude_degrees).to_radians();

    // Haversine formula:
    // a = sin²(Δφ/2) + cos φ1 ⋅ cos φ2 ⋅ sin²(Δλ/2)
    // c = 2 ⋅ atan2( √a, √(1−a) ).
    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
        + paris_latitude.cos()
            * london_latitude.cos()
            * (delta_longitude / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    // Distance = radius * central angle.
    let distance = earth_radius_kilometer * central_angle;

    println!(
        "Distance between Paris and London on the surface of Earth is {distance:.1} kilometers"
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [review](https://github.com/john-cd/rust_howto/issues/1350)
