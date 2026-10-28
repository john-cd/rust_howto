//! Runs the internationalization example.
//!
//! TODO Reviewed ??-??-2026.

mod icu_formatting;

/// Runs the internationalization example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    icu_formatting::run()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
