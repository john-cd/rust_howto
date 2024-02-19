# Downloads

## Download a file to a temporary directory

[![reqwest-badge]][reqwest] [![tempdir-badge]][tempdir] [![cat-network-programming-badge]][cat-network-programming] [![cat-filesystem-badge]][cat-filesystem]

Creates a temporary directory with [`tempfile::Builder`][tempfile::Builder] and downloads
a file over HTTP using [`reqwest::get`][reqwest::get] asynchronously.

Creates a target [`File`][File] with name obtained from [`Response::url`][Response::url] within
[`tempdir()`][tempdir()] and copies downloaded data into it with [`io::copy`][io::copy]
The temporary directory is automatically removed on program exit.

```rust,editable,no_run
{{#include ../../../deps/examples/basic.rs}}
```

## POST a file to paste-rs

[![reqwest-badge]][reqwest] [![cat-network-programming-badge]][cat-network-programming]

[`reqwest::Client`][reqwest::Client] establishes a connection to <https://paste.rs>
following the [`reqwest::RequestBuilder`][reqwest::RequestBuilder] pattern.  Calling [`Client::post`][Client::post]
with a URL establishes the destination, [`RequestBuilder::body`][RequestBuilder::body] sets the
content to send by reading the file, and [`RequestBuilder::send`][RequestBuilder::send] blocks until
the file uploads and the response returns.  [`read_to_string`][read_to_string] returns the
response and displays in the console.

```rust,editable,no_run
{{#include ../../../deps/examples/post-file.rs}}
```

## Make a partial download with HTTP range headers

[![reqwest-badge]][reqwest] [![cat-network-programming-badge]][cat-network-programming]

Uses [`reqwest::blocking::Client::head`][reqwest::blocking::Client::head] to get the [Content-Length][Content-Length] of the response.

The code then uses [`reqwest::blocking::Client::get`][reqwest::blocking::Client::get] to download the content in
chunks of 10240 bytes, while printing progress messages. This exmple uses the synchronous
reqwest module.  The [Range][Range] header specifies the chunk size and position.

The Range header is defined in [RFC7233][HTTP Range RFC7233].

```rust,editable,no_run
{{#include ../../../deps/examples/partial.rs}}
```

{{#include ../../refs/link-refs.md}}
