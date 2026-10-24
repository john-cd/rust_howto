// ANCHOR: example
//! Hides a hash map behind a user-directory API.

use std::collections::HashMap;

/// A small user directory facade over an internal hash map.
#[derive(Default)]
struct UserDirectory {
    users_by_name: HashMap<String, String>,
}

impl UserDirectory {
    /// Registers or replaces the email address for a user name.
    fn register(&mut self, name: impl Into<String>, email: impl Into<String>) {
        self.users_by_name.insert(name.into(), email.into());
    }

    /// Looks up a user's email address without exposing the map.
    fn email_for(&self, name: &str) -> Option<&str> {
        self.users_by_name.get(name).map(String::as_str)
    }
}

/// Registers and looks up a user through the facade.
fn main() -> anyhow::Result<()> {
    let mut directory = UserDirectory::default();
    directory.register("ada", "ada@example.test");
    let email = directory
        .email_for("ada")
        .ok_or_else(|| anyhow::anyhow!("the registered user was not found"))?;

    println!("Ada's email: {email}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the facade example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::UserDirectory;

    /// Verifies callers can register and look up a user through the facade.
    #[test]
    fn registration_is_visible_through_lookup() {
        let mut directory = UserDirectory::default();
        directory.register("ada", "ada@example.test");

        assert_eq!(directory.email_for("ada"), Some("ada@example.test"));
    }
}
