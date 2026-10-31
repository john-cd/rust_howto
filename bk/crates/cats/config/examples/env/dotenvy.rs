// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Examples for working with `.env` files using `dotenvy`.
//!
//! A `.env` file lists `NAME=value` pairs for local development. `dotenvy`
//! loads them into the process environment. Never commit a `.env` file that
//! holds secrets, and do not rely on it in production, where the platform sets
//! the variables.

use std::env;

use anyhow::Result;

/// Load environment variables from a `.env` file in the current directory.
fn main() -> Result<()> {
    // Load environment variables from .env file.
    // Fails if .env file not found, not readable or invalid.
    // If variables with the same names already exist in the environment,
    // their values will be preserved. If multiple declarations for the
    // same environment variable exist in your .env file, the first one is
    // applied.
    dotenvy::dotenv()?;
    // OR: dotenvy::dotenv().ok();

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the dotenvy example through its entry point.
    #[test]
    fn test_dotenvy_example_entry_point() -> Result<()> {
        main()?;
        Ok(())
    }

    #[test]
    fn dotenv_returns_an_error_when_no_env_file_is_found()
    -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let missing = dir.path().join("nope.env");
        assert!(dotenvy::from_path(&missing).is_err());
        Ok(())
    }

    #[test]
    fn variables_from_an_env_file_are_parsed()
    -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let file = dir.path().join("test.env");
        std::fs::write(&file, "A_KEY=value\n# comment\nB_KEY=\"two words\"\n")?;
        let vars: Vec<(String, String)> = dotenvy::from_path_iter(&file)?
            .map(Result::unwrap)
            .collect();
        assert_eq!(
            vars,
            [
                ("A_KEY".to_owned(), "value".to_owned()),
                ("B_KEY".to_owned(), "two words".to_owned()),
            ]
        );
        Ok(())
    }
}
