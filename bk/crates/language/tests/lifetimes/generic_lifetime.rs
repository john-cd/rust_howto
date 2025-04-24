// ANCHOR: example
/// This function takes two string slices, `x` and `y`, both with the same
/// lifetime `'a`. It returns a string slice that is the longer of the two, also
/// with the lifetime `'a`.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let (x, y) = ("short", "looooooong");
    println!("{}", longest(x, y));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
