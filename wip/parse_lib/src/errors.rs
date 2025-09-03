//! Custom parsing error.
//!
//! See <https://docs.rs/winnow/0.7.11/winnow/_tutorial/chapter_7/index.html>.

use annotate_snippets::AnnotationKind;
use annotate_snippets::Group;
use annotate_snippets::Level;
use annotate_snippets::Renderer;
use annotate_snippets::Snippet;
use winnow::error::ContextError;
use winnow::error::ParseError;

/// Custom parsing error.
#[derive(Debug)]
pub struct ParsingError {
    message: String,
    // Byte spans are tracked, rather than line and column.
    // This makes it easier to operate on programmatically
    // and doesn't limit us to one definition for column count
    // which can depend on the output medium and application.
    span: std::ops::Range<usize>,
    input: String,
}

impl std::error::Error for ParsingError {}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message =
            &[
                Group::with_title(annotate_snippets::Level::ERROR.primary_title(&self.message))
                    .element(
                        annotate_snippets::Snippet::source(&self.input)
                            .fold(true)
                            .annotation(AnnotationKind::Primary.span(self.span.clone())),
                    ),
            ];
        let renderer = annotate_snippets::Renderer::styled();
        let rendered = renderer.render(message);
        rendered.fmt(f)
    }
}

impl ParsingError {
    /// Convert from `winnow::error::ParseError`.
    ///
    /// Avoiding `From`, so `winnow` types don't become part of our public API.
    pub fn from_parse(error: ParseError<&str, ContextError>) -> Self {
        // The default renderer for `ContextError` is still used but that can be
        // customized as well to better fit your needs.
        let message = error.inner().to_string();
        let input = (*error.input()).to_owned();
        // Assume the error span is only for the first `char`.
        let span = error.char_span();
        Self {
            message,
            span,
            input,
        }
    }
}
