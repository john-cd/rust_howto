//! Mocking a trait with `mockall`.

#![allow(dead_code)]

/// Runs the example.
pub fn run() {
    main();
}

// TODO Reviewed ??-??-2026

// ANCHOR: example
// This example demonstrates how to use the `mockall` crate for mocking.
//
// First, add `mockall` to your `Cargo.toml` dependencies:
// ```toml
// [dependencies]
// mockall = "0.11.3"
// ```
//
// Then define a `UserRepository` trait and a `UserService` struct that uses
// it. We then use `mockall` to create a mock implementation of
// `UserRepository` for testing `UserService`.

use mockall::predicate::*;
use mockall::*;

/// Define our data model.
#[derive(Debug, Clone, PartialEq)]
struct User {
    /// Unique identifier of the user.
    id: u64,
    /// Name of the user.
    name: String,
    /// Email address of the user.
    email: String,
}

/// The `UserRepository` trait defines the interface for interacting with user
/// data. The `#[automock]` attribute automatically generates a mock
/// implementation of your trait, named Mock{TraitName}. In our case, it creates
/// a `MockUserRepository`.
#[automock]
trait UserRepository {
    /// Finds the user with the given `id`.
    fn find_by_id(&self, id: u64) -> Option<User>;

    /// Saves the `user`; returns an error message on failure.
    fn save(&self, user: User) -> Result<(), String>;
}

/// The service that uses the repository.
struct UserService<T: UserRepository> {
    /// The repository used to load and store users.
    repository: T,
}

impl<T: UserRepository> UserService<T> {
    /// Creates a service backed by `repository`.
    fn new(repository: T) -> Self {
        Self { repository }
    }

    /// Retrieves a user by their ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the user to retrieve.
    fn get_user(&self, id: u64) -> Result<User, String> {
        self.repository
            .find_by_id(id)
            .ok_or_else(|| format!("User with id {id} not found"))
    }

    /// Updates a user's email.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the user to update.
    /// * `new_email` - The new email address.
    fn update_user_email(
        &self,
        id: u64,
        new_email: String,
    ) -> Result<User, String> {
        // Get the user:
        let mut user = self.get_user(id)?;

        // Update the email:
        user.email = new_email;

        // Save the user:
        self.repository.save(user.clone())?;

        Ok(user)
    }
}

/// A stand-in for a real repository.
struct ActualRepository;

impl UserRepository for ActualRepository {
    fn find_by_id(&self, id: u64) -> Option<User> {
        // In a real implementation, this would query a database. For this
        // example, we'll just return a dummy user if the ID is 1.
        (id == 1).then(|| User {
            id,
            name: "John Doe".to_owned(),
            email: "john@example.com".to_owned(),
        })
    }

    fn save(&self, user: User) -> Result<(), String> {
        println!("Persisted user: {user:?}");
        Ok(())
    }
}

/// Updates a user's email through the service.
fn main() {
    let service = UserService::new(ActualRepository);
    let updated = service
        .update_user_email(1, "updated@example.com".to_owned())
        .expect("Repository should return a user");

    println!("Updated user from main(): {updated:?}");
}

/// The `tests` module contains unit tests for `UserService` using the mock
/// repository.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that `get_user` returns the user found by the repository.
    #[test]
    fn test_get_user_success() {
        let mut mock_repo = MockUserRepository::new();

        // Set up expectations for `find_by_id`.
        // We expect `find_by_id` to be called once with id 1:
        mock_repo
            .expect_find_by_id() // The method we're setting expectations for
            .with(eq(1)) // The argument we expect (using predicate). You can also use custom
            // functions: .withf(|user| user.id == 1)
            .times(1) // How many times we expect it to be called
            .returning(|_| {
                // What it should return
                Some(User {
                    id: 1,
                    name: "John Doe".to_owned(),
                    email: "john@example.com".to_owned(),
                })
            });

        // Create our service with the mock repository:
        let service = UserService::new(mock_repo);

        // Call the method we want to test:
        let result = service.get_user(1);

        // Assert the result:
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
    }

    /// Verifies that `get_user` returns an error when the repository finds no
    /// user.
    #[test]
    fn test_get_user_not_found() {
        let mut mock_repo = MockUserRepository::new();

        // Set up expectations:
        // We expect `find_by_id` to be called once with id 999 and return
        // `None`:
        mock_repo
            .expect_find_by_id()
            .with(eq(999))
            .times(1)
            .returning(|_| None);

        // Create our service with the mock repository:
        let service = UserService::new(mock_repo);

        // Call the method we want to test:
        let result = service.get_user(999);

        // Assert the result:
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "User with id 999 not found");
    }

    /// Verifies that `update_user_email` saves the new address to the
    /// repository.
    #[test]
    fn test_update_user_email() {
        let mut mock_repo = MockUserRepository::new();

        // Set up expectations for find_by_id, we expect it to be called once
        // with id 1.
        mock_repo
            .expect_find_by_id()
            .with(eq(1))
            .times(1)
            .returning(|_| {
                Some(User {
                    id: 1,
                    name: "John Doe".to_owned(),
                    email: "john@example.com".to_owned(),
                })
            });

        // Set up expectations for save, we expect it to be called once with a
        // user with id 1, name "John Doe" and email
        // "newemail@example.com".
        mock_repo
            .expect_save()
            .withf(|user: &User| {
                user.id == 1
                    && user.name == "John Doe"
                    && user.email == "newemail@example.com"
            })
            .times(1)
            .returning(|_| Ok(()));

        // Create our service with the mock repository.
        let service = UserService::new(mock_repo);

        // Call the method we want to test.
        let result =
            service.update_user_email(1, "newemail@example.com".to_owned());

        // Assert the result.
        assert!(result.is_ok());
        let updated_user = result.unwrap();
        assert_eq!(updated_user.id, 1);
        assert_eq!(updated_user.name, "John Doe");
        assert_eq!(updated_user.email, "newemail@example.com");
    }
}
// ANCHOR_END: example
