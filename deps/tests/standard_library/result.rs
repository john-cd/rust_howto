// ANCHOR: example
use std::fs::File;
use std::io::Read;
use std::io::{self};

fn read_file(file_path: &str) -> Result<String, io::Error> {
    // If opening the file fails, return with an Err
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    // If reading the file fails, return with an Err
    file.read_to_string(&mut contents)?;
    // Happy path: return the contents of the file as a String
    Ok(contents)
}

fn main() {
    let file_path = "example.txt";

    // Handle the `Result`
    match read_file(file_path) {
        Ok(contents) => println!("File content: \n{}", contents),
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
