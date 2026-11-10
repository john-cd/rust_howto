//! Runs the `mdbook-scrub` pre-link stage on an in-memory book.
//!
//! The example needs no files or services: it removes a hidden HTML section and
//! an include of a hidden chapter file, and prints the scrubbed chapter.
//!
//! Run with:
//! ```bash
//! cargo run -p mdbook-scrub --example scrub_chapter
//! ```
//!
//! TODO Reviewed ??-??-2026.

use mdbook_preprocessor::Preprocessor as _;
use mdbook_preprocessor::PreprocessorContext;
use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::book::BookItem;
use mdbook_preprocessor::book::Chapter;
use mdbook_preprocessor::config::Config;
use mdbook_scrub::Preproc;

/// Markdown with one hidden section and one hidden include between visible text.
const SAMPLE: &str = "Visible intro.\n\n<div class=\"hidden\">secret notes</div>\n\n{{#include _hidden.rs}}\n\nVisible outro.\n";

#[expect(
    clippy::print_stdout,
    reason = "Example reports the scrubbed chapter on standard output"
)]
fn main() -> anyhow::Result<()> {
    let mut book = Book::new();
    book.push_item(Chapter::new(
        "Sample",
        SAMPLE.into(),
        "sample.md",
        Vec::new(),
    ));
    let context = PreprocessorContext::new(".".into(), Config::default(), "html".into());
    let scrubbed = Preproc::new()
        .run(&context, book)
        .map_err(|error| anyhow::anyhow!("{error:#}"))?;
    for item in scrubbed.iter() {
        if let BookItem::Chapter(chapter) = item {
            println!("--- {} ---\n{}", chapter.name, chapter.content);
        }
    }
    Ok(())
}
