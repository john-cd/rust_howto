# Extracting Links

## Extract all links from a webpage HTML

[![reqwest-badge]][reqwest] [![select-badge]][select] [![cat-network-programming-badge]][cat-network-programming]

Use [`reqwest::get`][reqwest::get] to perform a HTTP GET request and then use
`[Document::from_read]` to parse the response into a HTML document.
`[find]` with the criteria of [`Name`][Name] is "a" retrieves all links.
Call [`filter_map`][filter_map] on the [`Selection`][Selection] retrieves URLs
from links that have the "href" [`attr`][attr] (attribute).

```rust,editable,no_run
{{#include ../../deps/examples/extract-links.rs}}
```

## Check a webpage for broken links

[![reqwest-badge]][reqwest] [![select-badge]][select] [![url-badge]][url] [![cat-network-programming-badge]][cat-network-programming]

Call `get_base_url` to retrieve the base URL. If the document has a base tag,
get the href [`attr`][attr] from base tag. [`Position::BeforePath`][Position::BeforePath] of the original
URL acts as a default.

Iterates through links in the document and creates a [`tokio::spawn`][tokio::spawn] task that will
parse an individual link with [`url::ParseOptions`][url::ParseOptions] and [`Url::parse`][Url::parse] .
The task makes a request to the links with [reqwest] and verifies
`[StatusCode]` Then the tasks `await` completion before ending the program.

```rust,editable,no_run
{{#include ../../deps/examples/broken.rs}}
```

## Extract all unique links from a MediaWiki markup

[![reqwest-badge]][reqwest] [![regex-badge]][regex] [![cat-network-programming-badge]][cat-network-programming]

Pull the source of a MediaWiki page using [`reqwest::get`][reqwest::get] and then
look for all entries of internal and external links with
`[Regex::captures_iter]` Using [`Cow`][Cow] avoids excessive [`String`][String] allocations.

MediaWiki link syntax is described [here][MediaWiki link syntax].

```rust,editable,no_run
{{#include ../../deps/examples/unique.rs}}
```

{{#include ../refs/link-refs.md}}
