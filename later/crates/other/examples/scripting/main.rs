use anyhow::anyhow;
use anyhow::Result;

mod rhai;

fn main() -> Result<()> {
    rhai::run().map_err(|error| anyhow!(error.to_string()))?;
    Ok(())
}
