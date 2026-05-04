mod async_tungstenite;
mod tungstenite;
mod tokio_tungstenite;

fn main() -> anyhow::Result<()> {
    async_tungstenite::run();
    tungstenite::run()?;
    tokio_tungstenite::run()?;
    Ok(())
}
