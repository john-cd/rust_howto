#![allow(dead_code)]
// ANCHOR: example
//! Examples using `for`.

fn main() {
    // Use a range to generate all numbers in sequence,
    // starting from one number and ending before another number.
    for number in 1..4 {
        println!("{number} ");
    }
    // a..=b can be used for a range that is inclusive on both ends.
    // We can also use `rev` for reverse enumeration.
    for number in (1..=3).rev() {
        println!("{number} ");
    }

    // Iterate over a collection, here an array.
    let a = [10, 20, 30];
    for element in a {
        println!("the value is: {element}");
    }

    // Pay attention to whether you're getting a reference (&T or &mut T)
    // or taking ownership (T) of the elements during iteration.
    // This impacts whether you can use the collection after the loop and
    // whether you can modify elements.
    //
    // By default, `for` will use the collection's implementation of
    // `std::iter::IntoIterator`, and call its `into_iter` function,
    // consuming the collection:
    let b = vec![42, 43];
    for element in b {
        // Implicit call to `into_iter()`.
        println!("the value is: {element}");
    }
    // ERROR: value borrowed here after move: println!("b: {b:?}");

    // We can instead iterate by immutable reference with `iter()`.
    // This is the most common way to iterate over a collection,
    // when you just want to read its elements without modifying them.
    let mut c = vec![10, 20, 30];
    for num in c.iter() {
        println!("Element: {num}"); // `num` is an immutable reference.
    }

    // If you need to modify the elements of a collection during iteration, you
    // use `iter_mut()`, which provides a mutable reference to each element.
    for num in c.iter_mut() {
        *num *= 2; // Dereference the mutable reference to modify the value
    }
    println!("Modified numbers: {c:?}");

    // Strings in Rust are UTF-8.
    let s = "Hello👋";
    // They can be iterated over Unicode scalar values (characters):
    for c in s.chars() {
        println!("Character: {c}");
    }
    // Or over the raw UTF-8 bytes of the string:
    for b in s.bytes() {
        println!("Byte: {b}");
    }

    // We can use `enumerate` to produce both the index and the value
    // while iterating.
    let fruits = ["apple", "banana", "cherry"];
    for (index, fruit) in fruits.iter().enumerate() {
        println!("Fruit at index {index}: {fruit}");
    }
    // Note that (index, value) above is a pattern that destructures elements as
    // you iterate.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
