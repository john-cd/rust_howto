// ANCHOR: example
//! # Backtrace
//!
//! This example demonstrates how to use backtraces to debug errors.
//!
//! ## Example
//!
//! The example code defines a struct `Rgb` that represents a color in RGB
//! format.
use std::fmt;

use anyhow::Result;
use anyhow::anyhow;
use serde::Deserialize;
// use anyhow::Context;

#[derive(Debug, Deserialize)]
struct Rgb {
    red: u8,
    blue: u8,
    green: u8,
}

impl Rgb {
    /// The `Rgb::from_csv` function parses a CSV string and returns an `Rgb`
    /// struct.
    fn from_csv(csv_data: &[u8]) -> Result<Rgb> {
        let color = csv::Reader::from_reader(csv_data)
            .deserialize() // Returns a borrowed iterator over deserialized records. Each item is a Result<Rgb, Error>
            .nth(0)
            .ok_or(anyhow!("First CSV row does not exist."))?; // Convert `None` into `Result::Err`.
        Ok(color?)
    }
}

// The `UpperHex` trait is used to format the `Rgb` struct as a hexadecimal
// string.
impl fmt::UpperHex for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hexa = (u32::from(self.red) << 16)
            | (u32::from(self.blue) << 8)
            | u32::from(self.green);
        write!(f, "{:X}", hexa)
    }
}

// The `main` function parses a CSV string and prints the result.
fn main() -> Result<()> {
    let csv = "red,blue,green
102,256,204"; // Note the invalid value.

    let rgb = Rgb::from_csv(csv.as_bytes())?;
    println!("{:?} to hexadecimal #{:X}", rgb, rgb);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    let res = main();
    println!("{:?}", res);
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "");
    }
    assert!(res.is_err());
}
