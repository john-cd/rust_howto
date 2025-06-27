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
        format!("Processed: {item:?}")
    }
}

fn main() {
    // Use generic functions:
    // `T` is `&str`.
    print_type("hello");
    // `T` is `i32`.
    print_type(42);

    // Use a generic type alias:
    // `K` is `String`.
    let _map = StringMap::<String>::new();
    // Note the `turbofish` notation `::<...>` to specify the type parameter
    // when it cannot be inferred. You can also write:
    let _map2: StringMap<String> = StringMap::new();

    // Use generic structs:
    // `T` is inferred to be `bool`.
    let s = MyStruct { data: false };
    print_type(s);
    // `T` is `f64`.
    let tuple = TupleStruct(10.0);
    println!("{}", tuple.process(10.0));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
