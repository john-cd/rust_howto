#![allow(dead_code)]
// ANCHOR: example
//! `faux` mocks the methods of structs for testing without
//! complicating or polluting our code.

/// Represents a user with an ID and a name.
#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u64,
    name: String,
}

/// A trait representing a database that can retrieve and save user data.
trait Database {
    /// Retrieves a user by their ID.
    ///
    /// Returns `Some(User)` if a user with the given ID exists, otherwise
    /// `None`.
    fn get_user(&self, id: u64) -> Option<User>;
    /// Saves a user to the database.
    ///
    /// Returns `true` if the save operation was successful, `false` otherwise.
    fn save_user(&self, user: &User) -> bool;
}

/// A service that interacts with a `Database` to manage user data.
struct UserService<D: Database> {
    /// The database implementation used by this service.
    database: D,
}

/// Implementation of `UserService` that uses a generic `Database`
/// implementation.
impl<D: Database> UserService<D> {
    /// Creates a new `UserService` with the given database.
    fn new(database: D) -> Self {
        Self { database }
    }

    fn get_user_name(&self, id: u64) -> Option<String> {
        self.database.get_user(id).map(|user| user.name)
    }

    fn update_user_name(&self, id: u64, name: String) -> bool {
        if let Some(mut user) = self.database.get_user(id) {
            user.name = name;
            self.database.save_user(&user)
        } else {
            false
        }
    }
}

/// A mock implementation of the `Database` trait.
#[faux::create]
struct MockDatabase {}

/// Implementation of the `Database` trait for `MockDatabase`.
#[faux::methods]
impl Database for MockDatabase {
    /// Mock implementation of `get_user`.
    /// This method will be mocked during testing.
    fn get_user(&self, _id: u64) -> Option<User> {
        unimplemented!()
    }

    fn save_user(&self, _user: &User) -> bool {
        // This will be mocked as well.
        unimplemented!()
    }
}

/// Test module for `UserService` using `MockDatabase`.
#[cfg(test)]
mod tests {
    use faux::when;

    use super::*;

    /// Tests the `get_user_name` method of `UserService`.
    #[test]
    fn test_get_user_name() {
        // Create a mock.
        let mut mock_db = MockDatabase::faux();

        // Define the behavior.
        when!(mock_db.get_user).then(|id| {
            if id == 1 {
                Some(User {
                    id: 1,
                    name: "Alice".to_string(),
                })
            } else {
                None
            }
        });

        // Create a service with the mock DB.
        let service = UserService::new(mock_db);

        // Test the service.
        assert_eq!(service.get_user_name(1), Some("Alice".to_string()));
        assert_eq!(service.get_user_name(2), None);
    }

    /// Tests the `update_user_name` method of `UserService`.
    #[test]
    fn test_update_user_name() {
        // Create a mock DB.
        let mut mock_db = MockDatabase::faux();
        let user = User {
            id: 1,
            name: "Alice".to_string(),
        };

        // Configure `get_user` to return our test user.
        when!(mock_db.get_user)
            .then(move |id| if id == 1 { Some(user.clone()) } else { None });

        // Configure `save_user` to always succeed.
        when!(mock_db.save_user).then(|_| true);

        // Create a service with a mock DB.
        let service = UserService::new(mock_db);

        // Test that `update_user_name` succeeds.
        assert!(service.update_user_name(1, "Bob".to_string()));
        // Test that an update fails for a non-existent user.
        assert!(!service.update_user_name(2, "Charlie".to_string()));
    }
}
// ANCHOR_END: example
