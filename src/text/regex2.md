# Regular Expressions

## Verify and extract login from an email address

[![regex-badge]][regex] [![lazy-static-badge]][lazy-static] [![cat-text-processing-badge]][cat-text-processing]

Validates that an email address is formatted correctly, and extracts everything
before the @ symbol.

```rust,editable
{#include ../../deps/examples/email.rs}
```

## Extract a list of unique #Hashtags from a text

[![regex-badge]][regex] [![lazy-static-badge]][lazy-static] [![cat-text-processing-badge]][cat-text-processing]

Extracts, sorts, and deduplicates list of hashtags from text.

The hashtag regex given here only catches Latin hashtags that start with a
letter. The complete [twitter hashtag regex] is much more complicated.

```rust,editable
{#include ../../deps/examples/hashtags.rs}
```

## Extract phone numbers from text

[![regex-badge]][regex] [![cat-text-processing-badge]][cat-text-processing]

Processes a string of text using `[Regex::captures_iter]` to capture multiple
phone numbers.  The example here is for US convention phone numbers.

```rust,editable
{#include ../../deps/examples/phone.rs}
```

## Filter a log file by matching multiple regular expressions

[![regex-badge]][regex] [![cat-text-processing-badge]][cat-text-processing]

Reads a file named `application.log` and only outputs the lines
containing “version X.X.X”, some IP address followed by port 443
(e.g. “192.168.0.1:443”), or a specific warning.

A `[regex::RegexSetBuilder]` composes a `[regex::RegexSet]`
Since backslashes are very common in regular expressions, using
[raw string literals] makes them more readable.

```rust,editable,no_run
{#include ../../deps/examples/filter-log.rs}
```

## Replace all occurrences of one text pattern with another pattern

[![regex-badge]][regex] [![lazy-static-badge]][lazy-static] [![cat-text-processing-badge]][cat-text-processing]

Replaces all occurrences of the standard ISO 8601 *YYYY-MM-DD* date pattern
with the equivalent American English date with slashes.
For example `2013-01-15` becomes `01/15/2013`.

The method `[Regex::replace_all]` replaces all occurrences of the whole regex.
`&str` implements the `Replacer` trait which allows variables like `$abcde` to
refer to corresponding named capture groups `(?P<abcde>REGEX)` from the search
regex. See the [replacement string syntax] for examples and escaping detail.

```rust,editable
{#include ../../deps/examples/replace.rs}
```

[Regex::captures_iter]: https://docs.rs/regex/*/regex/struct.Regex.html#method.captures_iter
[regex::RegexSet]: https://docs.rs/regex/*/regex/struct.RegexSet.html
[regex::RegexSetBuilder]: https://docs.rs/regex/*/regex/struct.RegexSetBuilder.html
[raw string literals]: https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
[twitter hashtag regex]: https://github.com/twitter/twitter-text/blob/c9fc09782efe59af4ee82855768cfaf36273e170/java/src/com/twitter/Regex.java#L255
[Regex::replace_all]: https://docs.rs/regex/*/regex/struct.Regex.html#method.replace_all
[replacement string syntax]: https://docs.rs/regex/*/regex/struct.Regex.html#replacement-string-syntax
{{#include ../refs/link-refs.md}}