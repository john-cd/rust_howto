// ANCHOR: example
use anyhow::Result;
use anyhow::anyhow;
use url::Url;

fn base_url(mut url: Url) -> Result<Url> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            return Err(anyhow!("This URL is cannot-be-a-base."));
        }
    }

    url.set_query(None);

    Ok(url)
}

fn main() -> Result<()> {
    let full = "https://github.com/rust-lang/cargo?asdf";

    let url = Url::parse(full)?;
    let base = base_url(url)?;

    assert_eq!(base.as_str(), "https://github.com/");
    println!("The base of the URL is: {}", base);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
// TODO P1
