// TODO Reviewed ??-??-2026
mod tera;

fn main() -> anyhow::Result<()> {
    tera::run()?;
    Ok(())
}
