#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Hexagonal architecture: a port (trait) with two adapters.

use std::collections::HashMap;

/// The port: how the core stores users, whatever the technology.
trait UserRepository {
    fn save(&mut self, id: u32, name: &str);
    fn find(&self, id: u32) -> Option<String>;
}

/// An adapter that keeps users in memory (ideal for tests).
#[derive(Default)]
struct InMemoryRepository {
    users: HashMap<u32, String>,
}

impl UserRepository for InMemoryRepository {
    fn save(&mut self, id: u32, name: &str) {
        self.users.insert(id, name.to_owned());
    }

    fn find(&self, id: u32) -> Option<String> {
        self.users.get(&id).cloned()
    }
}

/// An adapter that only logs; a real one would call a database.
struct LoggingRepository {
    inner: InMemoryRepository,
}

impl UserRepository for LoggingRepository {
    fn save(&mut self, id: u32, name: &str) {
        println!("saving user {id}");
        self.inner.save(id, name);
    }

    fn find(&self, id: u32) -> Option<String> {
        println!("looking up user {id}");
        self.inner.find(id)
    }
}

/// The core: business logic that depends only on the port.
struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    fn register(&mut self, id: u32, name: &str) {
        self.repository.save(id, &name.trim().to_uppercase());
    }

    fn display_name(&self, id: u32) -> String {
        self.repository
            .find(id)
            .unwrap_or_else(|| "UNKNOWN".to_owned())
    }
}

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
