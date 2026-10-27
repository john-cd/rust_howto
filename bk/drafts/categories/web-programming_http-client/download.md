<!-- TODO Reviewed ??-??-2026 -->

# Download and Upload

{{#include download.incl.md}}

## Download a File to a Temporary Directory {#download}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![tempfile][c~tempfile~docs~badge]][c~tempfile~docs]{{hi:tempfile}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

Creates a temporary directory with [`tempfile::Builder`][c~tempfile::Builder~docs]↗{{hi:tempfile::Builder}} and downloads a file over HTTP using [`reqwest::get`][c~reqwest::get~docs]↗{{hi:reqwest::get}} asynchronously. It calls [`reqwest::Response::error_for_status`][c~reqwest::Response::error_for_status~docs]↗{{hi:reqwest::Response::error_for_status}} before streaming the response body.

Uses [`reqwest::Response::bytes_stream`][c~reqwest::Response::bytes_stream~docs]↗{{hi:reqwest::Response::bytes_stream}} to yield binary chunks and writes them with Tokio's asynchronous file API. The temporary directory is automatically removed on program exit. Streaming keeps the whole response body out of memory.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/download/download.rs:example}}
```

## POST a File with Reqwest {#post-a-file-with-reqwest}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]{{hi:HTTP client}}

The example asynchronously reads the file, posts its bytes to `https://httpbin.org/post`, checks the response status, and prints the response body. Its tests use a local HTTP server.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/download/post_file.rs:example}}
```

## Make a Partial Download with HTTP Range Headers {#partial-download-with-http-range-headers}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]{{hi:HTTP client}}

Uses [`reqwest::blocking::Client::head`][c~reqwest::blocking::Client::head~docs]↗{{hi:reqwest::blocking::Client::head}} to get the [`Content-Length`][mozilla~content-length]↗{{hi:Content length}} of the response.

The code then uses [`reqwest::blocking::Client::get`][c~reqwest::blocking::Client::get~docs]↗{{hi:reqwest::blocking::Client::get}} to download verified ranges of at most 10240 bytes. It checks for `206 Partial Content`, validates `Content-Range`, and rejects short chunks before continuing. This example uses the synchronous [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}} module. The [`Range`][mozilla~range]↗{{hi:Range}} header specifies the requested byte positions. A command-line progress display can be added with [`indicatif`][c~indicatif~docs]↗{{hi:indicatif}} when the total size is known.

The Range header is defined in [`RFC7233`][ietf~http-range-rfc7233]↗{{hi:RFC-7233}}.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/download/partial.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[requests | Make HTTP Requests]].
- [[server | HTTP Server]].
- [[filesystem | Filesystem]].
- [[command-line-interface | Command Line Interface]] for progress bars with `indicatif`.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  This review changes the basic download example to stream binary chunks into a temporary file, verifies that behavior with a local server, and corrects the upload and byte-range descriptions to match the existing examples.
</div>
