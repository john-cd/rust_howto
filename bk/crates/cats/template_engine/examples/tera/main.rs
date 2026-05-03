mod tera;

fn main() -> anyhow::Result<()> {
    tera::run()?;
    Ok(())
}
