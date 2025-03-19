// ANCHOR: example
use std::fs;

use globset::Glob;
use globset::GlobSet;
use globset::GlobSetBuilder;

fn main() -> anyhow::Result<()> {
    // Create a glob set builder and add patterns
    let mut builder = GlobSetBuilder::new();
    builder.add(Glob::new("*.rs")?);
    builder.add(Glob::new("*.md")?);
    let glob_set: GlobSet = builder.build()?;

    // Specify the directory to search
    let dir = "./tests";

    // Iterate over the files in the directory
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        // Check if the file matches any of the patterns
        if glob_set.is_match(&path) {
            println!("Matched: {}", path.display());
        }
    }
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [WIP review](https://github.com/john-cd/rust_howto/issues/1003)
