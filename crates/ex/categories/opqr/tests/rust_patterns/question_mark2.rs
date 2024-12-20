// ANCHOR: example
use std::error::Error;

fn parse_port(s: &str) -> Result<u16, Box<dyn Error>> {
    // We need to use `Box<dyn Error>`, because the returned error type
    // cannot be determined during compile time: It will either
    // contain an instance of `std::num::ParseIntError` (from the parse
    // method, when parsing fails), or a string (when the port is
    // zero). Alternatively, you may use `anyhow::Result`.
    let port: u16 = s.parse()?;
    if port == 0 {
        Err(Box::from(format!("Invalid port: {}", port)))
    } else {
        Ok(port)
    }
}

fn main() {
    match parse_port("123") {
        Ok(port) => println!("Port: {}", port),
        Err(err) => panic!("{}", err),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
