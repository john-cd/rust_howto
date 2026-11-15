//! Runs the cryptocurrencies example.
//!
//! TODO Reviewed ??-??-2026.

/// Queries an Ethereum balance and signs a message with `alloy`, without
mod cryptocurrencies;

/// Runs the cryptocurrencies example.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cryptocurrencies::run().await
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
