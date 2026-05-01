#![allow(dead_code)]
// ANCHOR: example
fn main() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("something else"),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
