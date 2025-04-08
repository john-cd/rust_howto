#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates a layered architecture in Rust,
//! separating concerns into data access, business logic, and presentation
//! layers.

use std::sync::Arc;

use anyhow::Result;

#[derive(Debug, Clone)]
/// Represents a user in the system.
pub struct User {
    pub id: u32,
    pub name: String,
}

// --- Persistence / Data Access Layer ---
mod data {
    use std::collections::HashMap;

    use anyhow::Result;
    use async_trait::async_trait;
    use tokio::sync::Mutex;

    use super::User;

    #[async_trait]
    /// Defines the contract for interacting with user data.
    pub trait UserRepository {
        async fn get_user(&self, id: u32) -> Result<Option<User>>;
        async fn create_user(&self, user: User) -> Result<()>;
    }

    /// An in-memory implementation of `UserRepository` for testing and
    /// demonstration purposes.
    pub struct InMemoryUserRepository {
        users: Mutex<HashMap<u32, User>>,
    }

    impl InMemoryUserRepository {
        pub fn new() -> Self {
            InMemoryUserRepository {
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

mod business {
    use std::sync::Arc;

    use anyhow::Result;

    use super::User;
    use super::data::UserRepository;

    /// Defines the contract for user-related business operations.
    pub trait UserService {
        async fn get_user(&self, id: u32) -> Result<Option<User>>;
        async fn create_user(&self, user: User) -> Result<()>;
    }

    /// A simple implementation of `UserService` that delegates to a
    /// `UserRepository`.
    pub struct SimpleUserService<R: UserRepository + Send + Sync> {
        repo: Arc<R>,
    }

    impl<R: UserRepository + Send + Sync> SimpleUserService<R> {
        pub fn new(repo: Arc<R>) -> Self {
            SimpleUserService { repo }
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
mod presentation {
    use anyhow::Result;

    use super::User;
    use super::business::UserService;

    /// A simple CLI that interacts with the `UserService`.
    pub async fn run_cli(user_service: impl UserService) -> Result<()> {
        let new_user = User {
            id: 1,
            name: "Alice".to_string(),
        };

        user_service.create_user(new_user).await?;

        match user_service.get_user(1).await? {
            Some(user) => println!("Found user: {:?}", user),
            None => println!("User not found"),
        }

        match user_service.get_user(2).await? {
            Some(user) => println!("Found user: {:?}", user),
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

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
