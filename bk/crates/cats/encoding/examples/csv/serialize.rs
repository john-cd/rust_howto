#![allow(dead_code)]
// ANCHOR: example
//! This example shows how to serialize data to a CSV file.
use std::io;

use anyhow::Result;

fn main() -> Result<()> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(["Name", "Place", "ID"])?;

    wtr.serialize(("Mark", "Sydney", 87))?;
    wtr.serialize(("Ashley", "Dublin", 32))?;
    wtr.serialize(("Akshat", "Delhi", 11))?;

    wtr.flush()?;
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
