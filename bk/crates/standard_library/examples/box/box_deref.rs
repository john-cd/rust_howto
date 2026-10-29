// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Dereferencing a `Box`.

/// Reads the value inside a `Box` with `*`.
fn main() {
    // Dereferencing with `*`:
    let boxed: Box<u8> = Box::new(1);
    // Retrieve its inner value.
    let val: u8 = *boxed;
    println!("{val}");

    // Dereferencing with `.`: `Box<T>` behaves as if it were `T`.
    let boxed = Box::new("example");
    // Equivalent to (*boxed.deref()).len():
    let val = boxed.len();
    println!("{val}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_deref_example_entry_point() {
        main();
    }

    #[test]
    fn test_explicit_and_auto_deref() {
        let boxed: Box<u8> = Box::new(1);
        assert_eq!(*boxed, 1);
        let boxed = Box::new("example");
        // Auto-deref lets us call `str` methods directly.
        assert_eq!(boxed.len(), 7);
        assert_eq!(boxed.to_uppercase(), "EXAMPLE");
    }

    #[test]
    fn test_deref_mut() {
        let mut boxed = Box::new(1_u8);
        *boxed += 1;
        assert_eq!(*boxed, 2);
    }
}
