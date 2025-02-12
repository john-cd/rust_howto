mod conf;
mod regexes;

use conf::PreprocConfig;
use mdbook::BookItem;
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::Preprocessor;
use mdbook::preprocess::PreprocessorContext;
use regexes::*;
use tracing::info;
use tracing::warn;

// Inspired from https://github.com/rust-lang/mdBook/blob/master/examples/nop-preprocessor.rs

// The main preprocessor Struct
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

    fn run(
        &self,
        ctx: &PreprocessorContext,
        mut book: Book,
    ) -> Result<Book, Error> {
        info!("Running `mdbook-scrub` preprocessor");

        let conf: PreprocConfig = self.retrieve_config(&ctx.config);
        // Compile the replacement Regex(es) only once per book
        let regexes = get_regexes(&conf);

        // If the preprocessor configuration is fully disabled, return the
        // orginal book
        if !regexes.is_empty() {
            book.for_each_mut(|item: &mut BookItem| {
                if let BookItem::Chapter(ref mut chapter) = item {
                    info!("Processing chapter '{}'", chapter.name);
                    let content = &mut chapter.content;
                    for re in regexes.iter() {
                        // Remove all regex-matching sections
                        *content = re.replace_all(content, "").into_owned();
                        // tracing::debug!(content);
                    }
                };
            });
        }
        Ok(book)
    }

    // all usual renderers are supported
    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

impl Preproc {
    fn retrieve_config(&self, conf: &mdbook::Config) -> PreprocConfig {
        // Get the table associated with a particular preprocessor.
        match conf.get_preprocessor(self.name()) {
            Some(raw) => {
                // raw is toml::Table
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn preprocessor_run() {
        let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "pre": {}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.35"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
        let input_json = input_json.as_bytes();

        let (ctx, book) =
            mdbook::preprocess::CmdPreprocessor::parse_input(input_json)
                .unwrap();
        // TODO P1
        // let expected_book = book.clone();
        let result = Preproc::new().run(&ctx, book);
        assert!(result.is_ok());

        // let actual_book = result.unwrap();
        // assert_eq!(actual_book, expected_book);
    }
}
