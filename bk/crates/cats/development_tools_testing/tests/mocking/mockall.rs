// ANCHOR: example
use mockall::predicate::*;
use mockall::*;

// First, we need to add mockall to our Cargo.toml
// [dependencies]
// mockall = "0.11.3"

// Define our data model
#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// Define our repository trait with the `#[automock]` attribute.
// It automatically generates a mock implementation of your trait, named
// Mock{TraitName}. In our case, it creates a `MockUserRepository`.
#[automock]
trait UserRepository {
    fn find_by_id(&self, id: u64) -> Option<User>;
    fn save(&self, user: User) -> Result<(), String>;
}

// Define our service that uses the repository.
struct UserService<T: UserRepository> {
    repository: T,
}

impl<T: UserRepository> UserService<T> {
    fn new(repository: T) -> Self {
        Self { repository }
    }

    fn get_user(&self, id: u64) -> Result<User, String> {
        self.repository
            .find_by_id(id)
            .ok_or_else(|| format!("User with id {} not found", id))
    }

    fn update_user_email(
        &self,
        id: u64,
        new_email: String,
    ) -> Result<User, String> {
        // Get the user
        let mut user = self.get_user(id)?;

        // Update the email
        user.email = new_email;

        // Save the user
        self.repository.save(user.clone())?;

        Ok(user)
    }
}

// Now, let's write our tests using mockall
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_success() {
        // Create a mock repository
        let mut mock_repo = MockUserRepository::new();

        // Set up expectations
        mock_repo
            .expect_find_by_id() // The method we're setting expectations for
            .with(eq(1)) // The argument we expect (using predicate). You can also use custom functions: .withf(|user| user.id == 1)
            .times(1)  // How many times we expect it to be called
            .returning(|_| { // What it should return
                Some(User {
                    id: 1,
                    name: "John Doe".to_string(),
                    email: "john@example.com".to_string(),
                })
            });

        // Create our service with the mock repository
        let service = UserService::new(mock_repo);

        // Call the method we want to test
        let result = service.get_user(1);

        // Assert the result
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
    }

    #[test]
    fn test_get_user_not_found() {
        // Create a mock repository
        let mut mock_repo = MockUserRepository::new();

        // Set up expectations - return None for any id
        mock_repo
            .expect_find_by_id()
            .with(eq(999))
            .times(1)
            .returning(|_| None);

        // Create our service with the mock repository
        let service = UserService::new(mock_repo);

        // Call the method we want to test
        let result = service.get_user(999);

        // Assert the result
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "User with id 999 not found");
    }

    #[test]
    fn test_update_user_email() {
        // Create a mock repository
        let mut mock_repo = MockUserRepository::new();

        // Set up expectations for find_by_id
        mock_repo
            .expect_find_by_id()
            .with(eq(1))
            .times(1)
            .returning(|_| {
                Some(User {
                    id: 1,
                    name: "John Doe".to_string(),
                    email: "john@example.com".to_string(),
                })
            });

        // Set up expectations for save
        mock_repo
            .expect_save()
            .withf(|user: &User| {
                user.id == 1
                    && user.name == "John Doe"
                    && user.email == "newemail@example.com"
            })
            .times(1)
            .returning(|_| Ok(()));

        // Create our service with the mock repository
        let service = UserService::new(mock_repo);

        // Call the method we want to test
        let result =
            service.update_user_email(1, "newemail@example.com".to_string());

        // Assert the result
        assert!(result.is_ok());
        let updated_user = result.unwrap();
        assert_eq!(updated_user.id, 1);
        assert_eq!(updated_user.name, "John Doe");
        assert_eq!(updated_user.email, "newemail@example.com");
    }
}
// ANCHOR_END: example
