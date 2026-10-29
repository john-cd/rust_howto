// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Generic type parameters on functions, structs, traits and type aliases.

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::RandomState;

/// Generic function with a type parameter `T` (within < >).
///
/// Here, the type parameter is used as a placeholder for the type of the `t`
/// function argument.
fn print_type<T>(t: T) {
    println!("{}", std::any::type_name_of_val(&t));
}

/// - There may be multiple type parameters.
/// - Type parameters can be used to define the types of function arguments; or
///   the return type; or within the function.
/// - They may be used one or more times.
fn func<K, V, R>(key: K, value: V) -> R {
    let _k: K = key;
    let _v: V = value;
    unimplemented!();
}

/// Generic function with a type parameter that has a trait bound.
///
/// The `T: Debug` constraint means that `T` must implement the `Debug` trait,
/// allowing us to format it using the `{:?}` syntax.
fn print_debug<T: Debug>(t: T) {
    println!("{t:?}");
}

// This could also be written with a `where` clause:
/// Prints a value with its `Debug` representation.
fn print_debug2<T>(t: T)
where
    T: Debug,
{
    println!("{t:?}");
}

/// Struct with a type parameter.
///
/// `MyStruct<T>` is a struct that contains a single field `data` of type `T`.
struct MyStruct<T> {
    /// The value held by the struct.
    data: T,
}

/// Type parameters can constrain generic types.
///
/// Here, `U`, the type parameter of the function, is passed to the
/// `MyStruct<T>` generic type to guarantee that `T` equals `U`.
fn func2<U>(value: MyStruct<U>) -> U {
    value.data
}

/// Type parameters can have default types.
///
/// Here, `K` is the type of keys, `V` is the type of values, and `S` is the
/// type of the hasher. `S` has a default type of `RandomState`, which is a
/// common hasher type in Rust. This means that if you don't specify a hasher
/// type when creating a `HashMap`, it will use `RandomState` by default.
#[allow(unused)]
struct MyHashMap<K, V, S = RandomState> {
    /// The key-value pairs.
    inner: Vec<(K, V)>,
    /// The hasher.
    hasher: S,
}

/// Generic implementation of a generic struct.
///
/// The type parameters `K`, `V`, and `S` are declared after the `impl` keyword,
/// indicating that this implementation is generic over these types.
impl<K, V, S> MyHashMap<K, V, S> {
    /// Creates a new `MyHashMap` with the specified hasher.
    fn new(hasher: S) -> Self {
        Self {
            inner: Vec::new(),
            hasher,
        }
    }
}

/// Trait with a type parameter.
trait Processor<T> {
    /// Trait type parameters can be used in associated functions, methods, and
    /// other items defined in traits.
    fn process(&self, item: T) -> String;
}

/// A type that implements [`Processor`] for any item.
struct S;

// Generic implementation of a generic trait for a struct.
//
// The type parameter `T` is declared after the `impl` keyword, indicating
// that this implementation is generic over `T`, and used to set the type
// parameter of the trait.
impl<T> Processor<T> for S {
    fn process(&self, _item: T) -> String {
        unimplemented!();
    }
}

/// Let's define a tuple struct with a type parameter.
///
/// This is a struct that contains two fields of type `T`.
struct TupleStruct<T>(T, T);

// Generic implementation of a generic trait for a generic struct.
impl<T> Processor<T> for TupleStruct<T> {
    fn process(&self, _item: T) -> String {
        unimplemented!();
    }
}

/// Type alias with a type parameter.
///
/// `StringMap<K>` is a type alias for `HashMap<K, String>`.
/// This allows you to create maps where the keys can be of any type, but the
/// values are always `String`.
type StringMap<K> = HashMap<K, String>;

/// Calls the generic functions and uses the generic types.
fn main() {
    // Use generic functions:
    //
    // The type of the generic parameter `T` is usually inferred by the compiler
    // from the type of the argument passed to the function. Here, `T` is
    // inferred to be `&str`:
    print_type("hello");
    // `T` is `i32`.
    print_type(42);
    // We can also specify the type explicitly using the `turbofish` notation
    // `::<...>`.
    print_type::<f64>(3.1);

    // Use generic types with trait bounds.
    // `T` is inferred to be `i32`, which implements the `Debug` trait:
    print_debug(42);
    // ERROR: print_debug(S); // `S` does not implement the `Debug` trait.

    // Use generic structs:
    // `T` is inferred to be `bool` from the value passed to the field.
    let s = MyStruct { data: false };
    print_type(s);

    // `T` is `f64`.
    let tuple = TupleStruct(10.0, 20.0);
    print_type(tuple);

    // Use a generic type alias:
    // The turbofish notation `::<String>` is used to specify the type parameter
    // explicitly, since it cannot be inferred from the context.
    let _map = StringMap::<String>::new();

    // We could also write:
    let _map2: StringMap<String> = StringMap::new();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the generic type parameter example through its entry point.
    #[test]
    fn test_generic_type_parameter_example_entry_point() {
        main();
    }

    /// `func2` unwraps the generic payload, whatever its type.
    #[test]
    fn test_func2() {
        assert_eq!(func2(MyStruct { data: 42 }), 42);
        assert_eq!(func2(MyStruct { data: "text" }), "text");
        assert!(!func2(MyStruct { data: false }));
    }

    /// The generic hash map wrapper starts empty.
    #[test]
    fn test_my_hash_map_new() {
        let map: MyHashMap<String, i32, ()> = MyHashMap::new(());
        assert!(map.inner.is_empty());
    }

    /// The type alias fixes the value type to `String`.
    #[test]
    fn test_string_map_alias() {
        let mut map = StringMap::<u8>::new();
        map.insert(1, "one".to_owned());
        assert_eq!(map.get(&1).map(String::as_str), Some("one"));
        assert_eq!(map.get(&2), None);
    }

    /// The `Debug` helpers accept anything `Debug`.
    #[test]
    fn test_print_helpers() {
        print_debug(vec![1, 2]);
        print_debug2("x");
        print_type(1.5_f32);
    }
}
