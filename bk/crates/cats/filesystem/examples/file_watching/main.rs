mod notify;

fn main() -> anyhow::Result<()> {
    notify::run()?;
    Ok(())
}
