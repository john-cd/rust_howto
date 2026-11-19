// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Compares a measured route with a constant speed model.

use geo::Distance as _;
use geo::Euclidean;
use geo::Point;

/// A reading in a local Cartesian coordinate system, in seconds and meters.
#[derive(Clone, Copy)]
struct Reading {
    seconds: f64,
    position: Point<f64>,
}

/// Computes traveled distance and elapsed time from ordered readings.
///
/// # Errors
///
/// Returns an error if the route has fewer than two readings, nonfinite data,
/// or timestamps that fail to increase.
fn summarize(readings: &[Reading]) -> Result<(f64, f64), &'static str> {
    if readings.len() < 2 {
        return Err("at least two readings are required");
    }
    if readings.iter().any(|r| {
        !r.seconds.is_finite() || !r.position.x().is_finite() || !r.position.y().is_finite()
    }) {
        return Err("readings must be finite");
    }

    let mut distance = 0.0;
    let mut duration = 0.0;
    for pair in readings.windows(2) {
        let elapsed = pair[1].seconds - pair[0].seconds;
        if elapsed <= 0.0 {
            return Err("timestamps must increase");
        }
        distance += Euclidean.distance(pair[0].position, pair[1].position);
        duration += elapsed;
    }
    if !distance.is_finite() || !duration.is_finite() {
        return Err("route totals exceed the numeric range");
    }
    Ok((distance, duration))
}

/// Prints the measured route and the model's predicted travel time.
fn main() -> Result<(), &'static str> {
    let readings = [
        Reading {
            seconds: 0.0,
            position: Point::new(0.0, 0.0),
        },
        Reading {
            seconds: 5.0,
            position: Point::new(3.0, 4.0),
        },
        Reading {
            seconds: 11.0,
            position: Point::new(3.0, 16.0),
        },
    ];
    let (metres, seconds) = summarize(&readings)?;
    let model_seconds = metres / 2.0; // Constant speed of 2 m/s.
    assert!((metres - 17.0).abs() < 1e-9);
    assert!((seconds - 11.0).abs() < 1e-9);
    println!("{metres:.1} m in {seconds:.1} s (constant speed model: {model_seconds:.1} s)");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the known distance and elapsed time of the fixture.
    #[test]
    fn fixture_matches_expected_totals() -> Result<(), &'static str> {
        main()
    }

    /// Rejects a repeated timestamp before using it in a speed calculation.
    #[test]
    fn rejects_repeated_timestamps() {
        let reading = Reading {
            seconds: 1.0,
            position: Point::new(0.0, 0.0),
        };
        assert_eq!(
            summarize(&[reading, reading]),
            Err("timestamps must increase")
        );
    }
}
