// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Mocking a struct's methods with `faux`.
//!
//! A mock replaces a real dependency (a database or a web service) in a test,
//! with controlled answers, so the test is fast and deterministic. `faux` mocks
//! the methods of a struct without requiring a trait.

/// Runs the example.
// `faux` mocks the methods of structs for testing without
// complicating or polluting our code.
/// Represents a user with an ID and a name.
#[derive(Debug, Clone, PartialEq)]
struct User {
    /// Unique identifier of the user.
    id: u64,
    /// Name of the user.
    name: String,
}

/// A trait representing a database that can retrieve and save user data.
trait Database {
    /// Retrieves a user by their ID.
    ///
    /// Returns `Some(User)` if a user with the given ID exists, otherwise
    /// `None`.
    fn get_user(&self, id: u64) -> Option<User>;
    /// Returns `true` if the save operation was successful, `false` otherwise.
    ///
    /// # Returns
    /// `true` when the condition described above holds, and `false` otherwise.
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

    /// Returns the name of the user `id`, if it exists.
    fn get_user_name(&self, id: u64) -> Option<String> {
        self.database.get_user(id).map(|user| user.name)
    }

    /// Renames the user `id`; returns whether the update succeeded.
    ///
    /// # Arguments
    /// * `id` - Identifier used to select the user service or related record being processed.
    /// * `name` - Human-readable value used to identify or display the selected user service.
    ///
    /// # Returns
    /// `true` when the condition described above holds, and `false` otherwise.
    fn update_user_name(&self, id: u64, name: String) -> bool {
        if let Some(mut user) = self.database.get_user(id) {
            user.name = name;
            self.database.save_user(&user)
        } else {
            false
        }
    }
}

/// A stand-in for a real database.
struct ActualDatabase; // Well, sort of...

impl Database for ActualDatabase {
    /// # Arguments
    /// * `id` - Identifier used to select the actual database record or cache entry affected by
    ///   this operation.
    fn get_user(&self, id: u64) -> Option<User> {
        (id == 1).then(|| User {
            id,
            name: "Alice".to_owned(),
        })
    }

    /// Persists the user produced by the faux workflow so later operations can retrieve it.
    fn save_user(&self, user: &User) -> bool {
        println!("Saved user: {user:?}");
        true
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
        None
    }

    /// Persists the user produced by the faux workflow so later operations can retrieve it.
    fn save_user(&self, _user: &User) -> bool {
        // This will be mocked as well.
        false
    }
}
/// Looks up and renames a user through the service.
fn main() {
    let service = UserService::new(ActualDatabase);
    let original_name = service
        .get_user_name(1)
        .expect("demo database should return a user");
    let updated = service.update_user_name(1, "Bob".to_owned());

    println!("Original user name: {original_name}");
    println!("Update succeeded: {updated}");
}
// ANCHOR_END: example

/// Mocking a struct's methods with `faux`.
pub fn run() {
    main();
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
            (id == 1).then(|| User {
                id: 1,
                name: "Alice".to_owned(),
            })
        });

        // Create a service with the mock DB.
        let service = UserService::new(mock_db);

        // Test the service.
        assert_eq!(service.get_user_name(1), Some("Alice".to_owned()));
        assert_eq!(service.get_user_name(2), None);
    }

    /// Tests the `update_user_name` method of `UserService`.
    #[test]
    fn test_update_user_name() {
        // Create a mock DB.
        let mut mock_db = MockDatabase::faux();
        let user = User {
            id: 1,
            name: "Alice".to_owned(),
        };

        // Configure `get_user` to return our test user.
        when!(mock_db.get_user).then(move |id| (id == 1).then(|| user.clone()));

        // Configure `save_user` to always succeed.
        when!(mock_db.save_user).then(|_| true);

        // Create a service with a mock DB.
        let service = UserService::new(mock_db);

        // Test that `update_user_name` succeeds.
        assert!(service.update_user_name(1, "Bob".to_owned()));
        // Test that an update fails for a non-existent user.
        assert!(!service.update_user_name(2, "Charlie".to_owned()));
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
