//! Runs the `tinytemplate` template example.
//!
//! TODO Reviewed ??-??-2026.

mod tinytemplate;

/// Runs the tinytemplate example.
fn main() {
    tinytemplate::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
