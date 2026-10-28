// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates a simple HTTP GET request using the `reqwest`
//! crate.

use anyhow::Result;

/// Sends a `GET` request and prints the status, headers and body of the
/// response.
#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://httpbin.org/get")
        .await?
        .error_for_status()?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{body}");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Marks this live-service demonstration as an opt-in network test.
    #[test]
    #[ignore = "needs network"]
    fn require_network() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
