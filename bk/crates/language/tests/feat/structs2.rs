#![allow(dead_code)]
// ANCHOR: example

/// Represents a user with their active status, username, and email.
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
}

// It is common to define a function (or an associated function, see below) that
// initializes the struct:
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, /* Field init shorthand, instead of writing `username:
                   * username` */
        email, // Same
    }
}

fn main() {
    // We create an instance of the struct:
    let user1: User = build_user("user@example.com".into(), "user".to_string());

    // Then update the struct.
    // .. is used to fill in the rest
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 /* The remaining fields not explicitly set will have the
                 * same value as the fields in the given instance. */
    };
    println!("{:?}", user2);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
