#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Examples for working with `.env` files and `envy`.

use serde::Deserialize;

// Define a Configuration struct that can be deserialized from environment
// variables.
/// Settings deserialized from environment variables.
#[derive(Deserialize, Debug)]
struct Configuration {
    /// The port to listen on, from the `PORT` environment variable.
    port: u16,
}

/// Deserializes the settings from the environment and prints them.
fn main() {
    // Deserialize the Configuration struct from environment variables.
    // It expects a PORT environment variable to be set.
    let c = envy::from_env::<Configuration>()
        .expect("Please provide the PORT env variable");

    // Deserialize the Configuration struct from environment variables with a
    // prefix. It expects a MY_APP__PORT environment variable to be set.
    let c2 = envy::prefixed("MY_APP__")
        .from_env::<Configuration>()
        .expect("Please provide MY_APP__PORT env variable");

    println!("c: {c:?} c2: {c2:?}");
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_main() {
        let _guard =
            ENV_LOCK.lock().expect("environment variable lock poisoned");
        unsafe {
            std::env::set_var("PORT", "80");
            std::env::set_var("MY_APP__PORT", "8080");
        }
        main();
    }
}
