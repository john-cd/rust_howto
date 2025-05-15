// ANCHOR: example
/// This function takes a generic type `T` that implements the `AsRef<str>`
/// trait. It converts the input `s` to a string slice (`&str`) using
/// `as_ref()`.
///
/// `s` is a value of type `T` that can be converted to a string slice.
fn print_length<T: AsRef<str>>(s: T) {
    // The first line of a function accepting an `AsRef` is usually a call to
    // `as_ref`.
    let s_ref: &str = s.as_ref();
    // Print the string slice and its length.
    println!("The length of '{}' is {}", s_ref, s_ref.len());
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
fn print_vec<T: AsRef<[i32]>>(input: T) {
    let input = input.as_ref(); // Convert to a slice.
    println!("{:?}", input);
}

// `Vec<T>` and `[T; N]` implement `AsRef<[T]>`.
// You can also pass a reference to a vector or array, thanks to a blanket
// `impl AsRef<U> for &T where T: AsRef<U>` in the standard library.
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

fn main() {
    string_slice();
    vec_example();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
