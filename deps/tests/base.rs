use anyhow::anyhow;
use anyhow::Result;
use url::Url;

fn main() -> Result<()> {
    let full = "https://github.com/rust-lang/cargo?asdf";

    let url = Url::parse(full)?;
    let base = base_url(url)?;

    assert_eq!(base.as_str(), "https://github.com/");
    println!("The base of the URL is: {}", base);

    Ok(())
}

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

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
