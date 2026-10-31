// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Accepting several types through `AsRef`.
//!
//! `AsRef<T>` is a cheap conversion to a reference. A function that takes `impl
//! AsRef<str>` accepts `&str`, `String` and more, without the caller converting
//! first; `AsRef<Path>` is why file functions take strings and paths alike.

/// This function takes a generic type `T` that implements the `AsRef<str>`
/// trait. It converts the input `s` to a string slice (`&str`) using
/// `as_ref()`.
///
/// `s` is a value of type `T` that can be converted to a string slice.
fn print_length<T: AsRef<str>>(s: T) -> usize {
    // The first line of a function accepting an `AsRef` is usually a call to
    // `as_ref`.
    let s_ref: &str = s.as_ref();
    // Print the string slice and its length.
    println!("The length of '{s_ref}' is {}", s_ref.len());
    s_ref.len()
}

/// Let's call `print_length` with arguments of various types.
fn string_slice() {
    let string = String::from("Hello, world!");
    let str_slice: &str = "Hello, Rust!";

    // Using `print_length` with a `String`, which implements `AsRef<str>`.
    print_length(string);

    // Using `print_length` with a `&str`.
    print_length(str_slice);
}

/// `AsRef<[T]>` can also be used with slices, arrays, and vectors,
///  although it is more common to use `&[T]` than `AsRef<T>`.
fn print_vec<T: AsRef<[i32]>>(input: T) -> usize {
    let input = input.as_ref(); // Convert to a slice.
    println!("{input:?}");
    input.len()
}

// `Vec<T>` and `[T; N]` implement `AsRef<[T]>`.
// We can also pass a reference to a vector or array, thanks to a blanket
// `impl AsRef<U> for &T where T: AsRef<U>` in the standard library.
/// Passes a vector and an array to a function that takes `AsRef<[i32]>`.
fn vec_example() {
    let vec: Vec<i32> = vec![1, 2, 3];
    print_vec(&vec);
    print_vec(vec);
    let arr: [i32; 3] = [1, 2, 3];
    print_vec(arr);
    let arr_ref: &[i32; 3] = &[1, 2, 3];
    print_vec(arr_ref);
    let sli: &[i32] = &[1, 2, 3];
    print_vec(sli);
}

/// Runs both examples.
fn main() {
    string_slice();
    vec_example();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asref_example_entry_point() {
        main();
    }

    #[test]
    fn test_print_length_accepts_string_and_str() {
        assert_eq!(print_length(String::from("Hello, world!")), 13);
        assert_eq!(print_length("Hello, Rust!"), 12);
        assert_eq!(print_length(""), 0);
    }

    #[test]
    fn test_print_vec_accepts_vec_array_and_slice() {
        let vec = vec![1, 2, 3];
        assert_eq!(print_vec(&vec), 3);
        assert_eq!(print_vec(vec), 3);
        assert_eq!(print_vec([1, 2, 3]), 3);
        let slice: &[i32] = &[];
        assert_eq!(print_vec(slice), 0);
    }
}
