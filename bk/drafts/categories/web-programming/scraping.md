# Extracting Links

{{#include scraping.incl.md}}

## Extract all Links from the HTML of a Webpage {#extract-all-links-from-a-webpage-html}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![select][c~select~docs~badge]][c~select~docs]{{hi:select}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

Use [`reqwest::get`][c~reqwest::get~docs]{{hi:reqwest::get}}↗ to perform a HTTP GET request{{hi:HTTP GET request}} and then use [`select::document::Document::from_read`][c~select::document::Document::from_read~docs]{{hi:select::document::Document::from_read}}↗ to parse the response into a HTML document{{hi:HTML document}}. [`select::document::Document::find`][c~select::document::Document::find~docs]{{hi:select::document::Document::find}}↗ with the criteria of [`select::predicate::Name`][c~select::predicate::Name~docs]{{hi:select::predicate::Name}}↗ is "a" retrieves all links. Call [`std-core::iter::Iterator::filter_map`][c~core::iter::Iterator::filter_map~docs]{{hi:std-core::iter::Iterator::filter_map}}↗ on the [`select::selection::Selection`][c~select::selection::Selection~docs]{{hi:select::selection::Selection}}↗ retrieves URLs from links that have the "href" [`select::node::Node::attr`][c~select::node::Node::attr~docs]{{hi:select::node::Node::attr}}↗ (attribute).

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/scraping/extract_links.rs:example}}
```

## Check a Webpage for Broken Links {#check-a-webpage-for-broken-links}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![select][c~select~docs~badge]][c~select~docs]{{hi:select}} [![url][c~url~docs~badge]][c~url~docs]{{hi:url}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

Call [`get_base_url`][]{{hi:get_base_url}} to retrieve the base [URL][p~url]{{hi:Base URL}}. If the document has a base tag, get the href{{hi:href}} [`select::node::Node::attr`][c~select::node::Node::attr~docs]{{hi:select::node::Node::attr}}↗ from base tag. [`select::node::Node::attr`][c~select::node::Node::attr~docs]{{hi:select::node::Node::attr}}↗ of the original URL acts as a default.

Iterates through links{{hi:Links}} in the document and creates a [`tokio::task::spawn`][c~tokio::task::spawn~docs]{{hi:tokio::task::spawn}}↗ task that will parse an individual link with [`url::ParseOptions`][c~url::ParseOptions~docs]{{hi:url::ParseOptions}}↗ and [`tokio::task::spawn`][c~tokio::task::spawn~docs]{{hi:tokio::task::spawn}}↗. The task makes a request to the links with [`reqwest`][c~reqwest~docs]{{hi:reqwest}}↗ and verifies
[`reqwest::StatusCode`][c~reqwest::StatusCode~docs]{{hi:reqwest::StatusCode}}↗. Then the tasks [`await`][book~rust-reference~await]{{hi:await}}↗ completion before ending the program.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/scraping/broken.rs:example}}
```

## Extract all Unique Links from a MediaWiki Markup {#extract-all-unique-links-from-a-mediawiki-markup}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]

Pull the source of a MediaWiki page using [`reqwest::get`][c~reqwest::get~docs]{{hi:reqwest::get}}↗ and then look for all entries of internal and external links with [`regex::Regex::captures_iter`][c~regex::Regex::captures_iter~docs]{{hi:regex::Regex::captures_iter}}↗. Using [`std::borrow::Cow`][c~std::borrow::Cow~docs]{{hi:std::borrow::Cow}}↗ avoids excessive [`std::string::String`][c~std::string::String~docs]{{hi:std::string::String}}↗ allocations.

MediaWiki link syntax is described [here][mediawiki~link-syntax]↗.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/scraping/unique.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/972)

- [rust-headless-chrome/rust-headless-chrome][rust-headless-chrome~github]↗: A high-level API to control headless Chrome or Chromium over the DevTools Protocol. It is the Rust equivalent of Puppeteer, a Node library maintained by the Chrome DevTools team.

</div>
