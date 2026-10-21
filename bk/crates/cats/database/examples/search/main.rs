// TODO Reviewed ??-??-2026
#[cfg(feature = "elasticsearch")]
mod elasticsearch;

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "elasticsearch")]
    {
        elasticsearch::run()?;
    }
    Ok(())
}
