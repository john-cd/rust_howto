#![allow(dead_code)]
// ANCHOR: example
mod my {
    use std::fmt::Display;

    pub trait VeryUsefulTrait {
        fn my_method(&self) -> String;
    }

    // Blanket implementation for ALL types that implement `Display`:
    impl<T: Display> VeryUsefulTrait for T {
        fn my_method(&self) -> String {
            format!("{}", self)
        }
    }
}

fn main() {
    // Don't forget to bring the trait in scope:
    use my::VeryUsefulTrait;

    let s = String::from("Hello, world!");

    // There is no need for an individual implementation
    // of `VeryUsefulTrait` for `String`, since it implements `Display`:
    println!("{}", s.my_method());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
