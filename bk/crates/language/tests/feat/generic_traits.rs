// ANCHOR: example

/// A trait that can be implemented for different types.
trait Test<T> {
    /// A function that takes a generic type `T` as input.
    fn test(_t: T);
}

struct SomeStruct;

/// Implement the `Test` trait for a `struct`.
// Note that the `<>` brackets appear in two places.
impl<T> Test<T> for SomeStruct {
    fn test(_t: T) {
        println!("This is the test function.");
    }
}

fn main() {
    SomeStruct::test(1);
    SomeStruct::test(true);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
