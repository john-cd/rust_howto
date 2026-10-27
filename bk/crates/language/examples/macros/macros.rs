#![allow(dead_code)]
#![allow(clippy::useless_vec)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Ways to use function-like macros: as expressions, statements, items and
//! types.

use std::cell::RefCell;

// Example uses of function-like macros:
/// Shows several uses of function-like macros.
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
/// Defines a tuple type of the two given types.
macro_rules! Tuple {
    { $A:ty, $B:ty } => { ($A, $B) };
}

// Define a tuple type alias with the macro above:
/// A tuple type created with a macro.
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
/// Defines a constant `CONST` of the given type and value.
macro_rules! const_maker {
    ($t:ty, $v:tt) => {
        /// Constant emitted by the macro with the caller-supplied type and
        /// value.
        const CONST: $t = $v;
    };
}

/// A trait whose associated constant is created by a macro.
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
/// Expands to a call to the `println!` macro.
macro_rules! _example {
    () => {
        println!("Macro call in a macro!")
    };
}

/// Calls the functions that use macros.
fn main() {
    examples();
    print_tuple((1, 3));
    use_thread_local();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the macros example through its entry point.
    #[test]
    fn test_macros_example_entry_point() {
        main();
    }
}
