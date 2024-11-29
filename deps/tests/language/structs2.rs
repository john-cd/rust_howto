#![allow(dead_code)]
// ANCHOR: example

struct User {
    active: bool,
    username: String,
    email: String,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // Instead of `username: username` - field init shorthand
        email,    // Same
    }
}

fn main() {
    let user1 = build_user("<user@example.com>".into(), "user".to_string());

    // Struct update
    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1 /* The remaining fields not explicitly set should have the
                 * same value as the fields in the given instance. */
    };
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
