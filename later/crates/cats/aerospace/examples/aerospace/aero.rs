// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Sample a satellite's orbit from a fixed two-line element set using `sgp4`.

use sgp4::Constants;
use sgp4::Elements;
use sgp4::MinutesSinceEpoch;
use sgp4::Prediction;

/// Builds propagation constants from a fixed historical ISS element set.
fn iss_constants() -> Constants {
    let elements = Elements::from_tle(
        Some("ISS (ZARYA)".to_owned()),
        b"1 25544U 98067A   26113.55475666  .00008745  00000+0  16716-3 0  9994",
        b"2 25544  51.6320 210.5007 0006819 341.8336  18.2408 15.48911267563287",
    )
    .expect("the fixed ISS element set should parse");
    Constants::from_elements(&elements)
        .expect("the fixed ISS elements should yield propagation constants")
}

/// Predicts position and velocity at a number of minutes from the TLE epoch.
fn predict(constants: &Constants, minutes: f64) -> Prediction {
    constants
        .propagate(MinutesSinceEpoch(minutes))
        .expect("the fixed ISS elements should propagate over one orbit")
}

/// Samples roughly one orbit at ten-minute intervals.
fn main() {
    let constants = iss_constants();
    for minute in (0..=90).step_by(10) {
        let prediction = predict(&constants, f64::from(minute));
        println!("{minute:>2} min: {:?} km", prediction.position);
    }
}
// ANCHOR_END: example

/// Runs the orbit sampling example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the ISS orbital radius at epoch and approximately one orbit
    /// later.
    #[test]
    fn predicted_positions_have_plausible_orbital_radii() {
        let constants = iss_constants();
        for minutes in [0.0, 90.0] {
            let position = predict(&constants, minutes).position;
            let radius = position
                .iter()
                .map(|coordinate| coordinate * coordinate)
                .sum::<f64>()
                .sqrt();
            assert!(
                (6_700.0..=6_800.0).contains(&radius),
                "unexpected radius at {minutes} minutes: {radius} km"
            );
        }
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
