//! Describe an HTTP API and its data types.
//!
//!
//! OpenAPI is a standard, machine-readable description of an HTTP API: its
//! paths, parameters and data types. Tools use it to generate documentation
//! pages, client libraries and request validators, which keeps the server and
//! its callers in agreement.
//!
//! TODO Reviewed ??-??-2026.

/// Derive a JSON Schema with [`schemars`] and validate data against it with
mod schemars;
/// Generate an OpenAPI document from code with [`utoipa`].
mod utoipa;

/// Prints the generated OpenAPI document, propagating serialization errors.
fn main() -> anyhow::Result<()> {
    println!("{}", utoipa::openapi_json()?);

    // The router that would serve the API and its Swagger UI:
    let _router = utoipa::run();

    // A JSON Schema of the configuration, and a validation against it:
    let schema = schemars::run()?;
    println!("{schema}");
    let valid = schemars::is_valid(&serde_json::json!({ "port": 8080 }))?;
    let invalid = schemars::is_valid(&serde_json::json!({ "port": "text" }))?;
    println!("valid: {valid}, invalid: {invalid}");
    let config: schemars::Config = serde_json::from_value(serde_json::json!({ "port": 8080 }))?;
    println!("port: {}, verbose: {}", config.port, config.verbose);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_runs() {
        main().expect("the example should run");
    }
}
