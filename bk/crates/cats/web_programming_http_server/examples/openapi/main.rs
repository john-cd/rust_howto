//! Describe an HTTP API and its data types.
//!
//! TODO Reviewed ??-??-2026.

mod schemars;
mod utoipa;

fn main() -> anyhow::Result<()> {
    println!("{}", utoipa::openapi_json()?);
    Ok(())
}
