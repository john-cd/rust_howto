// ANCHOR: example
//! This example demonstrates how to use the `anyhow` crate to handle errors
//! in a Rust program.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! reqwest = { version = "0.12.12", features = ["blocking"] }
//! ```

fn parse_response(
    response: reqwest::blocking::Response,
) -> anyhow::Result<u32> {
    let body = response.text()?;
    let body = body.trim();
    let b = body.parse::<u32>()?;
    Ok(b)
}

// The `main` function may return a `Result` itself.
// We return `anyhow::Result<()>` but you could use
// `Result<(), Box<dyn Error>>` as the return type.
fn main() -> anyhow::Result<()> {
    let url = "https://www.random.org/integers/?num=1&min=0&max=10&col=1&base=10&format=plain".to_string();
    // Issue a HTTP GET request to the API above.
    let response = reqwest::blocking::get(url)?;
    println!("Response status: {}", response.status());
    // Parse the returned `Response` into an integer.
    let random_value: u32 = parse_response(response)?;
    println!("A random number between 0 and 10: {}", random_value);
    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_network() {
    match main() {
        Ok(()) => println!("Success."),
        Err(e) => eprintln!("Error: {}", e),
    }
}
// TODO flaky service. rethink this example.
