# Text Processing

Deal with the complexities of human language when expressed in textual form.

[![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}

## Find, extract, and replace text using regular expressions

{{#include regex.incl.md}}

## Parse strings

{{#include string_parsing.incl.md}}

## Concatenate strings

{{#include string_concat.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P1 review](https://github.com/john-cd/rust_howto/issues/963)

This table outlines common text processing tasks and relevant Rust crates.

| Topic | Rust Crates | Notes |
|---|---|---|
| String Manipulation | `std::string`, [`regex`][c-regex]⮳{{hi:regex}}, [`unicode-segmentation`][c-unicode_segmentation]⮳{{hi:unicode-segmentation}} | `std::string` provides basic string operations. [`regex`][c-regex]⮳{{hi:regex}} is for regular expressions. [`unicode-segmentation`][c-unicode_segmentation]⮳{{hi:unicode-segmentation}} is crucial for correctly handling Unicode graphemes and word boundaries. |
| Regular Expressions | [`regex`][c-regex]⮳{{hi:regex}}, [`fancy-regex`][c-fancy_regex]⮳{{hi:fancy-regex}} | [`regex`][c-regex]⮳{{hi:regex}} is the standard regex crate. [`fancy-regex`][c-fancy_regex]⮳{{hi:fancy-regex}} provides more advanced features. |
| Parsing | [`nom`][c-nom]⮳{{hi:nom}}, [`pest`][c-pest]⮳{{hi:pest}}, [`lalrpop`][c-lalrpop]⮳{{hi:lalrpop}}, [`combine`][c-combine]⮳{{hi:combine}} | These crates offer different approaches to parsing, from combinators ([`nom`][c-nom]⮳{{hi:nom}}, [`combine`][c-combine]⮳{{hi:combine}}) to parser generators ([`pest`][c-pest]⮳{{hi:pest}}, [`lalrpop`][c-lalrpop]⮳{{hi:lalrpop}}). |
| Serialization/Deserialization | [`serde`][c-serde]⮳{{hi:serde}}, [`serde_json`][c-serde_json]⮳{{hi:serde_json}}, [`serde_yml`][c-serde_yml]⮳{{hi:serde_yml}}, [`toml`][c-toml]⮳{{hi:toml}} | [`serde`][c-serde]⮳{{hi:serde}} is a powerful framework for serialization and deserialization, often used with text-based formats like JSON, YAML, and TOML. |
| Text Encoding/Decoding | [`encoding`][c-encoding]⮳{{hi:encoding}}, [`iconv`][c-iconv]⮳{{hi:iconv}} (bindings) | These crates handle different character encodings. [`encoding`][c-encoding]⮳{{hi:encoding}} is a pure Rust solution, while [`iconv`][c-iconv]⮳{{hi:iconv}} provides bindings to the iconv library. |
| Tokenization | [`tokenizer`][c-tokenizer]⮳{{hi:tokenizer}}, [`unicode-segmentation`][c-unicode_segmentation]⮳{{hi:unicode-segmentation}} | [`tokenizer`][c-tokenizer]⮳{{hi:tokenizer}} provides tools for breaking text into tokens. [`unicode-segmentation`][c-unicode_segmentation]⮳{{hi:unicode-segmentation}} is useful for Unicode-aware word and grapheme segmentation. |
| Stemming & Lemmatization | [`rust-stemmers`][c-rust_stemmers]⮳{{hi:rust-stemmers}}, [`lingua`][c-lingua]⮳{{hi:lingua}} | [`rust-stemmers`][c-rust_stemmers]⮳{{hi:rust-stemmers}} provides stemming algorithms. [`lingua`][c-lingua]⮳{{hi:lingua}} offers more comprehensive linguistic analysis, including lemmatization. |
| Natural Language Processing (NLP) | (Many crates for specific tasks) | NLP tasks often use the crates mentioned above, along with specialized crates for things like part-of-speech tagging, named entity recognition, etc. The Rust NLP ecosystem is evolving. |
| Markdown Processing | [`pulldown-cmark`][c-pulldown_cmark]⮳{{hi:pulldown-cmark}}, [`comrak`][c-comrak]⮳{{hi:comrak}} | These crates parse and render Markdown. |
| HTML Parsing | [`scraper`][c-scraper]⮳{{hi:scraper}}, [`kuchiki`][c-kuchiki]⮳{{hi:kuchiki}} | These crates parse HTML documents. |
| XML Parsing | [`xmltree`][c-xmltree]⮳{{hi:xmltree}}, [`quick-xml`][c-quick_xml]⮳{{hi:quick-xml}} | These crates parse XML documents. |
| CSV Parsing | [`csv`][c-csv]⮳{{hi:csv}} | This crate provides efficient CSV parsing. |
| Text Formatting & Templating | [`minijinja`][c-minijinja]⮳{{hi:minijinja}}, [`tera`][c-tera]⮳{{hi:tera}}, [`handlebars`][c-handlebars]⮳{{hi:handlebars}}, [`askama`][c-askama]⮳{{hi:askama}} | These crates are used for generating text-based output with dynamic content. |
| Diffing & Patching | [`diff`][c-diff]⮳{{hi:diff}} | This crate calculates differences between text files. |
| Fuzzy Matching | [`fuzzy-matcher`][c-fuzzy_matcher]⮳{{hi:fuzzy-matcher}} | This crate provides fuzzy string matching. |
| String Search | (Often uses [`regex`][c-regex]⮳{{hi:regex}} or specialized search algorithms) | String search can be done with regular expressions or with algorithms like Aho-Corasick. |
| Command-Line Argument Parsing | [`clap`][c-clap]⮳{{hi:clap}}, [`structopt`][c-structopt]⮳{{hi:structopt}} | These crates help with parsing command-line arguments, which often involve text processing. |
| Logging | [`log`][c-log]⮳{{hi:log}}, [`env_logger`][c-env_logger]⮳{{hi:env_logger}}, [`tracing`][c-tracing]⮳{{hi:tracing}} | Logging often involves formatting and processing text. |

## Key Considerations

- Unicode: Always be mindful of Unicode when processing text. Use crates like [`unicode-segmentation`][c-unicode_segmentation]⮳{{hi:unicode-segmentation}} to handle graphemes and word boundaries correctly.
- Performance: For performance-critical tasks, consider using crates optimized for speed, such as [`regex`][c-regex]⮳{{hi:regex}} for regular expressions or specialized parsing libraries.
- Specific Task: Choose the crate that is most appropriate for your specific text processing task. Don't use a full-fledged parsing library if you only need basic string manipulation.
- Ecosystem: The Rust text processing ecosystem is rich and evolving, so explore different options to find the best fit for your project.

</div>
