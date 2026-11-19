<!-- TODO Reviewed ??-??-2026. -->

# Regular Expressions

{{#include regex.incl.md}}

Use [`regex`][c~regex~docs]↗{{hi:regex}} to extract patterned text and rewrite matches. Compile a pattern once when reusing it. The crate guarantees linear-time matching and omits features that need backtracking, such as look-around and backreferences. Use [`fancy-regex`][c~fancy-regex~docs]↗{{hi:fancy-regex}} only when those features are required, and bound input size because backtracking can take much longer on adversarial text. Prefer string methods for literal prefixes, suffixes, and delimiters.

## Extract a list of Unique #hashtags from a Text {#extract-a-list-of-unique-hashtags-from-a-text}

[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![lazy_static][c~lazy_static~docs~badge]][c~lazy_static~docs]{{hi:lazy_static}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Extracts distinct hashtags{{hi:Hashtags}} into a `HashSet<&str>` that borrows from the input. A set removes duplicates but does not sort them; iteration and printed order are unspecified. Matching is case-sensitive, so `#Rust` and `#rust` are distinct.

The pattern matches `#` followed by an ASCII letter, then ASCII letters, digits, or underscores. It skips `#2` and `#_`, can match a prefix of a non-ASCII tag, and does not enforce a word boundary before `#`. This is a small extraction example, not a social platform’s hashtag grammar.

```rust,noplayground
{{#include ../../../crates/cats/text_processing/examples/regex/hashtags.rs:example}}
```

## Extract Phone Numbers from Text {#extract-phone-numbers-from-text}

[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Processes a string of text using [`regex::Regex::captures_iter`][c~regex::Regex::captures_iter~docs]↗{{hi:regex::Regex::captures_iter}} to capture multiple phone numbers{{hi:Phone numbers}}. The example recognizes several US-style layouts, with an optional `1` country code, and formats the captures as `1 (area) exchange-subscriber`. It accepts spaces, dots, and hyphens as shown in the fixture.

This pattern extracts substrings; it does not validate an entire input or prove that a number is assigned. It has no surrounding digit boundaries, and `\d` also matches Unicode decimal digits. For production validation or international numbering rules, use a dedicated phone-number library.

```rust,noplayground
{{#include ../../../crates/cats/text_processing/examples/regex/phone.rs:example}}
```

## Replace all Occurrences of one text Pattern with Another Pattern {#replace-all-occurrences-of-one-text-pattern-with-another-pattern}

[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![lazy_static][c~lazy_static~docs~badge]][c~lazy_static~docs]{{hi:lazy_static}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Replaces all occurrences of the standard ISO 8601{{hi:ISO 8601}} _YYYY-MM-DD_ date pattern with the equivalent American English date{{hi:Date}} with slashes. For example `2013-01-15` becomes `01/15/2013`.

This rewrites digit patterns, not calendar dates: it will also rewrite `2026-99-99`. The pattern uses Unicode-aware `\d` and can match inside a longer string. Parse a date with a date library when calendar validity matters.

The method [`regex::Regex::replace_all`][c~regex::Regex::replace_all~docs]↗{{hi:regex::Regex::replace_all}} replaces all occurrences of the whole regex{{hi:regex}}.
`&str` implements the [`regex::Replacer`][c~regex::Replacer~docs]↗{{hi:regex::Replacer}} trait which allows variables like `$abcde` to refer to corresponding named capture groups{{hi:Named capture groups}} `(?P<abcde>REGEX)` from the search regex. See the [`replacement string syntax`][c~regex::Regex~replacement-string-syntax]↗{{hi:Replacement string syntax}} for examples and escaping detail.

```rust,noplayground
{{#include ../../../crates/cats/text_processing/examples/regex/replace.rs:example}}
```

## Verify and Extract Login from an Email Address {#verify-and-extract-login-from-an-email-address}

[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![lazy_static][c~lazy_static~docs~badge]][c~lazy_static~docs]{{hi:lazy_static}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Extracts the login before `@` from a simple `login@domain` shape. This pattern rejects whitespace and requires a non-empty domain, but it is not a full email-address validator; use a confirmation flow when an address must be verified.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/regex/email.rs:example}}
```

## Filter a Log File by Matching Multiple Regular Expressions {#filter-a-log-file-by-matching-multiple-regular-expressions}

[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Reads `temp/application.log` and prints lines containing a version, an IP address followed by port 443, or a warning. A [`regex::RegexSetBuilder`][c~regex::RegexSetBuilder~docs]↗{{hi:regex::RegexSetBuilder}} combines patterns and tests them in one pass. The example test creates the log fixture; to run the command manually, create that file first. Raw string literals keep patterns with backslashes readable.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/regex/filter_log.rs:example}}
```

## Use Regular Expressions with Back-references and Lookarounds {#fancy-regex}

{{#badges fancy-regex text-processing="Text processing"}}

The standard `regex` crate does not support backtracking features such as backreferences and look-arounds. Use [`fancy-regex`][c~fancy-regex~docs]↗{{hi:fancy-regex}} when a pattern requires them. Backtracking can take much longer than a linear-time regex, so bound the input and test patterns against worst-case data.

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p text_processing --example regex`.

```rust,editable,noplayground
{{#include ../../../crates/cats/text_processing/examples/regex/fancy_regex.rs:example}}
```

## Longer Regex Example {#longer-regex-example}

[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![regex~repo][c~regex~repo~badge]][c~regex~repo] [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

This example parses Markdown links and images with several regular expressions. It demonstrates captures, replacement, and splitting; for general Markdown input, use a Markdown parser because regex patterns do not model nested syntax.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/regex/regex.rs:example}}
```

## Generate Patterns or Choose a Lower-Level Engine

- [`grex`][c~grex~repo]↗ proposes regular expressions from sample strings. Test the result against inputs that should fail as well as inputs that should match.
- [`regex-automata`][c~regex-automata~docs]↗ exposes lower-level matching engines. Use it when you need explicit control of engine configuration or multi-pattern searches.

Run these Cargo examples from `bk/crates` (external crates are required):

```sh
cargo run -p text_processing --example regex -- hashtags
cargo run -p text_processing --example regex -- phone
cargo run -p text_processing --example regex -- replace
cargo test -p text_processing --example regex hashtags::
cargo test -p text_processing --example regex phone::
cargo test -p text_processing --example regex replace::
```

[`regex-lite`][c~regex-lite~docs] reduces compile time and binary size for patterns that fit its smaller Unicode feature set.

## Related Topics {#related-topics .skip}

- [Strings](../data-structures/strings.md).
- [Unicode text measurements](unicode.md).
- [String search and fuzzy matching](string_search.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
