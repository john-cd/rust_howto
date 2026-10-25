#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Concatenating identifiers in macros with `paste`.

use paste::paste;

/// Defines a family of value structs with identifiers joined by `paste!`.
macro_rules! create_structs {
    ($($name:ident),* $(,)?) => {
        paste! {
            $(
                struct [<$name Struct>] {
                    /// Value stored by the generated struct.
                    value: i32,
                }

                impl [<$name Struct>] {
                    /// Creates a struct containing `value`.
                    fn new(value: i32) -> Self {
                        Self { value }
                    }

                    /// Returns the value stored in this struct.
                    fn get_value(&self) -> i32 {
                        self.value
                    }
                }
            )*
        }
    };
}

// Create the structs FooStruct, BarStruct, and BazStruct.
create_structs!(Foo, Bar, Baz);

/// Demonstrates the generated structs and their associated methods.
fn main() {
    let x = FooStruct::new(10);
    let y = BarStruct::new(20);
    let z = BazStruct::new(30);

    println!("Foo: {}", x.get_value());
    println!("Bar: {}", y.get_value());
    println!("Baz: {}", z.get_value());
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that pasted identifiers can be used by the example.
    #[test]
    fn test_main() {
        main();
    }
}
