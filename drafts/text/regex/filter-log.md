## Filter a log file by matching multiple regular expressions

[![regex-badge]][regex] [![cat-text-processing-badge]][cat-text-processing]

Reads a file named `application.log` and only outputs the lines
containing “version X.X.X”, some IP address followed by port 443
(e.g. “192.168.0.1:443”), or a specific warning.

A [`regex::RegexSetBuilder`] composes a [`regex::RegexSet`].
Since backslashes are very common in regular expressions, using
[raw string literals] makes them more readable.

```rust,editable,no_run
{#include ../../../deps/examples/filter-log.rs}
```

[`regex::RegexSet`]: https://docs.rs/regex/*/regex/struct.RegexSet.html
[`regex::RegexSetBuilder`]: https://docs.rs/regex/*/regex/struct.RegexSetBuilder.html

[raw string literals]: https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
