// TODO Reviewed ??-??-2026.

#![allow(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Creating a struct from another instance with the struct update syntax.

/// Represents a user with their active status and username.
#[derive(Debug)]
struct User {
    /// Whether the user is active.
    active: bool,
    /// Name of the user.
    username: String,
}

/// Creates a user from another one, overriding some fields.
fn main() {
    // Create a first instance of the struct:
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
    };

    // Create a second instance by updating it:
    let user2 = User {
        // Modify the `username` field only.
        username: String::from("Bob"),

        // `..` is used to "fill in the rest".
        // The remaining fields not explicitly set will have the same value
        // as the fields in the given instance.
        ..user1
    };
    println!("{user2:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the struct update example through its entry point.
    #[test]
    fn test_struct_update_example_entry_point() {
        main();
    }
}
