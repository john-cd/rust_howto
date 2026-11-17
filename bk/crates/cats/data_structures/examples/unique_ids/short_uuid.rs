// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Generate compact, URL-safe identifiers with `short-uuid`.
//!
//! A UUID is a 128-bit identifier that is unique for practical purposes without
//! any central coordination. Its usual form is long (36 characters); a short
//! form, using a bigger alphabet, is friendlier in URLs.

use short_uuid::ShortUuid;

/// Generates a short UUID and converts it to and from a regular UUID.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn example_body() -> anyhow::Result<()> {
    // A random (v4) UUID in a shorter, URL-safe form.
    let short = ShortUuid::generate();
    let text = short.to_string();
    println!("{text}");

    // The text parses back to the same value.
    // The error type does not implement `std::error::Error`.
    let parsed =
        ShortUuid::parse_str(&text).map_err(|e| anyhow::anyhow!("invalid short UUID: {e:?}"))?;
    assert_eq!(parsed, short);

    // It converts losslessly to and from a regular UUID.
    let uuid = short.to_uuid();
    assert_eq!(ShortUuid::from_uuid(&uuid).to_string(), text);

    // Invalid input is an error.
    assert!(ShortUuid::parse_str("not a short uuid!").is_err());
    Ok(())
}
// ANCHOR_END: example

/// Runs the example function and returns its result.
///
/// # Errors
///
/// Returns an error if the example operation fails.
pub fn run() -> anyhow::Result<()> {
    example_body()
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs succeeds.
    #[test]
    fn run_succeeds() -> anyhow::Result<()> {
        run()
    }
}
