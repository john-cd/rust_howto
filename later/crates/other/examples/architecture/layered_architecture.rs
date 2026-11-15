// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! This example demonstrates a layered architecture in Rust,
//! separating concerns into data access, business logic, and presentation
//! layers.

use std::sync::Arc;

use anyhow::Result;

/// Account record used by the layered architecture example.
///
/// Represents a user in the system.
#[derive(Debug, Clone)]
pub struct User {

    /// The identifier of the user.
    pub id: u32,

    /// The name of the user.
    pub name: String,
}

// --- Persistence / Data Access Layer ---

/// The data access layer.
mod data {
    use std::collections::HashMap;

    use anyhow::Result;
    use async_trait::async_trait;
    use tokio::sync::Mutex;

    use super::User;

    /// Specifies the operations implementations provide to architecture callers.
    #[async_trait]
    pub trait UserRepository {

        /// Returns the user with the given identifier, if any.
        async fn get_user(&self, id: u32) -> Result<Option<User>>;

        /// Stores a new user.
        async fn create_user(&self, user: User) -> Result<()>;
    }

    /// An in-memory implementation of `UserRepository` for testing and
    /// demonstration purposes.
    pub struct InMemoryUserRepository {

        /// The users, by identifier.
        users: Mutex<HashMap<u32, User>>,
    }

    impl InMemoryUserRepository {

        /// Creates an empty repository.
        pub fn new() -> Self {
            Self {
                users: Mutex::new(HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl UserRepository for InMemoryUserRepository {

/// Returns the user with the requested ID from the repository.
///
/// # Arguments
///
/// * `id` - ID of the user to retrieve.
        async fn get_user(&self, id: u32) -> Result<Option<User>> {
            let users = self.users.lock().await;
            Ok(users.get(&id).cloned())
        }

        /// Assembles the user from its component values so callers receive a ready-to-use result.
        async fn create_user(&self, user: User) -> Result<()> {
            let mut users = self.users.lock().await;
            users.insert(user.id, user);
            Ok(())
        }
    }
}

// --- Service Layer ---

/// The business layer.
mod business {
    use std::sync::Arc;

    use anyhow::Result;

    use super::User;
    use super::data::UserRepository;

    /// Specifies the operations implementations provide to architecture callers.
    pub trait UserService {

        /// Returns the user with the given identifier, if any.
        async fn get_user(&self, id: u32) -> Result<Option<User>>;

        /// Creates a user.
        async fn create_user(&self, user: User) -> Result<()>;
    }

    /// A simple implementation of `UserService` that delegates to a
    /// `UserRepository`.
    pub struct SimpleUserService<R: UserRepository + Send + Sync> {

        /// The repository that stores the users.
        repo: Arc<R>,
    }

    impl<R: UserRepository + Send + Sync> SimpleUserService<R> {

        /// Creates a service that uses `repo`.
        pub fn new(repo: Arc<R>) -> Self {
            Self { repo }
        }
    }

    impl<R: UserRepository + Send + Sync> UserService for SimpleUserService<R> {

/// Looks up a user by ID through the service's repository layer.
///
/// # Arguments
///
/// * `id` - ID of the user to retrieve.
        async fn get_user(&self, id: u32) -> Result<Option<User>> {
            // Business logic goes here.
            self.repo.get_user(id).await
        }

        /// Assembles the user from its component values so callers receive a ready-to-use result.
        async fn create_user(&self, user: User) -> Result<()> {
            // Business logic goes here.
            self.repo.create_user(user).await
        }
    }
}

// --- Presentation Layer (simplistic CLI in this case) ---

/// The presentation layer.
mod presentation {
    use anyhow::Result;

    use super::User;
    use super::business::UserService;

    /// A simple CLI that interacts with the `UserService`.
    pub async fn run_cli(user_service: impl UserService) -> Result<()> {
        let new_user = User {
            id: 1,
            name: "Alice".to_owned(),
        };

        user_service.create_user(new_user).await?;

        match user_service.get_user(1).await? {
            Some(user) => {
                println!("Found user: {user:?}");
                println!("User #{} is named {}.", user.id, user.name);
            }
            None => println!("User not found"),
        }

        match user_service.get_user(2).await? {
            Some(user) => println!("Found user: {user:?}"),
            None => println!("User not found"),
        }

        Ok(())
    }
}

/// The main entry point of the application.
#[tokio::main]
async fn main() -> Result<()> {
    let repo = Arc::new(data::InMemoryUserRepository::new());
    let user_service = business::SimpleUserService::new(repo);
    presentation::run_cli(user_service).await?;
    Ok(())
}
// ANCHOR_END: example

/// The main entry point of the application.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the layered architecture example through its entry point.
    #[test]
    fn test_layered_architecture_example_entry_point() -> Result<()> {
        main()?;
        Ok(())
    }

    /// Creates a user, retrieves that user, and confirms an unknown user is absent.
    #[tokio::test]
    async fn created_users_can_be_found_and_others_cannot() -> Result<()> {
        use business::UserService as _;

        let service =
            business::SimpleUserService::new(Arc::new(data::InMemoryUserRepository::new()));
        service
            .create_user(User {
                id: 1,
                name: "Alice".to_owned(),
            })
            .await?;
        let found = service.get_user(1).await?.expect("user 1 was created");
        assert_eq!(found.name, "Alice");
        assert!(service.get_user(2).await?.is_none());
        Ok(())
    }

    /// Replaces a user's stored record when another record with the same ID is created.
    #[tokio::test]
    async fn creating_a_user_twice_replaces_it() -> Result<()> {
        use data::UserRepository as _;

        let repo = data::InMemoryUserRepository::new();
        for name in ["Alice", "Alicia"] {
            repo.create_user(User {
                id: 1,
                name: name.to_owned(),
            })
            .await?;
        }
        let user = repo.get_user(1).await?.expect("user 1 exists");
        assert_eq!(user.name, "Alicia");
        Ok(())
    }
}
