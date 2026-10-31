//! Reads a GeoJSON feature and checks its planar geometry.
//!
//! GeoJSON is a JSON format for geographic features such as points and
//! polygons. Web maps and many APIs exchange it. Coordinates are listed as
//! [longitude, latitude], in that order, which is easy to get backwards.

// ANCHOR: example
use std::error::Error;
use std::io;

use geo::Area;
use geo::Centroid;
use geo::Contains;
use geo::Geometry;
use geo::Point;
use geojson::GeoJson;

/// Parses an inline feature collection and computes its polygon's planar area.
fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"{"type":"FeatureCollection","features":[
        {"type":"Feature","properties":{},"geometry":{"type":"Polygon",
        "coordinates":[[[0,0],[2,0],[2,2],[0,2],[0,0]]]}}
    ]}"#;
    let GeoJson::FeatureCollection(collection) = input.parse()? else {
        return Err(
            io::Error::other("expected a GeoJSON feature collection").into()
        );
    };
    let geometry = collection
        .features
        .first()
        .and_then(|feature| feature.geometry.as_ref())
        .ok_or_else(|| io::Error::other("the first feature has no geometry"))?;
    let Geometry::Polygon(polygon) =
        Geometry::<f64>::try_from(&geometry.value)?
    else {
        return Err(io::Error::other("expected a polygon feature").into());
    };
    let area = polygon.unsigned_area();
    assert!((area - 4.0).abs() < f64::EPSILON);
    assert!(polygon.contains(&Point::new(1.0, 1.0)));
    let centroid = polygon
        .centroid()
        .ok_or_else(|| io::Error::other("the polygon has no centroid"))?;
    assert!((centroid.x() - 1.0).abs() < f64::EPSILON);
    assert!((centroid.y() - 1.0).abs() < f64::EPSILON);
    println!("Planar area: {area} square coordinate units");
    Ok(())
}
// ANCHOR_END: example

/// Runs the in-memory GeoJSON conversion and geometry assertions.
pub fn run() -> Result<(), Box<dyn Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Converts a feature to a polygon with the expected area, centroid, and
    /// interior.
    #[test]
    fn parses_a_polygon_from_an_inline_feature_collection()
    -> Result<(), Box<dyn Error>> {
        main()
    }
}
