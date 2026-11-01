// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Format a byte count as a human-readable size.
//!
//! People read "1.5 MiB" faster than "1572864". Binary units (KiB, MiB) use
//! powers of 1024; decimal units (kB, MB) use 1000, so pick the one your
//! audience expects.

use humansize::BINARY;
use humansize::DECIMAL;
use humansize::format_size;
use humansize::format_size_i;

/// Formats sizes with SI (powers of 1000) and IEC (powers of 1024) units.
pub fn run() {
    // SI units: kB, MB, GB...
    assert_eq!(format_size(1_536_000_u64, DECIMAL), "1.54 MB");
    // IEC units: KiB, MiB, GiB...
    assert_eq!(format_size(1_536_000_u64, BINARY), "1.46 MiB");

    // Signed values, such as a change in memory use, use `format_size_i`.
    assert_eq!(format_size_i(-1_572_864_i64, BINARY), "-1.50 MiB");
    println!("{}", format_size(1_536_000_u64, DECIMAL));
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_succeeds() {
        run();
    }
}
