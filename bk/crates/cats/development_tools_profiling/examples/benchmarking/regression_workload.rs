//! Fixed-input executable for the CI benchmark comparison recipe.

// ANCHOR: example
fn main() {
    let mut values: Vec<u64> = (0..100_000).rev().map(std::hint::black_box).collect();
    values.sort_unstable();
    let checksum: u64 = values.into_iter().map(std::hint::black_box).sum();
    println!("{checksum}");
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    #[test]
    fn fixed_input_has_expected_checksum() {
        assert_eq!((0..100_000_u64).sum::<u64>(), 4_999_950_000);
    }
}
