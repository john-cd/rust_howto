#[cfg(feature = "sqlparser")]
mod sqlparser;

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "sqlparser")]
    {
        sqlparser::run()?;
    }
    Ok(())
}
