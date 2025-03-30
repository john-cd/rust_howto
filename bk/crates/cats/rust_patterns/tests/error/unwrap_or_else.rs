// ANCHOR: example
use std::fs;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Simple example
    let some_option: Option<&str> = None;
    // Use `unwrap_or_else` to provide a default value with custom logic
    let result = some_option.unwrap_or_else(|| {
        println!("Option was None. Providing default value.");
        "default_value"
    });
    println!("Result: {}", result);

    // Fallback example
    if !fs::exists("temp").unwrap() {
        fs::create_dir("temp").unwrap();
    }
    let _greeting_file = File::open("temp/hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("temp/hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
