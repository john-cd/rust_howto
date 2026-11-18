// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Streams CSV rows into typed Rust values and writes a typed value back to CSV.

use serde::Deserialize;
use serde::Serialize;

/// A sale record whose amount is stored in integer minor units.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Sale {
    store: String,
    amount_cents: u64,
}

/// Reads CSV records one at a time, then round-trips one record through a writer.
fn main() -> anyhow::Result<()> {
    let input = b"store,amount_cents\nNorth,1299\nSouth,2501\n";
    let mut reader = csv::Reader::from_reader(&input[..]);
    let mut total_cents = 0_u64;

    for row in reader.deserialize::<Sale>() {
        let sale = row?;
        total_cents = total_cents
            .checked_add(sale.amount_cents)
            .ok_or_else(|| anyhow::anyhow!("CSV total exceeds u64"))?;
    }
    assert_eq!(total_cents, 3_800);

    let sale = Sale {
        store: "West".to_owned(),
        amount_cents: 4_275,
    };
    let mut output = Vec::new();
    {
        let mut writer = csv::Writer::from_writer(&mut output);
        writer.serialize(&sale)?;
        writer.flush()?;
    }

    let mut written = csv::Reader::from_reader(output.as_slice());
    let restored = written
        .deserialize::<Sale>()
        .next()
        .transpose()?
        .ok_or_else(|| anyhow::anyhow!("CSV writer produced no record"))?;
    assert_eq!(restored, sale);

    Ok(())
}
// ANCHOR_END: example

/// Reads typed CSV rows and verifies a serialized row can be read back.
///
/// # Errors
///
/// Returns an error if CSV parsing, serialization, or flushing fails, or if the total exceeds
/// `u64`.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// Verifies typed CSV streaming and round-trip serialization.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies typed CSV streaming and round-trip serialization.
    #[test]
    fn reads_and_writes_typed_records() -> anyhow::Result<()> {
        main()
    }
}
