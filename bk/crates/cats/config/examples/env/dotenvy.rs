// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Examples for working with `.env` files using `dotenvy`.

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
    fn dotenv_returns_an_error_when_no_env_file_is_found() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("nope.env");
        assert!(dotenvy::from_path(&missing).is_err());
    }

    #[test]
    fn variables_from_an_env_file_are_parsed() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("test.env");
        std::fs::write(&file, "A_KEY=value\n# comment\nB_KEY=\"two words\"\n")
            .unwrap();
        let vars: Vec<(String, String)> = dotenvy::from_path_iter(&file)
            .unwrap()
            .map(Result::unwrap)
            .collect();
        assert_eq!(
            vars,
            [
                ("A_KEY".to_owned(), "value".to_owned()),
                ("B_KEY".to_owned(), "two words".to_owned()),
            ]
        );
    }
}
