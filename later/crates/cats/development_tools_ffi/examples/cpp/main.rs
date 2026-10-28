//! Runs the C++ interoperability example.
//!
//! TODO Reviewed ??-??-2026.

mod cxx;

/// Runs the cpp example.
fn main() {
    cxx::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
