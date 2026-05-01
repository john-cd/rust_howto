#![allow(dead_code)]
// ANCHOR: example
use std::cell::OnceCell;

fn main() {
    // Create a new OnceCell instance.
    let cell = OnceCell::new();
    // The cell is initially empty.
    assert!(cell.get().is_none());

    let value: &String = cell.get_or_init(|| "Hello, World!".to_string());
    println!("{value}");
    assert_eq!(value, "Hello, World!");
    assert!(cell.get().is_some());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
