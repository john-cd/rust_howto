use url::ParseError;
use url::Position;
use url::Url;

#[test]
fn test() -> Result<(), ParseError> {
    let parsed = Url::parse(
        "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open",
    )?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {}", cleaned);
    Ok(())
}
