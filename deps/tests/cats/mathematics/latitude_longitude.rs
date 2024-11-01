// ANCHOR: example
fn main() {
    let earth_radius_kilometer = 6371.0_f64;
    let (paris_latitude_degrees, paris_longitude_degrees) =
        (48.85341_f64, -2.34880_f64);
    let (london_latitude_degrees, london_longitude_degrees) =
        (51.50853_f64, -0.12574_f64);

    let paris_latitude = paris_latitude_degrees.to_radians();
    let london_latitude = london_latitude_degrees.to_radians();

    let delta_latitude =
        (paris_latitude_degrees - london_latitude_degrees).to_radians();
    let delta_longitude =
        (paris_longitude_degrees - london_longitude_degrees).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
        + paris_latitude.cos()
            * london_latitude.cos()
            * (delta_longitude / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    let distance = earth_radius_kilometer * central_angle;

    println!(
        "Distance between Paris and London on the surface of Earth is {:.1} kilometers",
        distance
    );
}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
