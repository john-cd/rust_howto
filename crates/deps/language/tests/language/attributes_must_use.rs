// ANCHOR: example
// Must use the results of the fn
// Also applies to traits, structs, enums...
#[must_use]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", add(1, 2));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
