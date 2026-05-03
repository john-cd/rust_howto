mod symbolic_links;

fn main() -> anyhow::Result<()> {
    symbolic_links::run()?;
    Ok(())
}
