mod tempfile;

fn main() -> anyhow::Result<()> {
    tempfile::run()?;
    Ok(())
}
