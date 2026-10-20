#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `assert!` macro with a custom
//! failure message.

fn check_name(name: &str) {
    assert!(
        name.contains("Carol"),
        "Name did not contain 'Carol', value was `{name}`"
    );
}

fn main() {
    println!("Checking if 'Carl' contains 'Carol'...");
    check_name("Carl");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[should_panic]
    #[test]
    fn test_main() {
        main();
    }
}
