#[cfg(feature = "polars")]
mod polars;

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "polars")]
    polars::run()?;
    Ok(())
}
