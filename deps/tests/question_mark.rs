use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("temp/hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

#[test]
fn test() {
    match read_username_from_file() {
        Ok(name) => println!("User name: {}", name),
        Err(err) => println!("Error: {}", err),
    }
}
