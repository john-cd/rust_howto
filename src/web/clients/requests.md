# Making Requests

## Make a HTTP GET request

[![reqwest-badge]][reqwest] [![cat-network-programming-badge]][cat-network-programming]

Parses the supplied URL and makes a synchronous HTTP GET request
with [`reqwest::blocking::get`][reqwest::blocking::get] Prints obtained `[reqwest::blocking::Response]`
status and headers. Reads HTTP response body into an allocated `[String]`
using `[read_to_string]`

```rust,editable,no_run
{{#include ../../../deps/examples/get.rs}}
```

## Async

A similar approach can be used by including the [`tokio`][tokio] executor
to make the main function asynchronous, retrieving the same information.

In this example, [`tokio::main`][tokio::main] handles all the heavy executor setup
and allows sequential code implemented without blocking until `.await`.

Uses the asynchronous versions of [reqwest], both [`reqwest::get`][reqwest::get] and
`[reqwest::Response]`

```rust,no_run
{{#include ../../../deps/examples/get1.rs}}
```

## Set custom headers and URL parameters for a REST request

[![reqwest-badge]][reqwest] [![hyper-badge]][hyper] [![url-badge]][url] [![cat-network-programming-badge]][cat-network-programming]

Sets both standard and custom HTTP headers as well as URL parameters
for a HTTP GET request. Creates a custom header of type `XPoweredBy`
with [`hyper::header!`][hyper::header!] macro.

Builds complex URL with [`Url::parse_with_params`][Url::parse_with_params] Sets standard headers
`[header::UserAgent]` [`header::Authorization`][header::Authorization] and custom `XPoweredBy`
with [`RequestBuilder::header`][RequestBuilder::header] then makes the request with
`[RequestBuilder::send]`

The request targets <http://httpbin.org/headers> service which responds with
a JSON dict containing all request headers for easy verification.

```rust,editable,no_run
{{#include ../../../deps/examples/header.rs}}
```

[header::Authorization]: https://doc.servo.org/hyper/header/struct.Authorization.html
[header::UserAgent]: https://doc.servo.org/hyper/header/struct.UserAgent.html
[hyper::header!]: https://doc.servo.org/hyper/macro.header.html
[RequestBuilder::header]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.header
[RequestBuilder::send]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send
[Url::parse_with_params]: https://docs.rs/url/*/url/struct.Url.html#method.parse_with_params
[read_to_string]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string
[reqwest::blocking::get]: https://docs.rs/reqwest/*/reqwest/blocking/fn.get.html
[reqwest::blocking::Response]: https://docs.rs/reqwest/*/reqwest/blocking/struct.Response.html
[reqwest::get]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[reqwest::Response]: https://docs.rs/reqwest/*/reqwest/struct.Response.html
[String]: https://doc.rust-lang.org/std/string/struct.String.html
[tokio]: https://docs.rs/crate/tokio/0.2.11
[tokio::main]: https://tokio.rs/tokio/tutorial/hello-tokio#the-code
{{#include ../../refs/link-refs.md}}
