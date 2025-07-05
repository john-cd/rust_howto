mod conf;
mod regexes;

use conf::PreprocConfig;
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::Preprocessor;
use mdbook::preprocess::PreprocessorContext;
use mdbook::BookItem;
use regexes::*;
use tracing::info;
use tracing::warn;

// Inspired from <https://github.com/rust-lang/mdBook/blob/master/examples/nop-preprocessor.rs>

// The main preprocessor struct.
pub struct Preproc;

impl Preproc {
    pub(crate) const NAME: &'static str = "scrub";

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

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        info!("Running `mdbook-scrub` preprocessor");

        let conf: PreprocConfig = self.retrieve_config(&ctx.config);
        // Compile the replacement Regex(es) only once per book.
        let rrs = get_regexes_and_replacements(&conf);

        // If the preprocessor configuration is fully disabled,
        // return the orginal book.
        if !rrs.is_empty() {
            book.for_each_mut(|item: &mut BookItem| {
                if let BookItem::Chapter(ref mut chapter) = item {
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
    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

impl Preproc {
    fn retrieve_config(&self, conf: &mdbook::Config) -> PreprocConfig {
        // Get the table associated with a particular preprocessor.
        match conf.get_preprocessor(self.name()) {
            Some(raw) => {
                // `raw` is `toml::Table`.
                let s = toml::to_string(&raw).expect("toml::to_string failed!");
                let pc: PreprocConfig = match toml::from_str(&s) {
                    Ok(pc) => pc,
                    Err(e) => {
                        warn!(
                            "Error in `preprocessor.{}` configuration: {}; using defaults",
                            self.name(),
                            e
                        );
                        PreprocConfig::default()
                    }
                };
                pc
            }
            None => {
                warn!(
                    "Could not retrieve `preprocessor.{}` configuration; using defaults",
                    self.name()
                );
                PreprocConfig::default()
            }
        }
    }
}
