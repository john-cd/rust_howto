# Extracting Links

{{#include scraping.incl.md}}

## Extract all links from a webpage HTML

[![reqwest][reqwest-badge]][reqwest]  [![select][select-badge]][select]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming] |

Use [`reqwest::get`][reqwest::get]⮳ to perform a {{i:HTTP GET request}} and then use [`Document::from_read`][select::document::Document::from_read]⮳ to parse the response into a {{i:HTML document}}. [`find`][select::document::Document::find]⮳ with the criteria of [`Name`][select::predicate::Name]⮳ is "a" retrieves all links. Call [`filter_map`][std-core::iter::Iterator::filter_map]⮳ on the [`Selection`][select::selection::Selection]⮳ retrieves URLs from links that have the "href" [`attr`][select::node::Node::attr]⮳ (attribute).

```rust,editable,no_run
{{#include ../../../deps/tests/extract-links.rs}}
```

## Check a webpage for broken links

[![reqwest][reqwest-badge]][reqwest]  [![select][select-badge]][select]  [![url][url-badge]][url]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming] |

Call `get_base_url` to retrieve the {{i:base URL}}. If the document has a base tag, get the {{i:href}} [`attr`][select::node::Node::attr]⮳ from base tag. [`Position::BeforePath`][url::Position::BeforePath]⮳ of the original URL acts as a default.

Iterates through {{i:links}} in the document and creates a [`tokio::spawn`][tokio::task::spawn]⮳ task that will parse an individual link with [`url::ParseOptions`][url::ParseOptions]⮳ and [`Url::parse`][url::Url::parse]⮳. The task makes a request to the links with [`reqwest`][reqwest]⮳ and verifies
[`StatusCode`][reqwest::StatusCode]⮳. Then the tasks [`await`][book-rust-reference-await]⮳ completion before ending the program.

```rust,editable,no_run
{{#include ../../../deps/tests/broken.rs}}
```

## Extract all unique links from a MediaWiki markup

[![reqwest][reqwest-badge]][reqwest]  [![regex][regex-badge]][regex]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]

Pull the source of a MediaWiki page using [`reqwest::get`][reqwest::get]⮳ and then look for all entries of internal and external links with [`Regex::captures_iter`][regex::Regex::captures_iter]⮳. Using [`Cow`][std::borrow::Cow]⮳ avoids excessive [`String`][std::string::String]⮳ allocations.

MediaWiki link syntax is described [here][mediawiki-link-syntax]⮳.

```rust,editable,no_run
{{#include ../../../deps/tests/unique.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
