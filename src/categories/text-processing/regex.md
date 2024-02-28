# Regular Expressions

{{#include regex.incl.md}}

## Verify and extract login from an email address

[![regex][regex-badge]][regex]  [![lazy-static][lazy-static-badge]][lazy-static]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Validates that an {{i:email address}} is formatted correctly, and extracts everything before the @ symbol.

```rust,editable
{{#include ../../../deps/tests/email.rs}}
```

## Extract a list of unique #Hashtags from a text

[![regex][regex-badge]][regex]  [![lazy-static][lazy-static-badge]][lazy-static]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Extracts, sorts, and deduplicates list of {{i:hashtags}} from text.

The hashtag regex given here only catches Latin hashtags that start with a letter. The complete [Twitter hashtag regex][twitter-hashtag-regex]  [![twitter-hashtag-regex][twitter-hashtag-badge]][twitter-hashtag-regex] is much more complicated.

```rust,editable
{{#include ../../../deps/tests/hashtags.rs}}
```

## Extract phone numbers from text

[![regex][regex-badge]][regex]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Processes a string of text using [`Regex::captures_iter`][regex::Regex::captures_iter] to capture multiple {{i:phone numbers}}. The example here is for US convention phone numbers.

```rust,editable
{{#include ../../../deps/tests/phone.rs}}
```

## Filter a log file by matching multiple regular expressions

[![regex][regex-badge]][regex]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Reads a file named `application.log` and only outputs the lines containing “version X.X.X”, some IP address followed by port 443 (e.g. “192.168.0.1:443”), or a specific warning.

A [`regex::RegexSetBuilder`][regex::RegexSetBuilder] composes a [`regex::RegexSet`][regex::RegexSet] Since backslashes are very common in {{i:regular expressions}}, using [raw string literals][book-rust-raw-string-literals] makes them more readable.

```rust,editable,no_run
{{#include ../../../deps/tests/filter-log.rs}}
```

## Replace all occurrences of one text pattern with another pattern

[![regex][regex-badge]][regex]  [![lazy-static][lazy-static-badge]][lazy-static]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Replaces all occurrences of the standard {{i:ISO 8601}} *YYYY-MM-DD* date pattern with the equivalent American English {{i:date}} with slashes. For example `2013-01-15` becomes `01/15/2013`.

The method [`Regex::replace_all`][regex::Regex::replace_all] replaces all occurrences of the whole {{i:regex}}.
`&str` implements the `Replacer` trait which allows variables like `$abcde` to refer to corresponding {{i:named capture groups}} `(?P<abcde>REGEX)` from the search regex. See the [`replacement string syntax`][regex::Regex-replacement-string-syntax] for examples and escaping detail.

```rust,editable
{{#include ../../../deps/tests/replace.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
