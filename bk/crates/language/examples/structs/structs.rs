// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! The `main` function creates an instance of the `User` struct.
//! It then prints the struct to the console.

/// We first define the struct's fields (which can be of any type).
/// The `User` struct has four fields: `active`, `username`, `email`, and
/// `sign_in_count`.
#[derive(Debug)]
struct User {
    /// Whether the user is active.
    active: bool,
    /// The user name.
    username: String,
    /// The email address.
    email: String,
    /// The number of times the user signed in.
    sign_in_count: u64,
}
// The `#[derive(Debug)]` attribute is not required; it enables the use of
// `println!` below.

/// The `main` function creates an instance of the `User` struct.
/// It then prints the struct to the console.
fn main() {
    // Note that there is no `new` keyword or similar.
    // The order of fields doesn't need to match the definition.
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("{user1:?}");

    // Access fields with the `.` operator:
    let _ = user1.active;
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the structs example through its entry point.
    #[test]
    fn test_structs_example_entry_point() {
        main();
    }

    /// The fields hold the initial values.
    #[test]
    fn test_user_fields() {
        let user = User {
            active: true,
            username: "u".to_owned(),
            email: "e@x.com".to_owned(),
            sign_in_count: 1,
        };
        assert!(user.active);
        assert_eq!(user.sign_in_count, 1);
        assert_eq!(
            format!("{user:?}"),
            "User { active: true, username: \"u\", email: \"e@x.com\", sign_in_count: 1 }"
        );
    }
}
