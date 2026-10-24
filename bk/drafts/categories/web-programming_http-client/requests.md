<!-- TODO Reviewed ??-??-2026 -->
# Make HTTP Requests

{{#include requests.incl.md}}

## Make a HTTP GET Request {#make-a-http-get-request}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

Parses the supplied [URL][p~url] and makes a synchronous HTTP GET request with [`reqwest::blocking::get`][c~reqwest::blocking::get~docs]↗{{hi:reqwest::blocking::get}}. It checks the status with [`reqwest::Response::error_for_status`][c~reqwest::Response::error_for_status~docs]↗{{hi:reqwest::Response::error_for_status}} before consuming the response body. Use the blocking client from synchronous code; in async code, use the asynchronous client and await its operations.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/requests/get.rs:example}}
```

## Make a HTTP GET Request Asynchronously {#async}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![tokio~website][c~tokio~website~badge]][c~tokio~website] [![tokio][c~tokio~docs~badge]][c~tokio~docs] [![tokio~crates.io][c~tokio~crates.io~badge]][c~tokio~crates.io] [![tokio~repo][c~tokio~repo~badge]][c~tokio~repo] [![tokio~lib.rs][c~tokio~lib.rs~badge]][c~tokio~lib.rs]{{hi:tokio}}{{hi:Async}}{{hi:Futures}}{{hi:Io}}{{hi:Non-blocking}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

A similar approach can be used by including the [`tokio`][c~tokio~docs]↗{{hi:tokio}} executor to make the main function asynchronous, retrieving the same information.

In this example, [`tokio::main`][c~tokio~tutorial_hello_tokio~website]↗{{hi:tokio::main}} handles all the heavy executor setup and allows sequential code implemented without blocking until [`.await`][book~rust-reference~await]↗{{hi:.await}}.

Uses the [asynchronous][p~asynchronous] versions of [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}}, both [`reqwest::get`][c~reqwest::get~docs]↗{{hi:reqwest::get}} and
[`reqwest::Response`][c~reqwest::Response~docs]↗{{hi:reqwest::Response}}. The example checks for an unsuccessful status before reading the body.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/requests/get1.rs:example}}
```

## Set Custom Headers and URL Parameters for a REST Request {#custom-headers-and-url-parameters}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![hyper][c~hyper~docs~badge]][c~hyper~docs]{{hi:hyper}} [![url][c~url~docs~badge]][c~url~docs]{{hi:url}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]{{hi:HTTP client}}

Sets standard and custom HTTP headers as well as [URL][p~url] parameters for an HTTP GET request. The live HTTP echo example is opt-in; a local test verifies the request headers and JSON parsing without network access.

Builds a URL with query parameters using [`url::Url::parse_with_params`][c~url::Url::parse_with_params~docs]↗{{hi:url::Url::parse_with_params}}. Sets a default user agent and custom header on the client, then adds a content type and bearer authorization header to this request. It sends the request with
[`reqwest::RequestBuilder::send`][c~reqwest::RequestBuilder::send~docs]↗{{hi:reqwest::RequestBuilder::send}}.

The request targets [httpbin.org][httpbin.org~headers]↗, which responds with a [JSON][p~json] object containing the received request headers. The response is decoded with [`reqwest::Response::json`][c~reqwest::Response::json~docs]↗{{hi:reqwest::Response::json}}; enable reqwest's `json` feature to use this method.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/requests/header.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[download | Download and Upload]].
- [[server | HTTP Server]].
- [[url | Uniform Resource Locator]].
- [[json | JSON]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
This review makes success-status handling explicit, uses HTTPS for the remote examples, and keeps network-dependent checks opt-in. A local HTTP echo test covers default headers, bearer authentication, and JSON decoding without relying on an external service.
</div>
