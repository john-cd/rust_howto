mod metadata;

fn main() -> anyhow::Result<()> {
    metadata::run()?;
    Ok(())
}
