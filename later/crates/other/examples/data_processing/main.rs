// TODO Reviewed ??-??-2026
use anyhow::Result;

#[cfg(feature = "arrow")]
mod arrow;

fn main() -> Result<()> {
    #[cfg(feature = "arrow")]
    arrow::run()?;
    Ok(())
}
