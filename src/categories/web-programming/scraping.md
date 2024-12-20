# Extracting Links

{{#include scraping.incl.md}}

## Extract all links from the HTML of a webpage {#extract-all-links-from-a-webpage-html}

[![reqwest][c-reqwest-badge]][c-reqwest]{{hi:reqwest}} [![select][c-select-badge]][c-select]{{hi:select}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}}

Use [`reqwest::get`][c-reqwest::get]{{hi:reqwest::get}}⮳ to perform a HTTP GET request{{hi:HTTP GET request}} and then use [`select::document::Document::from_read`][c-select::document::Document::from_read]{{hi:select::document::Document::from_read}}⮳ to parse the response into a HTML document{{hi:HTML document}}. [`select::document::Document::find`][c-select::document::Document::find]{{hi:select::document::Document::find}}⮳ with the criteria of [`select::predicate::Name`][c-select::predicate::Name]{{hi:select::predicate::Name}}⮳ is "a" retrieves all links. Call [`std-core::iter::Iterator::filter_map`][c-std-core::iter::Iterator::filter_map]{{hi:std-core::iter::Iterator::filter_map}}⮳ on the [`select::selection::Selection`][c-select::selection::Selection]{{hi:select::selection::Selection}}⮳ retrieves URLs from links that have the "href" [`select::node::Node::attr`][c-select::node::Node::attr]{{hi:select::node::Node::attr}}⮳ (attribute).

```rust,editable
{{#include ../../../crates/ex/categories/wxyz/tests/web_programming/extract_links.rs:example}}
```

## Check a webpage for broken links {#check-a-webpage-for-broken-links}

[![reqwest][c-reqwest-badge]][c-reqwest]{{hi:reqwest}} [![select][c-select-badge]][c-select]{{hi:select}} [![url][c-url-badge]][c-url]{{hi:url}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}}

Call `get_base_url` to retrieve the base URL{{hi:Base URL}}. If the document has a base tag, get the href{{hi:href}} [`select::node::Node::attr`][c-select::node::Node::attr]{{hi:select::node::Node::attr}}⮳ from base tag. [`select::node::Node::attr`][c-select::node::Node::attr]{{hi:select::node::Node::attr}}⮳ of the original URL acts as a default.

Iterates through links{{hi:Links}} in the document and creates a [`tokio::task::spawn`][c-tokio::task::spawn]{{hi:tokio::task::spawn}}⮳ task that will parse an individual link with [`url::ParseOptions`][c-url::ParseOptions]{{hi:url::ParseOptions}}⮳ and [`tokio::task::spawn`][c-tokio::task::spawn]{{hi:tokio::task::spawn}}⮳. The task makes a request to the links with [`reqwest`][c-reqwest]{{hi:reqwest}}⮳ and verifies
[`reqwest::StatusCode`][c-reqwest::StatusCode]{{hi:reqwest::StatusCode}}⮳. Then the tasks [`await`][book-rust-reference-await]{{hi:await}}⮳ completion before ending the program.

```rust,editable
{{#include ../../../crates/ex/categories/wxyz/tests/web_programming/broken.rs:example}}
```

## Extract all unique links from a MediaWiki markup {#extract-all-unique-links-from-a-mediawiki-markup}

[![reqwest][c-reqwest-badge]][c-reqwest]{{hi:reqwest}} [![regex][c-regex-badge]][c-regex]{{hi:regex}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]

Pull the source of a MediaWiki page using [`reqwest::get`][c-reqwest::get]{{hi:reqwest::get}}⮳ and then look for all entries of internal and external links with [`regex::Regex::captures_iter`][c-regex::Regex::captures_iter]{{hi:regex::Regex::captures_iter}}⮳. Using [`std::borrow::Cow`][c-std::borrow::Cow]{{hi:std::borrow::Cow}}⮳ avoids excessive [`std::string::String`][c-std::string::String]{{hi:std::string::String}}⮳ allocations.

MediaWiki link syntax is described [here][mediawiki-link-syntax]⮳.

```rust,editable
{{#include ../../../crates/ex/categories/wxyz/tests/web_programming/unique.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
