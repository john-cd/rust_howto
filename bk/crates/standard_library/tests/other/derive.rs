// ANCHOR: example
/// `derive` is a powerful tool in Rust that allows you to automatically
/// implement certain traits for your structs and enums.
///
/// Here we derive several common traits for a simple struct `S` that wraps an
/// `i32`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
struct S(i32);

fn main() {
    println!("{:?}", S(0)); // Courtesy of `Debug`.
    println!("{}", S(1) == S(1)); // Courtesy of `PartialEq`, `Eq`.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
