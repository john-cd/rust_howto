# Extracting Links

## Extract all links from a webpage HTML

[![reqwest-badge]][reqwest] [![select-badge]][select] [![cat-network-programming-badge]][cat-network-programming]

Use `[reqwest::get]` to perform a HTTP GET request and then use
`[Document::from_read]` to parse the response into a HTML document.
`[find]` with the criteria of `[Name]` is "a" retrieves all links.
Call `[filter_map]` on the `[Selection]` retrieves URLs
from links that have the "href" `[attr]` (attribute).

```rust,editable,no_run
{#include ../../deps/examples/extract-links.rs}
```

## Check a webpage for broken links

[![reqwest-badge]][reqwest] [![select-badge]][select] [![url-badge]][url] [![cat-network-programming-badge]][cat-network-programming]

Call `get_base_url` to retrieve the base URL. If the document has a base tag,
get the href `[attr]` from base tag. `[Position::BeforePath]` of the original
URL acts as a default.

Iterates through links in the document and creates a `[tokio::spawn]` task that will
parse an individual link with `[url::ParseOptions]` and `[Url::parse]` .
The task makes a request to the links with [reqwest] and verifies
`[StatusCode]` Then the tasks `await` completion before ending the program.

```rust,editable,no_run
{#include ../../deps/examples/broken.rs}
```

## Extract all unique links from a MediaWiki markup

[![reqwest-badge]][reqwest] [![regex-badge]][regex] [![cat-network-programming-badge]][cat-network-programming]

Pull the source of a MediaWiki page using `[reqwest::get]` and then
look for all entries of internal and external links with
`[Regex::captures_iter]` Using `[Cow]` avoids excessive `[String]` allocations.

MediaWiki link syntax is described [here][MediaWiki link syntax].

```rust,editable,no_run
{#include ../../deps/examples/unique.rs}
```

[attr]: https://docs.rs/select/*/select/node/struct.Node.html#method.attr
[Document::from_read]: https://docs.rs/select/*/select/document/struct.Document.html#method.from_read
[filter_map]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.filter_map
[find]: https://docs.rs/select/*/select/document/struct.Document.html#method.find
[Name]: https://docs.rs/select/*/select/predicate/struct.Name.html
[reqwest::get]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[Selection]: https://docs.rs/select/*/select/selection/struct.Selection.html
[Position::BeforePath]: https://docs.rs/url/*/url/enum.Position.html#variant.BeforePath
[StatusCode]: https://docs.rs/reqwest/*/reqwest/struct.StatusCode.html
[tokio::spawn]: https://docs.rs/tokio/*/tokio/fn.spawn.html
[url::Parse]: https://docs.rs/url/*/url/struct.Url.html#method.parse
[url::ParseOptions]: https://docs.rs/url/*/url/struct.ParseOptions.html
[Cow]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
[Regex::captures_iter]: https://docs.rs/regex/*/regex/struct.Regex.html#method.captures_iter
[String]: https://doc.rust-lang.org/std/string/struct.String.html
[MediaWiki link syntax]: https://www.mediawiki.org/wiki/Help:Links
{{#include ../refs/link-refs.md}}
