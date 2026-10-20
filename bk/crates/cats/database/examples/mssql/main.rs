#[cfg(feature = "mssql")]
mod tiberius;

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "mssql")]
    {
        tiberius::run()?;
    }
    Ok(())
}
