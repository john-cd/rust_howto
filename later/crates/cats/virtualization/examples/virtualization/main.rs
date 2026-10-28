//! Runs the virtualization example.
//!
//! TODO Reviewed ??-??-2026.

mod virtualization1;

/// Runs the virtualization example.
fn main() {
    virtualization1::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
