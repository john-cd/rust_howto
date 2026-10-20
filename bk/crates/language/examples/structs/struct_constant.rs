#![allow(dead_code)]
// ANCHOR: example
struct MyToken;

impl MyToken {
    // Inherent implementations can contain associated constants,
    // in addition to associated functions or methods.
    const ID: i32 = 1;
}

fn main() {
    println!("ID: {}", MyToken::ID);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main();
    }
}
