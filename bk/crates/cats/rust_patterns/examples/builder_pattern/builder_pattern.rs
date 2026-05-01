// ANCHOR: example
#![allow(dead_code)]
//! Example builder pattern.

/// The object to build.
#[derive(Debug)]
struct DatabaseConfig {
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    pool_size: u32,
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
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    pool_size: u32,
    connection_timeout_seconds: u64,
}

impl DatabaseConfigBuilder {
    /// The builder's constructor (with required fields, if necessary).
    fn new(host: String, port: u16) -> DatabaseConfigBuilder {
        DatabaseConfigBuilder {
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

    fn password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }

    fn pool_size(mut self, pool_size: u32) -> Self {
        self.pool_size = pool_size;
        self
    }

    fn connection_timeout_seconds(
        mut self,
        connection_timeout_seconds: u64,
    ) -> Self {
        self.connection_timeout_seconds = connection_timeout_seconds;
        self
    }

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

fn main() {
    // Create a builder struct.
    let config = DatabaseConfigBuilder::new("localhost".to_string(), 5432)
        // Call (some of) the builder's methods to initialize or update the fields.
        .username("admin".to_string())
        .password("secret".to_string())
        .pool_size(20)
        .build();

    println!("Database Configuration: {config:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
