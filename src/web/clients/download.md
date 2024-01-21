# Downloads

## Download a file to a temporary directory

[![reqwest-badge]][reqwest] [![tempdir-badge]][tempdir] [![cat-network-programming-badge]][cat-network-programming] [![cat-filesystem-badge]][cat-filesystem]

Creates a temporary directory with `[tempfile::Builder]` and downloads
a file over HTTP using `[reqwest::get]` asynchronously.

Creates a target `[File]` with name obtained from `[Response::url]` within
`[tempdir()]` and copies downloaded data into it with `[io::copy]`
The temporary directory is automatically removed on program exit.

```rust,editable,no_run
{#include ../../deps/examples/basic.rs}
```

## POST a file to paste-rs

[![reqwest-badge]][reqwest] [![cat-network-programming-badge]][cat-network-programming]

`[reqwest::Client]` establishes a connection to <https://paste.rs>
following the `[reqwest::RequestBuilder]` pattern.  Calling `[Client::post]`
with a URL establishes the destination, `[RequestBuilder::body]` sets the
content to send by reading the file, and `[RequestBuilder::send]` blocks until
the file uploads and the response returns.  `[read_to_string]` returns the
response and displays in the console.

```rust,editable,no_run
{#include ../../deps/examples/post-file.rs}
```

## Make a partial download with HTTP range headers

[![reqwest-badge]][reqwest] [![cat-network-programming-badge]][cat-network-programming]

Uses `[reqwest::blocking::Client::head]` to get the [Content-Length] of the response.

The code then uses `[reqwest::blocking::Client::get]` to download the content in
chunks of 10240 bytes, while printing progress messages. This exmple uses the synchronous
reqwest module.  The [Range] header specifies the chunk size and position.

The Range header is defined in [RFC7233][HTTP Range RFC7233].

```rust,editable,no_run
{#include ../../../deps/examples/partial.rs}
```

[Client::post]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.post
[read_to_string]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string
[RequestBuilder::body]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.body
[RequestBuilder::send]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send
[reqwest::Client]: https://docs.rs/reqwest/*/reqwest/struct.Client.html
[reqwest::RequestBuilder]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html
[File]: https://doc.rust-lang.org/std/fs/struct.File.html
[io::copy]: https://doc.rust-lang.org/std/io/fn.copy.html
[reqwest::get]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[Response::url]: https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.url
[tempfile::Builder]: https://docs.rs/tempfile/*/tempfile/struct.Builder.html
[tempdir()]: https://docs.rs/tempfile/3.1.0/tempfile/struct.Builder.html#method.tempdir
[reqwest::blocking::Client::get]: https://docs.rs/reqwest/*/reqwest/blocking/struct.Client.html#method.get
[reqwest::blocking::Client::head]: https://docs.rs/reqwest/*/reqwest/blocking/struct.Client.html#method.head
[Content-Length]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Length
[Range]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Range
[HTTP Range RFC7233]: https://tools.ietf.org/html/rfc7233#section-3.1
{{#include ../../refs/link-refs.md}}
