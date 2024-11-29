// ANCHOR: example
fn main() {
    let x = 5; // x is immutable

    let x = x + 1; // Redefines x
    println!("{x}");

    let x = "example"; // The type can change
    println!("{x}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
