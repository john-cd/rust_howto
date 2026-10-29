// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Creating instances of a struct.

/// Represents a user with their active status, username, and email.
#[derive(Debug)]
struct User {
    /// Whether the user is active.
    active: bool,
    /// Name of the user.
    username: String,
    /// Email address of the user.
    email: String,
}

// It is common to define a function or an associated function (see below) that
// initializes the struct.
/// Creates a user from an email address and a username.
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
    /// Creates an active user.
    fn new(email: String, username: String) -> Self {
        Self {
            active: true,
            username,
            email,
        }
    }
}

/// Creates users with a function and with an associated function.
fn main() {
    // We create an instance of the struct by calling a function:
    let user1: User = build_user("user@example.com".into(), "user".to_owned());

    println!("user1: {user1:?}");

    // Or with an associated function. Note the path syntax
    // `<Type>::<function>`.
    let user2: User = User::new("user@example.com".into(), "user".to_owned());

    println!("user2: {user2:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the struct init example through its entry point.
    #[test]
    fn test_struct_init_example_entry_point() {
        main();
    }
}
