//! Runs the processor example.
//!
//! TODO Reviewed ??-??-2026.

mod cpu_count;

/// Runs the processor example.
fn main() {
    cpu_count::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
