use miette::Result;
use miette_example::this_fails;

// To get errors printed nicely in application code, just return a `Result<()>`
// Note: You can swap out the default reporter for a custom one using
// `miette::set_hook()`
fn main() -> Result<()> {
    this_fails()?;
    Ok(())
}
