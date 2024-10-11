# Downloads

{{#include download.incl.md}}

## Download a file to a temporary directory

[![reqwest][c-reqwest-badge]][c-reqwest]  [![tempdir][c-tempdir-badge]][c-tempdir]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]

Creates a temporary directory with {{hi:tempfile::Builder}}[`tempfile::Builder`][c-tempfile::Builder]⮳ and downloads a file over HTTP using {{hi:reqwest::get}}[`reqwest::get`][c-reqwest::get]⮳ asynchronously.

Creates a target {{hi:File}}[`File`][c-std::fs::File]⮳ with name obtained from {{hi:Response::url}}[`Response::url`][c-reqwest::Response::url]⮳ within
{{hi:tempdir()}}[`tempdir()`][c-tempfile::Builder::tempdir]⮳ and copies downloaded data into it with {{hi:io::copy}}[`io::copy`][c-std::io::copy]⮳. The temporary directory is automatically removed on program exit.

```rust,editable,no_run
{{#include ../../../deps/tests/basic.rs}}
```

## POST a file to paste-rs

[![reqwest][c-reqwest-badge]][c-reqwest]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-client][cat-http-client-badge]][cat-http-client]

{{hi:reqwest::Client}}[`reqwest::Client`][c-reqwest::Client]⮳ establishes a connection to <https://paste.rs> following the {{hi:reqwest::RequestBuilder}}[`reqwest::RequestBuilder`][c-reqwest::RequestBuilder]⮳ pattern. Calling {{hi:Client::post}}[`Client::post`][c-reqwest::Client::post]⮳ with a URL establishes the destination, {{hi:RequestBuilder::body}}[`RequestBuilder::body`][c-reqwest::RequestBuilder::body]⮳ sets the content to send by reading the file, and {{hi:RequestBuilder::send}}[`RequestBuilder::send`][c-reqwest::RequestBuilder::send]⮳ blocks until the file uploads and the response returns. {{hi:read_to_string}}[`read_to_string`][c-std::io::Read::read_to_string]⮳ returns the response and displays in the console.

```rust,editable,no_run
{{#include ../../../deps/tests/post-file.rs}}
```

## Make a partial download with HTTP range headers

[![reqwest][c-reqwest-badge]][c-reqwest]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-client][cat-http-client-badge]][cat-http-client]

Uses {{hi:reqwest::blocking::Client::head}}[`reqwest::blocking::Client::head`][c-reqwest::blocking::Client::head]⮳ to get the {{hi:Content-Length}}[`Content-Length`][mozilla-content-length]⮳ of the response.

The code then uses {{hi:reqwest::blocking::Client::get}}[`reqwest::blocking::Client::get`][c-reqwest::blocking::Client::get]⮳ to download the content in chunks of 10240 bytes, while printing progress messages. This exmple uses the synchronous reqwest module. The {{hi:Range}}[`Range`][mozilla-range]⮳ header specifies the chunk size and position.

The Range header is defined in {{hi:RFC7233}}[`RFC7233`][http-range-rfc7233]⮳.

```rust,editable,no_run
{{#include ../../../deps/tests/partial.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
