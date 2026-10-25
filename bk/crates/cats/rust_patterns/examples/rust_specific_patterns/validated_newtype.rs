#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A validated newtype: an `EmailAddress` that cannot hold invalid data, even
//! when it comes from `serde` deserialization.
//!
//! Use `derive_more` (`From`, `Display`, `Add`, ...) to avoid boilerplate for
//! unrestricted newtypes, but write the impls by hand when they enforce
//! invariants. Avoid `Deref` on such types: it lets callers bypass the checks.

use std::fmt;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

/// The reasons an address can be rejected.
#[derive(Debug, PartialEq, Eq)]
enum EmailError {
    /// There is no `@`, or more than one.
    MissingAt,
    /// The part before the `@` is empty.
    EmptyLocalPart,
    /// The domain is empty or has no dot.
    InvalidDomain,
}

impl fmt::Display for EmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailError::MissingAt => write!(f, "expected exactly one '@'"),
            EmailError::EmptyLocalPart => write!(f, "empty local part"),
            EmailError::InvalidDomain => write!(f, "invalid domain"),
        }
    }
}

impl std::error::Error for EmailError {}

/// A syntactically plausible email address. The field is private, so the only
/// ways to build one run the validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
struct EmailAddress(String);

impl TryFrom<String> for EmailAddress {
    type Error = EmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parts = value.split('@');
        let (Some(local), Some(domain), None) =
            (parts.next(), parts.next(), parts.next())
        else {
            return Err(EmailError::MissingAt);
        };
        if local.is_empty() {
            return Err(EmailError::EmptyLocalPart);
        }
        if !domain.contains('.')
            || domain.starts_with('.')
            || domain.ends_with('.')
        {
            return Err(EmailError::InvalidDomain);
        }
        Ok(EmailAddress(value))
    }
}

impl TryFrom<&str> for EmailAddress {
    type Error = EmailError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_owned().try_into()
    }
}

impl FromStr for EmailAddress {
    type Err = EmailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

impl From<EmailAddress> for String {
    fn from(email: EmailAddress) -> String {
        email.0
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Parses valid and invalid addresses, directly and through JSON.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let email: EmailAddress = "ferris@example.com".parse()?;
    println!("Valid: {email}");

    assert_eq!("no-at-sign".parse::<EmailAddress>(), Err(EmailError::MissingAt));
    assert_eq!("@example.com".parse::<EmailAddress>(), Err(EmailError::EmptyLocalPart));
    assert_eq!("a@localhost".parse::<EmailAddress>(), Err(EmailError::InvalidDomain));

    // Serialization round trip:
    let json = serde_json::to_string(&email)?;
    assert_eq!(json, "\"ferris@example.com\"");
    let back: EmailAddress = serde_json::from_str(&json)?;
    assert_eq!(back, email);

    // Deserialization validates, too:
    let bad = serde_json::from_str::<EmailAddress>("\"nope\"");
    assert!(bad.is_err());
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    if let Err(e) = main() {
        eprintln!("{e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> Result<(), Box<dyn std::error::Error>> {
        main()
    }
}
