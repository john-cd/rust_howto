//! Fixed-input sorting workload for benchmark comparison examples.

// ANCHOR: example
fn checksum(stable_sort: bool) -> u64 {
    let mut values: Vec<u64> = (0..100_000).rev().map(std::hint::black_box).collect();
    if stable_sort {
        values.sort();
    } else {
        values.sort_unstable();
    }
    let checksum: u64 = values.into_iter().map(std::hint::black_box).sum();
    checksum
}

fn main() {
    let algorithm = std::env::args().nth(1).unwrap_or_else(|| "unstable".to_owned());
    let stable_sort = match algorithm.as_str() {
        "stable" => true,
        "unstable" => false,
        _ => {
            eprintln!("usage: regression_workload [stable|unstable]");
            std::process::exit(2);
        }
    };
    println!("{}", checksum(stable_sort));
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::checksum;

    /// Checks that fixed input has expected checksum.
    #[test]
    fn fixed_input_has_expected_checksum() {
        assert_eq!((0..100_000_u64).sum::<u64>(), 4_999_950_000);
        assert_eq!(checksum(true), 4_999_950_000);
        assert_eq!(checksum(false), 4_999_950_000);
    }
}
