//! Runs GEOS spatial predicates and operations through the `geos` crate.

// ANCHOR: example
#[cfg(feature = "geos")]
use geos::Geom;
#[cfg(feature = "geos")]
use geos::Geometry;

/// Parses two polygons from WKT, tests them and measures their overlap.
#[cfg(feature = "geos")]
fn main() -> geos::GResult<()> {
    let square = Geometry::new_from_wkt("POLYGON((0 0, 4 0, 4 4, 0 4, 0 0))")?;
    let other = Geometry::new_from_wkt("POLYGON((2 2, 6 2, 6 6, 2 6, 2 2))")?;

    // Spatial predicates.
    assert!(square.intersects(&other)?);
    assert!(!square.contains(&other)?);

    // Overlay operations return new geometries. All values are planar,
    // in the units of the input coordinates.
    let overlap = square.intersection(&other)?;
    println!("overlap: {} (area {})", overlap.to_wkt()?, overlap.area()?);
    assert!((overlap.area()? - 4.0).abs() < 1e-9);

    // Buffering grows a geometry by a distance.
    let buffered = square.buffer(1.0, 8)?;
    assert!(buffered.area()? > square.area()?);
    Ok(())
}

/// Prints a message when the `geos` feature is not enabled.
#[cfg(not(feature = "geos"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Enable the `geos` feature (it needs the GEOS C library) to run this \
         example."
    );
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    main()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_succeeds() -> Result<(), Box<dyn std::error::Error>> {
        run()
    }

    #[test]
    fn test_main_entry_point() -> Result<(), Box<dyn std::error::Error>> {
        main()
    }
}
