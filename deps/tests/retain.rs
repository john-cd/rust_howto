use anyhow::Result;

fn parse_response(response: reqwest::blocking::Response) -> Result<u32> {
    let mut body = response.text()?;
    body.pop();
    let b = body.parse::<u32>()?;
    Ok(b)
}

fn run() -> Result<()> {
    let url = format!(
        "https://www.random.org/integers/?num=1&min=0&max=10&col=1&base=10&format=plain"
    );
    let response = reqwest::blocking::get(&url)?;
    let random_value: u32 = parse_response(response)?;
    println!("a random number between 0 and 10: {}", random_value);
    Ok(())
}

#[test]
fn test() {
    if let Err(_error) = run() {
        // TODO
        // match *error.kind() {
        //     ErrorKind::Io(_) => println!("Standard IO error: {:?}",
        // error),     ErrorKind::Reqwest(_) => println!(
        //         "Reqwest error: {:?}",
        //         error
        //     ),
        //     // ErrorKind::ParseIntError(_) => {
        //     //     println!("Standard parse int error: {:?}",
        // error)     // }
        //     // ErrorKind::RandomResponseError(_) => {
        //     //     println!("User defined error: {:?}", error)
        //     // }
        //     _ => println!("Other error: {:?}", error),
        // }
    }
}
