// ANCHOR: example
use std::env;

use anyhow::Result;

fn main() -> Result<()> {
    let cwd = env::current_dir()?;
    println!("The current directory is {}", cwd.display());
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}