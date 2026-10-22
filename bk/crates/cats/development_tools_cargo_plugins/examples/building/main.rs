//! Example building cargo plugins and executing shells
//!
//! TODO Reviewed ??-??-2026

mod xshell;

fn main() -> anyhow::Result<()> {
    xshell::run()?;
    Ok(())
}
