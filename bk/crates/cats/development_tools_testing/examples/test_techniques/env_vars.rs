// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Reads configuration from the process environment without mutating it in
//! tests.

use std::ffi::OsString;

const DEFAULT_LOG_LEVEL: &str = "info";

/// Typed configuration parsed from an injected environment value.
#[derive(Debug, PartialEq, Eq)]
struct Config {
    /// The selected logging verbosity.
    log_level: String,
}

impl Config {
    /// Parses an optional `APP_LOG` value, defaulting to `info`.
    fn from_value(value: Option<&str>) -> Result<Self, &'static str> {
        let log_level = value.unwrap_or(DEFAULT_LOG_LEVEL);
        match log_level {
            "error" | "warn" | "info" | "debug" | "trace" => Ok(Self {
                log_level: log_level.to_owned(),
            }),
            _ => Err("APP_LOG must be error, warn, info, debug, or trace"),
        }
    }

    /// Reads and validates `APP_LOG` from this process's environment.
    fn from_process_env() -> Result<Self, String> {
        let value = std::env::var_os("APP_LOG")
            .map(OsString::into_string)
            .transpose()
            .map_err(|_| "APP_LOG must contain valid Unicode".to_owned())?;
        Self::from_value(value.as_deref()).map_err(str::to_owned)
    }
}

/// Prints the configured log level.
fn main() -> Result<(), String> {
    let config = Config::from_process_env()?;
    println!("Log level: {}", config.log_level);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::*;

    /// Exercises injected values without changing the test process environment.
    #[test]
    fn parses_set_absent_invalid_and_default_values() {
        assert_eq!(
            Config::from_value(Some("debug")),
            Ok(Config {
                log_level: "debug".to_owned()
            })
        );
        assert_eq!(
            Config::from_value(None),
            Ok(Config {
                log_level: "info".to_owned()
            })
        );
        assert!(Config::from_value(Some("verbose")).is_err());
        assert_eq!(
            Config::from_value(Some(DEFAULT_LOG_LEVEL)),
            Ok(Config {
                log_level: "info".to_owned()
            })
        );
    }

    /// Runs this test in child processes with controlled environment values.
    #[test]
    fn process_environment_cases_are_isolated() -> std::io::Result<()> {
        let cases = [
            (Some("debug"), "debug"),
            (None, "info"),
            (Some("verbose"), "invalid"),
            (Some("info"), "info"),
        ];

        for (value, expected) in cases {
            let mut command = Command::new(std::env::current_exe()?);
            command
                .args([
                    "--exact",
                    "tests::child_checks_controlled_environment",
                    "--ignored",
                ])
                .env("APP_LOG_EXPECTED", expected);
            command.env_remove("APP_LOG");
            if let Some(value) = value {
                command.env("APP_LOG", value);
            } else {
                command.env_remove("APP_LOG");
            }

            let output = command.output()?;
            assert!(
                output.status.success(),
                "child environment case {expected:?} failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(())
    }

    /// Checks the child process environment selected by the parent test.
    #[test]
    #[ignore = "spawned by the parent test with a controlled environment"]
    fn child_checks_controlled_environment() {
        let expected = std::env::var("APP_LOG_EXPECTED")
            .expect("the parent supplies the expected result");
        if expected == "invalid" {
            assert!(Config::from_process_env().is_err());
            assert!(main().is_err());
        } else {
            let config = Config::from_process_env()
                .expect("the child environment should parse");
            assert_eq!(config.log_level, expected);
            assert!(main().is_ok());
        }
    }
}
// ANCHOR_END: example
