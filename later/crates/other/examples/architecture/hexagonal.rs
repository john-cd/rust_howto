// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Hexagonal architecture: a port (trait) with two adapters.
//!
//! Hexagonal (ports and adapters) architecture keeps the business logic
//! independent of technology. The logic talks to a port (a trait); adapters
//! implement it for a database, a web service or a test double, so you can swap
//! them or test without infrastructure.

use std::collections::HashMap;

/// The port: how the core stores users, whatever the technology.
trait UserRepository {
    /// # Arguments
    /// * `id` - Identifier used to select the user repository record or cache entry affected by
    ///   this operation.
    /// * `name` - Human-readable value used to identify or display the selected user repository.
    fn save(&mut self, id: u32, name: &str);
    /// # Arguments
    /// * `id` - Identifier used to select the user repository record or cache entry affected by
    ///   this operation.
    fn find(&self, id: u32) -> Option<String>;
}

/// An adapter that keeps users in memory (ideal for tests).
#[derive(Default)]
struct InMemoryRepository {
    /// Collection of users values consumed by in memory repository operations.
    users: HashMap<u32, String>,
}

impl UserRepository for InMemoryRepository {
    /// Saves the current value.
    /// # Arguments
    /// * `id` - Identifier used to select the in memory repository record or cache entry affected
    ///   by this operation.
    /// * `name` - Human-readable value used to identify or display the selected in memory
    ///   repository.
    fn save(&mut self, id: u32, name: &str) {
        self.users.insert(id, name.to_owned());
    }

    /// Finds the current value.

    /// # Arguments
    /// * `id` - Identifier used to select the in memory repository record or cache entry affected
    ///   by this operation.
    fn find(&self, id: u32) -> Option<String> {
        self.users.get(&id).cloned()
    }
}

/// An adapter that only logs; a real one would call a database.
struct LoggingRepository {
    /// Shared internal state.
    inner: InMemoryRepository,
}

impl UserRepository for LoggingRepository {
    /// Saves the current value.
    /// # Arguments
    /// * `id` - Identifier used to select the logging repository record or cache entry affected by
    ///   this operation.
    /// * `name` - Human-readable value used to identify or display the selected logging repository.
    fn save(&mut self, id: u32, name: &str) {
        println!("saving user {id}");
        self.inner.save(id, name);
    }

    /// Finds the current value.

    /// # Arguments
    /// * `id` - Identifier used to select the logging repository record or cache entry affected by
    ///   this operation.
    fn find(&self, id: u32) -> Option<String> {
        println!("looking up user {id}");
        self.inner.find(id)
    }
}

/// The core: business logic that depends only on the port.
struct UserService<R: UserRepository> {
    /// Repository value.
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    /// # Arguments
    /// * `id` - Identifier used to select the user service record or cache entry affected by this
    ///   operation.
    /// * `name` - Human-readable value used to identify or display the selected user service.

    fn register(&mut self, id: u32, name: &str) {
        self.repository.save(id, &name.trim().to_uppercase());
    }

    /// # Arguments
    /// * `id` - Identifier used to select the user service record or cache entry affected by this
    ///   operation.

    fn display_name(&self, id: u32) -> String {
        self.repository
            .find(id)
            .unwrap_or_else(|| "UNKNOWN".to_owned())
    }
}
/// Runs the example.
/// Runs the same core logic with two different adapters.

fn main() {
    let mut service = UserService {
        repository: InMemoryRepository::default(),
    };
    service.register(1, " ada ");
    assert_eq!(service.display_name(1), "ADA");
    assert_eq!(service.display_name(2), "UNKNOWN");

    let mut service = UserService {
        repository: LoggingRepository {
            inner: InMemoryRepository::default(),
        },
    };
    service.register(7, "grace");
    println!("{}", service.display_name(7));
}
// ANCHOR_END: example
/// Runs the example entry point.

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the hexagonal example through its entry point.
    #[test]
    fn test_hexagonal_example_entry_point() {
        main();
    }
}
