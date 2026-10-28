//! Runs the cryptocurrencies example.
//!
//! TODO Reviewed ??-??-2026.

mod cryptocurrencies;

/// Runs the cryptocurrencies example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    cryptocurrencies::run()
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
