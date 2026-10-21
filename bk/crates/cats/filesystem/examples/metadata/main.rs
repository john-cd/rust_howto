// TODO Reviewed ??-??-2026
mod metadata;

fn main() -> anyhow::Result<()> {
    metadata::run()?;
    Ok(())
}
