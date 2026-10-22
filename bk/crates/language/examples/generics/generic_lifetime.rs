#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! A struct that holds a reference of any type with a lifetime parameter.

/// Struct with a type parameter `T` and a lifetime parameter `'a`.
///
/// The `T: ?Sized` trait bound indicates that `T` can be dynamically sized,
/// which will be useful in the example below.
struct DataHolder<'a, T: ?Sized> {
    /// The borrowed data.
    data: &'a T, /* The lifetime parameter is used to specify the lifetime
                  * of the reference. */
}

/// Method implementation for the struct.
/// Note that the lifetime `'a` (declared after `impl`) is used in the struct
/// and in the method signature to ensure that the returned reference has the
/// same lifetime as the struct.
impl<'a, T> DataHolder<'a, T> {
    /// Returns the borrowed data.
    fn get_data(&self) -> &'a T {
        self.data
    }
}

/// Creates a holder around a reference and reads it back.
fn main() {
    // `T` is `u8`.
    let data = DataHolder { data: &10u8 };
    println!("Data: {:?}", data.get_data());

    // The lifetime parameter is most often elided, but can be specified:
    let literal = "This string literal is of type &'static str";
    let _data: DataHolder<'static, str> = DataHolder { data: literal };
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
