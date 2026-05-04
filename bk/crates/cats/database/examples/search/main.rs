#[cfg(feature = "elasticsearch")]
mod elasticsearch;

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "elasticsearch")]
    {
        elasticsearch::run()?;
    }
    Ok(())
}
