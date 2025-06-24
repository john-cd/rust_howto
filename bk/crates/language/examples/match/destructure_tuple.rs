#![allow(dead_code)]
// ANCHOR: example
fn main() {
    let pair = (0, -5);
    match pair {
        (0, y) => println!("First element is 0, y = {}.", y),
        (x, 0) => println!("Second element is 0, x = {}.", x),
        _ => println!("It doesn't matter what they are."),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
