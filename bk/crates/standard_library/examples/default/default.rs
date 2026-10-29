// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! The `Default` trait provides a way to create a default value for a type.
//! It initializes a struct without manually specifying every field.

// `derive` implements it automatically:
/// A configuration, with the default values of each type.
#[derive(Default, Debug)]
struct Config {
    /// The timeout.
    timeout: u32, // defaults to `0`.
    /// The number of retries.
    retries: u8, // defaults to `0`.
    /// The address of the endpoint.
    endpoint: String, // defaults to `"".to_string()`.
    /// Whether logging is enabled.
    logging_enabled: bool, // defaults to `false`.
}

// For custom defaults, implement the trait manually:
/// A configuration, with custom default values.
#[derive(Debug)]
struct Config2 {
    /// The timeout.
    timeout: u32,
    /// The number of retries.
    retries: u8,
    /// The address of the endpoint.
    endpoint: String,
    /// Whether logging is enabled.
    logging_enabled: bool,
}

impl Default for Config2 {
    fn default() -> Self {
        Self {
            timeout: 5000,
            retries: 3,
            endpoint: "https://api.example.com".to_owned(),
            logging_enabled: true,
        }
    }
}

/// Creates configurations with default values and overrides some fields.
fn main() {
    let config = Config::default();
    println!("{config:?}");
    println!(
        "{} {} {:?} {}",
        config.timeout, config.retries, config.endpoint, config.logging_enabled
    );

    let config2 = Config2::default();
    println!("{config2:?}");
    println!(
        "{} {} {} {}",
        config2.timeout,
        config2.retries,
        config2.endpoint,
        config2.logging_enabled
    );

    // We can also use struct update syntax to override specific fields:
    let custom_config = Config {
        timeout: 10000,
        ..Config::default()
    };
    println!("{}", custom_config.timeout);
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_example_entry_point() {
        main();
    }

    #[test]
    fn test_derived_default_uses_type_defaults() {
        let c = Config::default();
        assert_eq!(c.timeout, 0);
        assert_eq!(c.retries, 0);
        assert_eq!(c.endpoint, "");
        assert!(!c.logging_enabled);
    }

    #[test]
    fn test_custom_default() {
        let c = Config2::default();
        assert_eq!(c.timeout, 5000);
        assert_eq!(c.retries, 3);
        assert_eq!(c.endpoint, "https://api.example.com");
        assert!(c.logging_enabled);
    }

    #[test]
    fn test_struct_update_syntax() {
        let c = Config {
            timeout: 10000,
            ..Config::default()
        };
        assert_eq!(c.timeout, 10000);
        assert_eq!(c.retries, 0);
        assert_eq!(c.endpoint, String::new());
    }
}
