// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Examples for working with `.env` files using `dotenvy`.
//!
//! A `.env` file lists `NAME=value` pairs for local development. `dotenvy`
//! can load them into the process environment. Never commit a `.env` file that
//! holds secrets, and do not rely on it in production, where the platform sets
//! the variables.

use anyhow::Result;

/// Reads one setting from a `.env` file without changing the process environment.
fn main() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join(".env");
    std::fs::write(&path, "APP_MODE=development\n")?;

    let variables = dotenvy::from_path_iter(path)?.collect::<Result<Vec<_>, _>>()?;
    if let Some((_, value)) = variables.iter().find(|(key, _)| key == "APP_MODE") {
        println!("APP_MODE={value}");
    }

    Ok(())
}
// ANCHOR_END: example

/// Reads one setting from a `.env` file without changing the process environment.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> Result<()> {
    main()
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the dotenvy example through its entry point.
    #[test]
    fn test_dotenvy_example_entry_point() -> Result<()> {
        main()?;
        Ok(())
    }

    /// Returns an error when no env file is found for the dotenv.
    #[test]
    fn dotenv_returns_an_error_when_no_env_file_is_found() -> Result<(), Box<dyn std::error::Error>>
    {
        let dir = tempfile::tempdir()?;
        let missing = dir.path().join("nope.env");
        assert!(dotenvy::from_path(&missing).is_err());
        Ok(())
    }

    /// Checks that the variables from an env file are parsed.
    #[test]
    fn variables_from_an_env_file_are_parsed() -> Result<(), Box<dyn std::error::Error>> {
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
