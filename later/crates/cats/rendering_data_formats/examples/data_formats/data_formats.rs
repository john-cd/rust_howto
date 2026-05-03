#![allow(dead_code)]
// ANCHOR: example
// COMING SOON
// ANCHOR_END: example

fn main() {}

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
// [write LATER](https://github.com/john-cd/rust_howto/issues/829)
