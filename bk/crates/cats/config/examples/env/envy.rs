// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Examples for working with `.env` files and `envy`.
//!
//! `envy` reads environment variables into a typed struct with Serde, so a
//! missing or malformed variable becomes one clear error at startup instead of
//! a failure deep in the program.

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
    println!("ports: {} and {}", c.port, c2.port);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::*;

    /// Verifies the example with environment values isolated in a child
    /// process.
    #[test]
    fn test_envy_example_entry_point() -> std::io::Result<()> {
        let output = Command::new(std::env::current_exe()?)
            .args(["child_reads_controlled_environment", "--ignored"])
            .env("PORT", "80")
            .env("MY_APP__PORT", "8080")
            .output()?;
        assert!(
            output.status.success(),
            "child environment test failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(())
    }

    /// Runs the example in the child with values supplied by its parent.
    #[test]
    #[ignore = "needs a parent-controlled environment"]
    fn child_reads_controlled_environment() {
        main();
    }
}
