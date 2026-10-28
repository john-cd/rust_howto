#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Keeps a secret out of logs with a newtype whose `Debug` and `Display`
//! implementations redact the value, then does the same with `secrecy`.

use std::convert::Infallible;
use std::fmt;
use std::str::FromStr;

use secrecy::ExposeSecret as _;
use secrecy::SecretString;

/// An API key or password that never prints itself.
///
/// The only way to read the value is the explicit `expose_secret` call, which
/// is easy to find in a code review.
#[derive(Clone, PartialEq, Eq)]
struct RedactedSecret(String);

impl RedactedSecret {
    /// Returns the secret value. Call sites mark where the secret is used.
    fn expose_secret(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for RedactedSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RedactedSecret(***)")
    }
}

impl fmt::Display for RedactedSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("***")
    }
}

impl FromStr for RedactedSecret {
    type Err = Infallible;

    /// Lets the secret be parsed from an environment variable or a flag.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

/// A configuration struct can derive `Debug` safely if every secret field is
/// redacted.
#[derive(Debug)]
struct Config {
    /// Where to send requests.
    endpoint: String,
    /// Credential for the endpoint.
    api_key: RedactedSecret,
}

fn main() -> Result<(), Infallible> {
    // Hand-written newtype:
    let key: RedactedSecret = "sk-live-1234".parse()?;
    let config = Config {
        endpoint: "https://api.example.com".to_owned(),
        api_key: key,
    };
    println!("{config:?}");
    println!("{}", config.api_key);
    assert_eq!(config.api_key.expose_secret(), "sk-live-1234");

    // The same idea with the `secrecy` crate: the value is zeroized on drop
    // and `Debug` prints a redacted placeholder.
    let secret = SecretString::from("sk-live-1234");
    println!("{secret:?}");
    assert_eq!(secret.expose_secret(), "sk-live-1234");
    Ok(())
}

// ANCHOR_END: example

pub fn run() {
    let _ = main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_and_display_never_contain_the_secret() -> Result<(), Infallible> {
        let secret: RedactedSecret = "hunter2".parse()?;
        assert!(!format!("{secret:?}").contains("hunter2"));
        assert!(!format!("{secret}").contains("hunter2"));
        let config = Config {
            endpoint: String::new(),
            api_key: secret,
        };
        assert!(!format!("{config:?}").contains("hunter2"));
        assert!(!format!("{config:#?}").contains("hunter2"));
        Ok(())
    }

    #[test]
    fn secrecy_debug_does_not_contain_the_secret() {
        let secret = SecretString::from("hunter2");
        assert!(!format!("{secret:?}").contains("hunter2"));
    }

    /// Runs the redacted secret example through its entry point.
    #[test]
    fn test_redacted_secret_example_entry_point() {
        run();
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
