use anyhow::{Context, Result};

fn do_something() -> Result<()> {
    Err(anyhow::Error::msg("Some Error"))
}

fn main() -> anyhow::Result<()> {
    // ...
    do_something().context("Failed to do the important thing")?; // Provide context

    let _content = std::fs::read("/notafile.txt")
        .with_context(|| "Failed to read instrs from file".to_string())?;

    Ok(())
}
