use anyhow::Result;

// TODO rewrite

fn parse_response(response: reqwest::blocking::Response) -> Result<u32> {
    let mut body = response.text()?;
    body.pop();
    let b = body.parse::<u32>()?;
    Ok(b)
}

fn main() -> Result<()> {
    let url = "https://www.random.org/integers/?num=1&min=0&max=10&col=1&base=10&format=plain".to_string();
    let response = reqwest::blocking::get(url)?;
    let random_value: u32 = parse_response(response)?;
    println!("a random number between 0 and 10: {}", random_value);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
