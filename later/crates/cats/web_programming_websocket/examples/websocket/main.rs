mod async_tungstenite;
mod tokio_tungstenite;
mod tungstenite;

fn main() -> anyhow::Result<()> {
    async_tungstenite::run();
    tungstenite::run()?;
    tokio_tungstenite::run()?;
    Ok(())
}
