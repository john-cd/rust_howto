//! Runs the `criterion` benchmarking example.
//!
//! TODO Reviewed ??-??-2026.

mod criterion;

/// Runs the benchmarking example.
fn main() {
    criterion::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
