// TODO Reviewed ??-??-2026
//! Example building cargo plugins and executing shells

mod xshell;

fn main() -> anyhow::Result<()> {
    xshell::run()?;
    Ok(())
}
