// ANCHOR: example
fn main() {
    let x = 5; // Integer
    let y = x; // No MOVE

    println!("x = {}, y = {}", x, y); // OK
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
