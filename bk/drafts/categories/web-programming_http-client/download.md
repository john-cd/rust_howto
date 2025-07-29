# Download and Upload

{{#include download.incl.md}}

## Download a File to a Temporary Directory {#download}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![tempfile][c~tempfile~docs~badge]][c~tempfile~docs]{{hi:tempfile}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

Creates a temporary directory with [`tempfile::Builder`][c~tempfile::Builder~docs]{{hi:tempfile::Builder}}⮳ and downloads a file over HTTP using [`reqwest::get`][c~reqwest::get~docs]{{hi:reqwest::get}}⮳ asynchronously.

Creates a target [`std::fs::File`][c~std::fs::File~docs]{{hi:std::fs::File}}⮳ with name obtained from [`reqwest::Response::url`][c~reqwest::Response::url~docs]{{hi:reqwest::Response::url}}⮳ within
[`tempfile::Builder::tempdir`][c~tempfile::Builder::tempdir~docs]{{hi:tempfile::Builder::tempdir}}⮳ and copies downloaded data into it with [`std::io::copy`][c~std::io::copy~docs]{{hi:std::io::copy}}⮳. The temporary directory is automatically removed on program exit.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/download/download.rs:example}}
```

## POST a File to `paste.rs` {#post-a-file-to-paste-rs}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]{{hi:HTTP client}}

[`reqwest::Client`][c~reqwest::Client~docs]{{hi:reqwest::Client}}⮳ establishes a connection to [`https://paste.rs`][paste.rs] following the [`reqwest::RequestBuilder`][c~reqwest::RequestBuilder~docs]{{hi:reqwest::RequestBuilder}}⮳ pattern. Calling [`reqwest::Client::post`][c~reqwest::Client::post~docs]{{hi:reqwest::Client::post}}⮳ with a URL establishes the destination, [`reqwest::RequestBuilder::body`][c~reqwest::RequestBuilder::body~docs]{{hi:reqwest::RequestBuilder::body}}⮳ sets the content to send by reading the file, and [`reqwest::RequestBuilder::send`][c~reqwest::RequestBuilder::send~docs]{{hi:reqwest::RequestBuilder::send}}⮳ blocks until the file uploads and the response returns. [`std::io::Read::read_to_string`][c~std::io::Read::read_to_string~docs]{{hi:std::io::Read::read_to_string}}⮳ returns the response and displays in the console.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/download/post_file.rs:example}}
```

## Make a Partial Download with HTTP Range Headers {#partial-download-with-http-range-headers}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]{{hi:HTTP client}}

Uses [`reqwest::blocking::Client::head`][c~reqwest::blocking::Client::head~docs]{{hi:reqwest::blocking::Client::head}}⮳ to get the [`Content-Length`][mozilla~content-length]{{hi:Content length}}⮳ of the response.

The code then uses [`reqwest::blocking::Client::get`][c~reqwest::blocking::Client::get~docs]{{hi:reqwest::blocking::Client::get}}⮳ to download the content in chunks of 10240 bytes, while printing progress messages. This example uses the synchronous [`reqwest`][c~reqwest~docs]⮳{{hi:reqwest}} module. The [`Range`][mozilla~range]{{hi:Range}}⮳ header specifies the chunk size and position.

The Range header is defined in [`RFC7233`][ietf~http-range-rfc7233]{{hi:RFC-7233}}⮳.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/download/partial.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/975)
</div>
