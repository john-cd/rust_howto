// ANCHOR: example
use url::ParseError;
use url::Position;
use url::Url;

fn main() -> Result<(), ParseError> {
    let parsed = Url::parse(
        "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open",
    )?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("`cleaned`: {}", cleaned);
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
