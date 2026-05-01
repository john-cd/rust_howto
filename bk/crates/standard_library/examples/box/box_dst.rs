#![allow(dead_code)]
// ANCHOR: example
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
    let boxed_str: Box<str> = "Hello, Rust!".to_string().into_boxed_str();
    println!("Boxed string: {boxed_str}");

    // We can also create it directly from a `&str` literal (less common).
    let another_boxed_str: Box<str> = Box::from("Another string slice");
    println!("Another boxed string: {another_boxed_str}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
