#![allow(dead_code)]
// ANCHOR: example
//! Example: create a geographic polygon, compute its area, and find its
//! centroid using the `geo` crate.

use geo::Centroid;
use geo::GeoFloat;
use geo::LineString;
use geo::polygon;

fn main() {
    let outer = LineString::from(vec![
        (0.0, 0.0),
        (10.0, 0.0),
        (10.0, 5.0),
        (5.0, 10.0),
        (0.0, 5.0),
        (0.0, 0.0),
    ]);

    let park = polygon!(
        exterior: outer,
    );

    let area = park.unsigned_area();
    let centroid = park.centroid().expect("polygon should have a centroid");

    println!("Polygon area = {:.2}", area);
    println!("Centroid = ({:.2}, {:.2})", centroid.x(), centroid.y());
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[test]
fn test() {
    main();
}
// [review](https://github.com/john-cd/rust_howto/issues/839)
