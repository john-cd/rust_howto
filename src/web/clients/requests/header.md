## Set custom headers and URL parameters for a REST request

[![reqwest-badge]][reqwest] [![hyper-badge]][hyper] [![url-badge]][url] [![cat-net-badge]][cat-net]

Sets both standard and custom HTTP headers as well as URL parameters
for a HTTP GET request. Creates a custom header of type `XPoweredBy`
with [`hyper::header!`] macro.

Builds complex URL with [`Url::parse_with_params`].  Sets standard headers
[`header::UserAgent`], [`header::Authorization`], and custom `XPoweredBy`
with [`RequestBuilder::header`] then makes the request with
[`RequestBuilder::send`].

The request targets <http://httpbin.org/headers> service which responds with
a JSON dict containing all request headers for easy verification.

```rust,editable,no_run
{#include ../../../deps/examples/header.rs}
```

[`header::Authorization`]: https://doc.servo.org/hyper/header/struct.Authorization.html
[`header::UserAgent`]: https://doc.servo.org/hyper/header/struct.UserAgent.html
[`hyper::header!`]: https://doc.servo.org/hyper/macro.header.html
[`RequestBuilder::header`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.header
[`RequestBuilder::send`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send
[`Url::parse_with_params`]: https://docs.rs/url/*/url/struct.Url.html#method.parse_with_params
