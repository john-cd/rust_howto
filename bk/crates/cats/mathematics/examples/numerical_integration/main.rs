//! Runs the numerical integration example.
//!
//! TODO Reviewed ??-??-2026.

mod quadrature;

/// Prints the trapezoidal and Simpson estimates.
fn main() {
    quadrature::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
