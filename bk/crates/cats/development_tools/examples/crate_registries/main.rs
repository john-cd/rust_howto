//! Crate registries examples.

mod crates_io;

fn main() -> anyhow::Result<()> {
    crates_io::run()?;
    Ok(())
}
