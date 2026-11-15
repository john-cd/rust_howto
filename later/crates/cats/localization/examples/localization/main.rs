//! Runs the localization example.
//!
//! TODO Reviewed ??-??-2026.

/// Loads inline Fluent resources and formats greetings and plural messages.
mod fluent;

/// Runs the localization example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    fluent::run()
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
