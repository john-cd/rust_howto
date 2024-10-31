#![allow(dead_code)]

struct User {
    active: bool,
    username: String,
    email: String,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // instead of username: username - field init shorthand
        email,    // same
    }
}

fn main() {
    let user1 = build_user("<user@example.com>".into(), "user".to_string());

    // struct update
    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1 /* the remaining fields not explicitly set should have the
                 * same value as the fields in the given instance. */
    };
}

#[test]
fn test() {
    main();
}
