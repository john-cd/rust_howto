#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Sharing a configuration between threads with `Arc`, and cloning it on write
//! with `Arc::make_mut`.

use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// A configuration shared through an `Arc`.
mod cfg {

    use std::sync::Arc;

    /// A configuration struct to be shared among threads.
    #[derive(Debug, Clone)]
    pub struct Configuration {
        /// Address of the server.
        server_address: String,
        /// Maximum number of connections.
        max_connections: u8,
    }

    impl Configuration {
        /// Creates a shared configuration.
        pub fn new(server_address: String, max_connections: u8) -> Arc<Self> {
            Arc::new(Self {
                server_address,
                max_connections,
            })
        }

        /// Returns the address of the server.
        pub fn server_address(&self) -> &str {
            &self.server_address
        }

        /// Returns the maximum number of connections.
        pub fn max_connections(&self) -> u8 {
            self.max_connections
        }

        /// Updates the configuration, cloning it first if other `Arc` pointers
        /// to it exist (clone-on-write).
        pub fn update(
            self: &mut Arc<Self>,
            server_address: String,
            max_connections: u8,
        ) {
            // If there are other `Arc` pointers to the same allocation,
            // then `make_mut` will clone the inner value to a new allocation to
            // ensure unique ownership. This is also referred to as
            // clone-on-write.
            let mutable_config: &mut Self = Arc::make_mut(self);
            mutable_config.server_address = server_address;
            mutable_config.max_connections = max_connections;
            println!("Configuration updated to {mutable_config:?}");
            println!(
                "Original config after update (should be unchanged for readers): {self:?}"
            );
        }
    }
}

/// Shares a configuration with reader threads, then updates it without
/// disturbing them.
fn main() {
    // Initialize the configuration and wrap it in an `Arc` for shared read
    // access.
    let mut config = cfg::Configuration::new("localhost:8080".to_owned(), 100);

    // Simulate multiple readers accessing the configuration:
    let mut handles = vec![];
    for i in 0..3 {
        let config_clone = Arc::clone(&config);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50 * i));
            println!(
                "Reader {i} - Server Address: {}",
                config_clone.server_address()
            );
        });
        handles.push(handle);
    }

    // Simulate an administrative task updating the configuration
    // after some time:
    thread::sleep(Duration::from_millis(75));
    println!("\nAdmin task: Updating configuration...");
    config.update("api.example.com:8443".to_owned(), 200);

    // Spawn more readers after the update.
    for i in 3..5 {
        let config_clone = Arc::clone(&config);
        let handle = thread::spawn(move || {
            println!(
                "Reader {i} - Server Address: {}  Max Connections: {}",
                config_clone.server_address(),
                config_clone.max_connections()
            );
            thread::sleep(Duration::from_millis(50 * (i - 3)));
        });
        handles.push(handle);
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }
}

// ANCHOR_END: example

/// Runs the example.
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
