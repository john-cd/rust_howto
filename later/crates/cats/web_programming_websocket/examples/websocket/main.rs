//! Runs the WebSocket examples.
//!
//! TODO Reviewed ??-??-2026

mod async_tungstenite;
mod tokio_tungstenite;
mod tungstenite;

/// Runs each of the WebSocket examples.
fn main() -> anyhow::Result<()> {
    async_tungstenite::run();
    tungstenite::run()?;
    tokio_tungstenite::run()?;
    Ok(())
}
