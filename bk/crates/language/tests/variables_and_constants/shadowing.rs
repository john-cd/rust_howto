// ANCHOR: example
fn main() {
    // `x` is an immutable variable.
    let x = 5;
    // ERROR: x = x +1; // This would be an error because `x` is immutable.

    // But it can be redefined (shadowed). Notice the `let`.
    let x = x + 1;
    // The new `x` is a new variable that shadows the previous one.
    println!("{x}");

    // The type can change as well.
    let x = "example";
    println!("{x}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
