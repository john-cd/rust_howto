// HACK: the `config` package has the same name
// than the current crate, thus we renamed it `config1`
// See `Cargo.toml`

// ANCHOR: example
use std::collections::HashMap;

use config1 as config;

#[test]
fn test_config() -> anyhow::Result<(), config::ConfigError> {
    // The `Environment::source` method can be used when you want to test
    // your code, without the need to change the actual system
    // environment variables.
    let source = config::Environment::default().source(Some({
        let mut env = HashMap::new();
        env.insert("HOST".into(), "1.1.1.1".into());
        env
    }));

    let config = config::Config::builder()
        .add_source(source)
        .build()?
        .try_deserialize::<HashMap<String, String>>()?;

    assert_eq!(config.get("host"), Some(&"1.1.1.1".to_string()));

    Ok(())
}
// ANCHOR_END: example
