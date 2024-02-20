use std::error::Error;

fn parse_port(s: &str) -> Result<u16, Box<dyn Error>> {
    // needed to use Box<dyn Error>, because the returned error type
    // cannot be determined during compile time: It will either
    // contain an instance of std::num::ParseIntError (from the parse
    // method, when parsing fails), or a string (when the port is
    // zero).
    let port: u16 = s.parse()?;
    if port == 0 {
        Err(Box::from(format!("invalid: {}", port)))
    } else {
        Ok(port)
    }
}

#[test]
fn test() {
    match parse_port("123") {
        Ok(port) => println!("port: {}", port),
        Err(err) => panic!("{}", err),
    }
}
