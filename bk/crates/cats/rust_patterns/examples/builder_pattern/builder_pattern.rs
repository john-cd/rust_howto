// ANCHOR: example
//! Example builder pattern.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

/// The object to build.
#[derive(Debug)]
struct DatabaseConfig {
    /// The host name or address.
    host: String,
    /// The port.
    port: u16,
    /// The user name, if any.
    username: Option<String>,
    /// The password, if any.
    password: Option<String>,
    /// The size of the connection pool.
    pool_size: u32,
    /// The connection timeout in seconds.
    connection_timeout_seconds: u64,
    // More fields...
}

impl DatabaseConfig {
    /// Optional: provide a method that returns a builder.
    fn build(host: String, port: u16) -> DatabaseConfigBuilder {
        DatabaseConfigBuilder::new(host, port)
    }
    // Note that there are no constructor-like method otherwise.
}

/// The builder object.
struct DatabaseConfigBuilder {
    /// The host name or address.
    host: String,
    /// The port.
    port: u16,
    /// The user name, if any.
    username: Option<String>,
    /// The password, if any.
    password: Option<String>,
    /// The size of the connection pool.
    pool_size: u32,
    /// The connection timeout in seconds.
    connection_timeout_seconds: u64,
}

impl DatabaseConfigBuilder {
    /// The builder's constructor (with required fields, if necessary).
    fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            // The optional fields are set to None or default values.
            username: None,
            password: None,
            pool_size: 10,
            connection_timeout_seconds: 30,
        }
    }

    /// Note that the builder's methods consume `self` and returns `Self`.
    fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    /// Sets the password.
    fn password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }

    /// Sets the size of the connection pool.
    fn pool_size(mut self, pool_size: u32) -> Self {
        self.pool_size = pool_size;
        self
    }

    /// Sets the connection timeout in seconds.
    fn connection_timeout_seconds(
        mut self,
        connection_timeout_seconds: u64,
    ) -> Self {
        self.connection_timeout_seconds = connection_timeout_seconds;
        self
    }

    /// Creates the configuration from the builder.
    fn build(self) -> DatabaseConfig {
        DatabaseConfig {
            host: self.host,
            port: self.port,
            username: self.username,
            password: self.password,
            pool_size: self.pool_size,
            connection_timeout_seconds: self.connection_timeout_seconds,
        }
    }
}

/// Builds a configuration with the builder and prints it.
fn main() {
    // Create a builder struct.
    let config = DatabaseConfigBuilder::new("localhost".to_owned(), 5432)
        // Call (some of) the builder's methods to initialize or update the
        // fields.
        .username("admin".to_owned())
        .password("secret".to_owned())
        .pool_size(20)
        .build();

    println!("Database Configuration: {config:?}");
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
