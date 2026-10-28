// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Sharing a configuration between threads with `Arc`, and cloning it on write
//! with `Arc::make_mut`.

use std::sync::Arc;

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
        }
    }
}

/// Returns the old reader snapshot and the updated configuration.
fn updated_views() -> (Arc<cfg::Configuration>, Arc<cfg::Configuration>) {
    let mut current = cfg::Configuration::new("localhost:8080".to_owned(), 100);
    let reader_snapshot = Arc::clone(&current);
    current.update("api.example.com:8443".to_owned(), 200);
    (reader_snapshot, current)
}

/// Demonstrates that clone-on-write preserves an existing reader snapshot.
fn main() {
    let (reader_snapshot, current) = updated_views();
    assert_eq!(reader_snapshot.server_address(), "localhost:8080");
    assert_eq!(reader_snapshot.max_connections(), 100);
    assert_eq!(current.server_address(), "api.example.com:8443");
    assert_eq!(current.max_connections(), 200);
    println!("Reader snapshot: {reader_snapshot:?}");
    println!("Updated configuration: {current:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies updates clone the inner value while readers retain old data.
    #[test]
    fn clone_on_write_preserves_the_reader_snapshot() {
        let (reader_snapshot, current) = updated_views();
        assert_eq!(reader_snapshot.server_address(), "localhost:8080");
        assert_eq!(reader_snapshot.max_connections(), 100);
        assert_eq!(current.server_address(), "api.example.com:8443");
        assert_eq!(current.max_connections(), 200);
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
