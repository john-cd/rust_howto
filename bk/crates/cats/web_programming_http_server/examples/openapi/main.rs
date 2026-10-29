//! Describe an HTTP API and its data types.
//!
//! TODO Reviewed ??-??-2026.

mod schemars;
mod utoipa;

/// Prints the generated OpenAPI document, propagating serialization errors.
fn main() -> anyhow::Result<()> {
    println!("{}", utoipa::openapi_json()?);
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
