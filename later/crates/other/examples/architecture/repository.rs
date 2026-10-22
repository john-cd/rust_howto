//! Example usage of the `InMemoryUserRepository`.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

/// Define a simple data model.
/// Represents a user with an ID, name, and email.
#[derive(Debug, Clone, PartialEq, Eq)]
struct User {
    /// The unique identifier for the user.
    id: i32,
    /// The name of the user.
    name: String,
    /// The email address of the user.
    email: String,
}

/// Define the `UserRepository` trait, the contract for a user repository.
/// This trait outlines the operations that can be performed on a collection of
/// users, such as retrieving, adding, updating, deleting, and listing users.
trait UserRepository {
    /// Returns the user with the given identifier, if any.
    fn get_user(&self, id: i32) -> Option<User>;

    /// Adds a user.
    fn add_user(&self, user: User);

    /// Replaces the user that has the same identifier.
    fn update_user(&self, user: User);

    /// Deletes the user with the given identifier.
    fn delete_user(&self, id: i32);

    /// Returns all users.
    fn get_all_users(&self) -> Vec<User>;
}

/// An in-memory implementation of the `UserRepository` trait.
///
/// This struct uses a `HashMap` to store users and `Arc<Mutex<>>` to allow for
/// concurrent access and modification.
struct InMemoryUserRepository {
    /// The users, by identifier.
    users: Arc<Mutex<HashMap<i32, User>>>,
}

impl InMemoryUserRepository {
    /// Creates an empty repository.
    fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

/// Implementation of the `UserRepository` trait for `InMemoryUserRepository`.
impl UserRepository for InMemoryUserRepository {
    fn get_user(&self, id: i32) -> Option<User> {
        let users = self.users.lock().unwrap();
        users.get(&id).cloned()
    }

    fn add_user(&self, user: User) {
        let mut users = self.users.lock().unwrap();
        users.insert(user.id, user);
    }

    fn update_user(&self, user: User) {
        let mut users = self.users.lock().unwrap();
        users.insert(user.id, user);
    }

    fn delete_user(&self, id: i32) {
        let mut users = self.users.lock().unwrap();
        users.remove(&id);
    }

    fn get_all_users(&self) -> Vec<User> {
        let users = self.users.lock().unwrap();
        users.values().cloned().collect()
    }
}

/// Example usage of the `InMemoryUserRepository`.
///
/// This function demonstrates how to create a repository, add users, retrieve
/// users, update users, delete users, and list all users.
fn main() {
    let repository: Arc<Mutex<dyn UserRepository + Send>> =
        Arc::new(Mutex::new(InMemoryUserRepository::new()));

    let user1 = User {
        id: 1,
        name: "Alice".to_owned(),
        email: "alice@example.com".to_owned(),
    };
    let user2 = User {
        id: 2,
        name: "Bob".to_owned(),
        email: "bob@example.com".to_owned(),
    };

    {
        let repo = repository.lock().unwrap();
        repo.add_user(user1);
        repo.add_user(user2);
    }

    {
        let repo = repository.lock().unwrap();
        let retrieved_user = repo.get_user(1);
        println!("Retrieved user: {retrieved_user:?}");

        let all_users = repo.get_all_users();
        println!("All users: {all_users:?}");

        let updated_user = User {
            id: 1,
            name: "Alice Updated".to_owned(),
            email: "alice_updated@example.com".to_owned(),
        };
        repo.update_user(updated_user);

        let retrieved_updated_user = repo.get_user(1);
        println!("Retrieved updated user: {retrieved_updated_user:?}");

        repo.delete_user(2);

        let remaining_users = repo.get_all_users();
        println!("Remaining users: {remaining_users:?}");
    }
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
