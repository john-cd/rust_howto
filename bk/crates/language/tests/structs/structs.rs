#![allow(dead_code)]
// ANCHOR: example
/// We first define the struct's fields (which can be of any type).
/// The `User` struct has four fields: `active`, `username`, `email`, and
/// `sign_in_count`.
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// The `#[derive(Debug)]` attribute enables the use of `println!` below.

/// The `main` function creates an instance of the `User` struct.
/// It then prints the struct to the console.
fn main() {
    // Note that there is no `new` or similar.
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("{:?}", user1);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
