// ANCHOR: example
use anyhow::Result;
use select::document::Document;
use select::predicate::Name;

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://www.rust-lang.org/en-US/")
        .await?
        .text()
        .await?;

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}
// ANCHOR_END: example

// Requires network access
#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
