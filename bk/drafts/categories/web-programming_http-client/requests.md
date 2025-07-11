# Make HTTP Requests

{{#include requests.incl.md}}

## Make a HTTP GET Request {#make-a-http-get-request}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

Parses the supplied [URL][p~url] and makes a synchronous HTTP GET request with [`reqwest::blocking::get`][c~reqwest::blocking::get~docs]{{hi:reqwest::blocking::get}}⮳ Prints obtained [`reqwest::blocking::Response`][c~reqwest::blocking::Response~docs]{{hi:reqwest::blocking::Response}}⮳ status and headers. Reads HTTP response body into an allocated [`std::string::String`][c~std::string::String~docs]{{hi:std::string::String}}⮳ using [`std::io::Read::read_to_string`][c~std::io::Read::read_to_string~docs]{{hi:std::io::Read::read_to_string}}⮳.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/requests/get.rs:example}}
```

## Make a HTTP GET Request Asynchronously {#async}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![tokio~website][c~tokio~website~badge]][c~tokio~website] [![tokio][c~tokio~docs~badge]][c~tokio~docs] [![tokio~crates.io][c~tokio~crates.io~badge]][c~tokio~crates.io] [![tokio~github][c~tokio~github~badge]][c~tokio~github] [![tokio~lib.rs][c~tokio~lib.rs~badge]][c~tokio~lib.rs]{{hi:tokio}}{{hi:Async}}{{hi:Futures}}{{hi:Io}}{{hi:Non-blocking}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

A similar approach can be used by including the [`tokio`][c~tokio~docs]{{hi:tokio}}⮳ executor to make the main function asynchronous, retrieving the same information.

In this example, [`tokio::main`][c~tokio_tutorial_hello_tokio~website]{{hi:tokio::main}}⮳ handles all the heavy executor setup and allows sequential code implemented without blocking until [`.await`][book~rust-reference~await]⮳{{hi:.await}}.

Uses the [asynchronous][p~asynchronous] versions of [`reqwest`][c~reqwest~docs]{{hi:reqwest}}⮳, both [`reqwest::get`][c~reqwest::get~docs]{{hi:reqwest::get}}⮳ and
[`reqwest::Response`][c~reqwest::Response~docs]{{hi:reqwest::Response}}⮳.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/requests/get1.rs:example}}
```

## Set Custom Headers and URL Parameters for a REST Request {#custom-headers-and-url-parameters}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![hyper][c~hyper~docs~badge]][c~hyper~docs]{{hi:hyper}} [![url][c~url~docs~badge]][c~url~docs]{{hi:url}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]{{hi:HTTP client}}

Sets both standard and custom HTTP headers as well as [URL][p~url] parameters for a HTTP GET request.

Builds complex [URL][p~url] with [`url::Url::parse_with_params`][c~url::Url::parse_with_params~docs]{{hi:url::Url::parse_with_params}}⮳. Sets standard headers
[`hyper::header::USER_AGENT`][c~hyper::header::USER_AGENT~docs]{{hi:hyper::header::USER_AGENT}}⮳ [`hyper::header::AUTHORIZATION`][c~hyper::header::AUTHORIZATION~docs]{{hi:hyper::header::AUTHORIZATION}}⮳ and custom `XPoweredBy` with [`reqwest::RequestBuilder::header`][c~reqwest::RequestBuilder::header~docs]{{hi:reqwest::RequestBuilder::header}}⮳, then makes the request with
[`reqwest::RequestBuilder::send`][c~reqwest::RequestBuilder::send~docs]{{hi:reqwest::RequestBuilder::send}}⮳.

The request targets [http://httpbin.org/headers][httpbin.org~headers] service which responds with a [JSON][p~json] dict containing all request headers for easy verification.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/requests/header.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/976)
</div>
