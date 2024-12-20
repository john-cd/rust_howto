// ANCHOR: example
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

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
