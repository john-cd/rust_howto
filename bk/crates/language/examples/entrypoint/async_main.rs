#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This is an example of an async main function.

use anyhow::Result;

/// This is an example of an async main function.
///
/// It uses the `tokio::main` macro to create an async runtime.
/// It returns a `Result` to allow for error handling.
///
/// Add `tokio` with at least the "macros" and
/// "rt-multi-thread" features to your `Cargo.toml`:
///
/// ```toml
/// tokio = { version = "1", features = ["full"] }
/// ```
#[tokio::main]
async fn main() -> Result<()> {
    println!("I'm async!");
    // You can .await futures here.
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the async main example through its entry point.
    #[test]
    fn test_async_main_example_entry_point() -> Result<()> {
        main()?;
        Ok(())
    }
}
