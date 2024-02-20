use anyhow::Result;
use glob::glob;

#[test]
fn test() -> Result<()> {
    for entry in glob("**/*.png")? {
        println!("{}", entry?.display());
    }

    Ok(())
}
