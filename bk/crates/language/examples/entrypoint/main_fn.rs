#![allow(dead_code)]
// ANCHOR: example
// `main` takes no parameters and, in this case,
// implicitly returns the "unit type" (),
// similar to 'void' in C/C++/Java.
fn main() {
    println!("Hello, world!");
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
