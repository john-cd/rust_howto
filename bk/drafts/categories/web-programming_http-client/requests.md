<!-- TODO Reviewed ??-??-2026. -->

# Make HTTP Requests

{{#include requests.incl.md}}

## Make an HTTP GET Request {#make-a-http-get-request}

The published [local HTTP client recipe](../../../src/categories/web-programming_http-client/offline_http_client.md) owns the runnable GET workflow, status checks, deadline, and JSON decoding.

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
