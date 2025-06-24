#![allow(dead_code)]
// ANCHOR: example
//! This example showcases how to create a custom environment source for (unit)
//! testing purposes, allowing you to simulate different environment variable
//! settings without modifying the actual system environment.
//!
//! Add to `Cargo.toml`:
//! ```toml
//! config = "0.15.6" # Or latest
//! ```

#[test]
fn test_config() -> anyhow::Result<(), ::config::ConfigError> {
    // The `Environment::source` method can be used when you want to test
    // your code, without the need to change the actual system
    // environment variables.
    let source = ::config::Environment::default().source(Some({
        let mut env = std::collections::HashMap::new();
        env.insert("HOST".into(), "1.1.1.1".into()); // Fakes the env. variable HOST="1.1.1.1"
        env
    }));

    let config = config::Config::builder()
        .add_source(source)
        .build()?
        .try_deserialize::<std::collections::HashMap<String, String>>(
    )?;

    assert_eq!(config.get("host"), Some(&"1.1.1.1".to_string()));

    Ok(())
}
// ANCHOR_END: example
