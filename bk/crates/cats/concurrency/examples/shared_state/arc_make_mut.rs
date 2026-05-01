#![allow(dead_code)]
// ANCHOR: example
use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod cfg {

    use std::sync::Arc;

    /// A configuration struct to be shared among threads.
    #[derive(Debug, Clone)]
    pub struct Configuration {
        server_address: String,
        max_connections: u8,
    }

    impl Configuration {
        pub fn new(
            server_address: String,
            max_connections: u8,
        ) -> Arc<Configuration> {
            Arc::new(Configuration {
                server_address,
                max_connections,
            })
        }

        pub fn server_address(&self) -> &str {
            &self.server_address
        }

        pub fn max_connections(&self) -> u8 {
            self.max_connections
        }

        pub fn update(
            self: &mut Arc<Self>,
            server_address: String,
            max_connections: u8,
        ) {
            // If there are other `Arc` pointers to the same allocation,
            // then `make_mut` will clone the inner value to a new allocation to
            // ensure unique ownership. This is also referred to as
            // clone-on-write.
            let mutable_config: &mut Configuration = Arc::make_mut(self);
            mutable_config.server_address = server_address;
            mutable_config.max_connections = max_connections;
            println!("Configuration updated to {mutable_config:?}");
            println!(
                "Original config after update (should be unchanged for readers): {self:?}"
            );
        }
    }
}

fn main() {
    // Initialize the configuration and wrap it in an `Arc` for shared read
    // access.
    let mut config = cfg::Configuration::new("localhost:8080".to_string(), 100);

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
    config.update("api.example.com:8443".to_string(), 200);

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

#[test]
fn test() {
    main();
}
