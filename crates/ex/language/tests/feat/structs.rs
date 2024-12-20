#![allow(dead_code)]
// ANCHOR: example

// We first define the struct's fields (which can be of any type).
// The `derive` attribute is not required - it just enables `println!` below.
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // We create an instance of the struct.
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
