// ANCHOR: example
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // `clone` deeply copies the heap data of the `String`,
    // not just the stack data.
    println!("{s2}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
