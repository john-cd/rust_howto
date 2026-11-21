// TODO Reviewed ??-??-2026.
//! Fixed-input sorting workload for benchmark comparison examples.

// ANCHOR: example
/// Number of values in the fixed benchmark input.
const INPUT_LEN: u64 = 100_000;

/// Sorts the same reversed integer sequence with the selected algorithm.
///
/// `black_box` discourages replacing the measured work with a known result.
fn sorted_workload(stable_sort: bool) -> Vec<u64> {
    let mut values: Vec<u64> = (0..INPUT_LEN).rev().map(std::hint::black_box).collect();
    if stable_sort {
        values.sort();
    } else {
        values.sort_unstable();
    }
    values
}

/// Checks that the workload produced every expected value in ascending order.
fn matches_expected_output(values: &[u64]) -> bool {
    values
        .iter()
        .copied()
        .map(std::hint::black_box)
        .eq(0..INPUT_LEN)
}

/// Verifies and reports the selected sort, exiting with status 2 for invalid output or arguments.
fn main() {
    let algorithm = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "unstable".to_owned());
    let stable_sort = match algorithm.as_str() {
        "stable" => true,
        "unstable" => false,
        _ => {
            eprintln!("usage: regression_workload [stable|unstable]");
            std::process::exit(2);
        }
    };
    let values = sorted_workload(stable_sort);
    if !matches_expected_output(&values) {
        eprintln!("sort did not produce the expected ascending sequence");
        std::process::exit(2);
    }
    println!("verified {} sorted values", values.len());
}
// ANCHOR_END: example

/// Checks the sort and the independent validation used by the benchmark.
#[cfg(test)]
mod tests {
    use super::INPUT_LEN;
    use super::matches_expected_output;
    use super::sorted_workload;

    /// Accepts the complete ascending sequence from both sorts.
    #[test]
    fn both_sorts_produce_expected_output() {
        assert!(matches_expected_output(&sorted_workload(true)));
        assert!(matches_expected_output(&sorted_workload(false)));
    }

    /// Rejects a sort that leaves the input in its original order.
    #[test]
    fn rejects_unchanged_input() {
        let input: Vec<_> = (0..INPUT_LEN).rev().collect();
        assert!(!matches_expected_output(&input));
    }
}
