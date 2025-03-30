// ANCHOR: example

// Add to your `Cargo.toml`:
// reqwest = { version = "0.12.12", features = ["blocking"] }

// A function that returns a `Result`:
fn parse_response(
    response: reqwest::blocking::Response,
) -> anyhow::Result<u32> {
    let body = response.text()?;
    let body = body.trim();
    let b = body.parse::<u32>()?;
    Ok(b)
}

// The `main` function may return a `Result` itself.
// Here we return `anyhow::Result<()>` but you could use `Result<(), Box<dyn
// Error>>` as the return type.
fn main() -> anyhow::Result<()> {
    let url = "https://www.random.org/integers/?num=1&min=0&max=10&col=1&base=10&format=plain".to_string();
    // Issue a HTTP GET request to the API above.
    let response = reqwest::blocking::get(url)?;
    // Parse the returned `Response` into an integer.
    let random_value: u32 = parse_response(response)?;
    println!("A random number between 0 and 10: {}", random_value);
    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_network() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
