# Regular Expressions

{{#include regex.incl.md}}

## Verify and extract login from an email address {#verify-and-extract-login-from-an-email-address}

[![regex][c-regex-badge]][c-regex]{{hi:regex}} [![lazy_static][c-lazy_static-badge]][c-lazy_static]{{hi:lazy_static}} [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}

Validates that an email address{{hi:Email address}} is formatted correctly, and extracts everything before the `@` symbol.

```rust,editable
{{#include ../../../deps/tests/categories/text_processing/email.rs:example}}
```

## Extract a list of unique #hashtags from a text {#extract-a-list-of-unique-hashtags-from-a-text}

[![regex][c-regex-badge]][c-regex]{{hi:regex}} [![lazy_static][c-lazy_static-badge]][c-lazy_static]{{hi:lazy_static}} [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}

Extracts, sorts, and deduplicates list of hashtags{{hi:Hashtags}} from text.

The hashtag regex given here only catches Latin hashtags that start with a letter. The complete [Twitter hashtag regex][twitter-hashtag-regex]⮳ [![twitter-hashtag-regex][twitter-hashtag-badge]][twitter-hashtag-regex] is much more complicated.

```rust,editable
{{#include ../../../deps/tests/categories/text_processing/hashtags.rs:example}}
```

## Extract phone numbers from text {#extract-phone-numbers-from-text}

[![regex][c-regex-badge]][c-regex]{{hi:regex}} [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}

Processes a string of text using [`regex::Regex::captures_iter`][c-regex::Regex::captures_iter]{{hi:regex::Regex::captures_iter}}⮳ to capture multiple phone numbers{{hi:Phone numbers}}. The example here is for US convention phone numbers.

```rust,editable
{{#include ../../../deps/tests/categories/text_processing/phone.rs:example}}
```

## Filter a log file by matching multiple regular expressions {#filter-a-log-file-by-matching-multiple-regular-expressions}

[![regex][c-regex-badge]][c-regex]{{hi:regex}} [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}

Reads a file named `application.log` and only outputs the lines containing “version X.X.X”, some IP address followed by port 443 (e.g. “192.168.0.1:443”), or a specific warning.

A [`regex::RegexSetBuilder`][c-regex::RegexSetBuilder]{{hi:regex::RegexSetBuilder}}⮳ composes a [`regex::RegexSetBuilder`][c-regex::RegexSetBuilder]{{hi:regex::RegexSetBuilder}}⮳ Since backslashes are very common in regular expressions{{hi:Regular expressions}}, using [raw string literals][book-rust-raw-string-literals]⮳ makes them more readable.

```rust,editable
{{#include ../../../deps/tests/categories/text_processing/filter_log.rs:example}}
```

## Replace all occurrences of one text pattern with another pattern {#replace-all-occurrences-of-one-text-pattern-with-another-pattern}

[![regex][c-regex-badge]][c-regex]{{hi:regex}} [![lazy_static][c-lazy_static-badge]][c-lazy_static]{{hi:lazy_static}} [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}

Replaces all occurrences of the standard ISO 8601{{hi:ISO 8601}} *YYYY-MM-DD* date pattern with the equivalent American English date{{hi:Date}} with slashes. For example `2013-01-15` becomes `01/15/2013`.

The method [`regex::Regex::replace_all`][c-regex::Regex::replace_all]{{hi:regex::Regex::replace_all}}⮳ replaces all occurrences of the whole regex{{hi:regex}}.
`&str` implements the [`regex::Replacer`][c-regex::Replacer]{{hi:regex::Replacer}}⮳ trait which allows variables like `$abcde` to refer to corresponding named capture groups{{hi:Named capture groups}} `(?P<abcde>REGEX)` from the search regex. See the [`replacement string syntax`][c-regex::Regex-replacement-string-syntax]{{hi:Replacement string syntax}}⮳ for examples and escaping detail.

```rust,editable
{{#include ../../../deps/tests/categories/text_processing/replace.rs:example}}
```

## Use regular expressions with backreferences and lookarounds {#fancy-regex}

[![fancy-regex][c-fancy_regex-badge]][c-fancy_regex] [![fancy-regex-crates.io][c-fancy_regex-crates.io-badge]][c-fancy_regex-crates.io] [![fancy-regex-github][c-fancy_regex-github-badge]][c-fancy_regex-github] [![fancy-regex-lib.rs][c-fancy_regex-lib.rs-badge]][c-fancy_regex-lib.rs]{{hi:fancy-regex}} [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}

`regex` is the de facto standard regex library. It is very fast, but does not support fancier features such as backtracking, backreferences, and look-arounds. Use `fancy-regex` if you need features that `regex` doesn't support.

```rust,editable
{{#include ../../../deps/tests/categories/text_processing/fancy_regex.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[regex: write (P1)](https://github.com/john-cd/rust_howto/issues/488)

</div>
