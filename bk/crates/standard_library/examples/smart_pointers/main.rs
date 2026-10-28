//! Runs the smart pointer example.
//!
//! TODO Reviewed ??-??-2026.

mod deref;

/// Runs the smart pointers example.
fn main() {
    deref::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
