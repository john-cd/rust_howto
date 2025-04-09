// ANCHOR: example
/// This function takes a generic type `T` that implements the `AsRef<str>`
/// trait. It converts the input `s` to a string slice (`&str`) using
/// `as_ref()`.
///
/// # Arguments
///
/// * `s` - A value of type `T` that can be converted to a string slice.
fn print_length<T: AsRef<str>>(s: T) {
    let s_ref: &str = s.as_ref();
    // Print the string slice and its length.
    println!("The length of '{}' is {}", s_ref, s_ref.len());
}

fn main() {
    let string = String::from("Hello, world!");
    let str_slice = "Hello, Rust!";

    // Using `print_length` with a `String`.
    print_length(string);

    // Using `print_length` with a `&str`.
    print_length(str_slice);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
