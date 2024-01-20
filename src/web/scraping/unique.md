## Extract all unique links from a MediaWiki markup

[![reqwest-badge]][reqwest] [![regex-badge]][regex] [![cat-net-badge]][cat-net]

Pull the source of a MediaWiki page using [`reqwest::get`] and then
look for all entries of internal and external links with
[`Regex::captures_iter`]. Using [`Cow`] avoids excessive [`String`] allocations.

MediaWiki link syntax is described [here][MediaWiki link syntax].

```rust,editable,no_run
{#include ../../../deps/examples/unique.rs}
```

[`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`Regex::captures_iter`]: https://docs.rs/regex/*/regex/struct.Regex.html#method.captures_iter
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html

[MediaWiki link syntax]: https://www.mediawiki.org/wiki/Help:Links
