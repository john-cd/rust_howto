// ANCHOR: example
use uom::si::f32::Length;
use uom::si::f32::Time;
use uom::si::length::kilometer;
use uom::si::length::meter;
use uom::si::time::second;
use uom::si::velocity::meter_per_second;

/// Calculates a speed from typed distance and time quantities.
fn calculate_speed() -> (f32, f32) {
    let distance = Length::new::<kilometer>(1.5);
    let duration = Time::new::<second>(180.0);
    let speed = distance / duration;

    (distance.get::<meter>(), speed.get::<meter_per_second>())
}

/// Prints the converted distance and calculated speed.
fn main() {
    let (distance_in_meters, speed_in_meters_per_second) = calculate_speed();
    println!("1.5 km = {distance_in_meters:.0} m");
    println!("speed = {speed_in_meters_per_second:.3} m/s");
}
// ANCHOR_END: example

/// Runs the example from the chapter's example runner.
pub fn run() {
    main();
}

/// Verifies `uom` converts distance and derives speed with matching dimensions.
#[test]
fn test_quantities_convert_distance_and_speed() {
    let (distance_in_meters, speed_in_meters_per_second) = calculate_speed();
    assert!((distance_in_meters - 1_500.0).abs() < 1e-3);
    assert!((speed_in_meters_per_second - (1_500.0 / 180.0)).abs() < 1e-4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
