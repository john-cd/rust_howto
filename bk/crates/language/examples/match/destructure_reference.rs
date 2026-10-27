#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Destructuring references.

/// Destructures references with `&` and `ref`.
fn main() {
    // Given a reference...
    let reference: &i32 = &42;

    // ...we can destructure it into its pointed-to type.
    // Note that `&` is on the left side.
    let &value = reference;

    // `value` is `i32`:
    println!("Type of `value`: {}", std::any::type_name_of_val(&value));
    assert_eq!(value, 42);

    // This is equivalent to dereferencing (following the pointer):
    let _value: i32 = *reference;

    // Note that it won't work if the underlying type is not `Copy`:
    /// A value type that does not implement `Copy`.
    struct NotCopyStruct;
    let _reference = &NotCopyStruct;
    // let &value = _reference; // Error[E0507]: cannot move out of `*reference`
    // which is behind a shared reference.
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the destructure reference example through its entry point.
    #[test]
    fn test_destructure_reference_example_entry_point() {
        main();
    }
}
