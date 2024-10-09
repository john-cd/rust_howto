# Downloads

{{#include download.incl.md}}

## Download a file to a temporary directory

[![reqwest][reqwest-badge]][reqwest]  [![tempdir][tempdir-badge]][tempdir]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]

Creates a temporary directory with [`{{i:tempfile::Builder}}`][tempfile::Builder]⮳ and downloads a file over HTTP using [`{{i:reqwest::get}}`][reqwest::get]⮳ asynchronously.

Creates a target [`{{i:File}}`][std::fs::File]⮳ with name obtained from [`{{i:Response::url}}`][reqwest::Response::url]⮳ within
[`{{i:tempdir()}}`][tempfile::Builder::tempdir]⮳ and copies downloaded data into it with [`{{i:io::copy}}`][std::io::copy]⮳. The temporary directory is automatically removed on program exit.

```rust,editable,no_run
{{#include ../../../deps/tests/basic.rs}}
```

## POST a file to paste-rs

[![reqwest][reqwest-badge]][reqwest]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-client][cat-http-client-badge]][cat-http-client]

[`{{i:reqwest::Client}}`][reqwest::Client]⮳ establishes a connection to <https://paste.rs> following the [`{{i:reqwest::RequestBuilder}}`][reqwest::RequestBuilder]⮳ pattern. Calling [`{{i:Client::post}}`][reqwest::Client::post]⮳ with a URL establishes the destination, [`{{i:RequestBuilder::body}}`][reqwest::RequestBuilder::body]⮳ sets the content to send by reading the file, and [`{{i:RequestBuilder::send}}`][reqwest::RequestBuilder::send]⮳ blocks until the file uploads and the response returns. [`{{i:read_to_string}}`][std::io::Read::read_to_string]⮳ returns the response and displays in the console.

```rust,editable,no_run
{{#include ../../../deps/tests/post-file.rs}}
```

## Make a partial download with HTTP range headers

[![reqwest][reqwest-badge]][reqwest]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-client][cat-http-client-badge]][cat-http-client]

Uses [`{{i:reqwest::blocking::Client::head}}`][reqwest::blocking::Client::head]⮳ to get the [`{{i:Content-Length}}`][mozilla-content-length]⮳ of the response.

The code then uses [`{{i:reqwest::blocking::Client::get}}`][reqwest::blocking::Client::get]⮳ to download the content in chunks of 10240 bytes, while printing progress messages. This exmple uses the synchronous reqwest module. The [`{{i:Range}}`][mozilla-range]⮳ header specifies the chunk size and position.

The Range header is defined in [`{{i:RFC7233}}`][http-range-rfc7233]⮳.

```rust,editable,no_run
{{#include ../../../deps/tests/partial.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
