// ANCHOR: example
//! Example of using the `println!` macro.

fn main() {
    let x = 5;
    let y = 10;
    // { } are placeholders.
    println!("x = {x} and y + 2 = {}", y + 2);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
