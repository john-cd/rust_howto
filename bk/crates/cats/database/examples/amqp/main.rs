#[cfg(feature = "lapin")]
mod lapin;

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "lapin")]
    {
        lapin::run()?;
    }
    Ok(())
}
