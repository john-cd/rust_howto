#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Measure a unit square and compare two city coordinates with `geo`.

use geo::Area as _;
use geo::Centroid as _;
use geo::Contains as _;
use geo::Distance as _;
use geo::Haversine;
use geo::Point;
use geo::polygon;

/// Creates a unit square, checks its geometry and containment, and measures a
/// city distance.
fn main() {
    let square = polygon![
        (x: 0.0, y: 0.0),
        (x: 1.0, y: 0.0),
        (x: 1.0, y: 1.0),
        (x: 0.0, y: 1.0),
        (x: 0.0, y: 0.0),
    ];
    let area = square.unsigned_area();
    let centroid = square.centroid().expect("unit square has a centroid");
    assert_eq!(area, 1.0);
    assert_eq!(centroid, Point::new(0.5, 0.5));

    let inside = Point::new(0.25, 0.75);
    let outside = Point::new(1.5, 0.5);
    assert!(square.contains(&inside));
    assert!(!square.contains(&outside));

    let london = Point::new(-0.1278, 51.5074);
    let new_york = Point::new(-74.0060, 40.7128);
    let distance_km: f64 = Haversine.distance(london, new_york) / 1_000.0;
    assert!((distance_km - 5_570.0).abs() < 30.0);

    println!("Square area = {area:.1}");
    println!("Centroid = ({:.1}, {:.1})", centroid.x(), centroid.y());
    println!("London to New York = {distance_km:.0} km");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies area, centroid, containment, and Haversine distance assertions.
    #[test]
    fn unit_square_geometry_and_city_distance_are_valid() {
        main();
    }
}
