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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
