// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Converts an encoded sRGB color to linear light and back.

use palette::LinSrgb;
use palette::Srgb;

/// Decodes an 8-bit sRGB color to linear-light values.
fn decode_srgb(color: Srgb<u8>) -> LinSrgb<f32> {
    color.into_linear()
}

/// Encodes linear-light values as 8-bit sRGB.
fn encode_srgb(color: LinSrgb<f32>) -> Srgb<u8> {
    Srgb::from_linear(color)
}

/// Prints the stored, linear, and display color values.
fn print_example() {
    let stored = Srgb::<u8>::new(128, 64, 32);
    let linear = decode_srgb(stored);
    let display = encode_srgb(linear);
    println!("stored={stored:?}, linear={linear:?}, display={display:?}");
}

/// Runs the color conversion from a standalone entry point.
fn main() {
    print_example();
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use palette::Srgb;

    use super::decode_srgb;
    use super::encode_srgb;

    /// Converts sRGB values to linear light and back without losing precision.
    #[test]
    fn srgb_round_trips_through_linear_light() {
        let stored = Srgb::<u8>::new(128, 64, 32);
        let linear = decode_srgb(stored);
        let display = encode_srgb(linear);

        assert!(linear.red < f32::from(stored.red) / 255.0);
        assert!((i16::from(display.red) - i16::from(stored.red)).abs() <= 1);
        assert!((i16::from(display.green) - i16::from(stored.green)).abs() <= 1);
        assert!((i16::from(display.blue) - i16::from(stored.blue)).abs() <= 1);
    }
}

/// Runs the example logic exposed by this module.
pub fn run() {
    let entry_point = main;
    entry_point();
}

/// Provides entry point tests behavior for the srgb linear conversion component.
#[cfg(test)]
mod entry_point_tests {
    use super::*;

    /// Checks that private main runs.
    #[test]
    fn private_main_runs() {
        let entry_point = main;
        entry_point();
    }
}
