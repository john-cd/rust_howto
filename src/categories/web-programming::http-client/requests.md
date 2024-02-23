# Making Requests

## Make a HTTP GET request

[![reqwest][reqwest-badge]][reqwest]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]

Parses the supplied URL and makes a synchronous HTTP GET request with [`reqwest::blocking::get`][reqwest::blocking::get] Prints obtained [`reqwest::blocking::Response`][reqwest::blocking::Response] status and headers. Reads HTTP response body into an allocated [`String`][std::string::String] using [`read_to_string`][std::io::Read::read_to_string]

```rust,editable,no_run
{{#include ../../../deps/tests/get.rs}}
```

## Async

A similar approach can be used by including the [`tokio`][tokio] executor to make the main function asynchronous, retrieving the same information.

In this example, [`tokio::main`][tokio-tutorial-hello-tokio] handles all the heavy executor setup and allows sequential code implemented without blocking until `.await`.

Uses the asynchronous versions of [`reqwest`][reqwest], both [`reqwest::get`][reqwest::get] and
[`reqwest::Response`][reqwest::Response]

```rust,no_run
{{#include ../../../deps/tests/get1.rs}}
```

## Set custom headers and URL parameters for a REST request

[![reqwest][reqwest-badge]][reqwest]  [![hyper][hyper-badge]][hyper]  [![url][url-badge]][url]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]

Sets both standard and custom HTTP headers as well as URL parameters for a HTTP GET request. Creates a custom header of type `XPoweredBy` with [`hyper::header!`][hyper::header!] macro.

Builds complex URL with [`Url::parse_with_params`][url::Url::parse_with_params]. Sets standard headers
[`header::UserAgent`][hyper::header::USER_AGENT]  [`header::Authorization`][hyper::header::AUTHORIZATION] and custom `XPoweredBy` with [`RequestBuilder::header`][reqwest::RequestBuilder::header] then makes the request with
[`RequestBuilder::send`][reqwest::RequestBuilder::send]

The request targets <http://httpbin.org/headers> service which responds with a JSON dict containing all request headers for easy verification.

```rust,editable,no_run
{{#include ../../../deps/tests/header.rs}}
```

{{#include ../../refs/link-refs.md}}
