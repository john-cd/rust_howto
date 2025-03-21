// ANCHOR: example
use std::fs;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
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
// [unwrap_or_else: println!("{}", ); NOW](https://github.com/john-cd/rust_howto/issues/172)
