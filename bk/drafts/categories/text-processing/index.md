# Text Processing

[![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Deal with the complexities of human language when expressed in textual form.

This table outlines common text processing tasks and relevant Rust crates.

| Topic | Rust Crates | Notes |
|---|---|---|
| String Concatenation | `std::string`, `joinery` | [`joinery`][c~joinery~docs]↗{{hi:joinery}} is a small crate for generically joining iterators with a separator. |
| String Manipulation | `std::string`, `heck` | `std::string` provides basic string operations. [`heck`][c~heck~docs]↗{{hi:heck}} is a case conversion library. [`textwrap`][c~textwrap~docs]↗{{hi:textwrap}} provides word wrapping, indenting, and dedenting strings. |
| Regular Expressions | [`regex`][c~regex~docs]↗{{hi:regex}}, [`fancy-regex`][c~fancy_regex~docs]↗{{hi:fancy-regex}} | [`regex`][c~regex~docs]↗{{hi:regex}} is the standard Regex crate. [`fancy-regex`][c~fancy_regex~docs]↗{{hi:fancy-regex}} provides more advanced features. |
| String Search | [`aho-corasick`][c~aho_corasick~docs]↗{{hi:aho-corasick}}, [`memchr`][c~memchr~docs]↗{{hi:memchr}} | String search can be done with regular expressions or with algorithms like Aho-Corasick. |
| Fuzzy Matching | [`fuzzy-matcher`][c~fuzzy_matcher~docs]↗{{hi:fuzzy-matcher}}, `strsim` | [`fuzzy-matcher`][c~fuzzy_matcher~docs]↗{{hi:fuzzy-matcher}} provides fuzzy string matching. [`strsim`][c~strsim~docs]↗{{hi:strsim}} implement string similarity metrics. |
| Diffing & Patching | [`diff`][c~diff~docs]↗{{hi:diff}}, [`similar`][c~similar~docs]↗{{hi:similar}} | These crates calculate differences between text files. |
| OS, C, and other strings | `std::ffi`, `bstr` | `std::ffi` provides types for platform-native strings and C-style, NUL-terminated strings. [`bstr`][c~bstr~docs]↗{{hi:bstr}} offers a string type that is not required to be valid UTF-8. |
| Unicode handling | [`unicode-segmentation`][c~unicode_segmentation~docs]↗{{hi:unicode-segmentation}} | [`unicode-segmentation`][c~unicode_segmentation~docs]↗{{hi:unicode-segmentation}} correctly handles Unicode graphemes and word boundaries. |

## Key Considerations

- Always be mindful of Unicode when processing text. Use crates like [`unicode-segmentation`][c~unicode_segmentation~docs]↗{{hi:unicode-segmentation}} to handle graphemes and word boundaries correctly.
- Choose the crate that is most appropriate for your specific text processing task. Don't use a full-fledged parsing library if you only need basic string manipulation.
- For performance-critical tasks, consider using crates optimized for speed, such as [`memchr`][c~memchr~docs]↗{{hi:memchr}}.

## Code Examples

### Concatenate Strings

{{#include string_concat.incl.md}}

### Manipulate Strings

{{#include string_manipulation.incl.md}}

### Find, Extract, and Replace Text Using Regular Expressions

{{#include regex.incl.md}}

### Search for Strings (incl. Fuzzy Matching)

{{#include string_search.incl.md}}

### Parse Strings

{{#include string_parsing.incl.md}}

### Diff Text

{{#include diffing.incl.md}}

## Work with Unicode

{{#include unicode.incl.md}}

### Create and Use OS, C, and Non-UTF8 Strings

{{#include other_strings.incl.md}}

## Related Topics

| Topic | Rust Crates | Notes |
|---|---|---|
| [[development-tools_debugging | Logging]] | [`log`][c~log~docs]↗{{hi:log}}, [`env_logger`][c~env_logger~docs]↗{{hi:env_logger}}, [`tracing`][c~tracing~docs]↗{{hi:tracing}} | Logging often involves formatting and processing text. |
| [[markdown | Markdown]] Processing | [`pulldown-cmark`][c~pulldown_cmark~docs]↗{{hi:pulldown-cmark}}, [`comrak`][c~comrak~docs]↗{{hi:comrak}} | These crates parse and render Markdown. |
| Natural Language Processing (NLP) | (Many crates for specific tasks) | NLP tasks often use the crates mentioned here, along with specialized crates for things like part-of-speech tagging, named entity recognition, etc. See [[deep_learning | Deep Learning]]. |
| [[parsing | Parsing]] | [`nom`][c~nom~docs]↗{{hi:nom}}, [`pest`][c~pest~docs]↗{{hi:pest}}, [`lalrpop`][c~lalrpop~docs]↗{{hi:lalrpop}}, [`combine`][c~combine~docs]↗{{hi:combine}} | These crates offer different approaches to parsing, from combinators ([`nom`][c~nom~docs]↗{{hi:nom}}, [`combine`][c~combine~docs]↗{{hi:combine}}) to parser generators ([`pest`][c~pest~docs]↗{{hi:pest}}, [`lalrpop`][c~lalrpop~docs]↗{{hi:lalrpop}}). |
|   [[csv | CSV]] Parsing | [`csv`][c~csv~docs]↗{{hi:csv}} | This crate provides efficient CSV parsing. |
|   [[html | HTML]] Parsing | [`scraper`][c~scraper~docs]↗{{hi:scraper}}, [`kuchiki`][c~kuchiki~docs]↗{{hi:kuchiki}} | These crates parse HTML documents. |
|   [[xml | XML]] Parsing | [`xmltree`][c~xmltree~docs]↗{{hi:xmltree}}, [`quick-xml`][c~quick_xml~docs]↗{{hi:quick-xml}} | These crates parse XML documents. |
|   Command-Line [[argument_parsing | Argument Parsing]] | [`clap`][c~clap~docs]↗{{hi:clap}}, [`structopt`][c~structopt~docs]↗{{hi:structopt}} | These crates help with parsing command-line arguments, which often involve text processing. |
| [[serde | Serialization/Deserialization]] | [`serde`][c~serde~docs]↗{{hi:serde}}, [`serde_json`][c~serde_json~docs]↗{{hi:serde_json}}, [`serde_yml`][c~serde_yml~docs]↗{{hi:serde_yml}}, [`toml`][c~toml~docs]↗{{hi:toml}} | [`serde`][c~serde~docs]↗{{hi:serde}} is a powerful framework for serialization and deserialization, often used with text-based formats like [[json | JSON]], [[yaml | YAML]], and [[toml | TOML]]. |
| Text [[encoding | Encoding]]/Decoding | [`encoding`][c~encoding~docs]↗{{hi:encoding}}, [`iconv`][c~iconv~docs]↗{{hi:iconv}} (bindings) | These crates handle different character encodings. [`encoding`][c~encoding~docs]↗{{hi:encoding}} is a pure Rust solution, while [`iconv`][c~iconv~docs]↗{{hi:iconv}} provides bindings to the iconv library. |
| Text Formatting & [[template-engine | Templating]] | [`minijinja`][c~minijinja~docs]↗{{hi:minijinja}}, [`tera`][c~tera~docs]↗{{hi:tera}}, [`handlebars`][c~handlebars~docs]↗{{hi:handlebars}}, [`askama`][c~askama~docs]↗{{hi:askama}} | These crates are used for generating text-based output with dynamic content. |
| Tokenization | [`tokenizers`][c~tokenizers~docs]↗{{hi:tokenizers}} | [`tokenizers`][c~tokenizers~docs]↗{{hi:tokenizers}} provides tools for breaking text into tokens. |
| Stemming & Lemmatization | [`rust-stemmers`][c~rust_stemmers~docs]↗{{hi:rust-stemmers}}, [`lingua`][c~lingua~docs]↗{{hi:lingua}} | [`rust-stemmers`][c~rust_stemmers~docs]↗{{hi:rust-stemmers}} provides stemming algorithms. [`lingua`][c~lingua~docs]↗{{hi:lingua}} is a natural language detection library, suitable for short text and mixed-language text. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review; address NLP](https://github.com/john-cd/rust_howto/issues/963)
</div>
