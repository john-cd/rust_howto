#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Extracting the login of an email address with a regular expression.

use lazy_static::lazy_static;
use regex::Regex;

/// Returns the login part of the first email address in `input`.
fn extract_login(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
            ^(?P<login>[^@\s]+)@
            ([[:word:]]+\.)*
            [[:word:]]+$
            "
        )
        .unwrap();
    }
    let cap = RE.captures(input)?;
    cap.name("login").map(|login| login.as_str())
}

/// Extracts the login from several strings.
fn main() {
    let login = extract_login("I❤email@example.com");
    println!("{login:?}");
    assert_eq!(login, Some("I❤email"));

    let login = extract_login("sdf+sdsfsd.as.sdsd@jhkk.d.rl");
    println!("{login:?}");
    assert_eq!(login, Some("sdf+sdsfsd.as.sdsd"));

    assert_eq!(extract_login("More@Than@One@at.com"), None);
    assert_eq!(extract_login("Not an email@email"), None);
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
