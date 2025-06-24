#![allow(dead_code)]
// ANCHOR: example
/// Represents a user with their active status, username, and email.
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
}

// It is common to define a function or an associated function (see below) that
// initializes the struct.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // If function parameters or variables have the same name as the struct
        // fields, you can use a shorthand.
        username, // Instead of writing `username: username`.
        email,    // Same.
    }
}

// Implementation block for the struct.
impl User {
    // Associated function.
    // By convention, the constructor is called "new", but that is not required.
    fn new(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
        }
    }
}

fn main() {
    // We create an instance of the struct by calling a function:
    let user1: User = build_user("user@example.com".into(), "user".to_string());

    println!("user1: {:?}", user1);

    // Or with an associated function. Note the path syntax
    // `<Type>::<function>`.
    let user2: User = User::new("user@example.com".into(), "user".to_string());

    println!("user2: {:?}", user2);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
