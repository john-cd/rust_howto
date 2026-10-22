// TODO Reviewed ??-??-2026
//! Command-line entry point of the `mdbook-private` preprocessor.
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
    match arguments.as_slice() {
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
