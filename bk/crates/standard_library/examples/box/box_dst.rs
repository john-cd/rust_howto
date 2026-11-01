// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Boxing dynamically sized types.
//!
//! Dynamically sized types (DSTs), such as `str`, slices and `dyn Trait`, have
//! no size known at compile time, so they must be behind a pointer. A `Box` is
//! the owning pointer for them.

/// Creates boxes of a slice and a string.
fn main() {
    // Creating a Box containing a slice.
    let boxed_slice: Box<[i32]> = Box::new([1, 2, 3]);

    println!("Boxed slice: {boxed_slice:?}");
    println!("Length of boxed slice: {}", boxed_slice.len()); // via `Deref`.

    // We can also convert a `Vec` into a `Box<[T]>`, consuming the `Vec` and
    // avoiding a copy. `Box<[T]>` may be more compact than `Vec` but is not
    // resizable.
    let my_vec = vec![4, 5, 6, 7];
    let boxed_from_vec: Box<[i32]> = my_vec.into_boxed_slice();
    println!("Boxed from Vec: {boxed_from_vec:?}");

    // Creating a Box containing a `str`.
    let boxed_str: Box<str> = "Hello, Rust!".to_owned().into_boxed_str();
    println!("Boxed string: {boxed_str}");

    // We can also create it directly from a `&str` literal (less common).
    let another_boxed_str: Box<str> = Box::from("Another string slice");
    println!("Another boxed string: {another_boxed_str}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_dst_example_entry_point() {
        main();
    }

    #[test]
    fn test_boxed_slice() {
        let boxed_slice: Box<[i32]> = Box::new([1, 2, 3]);
        assert_eq!(boxed_slice.len(), 3);
        assert_eq!(&*boxed_slice, &[1, 2, 3]);
    }

    #[test]
    fn test_vec_into_boxed_slice_and_back() {
        let boxed: Box<[i32]> = vec![4, 5, 6, 7].into_boxed_slice();
        assert_eq!(boxed.len(), 4);
        assert_eq!(boxed.into_vec(), vec![4, 5, 6, 7]);
    }

    #[test]
    fn test_boxed_str() {
        let boxed_str: Box<str> = "Hello, Rust!".to_owned().into_boxed_str();
        assert_eq!(&*boxed_str, "Hello, Rust!");
        let another: Box<str> = Box::from("Another string slice");
        assert_eq!(another.len(), 20);
        let empty: Box<str> = Box::from("");
        assert!(empty.is_empty());
    }
}
