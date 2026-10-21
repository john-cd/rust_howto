mod conf;
mod regexes;

use conf::PreprocConfig;
use mdbook_preprocessor::Preprocessor;
use mdbook_preprocessor::PreprocessorContext;
use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::book::BookItem;
use mdbook_preprocessor::errors::Result;
use regexes::*;
use tracing::info;
use tracing::warn;

// Inspired from <https://github.com/rust-lang/mdBook/blob/master/examples/nop-preprocessor.rs>

/// The main preprocessor struct.
pub struct Preproc;

impl Preproc {
    pub(crate) const NAME: &'static str = "scrub";

    /// Create a new `Preproc` object.
    pub fn new() -> Self {
        Self
    }
}

impl Default for Preproc {
    fn default() -> Self {
        Self::new()
    }
}

impl Preprocessor for Preproc {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        info!("Running `mdbook-scrub` preprocessor");

        let conf: PreprocConfig = self.retrieve_config(ctx);
        // Compile the replacement Regex(es) only once per book.
        let rrs = get_regexes_and_replacements(&conf);

        // If the preprocessor configuration is fully disabled,
        // return the orginal book.
        if !rrs.is_empty() {
            book.for_each_mut(|item: &mut BookItem| {
                if let BookItem::Chapter(chapter) = item {
                    info!("Processing chapter '{}'", chapter.name);
                    let content = &mut chapter.content;
                    for rr in rrs.iter() {
                        *content = if let Some(ref repl) = rr.replacement {
                            rr.re.replace_all(content, repl).into_owned()
                        } else {
                            // If replacement is `None`,
                            // just delete the matching text.
                            rr.re.replace_all(content, "").into_owned()
                        };
                        // tracing::debug!(content);
                    }
                };
            });
        }
        Ok(book)
    }

    // All usual renderers are supported.
    fn supports_renderer(&self, renderer: &str) -> Result<bool> {
        Ok(renderer != "not-supported")
    }
}

impl Preproc {
    fn retrieve_config(&self, ctx: &PreprocessorContext) -> PreprocConfig {
        // Deserialize the preprocessor's table through mdBook's supported Config API.
        // `get_preprocessor` was removed in mdBook 0.5.
        let config_key = format!("preprocessor.{}", self.name());
        match ctx.config.get::<PreprocConfig>(&config_key) {
            Ok(Some(config)) => config,
            Ok(None) => {
                warn!(
                    "Could not retrieve `preprocessor.{}` configuration; using defaults",
                    self.name()
                );
                PreprocConfig::default()
            }
            Err(error) => {
                warn!(
                    "Error in `preprocessor.{}` configuration: {error}; using defaults",
                    self.name()
                );
                PreprocConfig::default()
            }
        }
    }
}
