// ANCHOR: example
fn main() {

    let x = 5; // x is immutable

    let x = x + 1; // redefines x
    println!("{x}");

    let x = "example"; // the type can change
    println!("{x}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
