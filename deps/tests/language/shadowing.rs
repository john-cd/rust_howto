// ANCHOR: example
fn main() {
    // `x`` is an immutable variable
    let x = 5;
    // ERROR: x = x +1;

    // But it can be redefined:
    let x = x + 1;
    println!("{x}");

    // The type can change
    let x = "example";
    println!("{x}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
