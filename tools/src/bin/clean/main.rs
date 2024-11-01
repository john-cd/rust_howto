use rust_howto_tools::*;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        //.with_max_level(tracing::Level::INFO)
        .init();
    // hard-coded to avoid any issues with the current working directory
    clean_folder(Path::new("/code/deps/temp"))?;
    Ok(())
}
