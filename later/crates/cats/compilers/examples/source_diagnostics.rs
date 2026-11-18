// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Renders the first lexer error with a highlighted source span.

use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::diagnostic::Label;
use codespan_reporting::files::Error;
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::Config;
use codespan_reporting::term::{self};
use logos::Logos;

/// The toy expression language accepts words and plus signs.
#[derive(Logos)]
#[logos(skip r"[ \t]+")]
enum Token {
    #[regex(r"[a-zé]+")]
    Word,
    #[token("+")]
    Plus,
}

/// Renders the first invalid token, or returns no report if lexing succeeds.
///
/// # Errors
///
/// Returns a diagnostic rendering error if the file or its span is invalid.
fn report(source: &str) -> Result<Option<String>, Error> {
    let Some((_, span)) = Token::lexer(source)
        .spanned()
        .find(|(token, _)| token.is_err())
    else {
        return Ok(None);
    };

    let mut files = SimpleFiles::new();
    let file = files.add("expression.txt", source);
    let diagnostic = Diagnostic::error()
        .with_message("unexpected character")
        .with_labels(vec![
            Label::primary(file, span).with_message("not a word or '+'"),
        ]);
    term::emit_into_string(&Config::default(), &files, &diagnostic).map(Some)
}

/// Prints a color-free report for an invalid expression.
fn main() -> Result<(), Error> {
    if let Some(diagnostic) = report("café + @")? {
        print!("{diagnostic}");
    }
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Uses the lexer byte span even when a multibyte character precedes it.
    #[test]
    fn highlights_invalid_character_after_utf8() -> Result<(), Error> {
        let diagnostic = report("café + @")?.expect("invalid characters need a report");
        assert!(diagnostic.contains("expression.txt:1:8"));
        assert!(diagnostic.contains("not a word or '+'"));
        assert!(report("café + tea")?.is_none());
        Ok(())
    }
}
