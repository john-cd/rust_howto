#![allow(dead_code)]
// ANCHOR: example
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
    inner: Vec<(K, V)>,
    hasher: S,
}

/// Generic implementation of a generic struct.
///
/// The type parameters `K`, `V`, and `S` are declared after the `impl` keyword,
/// indicating that this implementation is generic over these types.
impl<K, V, S> MyHashMap<K, V, S> {
    /// Creates a new `MyHashMap` with the specified hasher.
    fn new(hasher: S) -> Self {
        MyHashMap {
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

fn main() {
    // Use generic functions:
    //
    // The type of the generic parameter `T` is usually inferred by the compiler
    // from the type of the argument passed to the function. Here, `T` is
    // inferred to be `&str`:
    print_type("hello");
    // `T` is `i32`.
    print_type(42);
    // You could specify the type explicitly using the `turbofish` notation
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

    // You could also write:
    let _map2: StringMap<String> = StringMap::new();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
