// ANCHOR: example
mod mylib {
    //! library code: define unique error types and error wrappers

    use miette::Diagnostic;
    use miette::NamedSource;
    use miette::Result;
    use miette::SourceSpan;

    // You can derive a `Diagnostic` from any `std::error::Error` type.
    // `thiserror` plays nicely with `miette`
    use thiserror::Error;

    #[derive(Error, Diagnostic, Debug)]
    pub enum MyLibError {

        #[error("A bad thing happened!")] // provided by `thisError`
        #[diagnostic(
            // Use `#[diagnostic(code(...))]` to set the unique code for this error.
            code(my_lib::bad_thing),
            // Set the URL that will be displayed as an actual link in supported terminals.
            // `url(docsrs)` automatically create a link to this diagnostic on docs.rs
            // or use a custom URL like `url("https://my_website.com/error_codes#{}", self.code)`
            url(docsrs),
            // Supply help text
            help("try doing it better next time?"))]
        BadThingHappened,

        #[error("Something went wrong!")]
        SomethingWentWrong {
            // The Source that we're gonna be printing snippets out of.
            // This can be a String if you don't have or care about file names.
            #[source_code]
            src: NamedSource<String>,

            // Snippets and highlights can be included in the diagnostic!
            // You may also use `(usize, usize)`, the byte-offset and length
            // into an associated SourceCode or
            // `Option<SourceSpan>`
            #[label("This bit highlighted here is the problem")]
            bad_bit: SourceSpan,

            // Programmatically supply the help text
            #[help]
            advice: Option<String>, // Can also be `String`

            // Related errors
            #[related]
            others: Vec<MyLibError>,
        },

        // Wrap an Error
        #[error(transparent)]
        // Forward the source and Display methods
        // straight through to the underlying error.
        #[diagnostic(code(my_lib::io_error))]
        IoError(#[from] std::io::Error),

        // Wrap another Diagnostic
        // Use `#[diagnostic(transparent)]` to wrap another `[Diagnostic]`
        // You won't see labels otherwise
        #[error(transparent)]
        #[diagnostic(transparent)]
        AnotherError(#[from] AnotherError),
    }

    #[derive(Error, Diagnostic, Debug)]
    #[error("another error")]
    pub struct AnotherError {

        #[label("here")]
        pub at: SourceSpan,
    
    }

    pub fn this_fails() -> Result<()> {
        // You can use plain strings as a `Source`,
        // or anything that implements the one-method `Source` trait.
        let src = "source\n  text\n    here".to_string();
        // You may also use `map_err(|error| {
        // error.with_source_code(String::from("source code")) })` later.

        Err(MyLibError::SomethingWentWrong {
            src: NamedSource::new("bad_file.rs", src),
            bad_bit: (9, 4).into(),
            advice: Some("Some help text".to_string()),
            others: vec![MyLibError::BadThingHappened],
        })?;
        Ok(())
    }
}

use miette::Result;

/// To get errors printed nicely in application code, just return a
/// `Result<()>`
///
/// Note: You can swap out the default reporter for a
/// custom one using `miette::set_hook()`
fn main() -> Result<()> {
    mylib::this_fails()?;
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    assert!(main().is_err());
}
