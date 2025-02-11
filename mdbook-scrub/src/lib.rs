#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod conf;
mod hidden;
mod links;
mod refdefs;

use hidden::*;
use links::*;
use refdefs::*;

use anyhow::Context;
use conf::PreprocConfig;
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::{BookItem, Config};
use std::borrow::Cow;
use std::io::Write;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::trace;
use tracing::warn;

// Inspired from https://github.com/rust-lang/mdBook/blob/master/examples/nop-preprocessor.rs

// The main preprocessor Struct
pub struct Preproc;

impl Preproc {
    pub(crate) const NAME: &'static str = "scrub";
    pub fn new() -> Preproc {
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

        let preprocconf: PreprocConfig = self.retrieve_config(&ctx.config);

        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = *item {
                info!("Processing chapter '{}'", &chapter.name);
                chapter.content = process(&preprocconf, &chapter.content);
            };
        });
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

fn process(conf: &PreprocConfig, content: &str) -> String {
    let mut output = Cow::from(content);
    if conf.remove_hidden_sections {
        remove_hidden_sections(&mut output);
    }
    if conf.do_not_include_hidden_chapters {
        remove_include_hidden_chapters(&mut output, conf.hidden_chapter_prefix.as_str());
    }
    // FUTURE
    // if conf.check_urls {
    //     check_links(content, conf.check_external_urls);
    // }
    // if conf.detect_unused_reference_definitions {
    //     let unused_refs = detect_unused_reference_definitions(content);
    //     if conf.delete_unused_reference_definitions {
    //          delete_unused_reference_definitions(content, unused_refs);
    //     }
    // }
    output.into_owned()
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

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        // TODO P1
        //let expected_book = book.clone();
        let result = Preproc::new().run(&ctx, book);
        assert!(result.is_ok());

        // let actual_book = result.unwrap();
        // assert_eq!(actual_book, expected_book);
    }
}
