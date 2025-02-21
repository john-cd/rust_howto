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
| String Manipulation | `std::string`, `regex`, `unicode-segmentation` | `std::string` provides basic string operations. `regex` is for regular expressions. `unicode-segmentation` is crucial for correctly handling Unicode graphemes and word boundaries. |
| Regular Expressions | `regex`, `fancy-regex` | `regex` is the standard regex crate. `fancy-regex` provides more advanced features. |
| Parsing | `nom`, `pest`, `lalrpop`, `combine` | These crates offer different approaches to parsing, from combinators (`nom`, `combine`) to parser generators (`pest`, `lalrpop`). |
| Serialization/Deserialization | `serde`, `serde_json`, `serde_yaml`, `toml` | `serde` is a powerful framework for serialization and deserialization, often used with text-based formats like JSON, YAML, and TOML. |
| Text Encoding/Decoding | `encoding`, `iconv` (bindings) | These crates handle different character encodings. `encoding` is a pure Rust solution, while `iconv` provides bindings to the iconv library. |
| Tokenization | `tokenizer`, `unicode-segmentation` | `tokenizer` provides tools for breaking text into tokens. `unicode-segmentation` is useful for Unicode-aware word and grapheme segmentation. |
| Stemming & Lemmatization | `rust-stemmers`, `lingua` | `rust-stemmers` provides stemming algorithms. `lingua` offers more comprehensive linguistic analysis, including lemmatization. |
| Natural Language Processing (NLP) | (Many crates for specific tasks) | NLP tasks often use the crates mentioned above, along with specialized crates for things like part-of-speech tagging, named entity recognition, etc. The Rust NLP ecosystem is evolving. |
| Markdown Processing | `pulldown-cmark`, `comrak` | These crates parse and render Markdown. |
| HTML Parsing | `scraper`, `kuchiki` | These crates parse HTML documents. |
| XML Parsing | `xmltree`, `quick-xml` | These crates parse XML documents. |
| CSV Parsing | `csv` | This crate provides efficient CSV parsing. |
| Text Formatting & Templating | `minijinja`, `tera`, `handlebars`, `askama` | These crates are used for generating text-based output with dynamic content. |
| Diffing & Patching | `diff` | This crate calculates differences between text files. |
| Fuzzy Matching | `fuzzy-matcher` | This crate provides fuzzy string matching. |
| String Search | (Often uses `regex` or specialized search algorithms) | String search can be done with regular expressions or with algorithms like Aho-Corasick. |
| Command-Line Argument Parsing | `clap`, `structopt` | These crates help with parsing command-line arguments, which often involve text processing. |
| Logging | `log`, `env_logger`, `tracing` | Logging often involves formatting and processing text. |

## Key Considerations

- Unicode: Always be mindful of Unicode when processing text. Use crates like `unicode-segmentation` to handle graphemes and word boundaries correctly.
- Performance: For performance-critical tasks, consider using crates optimized for speed, such as `regex` for regular expressions or specialized parsing libraries.
- Specific Task: Choose the crate that is most appropriate for your specific text processing task. Don't use a full-fledged parsing library if you only need basic string manipulation.
- Ecosystem: The Rust text processing ecosystem is rich and evolving, so explore different options to find the best fit for your project.

</div>
