// TODO Reviewed ??-??-2026.
//! Reports the field path for a JSON deserialization error.

// ANCHOR: example
use serde::Deserialize;

/// Top-level application settings.
#[derive(Debug, Deserialize)]
struct Settings {
    servers: Vec<Server>,
}

/// Settings for a single server.
#[derive(Debug, Deserialize)]
struct Server {
    port: u16,
}

/// Shows the path and original error for an invalid nested value.
///
/// # Errors
///
/// Returns an error if the valid settings fail to deserialize.
fn main() -> anyhow::Result<()> {
    let valid: Settings = serde_json::from_str(r#"{"servers":[{"port":8080}]}"#)?;
    assert_eq!(valid.servers[0].port, 8080);

    let mut deserializer =
        serde_json::Deserializer::from_str(r#"{"servers":[{"port":8080},{"port":"invalid"}]}"#);
    let error = serde_path_to_error::deserialize::<_, Settings>(&mut deserializer)
        .expect_err("a string is not a port number");

    assert_eq!(error.path().to_string(), "servers[1].port");
    assert!(error.inner().is_data());
    println!("{}: {}", error.path(), error.inner());
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Confirms a nested type mismatch reports its path and a Serde data error.
    #[test]
    fn reports_nested_type_mismatch() -> anyhow::Result<()> {
        super::main()
    }
}
