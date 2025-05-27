#![allow(dead_code)]
// ANCHOR: example
use std::collections::HashMap;
use std::fmt::Debug;

/// Function with a type parameter.
fn print_type<T>(t: T) {
    println!("{}", std::any::type_name_of_val(&t));
}

/// Type alias with a type parameter.
type StringMap<K> = HashMap<K, String>;

/// Struct with a type parameter.
struct MyStruct<T> {
    data: T,
}

/// Trait with a type parameter.
trait Processor<T> {
    fn process(&self, item: T) -> String;
}

/// Tuple struct with a type parameter.
struct TupleStruct<T>(T);

// Implementing the trait for the tuple struct.
impl<T: Debug> Processor<T> for TupleStruct<T> {
    fn process(&self, item: T) -> String {
        format!("Processed: {:?}", item)
    }
}

/// Const generic parameters allow items to be generic over constant values.
/// Note the `const` keyword and the type declaration.
struct InnerArray<T, const N: usize>([T; N]);

fn main() {
    // `T` is `&str`.
    print_type("hello");
    // `T` is `i32`.
    print_type(42);
    // `K` is `String`.
    let _map = StringMap::<String>::new();
    // `T` is `bool`.
    let s = MyStruct { data: false };
    print_type(s);
    // `T` is `f64`.
    let tuple = TupleStruct(10.0);
    println!("{}", tuple.process(10.0));
    // The array is `[i32; 3]`.
    let _inner_array: InnerArray<i32, 3> = InnerArray([1, 2, 3]);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
