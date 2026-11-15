//! Runs the WebSocket examples.
//!
//! TODO Reviewed ??-??-2026.

/// An asynchronous WebSocket client using async-tungstenite's Tokio adapter.
mod async_tungstenite;

/// A Tokio WebSocket client and one-connection echo server.
mod tokio_tungstenite;

/// A blocking WebSocket client using tungstenite.
mod tungstenite;

/// Runs each of the WebSocket examples.
fn main() -> anyhow::Result<()> {
    async_tungstenite::run()?;
    tungstenite::run()?;
    tokio_tungstenite::run()?;
    Ok(())
}
