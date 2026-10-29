//! Runs the WebSocket examples.
//!
//! TODO Reviewed ??-??-2026.

mod async_tungstenite;
mod tokio_tungstenite;
mod tungstenite;

/// Runs each of the WebSocket examples.
fn main() -> anyhow::Result<()> {
    async_tungstenite::run()?;
    tungstenite::run()?;
    tokio_tungstenite::run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    #[ignore = "needs network access to the public echo service"]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
