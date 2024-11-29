#![allow(dead_code)]
// ANCHOR: example

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // Create an instance
    let _user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
