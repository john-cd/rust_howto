//! Styles or removes `<!--private ... -->` blocks with the preprocessor.
//!
//! Run with `cargo run -p mdbook-private --example private_blocks`.
//!
//! TODO Reviewed ??-??-2026.

use mdbook_preprocessor::Preprocessor as _;
use mdbook_preprocessor::PreprocessorContext;
use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::book::BookItem;
use mdbook_preprocessor::book::Chapter;
use mdbook_preprocessor::config::Config;
use mdbook_private::Private;
use serde_json::json;

/// Runs the preprocessor with `settings` on a one-chapter book and returns the chapter text.
fn render(settings: serde_json::Value) -> anyhow::Result<String> {
    let mut config = Config::default();
    config.set("preprocessor.private", settings)?;
    let ctx = PreprocessorContext::new(".".into(), config, "html".into());
    let mut book = Book::new();
    book.push_item(Chapter::new(
        "Ch",
        "Visible\n<!--private SECRET\n-->\nEnd".into(),
        "ch.md",
        vec![],
    ));
    let book = Private.run(&ctx, book)?;
    match book.items.into_iter().next() {
        Some(BookItem::Chapter(chapter)) => Ok(chapter.content),
        _ => anyhow::bail!("chapter missing"),
    }
}

/// Runs the example.
fn main() -> anyhow::Result<()> {
    println!("{}", render(json!({ "remove": true }))?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Removing drops the secret; styling keeps it.
    #[test]
    fn remove_drops_secret() {
        main().expect("example should succeed");
        assert!(
            !render(json!({ "remove": true }))
                .expect("render")
                .contains("SECRET")
        );
        assert!(
            render(json!({ "remove": false }))
                .expect("render")
                .contains("SECRET")
        );
    }
}
