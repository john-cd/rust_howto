//! Runs the drone example.
//!
//! TODO Reviewed ??-??-2026

mod drones1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    drones1::run().map_err(std::convert::Into::into)
}
