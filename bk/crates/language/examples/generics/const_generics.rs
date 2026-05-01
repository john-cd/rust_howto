#![allow(dead_code)]
// ANCHOR: example
/// Struct with a const parameter.
///
/// Note the `const` keyword, the parameter name (here `N`),
/// which must be followed by a type declaration.
struct InnerArray<T, const N: usize>([T; N]);

impl<T: std::marker::Copy, const N: usize> InnerArray<T, N> {
    fn new(val: T) -> Self {
        InnerArray([val; N])
    }

    fn len(&self) -> usize {
        N // The size N is known at compile time.
    }
}

fn main() {
    // Use a type with a const generic.
    let array: InnerArray<i32, 3> = InnerArray::new(42);
    let len = array.len();
    println!("Array length: {len}");
    assert_eq!(len, 3);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
