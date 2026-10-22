// TODO Reviewed ??-??-2026
//! An `mdbook` preprocessor that scrubs hidden chapters and wikilinks.
mod conf;
mod regexes;

use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use anyhow::ensure;
use conf::DirectivePreprocConfig;
use conf::PreprocConfig;
use mdbook_preprocessor::Preprocessor;
use mdbook_preprocessor::PreprocessorContext;
use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::book::BookItem;
use mdbook_preprocessor::errors::Result;
use regexes::get_regexes;
use regexes::replace_outside_protected_spans;
use regexes::wikilink_regex;
use tracing::info;

/// The stage of the preprocessor.
#[derive(Clone, Copy)]
enum Stage {
    /// Removes hidden sections and includes of hidden chapters, before mdBook
    /// links the chapters.
    Scrub,
    /// Expands custom directives, after mdBook links the chapters.
    Directives,
}

/// The book preprocessor, configured for either its pre-link scrub or directive stage.
pub struct Preproc {
    /// The stage to run.
    stage: Stage,
}

impl Preproc {
    /// Create the pre-link stage that removes hidden includes and sections.
    #[must_use]
    pub fn new() -> Self {
        Self {
            stage: Stage::Scrub,
        }
    }

    /// Create the post-link stage that expands custom directives in memory.
    #[must_use]
    pub fn directives() -> Self {
        Self {
            stage: Stage::Directives,
        }
    }

    /// Returns the key of the stage in the book configuration.
    fn config_key(&self) -> &'static str {
        match self.stage {
            Stage::Scrub => "preprocessor.scrub",
            Stage::Directives => "preprocessor.directives",
        }
    }

    /// Removes the hidden sections, hidden includes and wikilinks from the
    /// book.
    fn run_scrub(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let conf = ctx
            .config
            .get::<PreprocConfig>(self.config_key())?
            .unwrap_or_default();
        let regexes = get_regexes(&conf);

        book.for_each_mut(|item: &mut BookItem| {
            let BookItem::Chapter(chapter) = item else {
                return;
            };

            info!("Removing hidden content from chapter '{}'", chapter.name);
            if let Some(regex) = &regexes.hidden_sections {
                chapter.content = regex.replace_all(&chapter.content, "").into_owned();
            }
            if let Some(regex) = &regexes.hidden_includes {
                chapter.content = regex.replace_all(&chapter.content, "").into_owned();
            }
        });

        Ok(book)
    }

    /// Expands the directives in the book.
    fn run_directives(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let conf = ctx
            .config
            .get::<DirectivePreprocConfig>(self.config_key())?
            .unwrap_or_default();
        let book_root = resolve_book_root(&ctx.root)?;
        let source_root = book_root.join(&ctx.config.book.src);
        let mut errors = Vec::new();

        book.for_each_mut(|item: &mut BookItem| {
            let BookItem::Chapter(chapter) = item else {
                return;
            };

            info!("Expanding directives in chapter '{}'", chapter.name);
            let chapter_source = chapter
                .source_path
                .as_ref()
                .or(chapter.path.as_ref())
                .map(|path| source_root.join(path));
            let transformed = book_tool::directives::process_markdown_with_example_resolver(
                &chapter.content,
                &conf.directives,
                |example| resolve_example(&book_root, chapter_source.as_deref(), example),
            )
            .and_then(|content| {
                let content =
                    replace_outside_protected_spans(&content, &wikilink_regex(), |captures| {
                        captures
                            .get(1)
                            .map_or("", |matched| matched.as_str().trim())
                            .to_owned()
                    })?;
                book_tool::directives::validate_no_unexpanded_directives(&content)?;
                Ok(content)
            });

            match transformed {
                Ok(content) => chapter.content = content,
                Err(error) => errors.push(format!("{}: {error:#}", chapter.name)),
            }
        });

        if !errors.is_empty() {
            anyhow::bail!(
                "Failed to process custom directives:\n{}",
                errors.join("\n")
            );
        }

        Ok(book)
    }
}

impl Default for Preproc {
    fn default() -> Self {
        Self::new()
    }
}

