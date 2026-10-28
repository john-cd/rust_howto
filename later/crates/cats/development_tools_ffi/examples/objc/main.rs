//! Runs the Objective-C interoperability example.
//!
//! TODO Reviewed ??-??-2026.

mod objc2;

/// Runs the objc example.
fn main() {
    objc2::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
