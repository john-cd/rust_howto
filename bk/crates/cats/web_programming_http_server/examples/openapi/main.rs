//! Describe an HTTP API and its data types.
//!
//! TODO Reviewed ??-??-2026.

mod schemars;
mod utoipa;

/// Prints the generated OpenAPI document, propagating serialization errors.
fn main() -> anyhow::Result<()> {
    println!("{}", utoipa::openapi_json()?);

    // The router that would serve the API and its Swagger UI:
    let _router = utoipa::app();

    // A JSON Schema of the configuration, and a validation against it:
    println!("{}", schemars::config_schema()?);
    let valid = schemars::is_valid(&serde_json::json!({ "port": 8080 }))?;
    let invalid = schemars::is_valid(&serde_json::json!({ "port": "text" }))?;
    println!("valid: {valid}, invalid: {invalid}");
    let config: schemars::Config =
        serde_json::from_value(serde_json::json!({ "port": 8080 }))?;
    println!("port: {}, verbose: {}", config.port, config.verbose);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn main_runs() {
        main().expect("the example should run");
    }
}
