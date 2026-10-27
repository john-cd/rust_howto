#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates a layered architecture in Rust,
//! separating concerns into data access, business logic, and presentation
//! layers.

use std::sync::Arc;

use anyhow::Result;

#[derive(Debug, Clone)]
/// Represents a user in the system.
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

    #[async_trait]
    /// Defines the contract for interacting with user data.
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
        async fn get_user(&self, id: u32) -> Result<Option<User>> {
            let users = self.users.lock().await;
            Ok(users.get(&id).cloned())
        }

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

    /// Defines the contract for user-related business operations.
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
        async fn get_user(&self, id: u32) -> Result<Option<User>> {
            // Business logic goes here.
            self.repo.get_user(id).await
        }

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
            Some(user) => println!("Found user: {user:?}"),
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
}
