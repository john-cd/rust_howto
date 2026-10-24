#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Parsing a color from a string by implementing `FromStr`.

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::ParseIntError;
use std::str::FromStr;

/// A color in RGB form.
#[derive(Debug, PartialEq)]
struct Rgb {
    /// The red component.
    r: u8,
    /// The green component.
    g: u8,
    /// The blue component.
    b: u8,
}

/// Describes why a string could not be parsed as an RGB hex color.
#[derive(Debug)]
enum RgbParseError {
    /// The input was not exactly seven ASCII bytes beginning with `#`.
    InvalidFormat,
    /// One of the three hexadecimal components was invalid.
    InvalidComponent(ParseIntError),
}

impl Display for RgbParseError {
    /// Formats an RGB parsing error for a user-facing diagnostic.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat => {
                formatter.write_str("expected an ASCII color in #RRGGBB form")
            }
            Self::InvalidComponent(error) => {
                write!(formatter, "invalid hexadecimal RGB component: {error}")
            }
        }
    }
}

impl Error for RgbParseError {
    /// Returns the underlying integer parsing error when a component is
    /// invalid.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidFormat => None,
            Self::InvalidComponent(error) => Some(error),
        }
    }
}

impl FromStr for Rgb {
    type Err = RgbParseError;

    /// Parses an ASCII color hex code in the exact `#RRGGBB` form.
    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        if !hex_code.is_ascii()
            || hex_code.len() != 7
            || !hex_code.starts_with('#')
        {
            return Err(RgbParseError::InvalidFormat);
        }

        // The ASCII and exact-length checks above make these byte boundaries
        // valid UTF-8 slice boundaries.
        let r = u8::from_str_radix(&hex_code[1..3], 16)
            .map_err(RgbParseError::InvalidComponent)?;
        let g = u8::from_str_radix(&hex_code[3..5], 16)
            .map_err(RgbParseError::InvalidComponent)?;
        let b = u8::from_str_radix(&hex_code[5..7], 16)
            .map_err(RgbParseError::InvalidComponent)?;

        Ok(Self { r, g, b })
    }
}

/// Parses a hexadecimal color code.
fn main() {
    let code: &str = "#fa7268";
    match Rgb::from_str(code) {
        Ok(rgb) => {
            println!(
                "The RGB color code is: R: {} G: {} B: {}",
                rgb.r, rgb.g, rgb.b
            );
        }
        Err(_) => {
            println!("{code} is not a valid color hex code!");
        }
    }
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies a valid six-digit ASCII color is parsed into RGB components.
    #[test]
    fn parses_rgb_hex_components() -> Result<(), RgbParseError> {
        let rgb = Rgb::from_str("#fa7268")?;

        assert_eq!(
            rgb,
            Rgb {
                r: 250,
                g: 114,
                b: 104,
            }
        );
        Ok(())
    }

    /// Verifies malformed lengths, prefixes, Unicode, and hex digits are
    /// errors.
    #[test]
    fn rejects_invalid_rgb_input_without_panicking() {
        for input in [
            "#fff",
            "#12345678",
            "123456",
            "##12345",
            "#12é456",
            "#12g456",
        ] {
            assert!(Rgb::from_str(input).is_err(), "accepted {input:?}");
        }
    }
}
