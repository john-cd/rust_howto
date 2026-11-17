<!-- TODO Reviewed ??-??-2026. -->

# Regular Expressions

{{#include regex.incl.md}}

Hashtag extraction, US-style phone extraction, and date replacement have moved to the [published Regular Expressions chapter](https://john-cd.com/rust_howto/categories/text-processing/regex.html).

Regular expressions are useful for searching and extracting text with a repeatable pattern. Prefer ordinary string methods for literal prefixes, suffixes, or delimiters. Rust's `regex` crate guarantees linear-time matching and intentionally omits features that require backtracking, such as look-around and backreferences; use `fancy-regex` only when those features are required and inputs are appropriately bounded.

## Verify and Extract Login from an Email Address {#verify-and-extract-login-from-an-email-address}

[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![lazy_static][c~lazy_static~docs~badge]][c~lazy_static~docs]{{hi:lazy_static}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Validates that an [email][p~email] address{{hi:Email address}} is formatted correctly, and extracts everything before the `@` symbol.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/regex/email.rs:example}}
```

## Filter a log File by Matching Multiple Regular Expressions {#filter-a-log-file-by-matching-multiple-regular-expressions}

[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Reads a file named `application.log` and only outputs the lines containing "version X.X.X", some IP address followed by port 443 (e.g. "192.168.0.1:443"), or a specific warning.

A [`regex::RegexSetBuilder`][c~regex::RegexSetBuilder~docs]↗{{hi:regex::RegexSetBuilder}} composes several patterns into one set. Since backslashes are common in regular expressions{{hi:Regular expressions}}, raw string literals make them more readable.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/regex/filter_log.rs:example}}
```

## Use Regular Expressions with Back-references and Lookarounds {#fancy-regex}

{{#badges fancy-regex text-processing="Text processing"}}

[`regex`][c~regex~docs]↗{{hi:regex}} is the de facto standard regex library. It is very fast, but does not support fancier features such as backtracking, backreferences, and look-arounds. Use [`fancy-regex`][c~fancy-regex~docs]↗{{hi:fancy-regex}} if you need features that [`regex`][c~regex~docs]↗{{hi:regex}} doesn't support.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/regex/fancy_regex.rs:example}}
```

## Longer Regex Example {#longer-regex-example}

[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![regex~repo][c~regex~repo~badge]][c~regex~repo] [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/regex/regex.rs:example}}
```

## Generate Patterns or Choose a Lower-Level Engine

- [`grex`][c~grex~repo]↗ proposes regular expressions from sample strings. Test the result against inputs that should fail as well as inputs that should match.
- [`regex-automata`][c~regex-automata~docs]↗ exposes lower-level matching engines. Use it when you need explicit control of engine configuration or multi-pattern searches.

## Related Topics {#related-topics .skip}

- [[rust_search_engines | Rust Search Engines]].
- [[search | Search]].
- [[strings | Strings]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
