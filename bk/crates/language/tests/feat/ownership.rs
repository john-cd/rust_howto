// ANCHOR: example
fn main() {
    // Strings have move semantics.
    let s1 = String::from("hello");
    // s1 is MOVED into s2 - this is NOT a shallow copy
    let s2 = s1;
    println!("{}, world!", s2);
    // ...but Rust has invalidated s1
    // ERROR: println!("{}, world!", s1);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
