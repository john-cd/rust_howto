# Making Requests

{{#include requests.incl.md}}

## Make a HTTP GET request

[![reqwest][c-reqwest-badge]][c-reqwest]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]

Parses the supplied URL and makes a synchronous HTTP GET request with [`{{i:reqwest::blocking::get}}`][c-reqwest::blocking::get]⮳ Prints obtained [`{{i:reqwest::blocking::Response}}`][c-reqwest::blocking::Response]⮳ status and headers. Reads HTTP response body into an allocated [`{{i:String}}`][c-std::string::String]⮳ using [`{{i:read_to_string}}`][c-std::io::Read::read_to_string]⮳.

```rust,editable,no_run
{{#include ../../../deps/tests/get.rs}}
```

## Async

A similar approach can be used by including the [`{{i:tokio}}`][c-tokio]⮳ executor to make the main function asynchronous, retrieving the same information.

In this example, [`{{i:tokio::main}}`][c-tokio-tutorial-hello-tokio]⮳ handles all the heavy executor setup and allows sequential code implemented without blocking until `.await`.

Uses the asynchronous versions of [`{{i:reqwest}}`][c-reqwest]⮳, both [`{{i:reqwest::get}}`][c-reqwest::get]⮳ and
[`{{i:reqwest::Response}}`][c-reqwest::Response]⮳.

```rust,no_run
{{#include ../../../deps/tests/get1.rs}}
```

## Set custom headers and URL parameters for a REST request

[![reqwest][c-reqwest-badge]][c-reqwest]  [![hyper][c-hyper-badge]][c-hyper]  [![url][c-url-badge]][c-url]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-client][cat-http-client-badge]][cat-http-client]

Sets both standard and custom HTTP headers as well as URL parameters for a HTTP GET request. Creates a custom header of type `XPoweredBy` with [`{{i:hyper::header!}}`][c-hyper::header!]⮳ macro.

Builds complex URL with [`{{i:Url::parse_with_params}}`][c-url::Url::parse_with_params]⮳. Sets standard headers
[`{{i:header::UserAgent}}`][c-hyper::header::USER_AGENT]⮳  [`{{i:header::Authorization}}`][c-hyper::header::AUTHORIZATION]⮳ and custom `XPoweredBy` with [`{{i:RequestBuilder::header}}`][c-reqwest::RequestBuilder::header]⮳, then makes the request with
[`{{i:RequestBuilder::send}}`][c-reqwest::RequestBuilder::send]⮳.

The request targets <http://httpbin.org/headers> service which responds with a JSON dict containing all request headers for easy verification.

```rust,editable,no_run
{{#include ../../../deps/tests/header.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
