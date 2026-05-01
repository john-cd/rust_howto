#![allow(dead_code)]
#![allow(clippy::useless_vec)]
// ANCHOR: example
use std::cell::RefCell;

// Example uses of function-like macros:
fn examples() {
    // 1. They can be used as expressions.
    // The following defines a vector:
    let _x = vec![1, 2, 3];

    // 2. They can be used as statements.
    // Print a string:
    println!("Hello!");

    // 3. They can be used in a pattern.
    // Let's first write a simple macro-by-example for the next example.
    // It wraps an identifier in `Some()`.
    macro_rules! pat {
        ($i:ident) => {
            Some($i)
        };
    }
    // Destructure an `Option` with the macro above:
    if let pat!(x) = Some(1) {
        assert_eq!(x, 1);
    }
}

// 4. Function-like macros can be used in a type alias.
// Let's first write a macro-by-example that defines a tuple type:
macro_rules! Tuple {
    { $A:ty, $B:ty } => { ($A, $B) };
}

// Define a tuple type alias with the macro above:
type T2 = Tuple!(i32, i32);

/// Print the elements of the tuple type alias:
fn print_tuple(tupl: T2) {
    println!("{} {}", tupl.0, tupl.1);
}

/// 5. Function-like macros can be used in a declaration:
fn use_thread_local() {
    // `thread_local!` declares a new thread-local storage key:
    thread_local!(static FOO: RefCell<u32> = const { RefCell::new(1) });
}

// 6. They can be used as an associated item (here, a `const`).
// This macro creates a constant of a given type and value:
macro_rules! const_maker {
    ($t:ty, $v:tt) => {
        const CONST: $t = $v;
    };
}

#[allow(dead_code)]
trait T {
    const_maker! {i32, 7}
}

// 7. It is possible to call macros within a macro definition.
//
// When used, the outer macro `example` is expanded,
// then the inner macro `println` is expanded.
//
// This macro calls the `println!` macro:
macro_rules! _example {
    () => {
        println!("Macro call in a macro!")
    };
}

fn main() {
    examples();
    print_tuple((1, 3));
    use_thread_local();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
