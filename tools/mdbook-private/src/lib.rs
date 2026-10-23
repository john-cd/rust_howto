// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
// Adapted from https://github.com/RealAtix/mdbook-private (Razvan Nicolescu).
//! mdBook 0.5-compatible private chapter and comment filtering.
//!
//! TODO Reviewed ??-??-2026.

use std::sync::LazyLock;

use mdbook_preprocessor::Preprocessor;
use mdbook_preprocessor::PreprocessorContext;
use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::book::BookItem;
use mdbook_preprocessor::book::SectionNumber;
use mdbook_preprocessor::errors::Result;
use regex::Captures;
use regex::Regex;
use serde::Deserialize;

/// Preserve the upstream configuration keys and defaults.
#[derive(Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct Config {
    /// Whether to remove private chapters instead of keeping them.
    remove: bool,
    /// Whether to style the private content.
    style: bool,
    /// Notice displayed on private content.
    notice: String,
    /// Prefix of the file names of private chapters.
    chapter_prefix: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            remove: false,
            style: true,
            notice: "CONFIDENTIAL".to_owned(),
            chapter_prefix: "_".to_owned(),
        }
    }
}

/// Filters private content before includes and rendering.
pub struct Private;

impl Preprocessor for Private {
    fn name(&self) -> &'static str {
        "private"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        // Invalid privacy settings must fail the build rather than expose content.
        let config = ctx
            .config
            .get::<Config>("preprocessor.private")?
            .unwrap_or_default();
        /// Matches private code blocks that should be removed from the rendered book.
        static BLOCK: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"<!--\s*private\b\s*[\r?\n]?((?s).*?)[\r?\n]?\s*-->[\r?\n]?")
                .expect("valid private comment expression")
        });
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                chapter.content = BLOCK.replace_all(&chapter.content, |captures: &Captures<'_>| {
                    if config.remove {
                        String::new()
                    } else if config.style {
                        format!(
                            "<blockquote style='position: relative; padding: 20px 20px;'><span style='position: absolute; top: 0; right: 5px; font-size: 80%; opacity: 0.4;'>{}</span>{}</blockquote>\n",
                            config.notice, &captures[1]
                        )
                    } else {
                        format!("{}\n", &captures[1])
                    }
                }).into_owned();
            }
        });
        if config.remove {
            remove_chapters(&mut book.items, &config.chapter_prefix);
            renumber(&mut book.items, &mut Vec::new());
        }
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool> {
        Ok(renderer != "not-supported")
    }
}

/// Removes the chapters, at any depth, whose file name starts with `prefix`.
fn remove_chapters(items: &mut Vec<BookItem>, prefix: &str) {
    items.retain_mut(|item| {
        if let BookItem::Chapter(chapter) = item {
            if chapter
                .source_path
                .as_ref()
                .and_then(|path| path.file_name())
                .is_some_and(|name| name.to_string_lossy().starts_with(prefix))
            {
                return false;
            }
            // Keep draft chapters and part titles; filter nested private chapters.
            remove_chapters(&mut chapter.sub_items, prefix);
        }
        true
    });
}

/// Renumbers the chapters that have a number, starting at 1 under `parent`.
fn renumber(items: &mut [BookItem], parent: &mut Vec<u32>) {
    let mut next = 1;
    for item in items {
        if let BookItem::Chapter(chapter) = item {
            if chapter.number.is_some() {
                parent.push(next);
                chapter.number = Some(SectionNumber::new(parent.clone()));
                renumber(&mut chapter.sub_items, parent);
                parent.pop();
                next += 1;
            } else {
                renumber(&mut chapter.sub_items, parent);
            }
        }
    }
}
