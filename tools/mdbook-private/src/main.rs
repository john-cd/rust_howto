//! Command-line entry point of the `mdbook-private` preprocessor.
//!
//! TODO Reviewed ??-??-2026.

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. See LICENSE for details.
use std::env;
use std::io;
use std::process;

use mdbook_preprocessor::Preprocessor as _;
use mdbook_private::Private;

/// Runs the preprocessor, or answers the `supports` query of mdBook.
fn run() -> mdbook_preprocessor::errors::Result<()> {
    let arguments: Vec<_> = env::args().skip(1).collect();
    run_with(&arguments)
}

/// Runs the preprocessor for the given command-line arguments.
fn run_with(arguments: &[String]) -> mdbook_preprocessor::errors::Result<()> {
    match arguments {
        [command, renderer] if command == "supports" => {
            process::exit(i32::from(!Private.supports_renderer(renderer)?));
        }
        [] => {
            let (context, book) = mdbook_preprocessor::parse_input(io::stdin())?;
            let processed = Private.run(&context, book)?;
            serde_json::to_writer(io::stdout().lock(), &processed)?;
        }
        _ => {
            return Err(mdbook_preprocessor::errors::Error::msg(
                "usage: mdbook-private [supports RENDERER]",
            ));
        }
    }
    Ok(())
}

/// Runs the preprocessor and reports its error, if any.
fn main() {
    if let Err(error) = run() {
        eprintln!("mdbook-private: {error:#}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that unexpected arguments are rejected with a usage message.
    #[test]
    fn rejects_unknown_arguments() {
        let error = run_with(&["--bogus".to_owned()]).expect_err("usage error");
        assert!(error.to_string().contains("usage: mdbook-private"));
    }

    /// Checks that `supports` needs a renderer.
    #[test]
    fn supports_without_renderer_is_rejected() {
        assert!(run_with(&["supports".to_owned()]).is_err());
    }
}
