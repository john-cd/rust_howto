// TODO Reviewed ??-??-2026.

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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    /// Serializes tests that change process environment variables.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    /// Verifies that the envy example's main path succeeds.
    /// Runs the envy example through its entry point.
    #[test]
    fn test_envy_example_entry_point() {
        let _guard =
            ENV_LOCK.lock().expect("environment variable lock poisoned");
        unsafe {
            std::env::set_var("PORT", "80");
            std::env::set_var("MY_APP__PORT", "8080");
        }
        main();
    }
}
