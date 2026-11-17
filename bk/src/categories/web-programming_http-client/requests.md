<!-- TODO Reviewed ??-??-2026. -->

# Make HTTP Requests

{{#include requests.incl.md}}

## Make an HTTP GET Request {#make-a-http-get-request}

This example uses a reusable client, a request deadline, query parameters, a success-status check, and JSON decoding against a loopback service. It illustrates the basic flow before the custom-header recipe below.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/offline_http_client.rs:example}}
```

## Set Custom Headers and URL Parameters for a REST Request {#custom-headers-and-url-parameters}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![hyper][c~hyper~docs~badge]][c~hyper~docs]{{hi:hyper}} [![url][c~url~docs~badge]][c~url~docs]{{hi:url}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]{{hi:HTTP client}}

Set default and per-request HTTP headers as well as [URL][p~url] query parameters. The live HTTP echo example is opt-in; a local test verifies the request headers and JSON parsing without network access.

Build a URL with query parameters using [`url::Url::parse_with_params`][c~url::Url::parse_with_params~docs]↗{{hi:url::Url::parse_with_params}}. Set a default user agent and custom header on the client, then request JSON with `Accept` and add a bearer authorization header to this request. The token in the sample is a placeholder; load real credentials from a secure source. It sends the request with
[`reqwest::RequestBuilder::send`][c~reqwest::RequestBuilder::send~docs]↗{{hi:reqwest::RequestBuilder::send}}.

The request targets [httpbin.org][httpbin.org~headers]↗, which responds with a [JSON][p~json] object containing the received request headers. The response is decoded with [`reqwest::Response::json`][c~reqwest::Response::json~docs]↗{{hi:reqwest::Response::json}}; enable reqwest's `json` feature to use this method.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/requests/header.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Download and Upload](download.md) for streaming files and byte ranges.
- [HTTP Servers](../web-programming_http-server/json_patch.md) for server-side request handling.
- [Uniform Resource Locator](../web-programming/url.md) for URL parsing and construction.
- [JSON and Serde](../encoding/serde.md) for decoding response bodies.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