impl Preprocessor for Preproc {
    fn name(&self) -> &str {
        self.config_key()
            .strip_prefix("preprocessor.")
            .expect("preprocessor config keys have a prefix")
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        info!("Running `{}` mdBook preprocessor", self.name());
        match self.stage {
            Stage::Scrub => self.run_scrub(ctx, book),
            Stage::Directives => self.run_directives(ctx, book),
        }
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool> {
        Ok(renderer != "not-supported")
    }
}

/// Returns the Rust snippet that the example `name` refers to, in a code fence.
fn resolve_example(
    book_root: &Path,
    chapter_source: Option<&Path>,
    name: &str,
) -> anyhow::Result<String> {
    let (path, anchor) = name
        .trim()
        .rsplit_once(':')
        .filter(|(_, anchor)| !anchor.trim().is_empty())
        .map_or((name.trim(), "example"), |(path, anchor)| {
            (path.trim(), anchor.trim())
        });
    ensure!(!path.is_empty(), "example path cannot be empty");

    let relative_path = Path::new(path);
    ensure!(
        !relative_path.is_absolute(),
        "example path must be relative to its chapter: {path}"
    );
    ensure!(
        relative_path
            .extension()
            .and_then(|extension| extension.to_str())
            == Some("rs"),
        "example path must name a Rust source file: {path}"
    );

    let chapter_source =
        chapter_source.context("example directive is in a chapter without a source file")?;
    let chapter_directory = chapter_source
        .parent()
        .context("chapter source path has no parent directory")?;
    let root = book_root
        .canonicalize()
        .with_context(|| format!("failed to resolve book root {}", book_root.display()))?;
    let source = chapter_directory
        .join(relative_path)
        .canonicalize()
        .with_context(|| {
            format!(
                "failed to resolve example {} from chapter {}",
                path,
                chapter_source.display()
            )
        })?;
    ensure!(
        source.starts_with(&root),
        "example path escapes the book root: {}",
        source.display()
    );

    let text = std::fs::read_to_string(&source)
        .with_context(|| format!("failed to read example {}", source.display()))?;
    let snippet = extract_anchor(&text, anchor).with_context(|| {
        format!(
            "failed to find anchor '{anchor}' in example {}",
            source.display()
        )
    })?;

    let fence = code_fence(&snippet);
    Ok(format!("{fence}rust\n{}\n{fence}", snippet.trim_end()))
}

/// Returns the canonical book root, falling back to the current directory.
fn resolve_book_root(root: &Path) -> anyhow::Result<PathBuf> {
    if let Ok(root) = root.canonicalize() {
        return Ok(root);
    }

    let current_dir = std::env::current_dir().context("failed to read current directory")?;
    ensure!(
        current_dir.join("book.toml").is_file(),
        "failed to resolve book root {}",
        root.display()
    );
    current_dir
        .canonicalize()
        .context("failed to resolve current book directory")
}

/// Returns the lines between the start and end markers of `anchor` in `source`.
fn extract_anchor(source: &str, anchor: &str) -> anyhow::Result<String> {
    let mut in_anchor = false;
    let mut found_start = false;
    let mut found_end = false;
    let mut lines = Vec::new();

    for line in source.lines() {
        let trimmed = line.trim_start();
        if !in_anchor
            && trimmed
                .strip_prefix("// ANCHOR:")
                .is_some_and(|value| value.trim() == anchor)
        {
            in_anchor = true;
            found_start = true;
            continue;
        }
        if in_anchor
            && trimmed
                .strip_prefix("// ANCHOR_END:")
                .is_some_and(|value| value.trim() == anchor)
        {
            found_end = true;
            break;
        }
        if in_anchor {
            lines.push(line);
        }
    }

    ensure!(found_start, "anchor start marker is missing");
    ensure!(found_end, "anchor end marker is missing");
    Ok(lines.join("\n"))
}

/// Returns a code fence longer than any run of backticks in `source`.
fn code_fence(source: &str) -> String {
    let mut longest_run = 0;
    let mut current_run = 0;
    for character in source.chars() {
        if character == '`' {
            current_run += 1;
            longest_run = longest_run.max(current_run);
        } else {
            current_run = 0;
        }
    }
    "`".repeat(longest_run.max(2) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the marker lines are not part of the snippet.
    #[test]
    fn example_anchor_excludes_marker_lines() {
        let source = "before\n// ANCHOR: example\nfn main() {}\n// ANCHOR_END: example\nafter\n";
        assert_eq!(
            extract_anchor(source, "example").expect("input should parse"),
            "fn main() {}"
        );
    }

    /// Checks that a missing marker is an error.
    #[test]
    fn example_anchor_reports_missing_markers() {
        let error =
            extract_anchor("fn main() {}\n", "example").expect_err("input should be rejected");
        assert!(error.to_string().contains("anchor start marker is missing"));
    }

    /// Checks that the fence cannot be closed by the snippet's own backticks.
    #[test]
    fn code_fence_is_longer_than_backticks_in_example() {
        assert_eq!(code_fence("let ticks = ```;"), "````");
    }

    /// Checks that a configuration of the wrong type is reported.
    #[test]
    fn malformed_directive_configuration_is_an_error() {
        use std::str::FromStr as _;

        let config = mdbook_preprocessor::config::Config::from_str(
            "[preprocessor.directives]\nprocess-example-directives = 'yes'\n",
        )
        .expect("input should parse");
        let context = PreprocessorContext::new(PathBuf::from("."), config, "html".into());
        let error = Preproc::directives()
            .run(&context, Book::new())
            .expect_err("input should be rejected");

        assert!(error.to_string().contains("preprocessor.directives"));
    }

    /// Checks that examples that cannot be found are errors.
    #[test]
    fn unresolved_example_paths_are_errors() {
        let root = PathBuf::from("test_book")
            .canonicalize()
            .expect("input should parse");
        let chapter = root.join("src/language/examples.md");
        let error = resolve_example(
            &root,
            Some(&chapter),
            "../../crates/language/examples/constants_and_statics/missing.rs:example",
        )
        .expect_err("input should be rejected");

        assert!(format!("{error:#}").contains("failed to resolve example"));
    }

    /// Checks that unknown directives are errors.
    #[test]
    fn unresolved_custom_directives_are_errors() {
        use std::str::FromStr as _;

        let config = mdbook_preprocessor::config::Config::from_str(
            "[preprocessor.directives]\nprocess-example-directives = true\n",
        )
        .expect("input should parse");
        let root = std::env::current_dir().expect("input should parse");
        let context = PreprocessorContext::new(root, config, "html".into());
        let mut book = Book::new();
        book.push_item(mdbook_preprocessor::book::Chapter::new(
            "Unknown directive",
            "{{#unknown value}}".into(),
            "unknown.md",
            Vec::new(),
        ));

        let error = Preproc::directives()
            .run(&context, book)
            .expect_err("input should be rejected");
        assert!(format!("{error:#}").contains("{{#unknown value}}"));
    }

    /// Checks that both stages support every renderer except `not-supported`.
    #[test]
    fn both_stages_report_renderer_support() {
        assert!(
            Preproc::new()
                .supports_renderer("html")
                .expect("input should parse")
        );
        assert!(
            Preproc::directives()
                .supports_renderer("markdown")
                .expect("input should parse")
        );
        assert!(
            !Preproc::directives()
                .supports_renderer("not-supported")
                .expect("input should parse")
        );
    }
}
