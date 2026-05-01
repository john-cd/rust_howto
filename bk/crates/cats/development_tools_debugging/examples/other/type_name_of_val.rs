#![allow(dead_code)]
// ANCHOR: example
fn get_iter() -> impl Iterator<Item = i32> {
    [1, 2, 3].into_iter()
}

fn main() {
    let iter = get_iter();
    // NEW in Rust 1.76
    // Returns the type name of the pointed-to value as a string slice.
    let iter_name = std::any::type_name_of_val(&iter);
    let sum: i32 = iter.sum();
    println!("The sum of the `{iter_name}` is {sum}.");
    // prints: The sum of the `core::array::iter::IntoIter<i32, 3>` is 6.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
