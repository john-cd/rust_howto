// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates composing standard iterator adapters with closures.
//!
//! Iterators process a sequence one item at a time. Adapters such as `map` and
//! `filter` are lazy: nothing happens until a consumer such as `collect` or
//! `sum` runs. The compiler usually optimizes the chain to a loop as fast as a
//! hand-written one.

/// Counts readings above `baseline` and sums their excess.
fn summarize_excess(readings: &[i32], baseline: i32) -> (usize, i32) {
    readings
        .iter()
        .copied()
        .filter(|reading| *reading > baseline)
        .map(|reading| reading - baseline)
        .fold((0, 0), |(count, total), excess| (count + 1, total + excess))
}

/// Prints the sum of readings above a baseline.
fn main() {
    let readings = [8, 12, 16, 20];
    let (count, total) = summarize_excess(&readings, 12);
    println!("{count} readings are above 12, with total excess {total}");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::main;
    use super::summarize_excess;

    /// Filters readings at the baseline and sums only their excess.
    /// Runs the iterator example through its entry point.
    #[test]
    fn iterator_adapters_compose_to_summarize_excess_readings() {
        assert_eq!(summarize_excess(&[8, 12, 16, 20], 12), (2, 12));
    }

    /// Runs the iterator example through its entry point.
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
