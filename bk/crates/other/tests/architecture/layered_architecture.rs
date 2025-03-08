// ANCHOR: example
use async_trait::async_trait;
use tokio;

// --- Persistence / Data Access Layer ---

mod data {

    #[derive(Debug, Clone)]
    pub struct User {
        pub id: u32,
        pub name: String,
    }

    #[async_trait]
    pub trait UserRepository {
        async fn get_user(&self, id: u32) -> Result<Option<User>, String>;
        async fn create_user(&self, user: User) -> Result<(), String>;
    }

    pub struct InMemoryUserRepository {
        users: tokio::sync::Mutex<std::collections::HashMap<u32, User>>,
    }

    impl InMemoryUserRepository {
        pub fn new() -> Self {
            InMemoryUserRepository {
                users: tokio::sync::Mutex::new(std::collections::HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl UserRepository for InMemoryUserRepository {
        async fn get_user(&self, id: u32) -> Result<Option<User>, String> {
            let users = self.users.lock().await;
            Ok(users.get(&id).cloned())
        }

        async fn create_user(&self, user: User) -> Result<(), String> {
            let mut users = self.users.lock().await;
            users.insert(user.id, user);
            Ok(())
        }
    }
}

// --- Service Layer ---

mod business {
    use super::data::User;
    use super::data::UserRepository;

    pub struct UserService<Repo: UserRepository + Send + Sync> {
        repo: std::sync::Arc<Repo>,
    }

    impl<Repo: UserRepository + Send + Sync> UserService<Repo> {
        pub fn new(repo: std::sync::Arc<Repo>) -> Self {
            UserService { repo }
        }

        pub async fn get_user(&self, id: u32) -> Result<Option<User>, String> {
            // Business logic here
            self.repo.get_user(id).await
        }

        pub async fn create_user(&self, user: User) -> Result<(), String> {
            // Business logic here
            self.repo.create_user(user).await
        }
    }
}

// --- Presentation Layer (CLI in this case) ---

async fn run_cli(
    user_service: UserService<InMemoryUserRepository>,
) -> Result<(), String> {
    let new_user = User {
        id: 1,
        name: "Alice".to_string(),
    };

    user_service.create_user(new_user.clone()).await?;

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

#[tokio::main]
async fn main() -> Result<(), String> {
    let repo = std::sync::Arc::new(InMemoryUserRepository::new());
    let user_service = UserService::new(repo);

    run_cli(user_service).await?;

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
