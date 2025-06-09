// ANCHOR: example
/// A trait that can be implemented for different types.
/// `T` is a generic type parameter:
trait Test<T> {
    /// A function that takes a value of generic type `T` as its input.
    fn test(_t: T);
}

struct SomeStruct;

/// Generic implementation of the `Test` trait for a `struct`.
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
