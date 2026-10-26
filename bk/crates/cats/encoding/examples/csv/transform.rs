#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example shows how to transform CSV data from one format to
//! another.

use std::str::FromStr;

use anyhow::Result;
use anyhow::anyhow;
use serde::Deserialize;
use serde::Deserializer;
use serde::de;

/// A row of CSV data.
#[derive(Debug, Deserialize)]
struct Row {
    /// The name of the color.
    color_name: String,
    /// The color.
    color: HexColor,
}

#[derive(Debug)]
/// A color represented as a hex triplet.
struct HexColor {
    /// The red component.
    red: u8,
    /// The green component.
    green: u8,
    /// The blue component.
    blue: u8,
}

impl FromStr for HexColor {
    type Err = anyhow::Error;

    /// Parse a hex color string into a `HexColor`.
    fn from_str(hex_color: &str) -> std::result::Result<Self, Self::Err> {
        let trimmed = hex_color.trim_matches('#');
        if trimmed.len() == 6 {
            Ok(Self {
                red: u8::from_str_radix(&trimmed[..2], 16)?,
                green: u8::from_str_radix(&trimmed[2..4], 16)?,
                blue: u8::from_str_radix(&trimmed[4..6], 16)?,
            })
        } else {
            Err(anyhow!("Invalid length of hex string"))
        }
    }
}

impl<'de> Deserialize<'de> for HexColor {
    /// Deserialize a hex color string into a `HexColor`.
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

/// Converts CSV rows containing hex colors into rows containing RGB channels.
fn transform_csv(data: &str) -> Result<String> {
    let mut out = csv::Writer::from_writer(vec![]);
    let mut reader = csv::Reader::from_reader(data.as_bytes());
    // Deserialize as `Row`, using the implementation above.
    for result in reader.deserialize() {
        // We need to provide a type hint for automatic deserialization.
        let res: Row = result?;

        // Serialize the tuple as CSV into `Vec<u8>`.
        out.serialize((
            res.color_name,
            res.color.red,
            res.color.green,
            res.color.blue,
        ))?;
    }
    Ok(String::from_utf8(out.into_inner()?)?)
}

/// Transforms and prints the inline color CSV data.
fn main() -> Result<()> {
    let data = "color_name,color
red,#ff0000
green,#00ff00
blue,#0000FF
periwinkle,#ccccff
magenta,#ff00ff";
    let written = transform_csv(data)?;
    println!("{written}");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies each source hex color is transformed into RGB channel columns.
    #[test]
    fn converts_hex_colors_to_rgb_columns() -> anyhow::Result<()> {
        let written = transform_csv(
            "color_name,color\nred,#ff0000\ngreen,#00ff00\nblue,#0000FF\nperiwinkle,#ccccff\nmagenta,#ff00ff",
        )?;
        assert_eq!(
            written,
            "red,255,0,0\ngreen,0,255,0\nblue,0,0,255\nperiwinkle,204,204,255\nmagenta,255,0,255\n"
        );
        Ok(())
    }
}
