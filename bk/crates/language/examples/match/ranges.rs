#![allow(dead_code)]
// ANCHOR: example
fn main() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        6..10 => println!("six through nine"),
        _ => println!("something else"),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
