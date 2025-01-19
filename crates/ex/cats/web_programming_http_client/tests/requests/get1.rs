// ANCHOR: example
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}
// ANCHOR_END: example

// Requires network access
#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
