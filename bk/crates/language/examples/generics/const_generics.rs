#![allow(dead_code)]
// ANCHOR: example
/// Struct with a const parameter.
///
/// Note the `const` keyword, the parameter name (here `N`),
/// which must be followed by a type declaration.
struct InnerArray<T, const N: usize>([T; N]);

fn main() {
    // Use a type with a const generic.
    // The `N` constant is 3, since the array is `[i32; 3]`.
    let _inner_array: InnerArray<i32, 3> = InnerArray([1, 2, 3]);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
