# Extracting Links

{{#include scraping.incl.md}}

## Extract all links from a webpage HTML

[![reqwest][c-reqwest-badge]][c-reqwest]  [![select][c-select-badge]][c-select]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming] |

Use {{hi:reqwest::get}}[`reqwest::get`][c-reqwest::get]⮳ to perform a {{i:HTTP GET request}} and then use {{hi:Document::from_read}}[`Document::from_read`][c-select::document::Document::from_read]⮳ to parse the response into a {{i:HTML document}}. {{hi:find}}[`find`][c-select::document::Document::find]⮳ with the criteria of {{hi:Name}}[`Name`][c-select::predicate::Name]⮳ is "a" retrieves all links. Call {{hi:filter_map}}[`filter_map`][c-std-core::iter::Iterator::filter_map]⮳ on the {{hi:Selection}}[`Selection`][c-select::selection::Selection]⮳ retrieves URLs from links that have the "href" {{hi:attr}}[`attr`][c-select::node::Node::attr]⮳ (attribute).

```rust,editable,no_run
{{#include ../../../deps/tests/extract-links.rs}}
```

## Check a webpage for broken links

[![reqwest][c-reqwest-badge]][c-reqwest]  [![select][c-select-badge]][c-select]  [![url][c-url-badge]][c-url]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming] |

Call `get_base_url` to retrieve the {{i:base URL}}. If the document has a base tag, get the {{i:href}} {{hi:attr}}[`attr`][c-select::node::Node::attr]⮳ from base tag. {{hi:Position::BeforePath}}[`Position::BeforePath`][c-select::node::Node::attr]⮳ of the original URL acts as a default.

Iterates through {{i:links}} in the document and creates a {{hi:tokio::spawn}}[`tokio::spawn`][c-tokio::task::spawn]⮳ task that will parse an individual link with {{hi:url::ParseOptions}}[`url::ParseOptions`][c-url::ParseOptions]⮳ and {{hi:Url::parse}}[`Url::parse`][c-tokio::task::spawn]⮳. The task makes a request to the links with {{hi:reqwest}}[`reqwest`][c-reqwest]⮳ and verifies
{{hi:StatusCode}}[`StatusCode`][c-reqwest::StatusCode]⮳. Then the tasks {{hi:await}}[`await`][book-rust-reference-await]⮳ completion before ending the program.

```rust,editable,no_run
{{#include ../../../deps/tests/broken.rs}}
```

## Extract all unique links from a MediaWiki markup

[![reqwest][c-reqwest-badge]][c-reqwest]  [![regex][c-regex-badge]][c-regex]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]

Pull the source of a MediaWiki page using {{hi:reqwest::get}}[`reqwest::get`][c-reqwest::get]⮳ and then look for all entries of internal and external links with {{hi:Regex::captures_iter}}[`Regex::captures_iter`][c-regex::Regex::captures_iter]⮳. Using {{hi:Cow}}[`Cow`][c-std::borrow::Cow]⮳ avoids excessive {{hi:String}}[`String`][c-std::string::String]⮳ allocations.

MediaWiki link syntax is described [here][mediawiki-link-syntax]⮳.

```rust,editable,no_run
{{#include ../../../deps/tests/unique.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
