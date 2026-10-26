#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Example: create a geographic polygon, compute its area, and find its
//! centroid using the `geo` crate.

use geo::Area as _;
use geo::Centroid as _;
use geo::Distance as _;
use geo::Haversine;
use geo::LineString;
use geo::Point;
use geo::Polygon;

/// Creates a polygon, computes its area and finds its centroid.
fn main() {
    let outer = LineString::from(vec![
        (0.0, 0.0),
        (10.0, 0.0),
        (10.0, 5.0),
        (5.0, 10.0),
        (0.0, 5.0),
        (0.0, 0.0),
    ]);

    let park = Polygon::new(outer, vec![]);

    let area = park.unsigned_area();
    let centroid = park.centroid().expect("polygon should have a centroid");

    println!("Polygon area = {area:.2}");
    println!("Centroid = ({:.2}, {:.2})", centroid.x(), centroid.y());
    let london = Point::new(-0.1278, 51.5074);
    let new_york = Point::new(-74.0060, 40.7128);
    let distance_km: f64 = Haversine.distance(london, new_york) / 1_000.0;
    assert!((distance_km - 5_570.0).abs() < 30.0);
    println!("London to New York = {distance_km:.0} km");
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
    // [review](https://github.com/john-cd/rust_howto/issues/839)
}
