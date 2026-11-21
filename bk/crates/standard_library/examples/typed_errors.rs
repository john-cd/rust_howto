// TODO Reviewed ??-??-2026.
//! Typed library errors and an application diagnostic boundary.

use std::io::Write;
use std::io::{self};
use std::process::ExitCode;

use anyhow::Context;
use thiserror::Error;

// ANCHOR: example
/// Distinguishes malformed or out-of-range counts from the forbidden zero value.
#[derive(Debug, Error)]
enum CountError {
    /// Parsing failed because the input is not a representable unsigned integer.
    #[error("count must be a positive integer")]
    InvalidInteger(
        /// Underlying error preserved by the InvalidInteger case.
        #[source]
        std::num::ParseIntError,
    ),
    /// Parsing succeeded, but zero is outside the application domain.
    #[error("count must be greater than zero")]
    Zero,
}

/// Parses a positive u32 while preserving the original integer parsing error.
fn parse_count(input: &str) -> Result<u32, CountError> {
    let count = input.parse().map_err(CountError::InvalidInteger)?;
    if count == 0 {
        return Err(CountError::Zero);
    }
    Ok(count)
}

/// Doubles a parsed count, adding input context and rejecting arithmetic overflow.
fn calculate(input: &str) -> anyhow::Result<String> {
    let count = parse_count(input).with_context(|| format!("reading count from {input:?}"))?;
    let doubled = count
        .checked_mul(2)
        .context("count is too large to double")?;
    Ok(format!("twice the count: {doubled}"))
}

/// Writes a result to stdout or a contextual error chain to stderr.
///
/// Missing or invalid input produces a failure exit code. Writer failures remain
/// I/O errors so the application boundary can report them separately.
fn run(input: Option<&str>, out: &mut impl Write, err: &mut impl Write) -> io::Result<ExitCode> {
    let Some(input) = input else {
        writeln!(err, "usage: typed_errors <positive-count>")?;
        return Ok(ExitCode::FAILURE);
    };
    match calculate(input) {
        Ok(message) => {
            writeln!(out, "{message}")?;
            Ok(ExitCode::SUCCESS)
        }
        Err(error) => {
            writeln!(err, "error: {error}")?;
            for cause in error.chain().skip(1) {
                writeln!(err, "  caused by: {cause}")?;
            }
            Ok(ExitCode::FAILURE)
        }
    }
}
// ANCHOR_END: example

/// Connects the first CLI argument and process streams to the diagnostic boundary.
fn main() -> io::Result<ExitCode> {
    run(
        std::env::args().nth(1).as_deref(),
        &mut io::stdout(),
        &mut io::stderr(),
    )
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use std::error::Error as _;

    use super::*;

    /// Checks that library error retains a source.
    #[test]
    fn library_error_retains_a_source() {
        let error = parse_count("not-a-number").expect_err("invalid input must fail");
        assert!(matches!(error, CountError::InvalidInteger(_)));
        assert!(error.source().is_some());
    }

    /// Checks that cli success is readable.
    #[test]
    fn cli_success_is_readable() -> io::Result<()> {
        let (mut out, mut err) = (Vec::new(), Vec::new());
        assert_eq!(run(Some("4"), &mut out, &mut err)?, ExitCode::SUCCESS);
        assert_eq!(out, b"twice the count: 8\n");
        assert!(err.is_empty());
        Ok(())
    }

    /// Checks that cli failure is nonzero and shows chain.
    #[test]
    fn cli_failure_is_nonzero_and_shows_chain() -> io::Result<()> {
        let (mut out, mut err) = (Vec::new(), Vec::new());
        assert_eq!(run(Some("oops"), &mut out, &mut err)?, ExitCode::FAILURE);
        assert!(out.is_empty());
        let text = String::from_utf8(err).expect("diagnostic is UTF-8");
        assert!(text.contains("reading count from \"oops\""));
        assert!(text.contains("count must be a positive integer"));
        assert!(text.contains("caused by:"));
        Ok(())
    }
}
