// ANCHOR: example
fn my_string() -> &'static str {
    // This string literal is stored directly in the binary, and therefore
    // has a `'static` lifetime.
    let s: &'static str = "I have a static lifetime.";
    s
}

fn main() {
    println!("{}", my_string());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
