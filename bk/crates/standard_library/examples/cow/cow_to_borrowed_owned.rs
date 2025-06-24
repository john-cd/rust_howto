#![allow(dead_code)]
// ANCHOR: example
use std::borrow::Borrow;
use std::borrow::Cow;
use std::ops::Deref;

/// Demonstrates various uses and conversions of `Cow` (Clone-on-Write).
fn main() {
    let mut my_string = String::new();

    // Create a `Cow` from the string literal "Example ".
    let example: Cow<str> = Cow::from("Example ");

    // Append the `Cow` to a `String` (conversion to a `&str`).
    my_string.push_str(&example);
    println!("{}", my_string);
    // Alternatively, you could use `borrow`, `asref` or `deref`:
    println!("{}", my_string);
    my_string.push_str(example.borrow());
    println!("{}", my_string);
    my_string.push_str(example.as_ref());
    println!("{}", my_string);
    my_string.push_str(example.deref());
    println!("{}", my_string);
    // Unstable feature: my_string.push_str(&example.as_str());

    // Convert a `Cow<str>` to an owned `String`:
    let s: String = example.to_string();
    println!("{}", s);
    // You could also convert the `Cow` to a borrowed reference using
    // `as_ref()`, then clone the borrowed reference to create an owned
    // `String`.
    println!("{}", example.as_ref().to_owned());
    // `into_owned` extracts the owned data and clones the data if it is not
    // already owned.
    println!("{}", example.into_owned());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
