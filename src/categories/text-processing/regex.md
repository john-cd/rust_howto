# Regular Expressions

{{#include regex.incl.md}}

## Verify and extract login from an email address

[![regex][c-regex-badge]][c-regex]  [![lazy_static][c-lazy_static-badge]][c-lazy_static]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Validates that an email address{{hi:email address}} is formatted correctly, and extracts everything before the `@` symbol.

```rust,editable
{{#include ../../../deps/tests/email.rs}}
```

## Extract a list of unique #Hashtags from a text

[![regex][c-regex-badge]][c-regex]  [![lazy_static][c-lazy_static-badge]][c-lazy_static]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Extracts, sorts, and deduplicates list of hashtags{{hi:Hashtags}} from text.

The hashtag regex given here only catches Latin hashtags that start with a letter. The complete [Twitter hashtag regex][twitter-hashtag-regex]⮳  [![twitter-hashtag-regex][twitter-hashtag-badge]][twitter-hashtag-regex] is much more complicated.

```rust,editable
{{#include ../../../deps/tests/hashtags.rs}}
```

## Extract phone numbers from text

[![regex][c-regex-badge]][c-regex]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Processes a string of text using [`regex::Regex::captures_iter`][c-regex::Regex::captures_iter]{{hi:regex::Regex::captures_iter}}⮳ to capture multiple phone numbers{{hi:phone numbers}}. The example here is for US convention phone numbers.

```rust,editable
{{#include ../../../deps/tests/phone.rs}}
```

## Filter a log file by matching multiple regular expressions

[![regex][c-regex-badge]][c-regex]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Reads a file named `application.log` and only outputs the lines containing “version X.X.X”, some IP address followed by port 443 (e.g. “192.168.0.1:443”), or a specific warning.

A [`regex::RegexSetBuilder`][c-regex::RegexSetBuilder]{{hi:regex::RegexSetBuilder}}⮳ composes a [`regex::RegexSetBuilder`][c-regex::RegexSetBuilder]{{hi:regex::RegexSetBuilder}}⮳ Since backslashes are very common in regular expressions{{hi:regular expressions}}, using [raw string literals][book-rust-raw-string-literals]⮳ makes them more readable.

```rust,editable,no_run
{{#include ../../../deps/tests/filter-log.rs}}
```

## Replace all occurrences of one text pattern with another pattern

[![regex][c-regex-badge]][c-regex]  [![lazy_static][c-lazy_static-badge]][c-lazy_static]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Replaces all occurrences of the standard ISO 8601{{hi:ISO 8601}} *YYYY-MM-DD* date pattern with the equivalent American English date{{hi:Date}} with slashes. For example `2013-01-15` becomes `01/15/2013`.

The method [`regex::Regex::replace_all`][c-regex::Regex::replace_all]{{hi:regex::Regex::replace_all}}⮳ replaces all occurrences of the whole regex{{hi:Regex}}.
`&str` implements the [`regex::Replacer`][c-regex::Replacer]{{hi:regex::Replacer}}⮳ trait which allows variables like `$abcde` to refer to corresponding named capture groups{{hi:named capture groups}} `(?P<abcde>REGEX)` from the search regex. See the [`replacement string syntax`][c-regex::Regex-replacement-string-syntax]{{hi:replacement string syntax}}⮳ for examples and escaping detail.

```rust,editable
{{#include ../../../deps/tests/replace.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
