#![allow(dead_code)]
// ANCHOR: example
use std::boxed::Box;

/// Demonstrates the difference between taking a reference to a `Box` and using
/// `as_ref()`.
fn main() {
    let my_box: Box<String> = Box::new("hello from box".to_string());

    // 1. Using `&my_box` - reference *to the Box*
    #[allow(clippy::borrowed_box)]
    let ref_to_box: &Box<String> = &my_box;
    println!("Debug of `ref_to_box`: {ref_to_box:?}");
    // To access the inner string through `ref_to_box`, you'd need to
    // dereference twice:
    println!("Content via deref on `ref_to_box`: {}", **ref_to_box);

    // 2. Using `my_box.as_ref()` - reference *to the contained value*
    let ref_to_contained_string: &String = my_box.as_ref();
    println!("Content via `as_ref()`: {ref_to_contained_string}");
    println!(
        "Content as `&str` via `Deref`: {}",
        &**ref_to_contained_string
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
