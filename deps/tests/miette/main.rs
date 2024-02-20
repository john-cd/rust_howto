mod mylib;

use miette::Result;

/// To get errors printed nicely in application code, just return a
/// `Result<()>`
///
/// Note: You can swap out the default reporter for a
/// custom one using `miette::set_hook()`
fn main() -> Result<()> {
    mylib::this_fails()?;
    Ok(())
}

#[test]
fn test() {
    assert!(main().is_err());
}
