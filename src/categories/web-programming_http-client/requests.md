# Making Requests

{{#include requests.incl.md}}

## Make a HTTP GET request

[![reqwest][c-reqwest-badge]][c-reqwest]{{hi:reqwest}}  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]

Parses the supplied URL and makes a synchronous HTTP GET request with [`reqwest::blocking::get`][c-reqwest::blocking::get]{{hi:reqwest::blocking::get}}⮳ Prints obtained [`reqwest::blocking::Response`][c-reqwest::blocking::Response]{{hi:reqwest::blocking::Response}}⮳ status and headers. Reads HTTP response body into an allocated [`std::string::String`][c-std::string::String]{{hi:std::string::String}}⮳ using [`std::io::Read::read_to_string`][c-std::io::Read::read_to_string]{{hi:std::io::Read::read_to_string}}⮳.

```rust,no_run
{{#include ../../../deps/tests/get.rs}}
```

## Async

A similar approach can be used by including the [`tokio`][c-tokio]{{hi:tokio}}⮳ executor to make the main function asynchronous, retrieving the same information.

In this example, [`tokio::main`][c-tokio_tutorial_hello_tokio-website]{{hi:tokio::main}}⮳ handles all the heavy executor setup and allows sequential code implemented without blocking until `.await`.

Uses the asynchronous versions of [`reqwest`][c-reqwest]{{hi:reqwest}}⮳, both [`reqwest::get`][c-reqwest::get]{{hi:reqwest::get}}⮳ and
[`reqwest::Response`][c-reqwest::Response]{{hi:reqwest::Response}}⮳.

```rust,no_run
{{#include ../../../deps/tests/get1.rs}}
```

## Set custom headers and URL parameters for a REST request

[![reqwest][c-reqwest-badge]][c-reqwest]{{hi:reqwest}}  [![hyper][c-hyper-badge]][c-hyper]{{hi:hyper}}  [![url][c-url-badge]][c-url]{{hi:url}}  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}}  [![cat-http-client][cat-http-client-badge]][cat-http-client]

Sets both standard and custom HTTP headers as well as URL parameters for a HTTP GET request. Creates a custom header of type `XPoweredBy` with [`hyper::header!`][c-hyper::header!]{{hi:hyper::header!}}⮳ macro.

Builds complex URL with [`url::Url::parse_with_params`][c-url::Url::parse_with_params]{{hi:url::Url::parse_with_params}}⮳. Sets standard headers
[`hyper::header::USER_AGENT`][c-hyper::header::USER_AGENT]{{hi:hyper::header::USER_AGENT}}⮳  [`hyper::header::AUTHORIZATION`][c-hyper::header::AUTHORIZATION]{{hi:hyper::header::AUTHORIZATION}}⮳ and custom `XPoweredBy` with [`reqwest::RequestBuilder::header`][c-reqwest::RequestBuilder::header]{{hi:reqwest::RequestBuilder::header}}⮳, then makes the request with
[`reqwest::RequestBuilder::send`][c-reqwest::RequestBuilder::send]{{hi:reqwest::RequestBuilder::send}}⮳.

The request targets [http://httpbin.org/headers][httpbin.org-headers] service which responds with a JSON dict containing all request headers for easy verification.

```rust,no_run
{{#include ../../../deps/tests/header.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
