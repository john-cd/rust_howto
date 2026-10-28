//! Runs the localization example.
//!
//! TODO Reviewed ??-??-2026.

mod localization1;

/// Runs the localization example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    localization1::run()
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
