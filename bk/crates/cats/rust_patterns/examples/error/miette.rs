// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to define custom error types using `miette`
//! and `thiserror`. It showcases various features like error codes, URLs, help
//! text, source code snippets, and wrapping other errors.

/// Example code for a library.
/// In real-life, this code would be in a separate crate.
///
/// We define unique error types and error wrappers below.
mod mylib {

    use miette::Diagnostic;
    use miette::NamedSource;
    use miette::Result;
    use miette::SourceSpan;
    // You can derive a `Diagnostic` from any `std::error::Error` type.
    // `thiserror` plays nicely with `miette`.
    use thiserror::Error;

    /// The errors of the library.
    #[derive(Error, Diagnostic, Debug)]
    pub enum MyLibError {
        #[error("A bad thing happened!")] // Provided by `thisError`.
        #[diagnostic(
            // Use `#[diagnostic(code(...))]` to set the unique code for this error.
            code(my_lib::bad_thing),
            // Set the URL that will be displayed as an actual link in supported terminals.
            // `url(docsrs)` automatically create a link to this diagnostic on docs.rs
            // or use a custom URL like `url("https://my_website.com/error_codes#{}", self.code)`
            url(docsrs),
            // Supply help text:
            help("try doing it better next time?"))]
        /// A bad thing happened.
        BadThingHappened,

        /// Something went wrong at a location in the source code.
        #[error("Something went wrong!")]
        SomethingWentWrong {
            // The `Source` that we're gonna be printing snippets out of.
            // This can be a `String` if we don't have or care about file
            // names.
            /// The source code to print snippets from.
            #[source_code]
            src: NamedSource<String>,

            // Snippets and highlights can be included in the diagnostic!
            // You may also use `(usize, usize)`, the byte-offset and length
            // into an associated `SourceCode` or
            // `Option<SourceSpan>`.
            /// The part of the source code that is highlighted.
            #[label("This bit highlighted here is the problem")]
            bad_bit: SourceSpan,

            // Programmatically supply the help text.
            /// Help text supplied at run time.
            #[help]
            advice: Option<String>, // Can also be `String`.

            // Related errors.
            /// Related errors.
            #[related]
            others: Vec<Self>,
        },

        // Wrap an Error:
        #[error(transparent)]
        // Forward the source and `Display` methods
        // straight through to the underlying error.
        /// An I/O error.
        #[diagnostic(code(my_lib::io_error))]
        IoError(#[from] std::io::Error),

        // Wrap another `Diagnostic`.
        // Use `#[diagnostic(transparent)]` to wrap another `[Diagnostic]`
        // You won't see labels otherwise.
        /// Another error, which is itself a diagnostic.
        #[error(transparent)]
        #[diagnostic(transparent)]
        AnotherError(#[from] AnotherError),
    }

    /// Another error:
    #[derive(Error, Diagnostic, Debug)]
    #[error("another error")]
    pub struct AnotherError {
        /// The location of the error in the source code.
        #[label("here")]
        pub at: SourceSpan,
    }

    /// Example function that fails.
    /// Returns an error with source-code context to demonstrate Miette
    /// diagnostics.
    #[expect(
        unreachable_code,
        reason = "the example returns an error on purpose"
    )]
    pub fn this_fails() -> Result<()> {
        // You can use plain strings as a `Source`,
        // or anything that implements the one-method `Source` trait.
        let src = "source\n  text\n    here".to_owned();
        // You may also use `map_err(|error| {
        // error.with_source_code(String::from("source code")) })` later.

        return Err(MyLibError::SomethingWentWrong {
            src: NamedSource::new("bad_file.rs", src),
            bad_bit: (9, 4).into(),
            advice: Some("Some help text".to_owned()),
            others: vec![MyLibError::BadThingHappened],
        }
        .into());
        Ok(())
    }
}

use miette::Result;

/// Runs the miette example.
fn main() -> Result<()> {
    mylib::this_fails()?;
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the miette example's main path succeeds.
    /// Runs the miette example through its entry point.
    #[test]
    fn test_miette_example_entry_point() {
        let res = main();
        println!("{res:?}");
        assert!(res.is_err());
    }
}
