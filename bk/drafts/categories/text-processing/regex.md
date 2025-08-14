# Regular Expressions

{{#include regex.incl.md}}

## Verify and Extract Login from an Email Address {#verify-and-extract-login-from-an-email-address}

[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![lazy_static][c~lazy_static~docs~badge]][c~lazy_static~docs]{{hi:lazy_static}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Validates that an [email][p~email] address{{hi:Email address}} is formatted correctly, and extracts everything before the `@` symbol.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/regex/email.rs:example}}
```

## Extract a list of Unique #hashtags from a Text {#extract-a-list-of-unique-hashtags-from-a-text}

[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![lazy_static][c~lazy_static~docs~badge]][c~lazy_static~docs]{{hi:lazy_static}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Extracts, sorts, and deduplicates list of hashtags{{hi:Hashtags}} from text.

The hashtag regex given here only catches Latin hashtags that start with a letter. The complete [Twitter hashtag regex][twitter~hashtag-regex~github]↗ [![twitter~hashtag-regex~github][twitter~hashtag-regex~github~badge]][twitter~hashtag-regex~github] is much more complicated.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/regex/hashtags.rs:example}}
```

## Extract Phone Numbers from Text {#extract-phone-numbers-from-text}

[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Processes a string of text using [`regex::Regex::captures_iter`][c~regex::Regex::captures_iter~docs]↗{{hi:regex::Regex::captures_iter}} to capture multiple phone numbers{{hi:Phone numbers}}. The example here is for US convention phone numbers.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/regex/phone.rs:example}}
```

## Filter a log File by Matching Multiple Regular Expressions {#filter-a-log-file-by-matching-multiple-regular-expressions}

[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Reads a file named `application.log` and only outputs the lines containing “version X.X.X”, some IP address followed by port 443 (e.g. “192.168.0.1:443”), or a specific warning.

A [`regex::RegexSetBuilder`][c~regex::RegexSetBuilder~docs]↗{{hi:regex::RegexSetBuilder}} composes a [`regex::RegexSetBuilder`][c~regex::RegexSetBuilder~docs]↗{{hi:regex::RegexSetBuilder}} Since backslashes are very common in regular expressions{{hi:Regular expressions}}, using [raw string literals][book~rust~raw-string-literals]↗ makes them more readable.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/regex/filter_log.rs:example}}
```

## Replace all Occurrences of one text Pattern with Another Pattern {#replace-all-occurrences-of-one-text-pattern-with-another-pattern}

[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![lazy_static][c~lazy_static~docs~badge]][c~lazy_static~docs]{{hi:lazy_static}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Replaces all occurrences of the standard ISO 8601{{hi:ISO 8601}} *YYYY-MM-DD* date pattern with the equivalent American English date{{hi:Date}} with slashes. For example `2013-01-15` becomes `01/15/2013`.

The method [`regex::Regex::replace_all`][c~regex::Regex::replace_all~docs]↗{{hi:regex::Regex::replace_all}} replaces all occurrences of the whole regex{{hi:regex}}.
`&str` implements the [`regex::Replacer`][c~regex::Replacer~docs]↗{{hi:regex::Replacer}} trait which allows variables like `$abcde` to refer to corresponding named capture groups{{hi:Named capture groups}} `(?P<abcde>REGEX)` from the search regex. See the [`replacement string syntax`][c~regex::Regex~replacement-string-syntax]↗{{hi:Replacement string syntax}} for examples and escaping detail.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/regex/replace.rs:example}}
```

## Use Regular Expressions with Back-references and Lookarounds {#fancy-regex}

[![fancy-regex][c~fancy-regex~docs~badge]][c~fancy-regex~docs] [![fancy-regex~crates.io][c~fancy-regex~crates.io~badge]][c~fancy-regex~crates.io] [![fancy-regex~github][c~fancy-regex~github~badge]][c~fancy-regex~github] [![fancy-regex~lib.rs][c~fancy-regex~lib.rs~badge]][c~fancy-regex~lib.rs]{{hi:fancy-regex}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

[`regex`][c~regex~docs]↗{{hi:regex}} is the de facto standard regex library. It is very fast, but does not support fancier features such as backtracking, backreferences, and look-arounds. Use [`fancy-regex`][c~fancy-regex~docs]↗{{hi:fancy-regex}} if you need features that [`regex`][c~regex~docs]↗{{hi:regex}} doesn't support.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/regex/fancy_regex.rs:example}}
```

## Longer Regex Example {#longer-regex-example}

[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![regex~github][c~regex~github~badge]][c~regex~github] [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/regex/regex.rs:example}}
```

## Related Topics {#related-topics}

- [[rust_search_engines | Rust Search Engines]].
- [[search | Search]].
- [[strings | Strings]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[regex: write](https://github.com/john-cd/rust_howto/issues/488)

- [grex][c~grex~github]↗: A command-line tool and Rust library with Python bindings for generating regular expressions from user-provided test cases.

- [regex_automata][c~regex-automata~docs]↗: Multi-pattern searches with capture groups.

</div>
