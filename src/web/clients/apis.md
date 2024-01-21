# Calling a Web API

## Query the GitHub API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-network-programming-badge]][cat-network-programming] [![cat-encoding-badge]][cat-encoding]

Queries GitHub [stargazers API v3](https://developer.github.com/v3/activity/starring/#list-stargazers)
with [`reqwest::get`][reqwest::get] to get list of all users who have marked a GitHub project with a star.
`[reqwest::Response]` is deserialized with [`Response::json`][Response::json] into `User` objects implementing `[serde::Deserialize]`

[tokio::main] is used to set up the async executor and the process waits for [`reqwest::get`][reqwest::get] to complete before
processing the response into User instances.

```rust,editable,no_run
{{#include ../../../deps/examples/rest-get.rs}}
```

## Check if an API resource exists

[![reqwest-badge]][reqwest] [![cat-network-programming-badge]][cat-network-programming]

Query the GitHub Users Endpoint using a HEAD
request (`[Client::head]` and then inspect the response code to determine
success. This is a quick way to query a rest resource without needing to receive
a body. [`reqwest::Client`][reqwest::Client] configured with [`ClientBuilder::timeout`][ClientBuilder::timeout] ensures
a request will not last longer than a timeout.

Due to both [`ClientBuilder::build`][ClientBuilder::build] and [`RequestBuilder::send`][RequestBuilder::send] returning `[reqwest::Error]`
types, the shortcut [`reqwest::Result`][reqwest::Result] is used for the main function return type.

```rust,editable,no_run
{{#include ../../../deps/examples/rest-head.rs}}
```

## Create and delete Gist with GitHub API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-network-programming-badge]][cat-network-programming] [![cat-encoding-badge]][cat-encoding]

Creates a gist with POST request to GitHub [gists API v3](https://developer.github.com/v3/gists/)
using [`Client::post`][Client::post] and removes it with DELETE request using `[Client::delete]`

The [`reqwest::Client`][reqwest::Client] is responsible for details of both requests including
URL, body and authentication. The POST body from [`serde_json::json!`][serde_json::json!] macro
provides arbitrary JSON body. Call to [`RequestBuilder::json`][RequestBuilder::json] sets the request
body. [`RequestBuilder::basic_auth`][RequestBuilder::basic_auth] handles authentication. The call to
`[RequestBuilder::send]` synchronously executes the requests.

```rust,editable,no_run
{{#include ../../../deps/examples/rest-post.rs}}
```

The example uses [HTTP Basic Auth] in order to authorize access to [GitHub API].
Typical use case would employ one of the much more complex [OAuth] authorization
flows.

## Consume a paginated RESTful API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-network-programming-badge]][cat-network-programming] [![cat-encoding-badge]][cat-encoding]

Wraps a paginated web API in a convenient Rust iterator. The iterator lazily
fetches the next page of results from the remote server as it arrives at the end
of each page.

```rust,editable,no_run
{{#include ../../../deps/examples/paginated.rs}}
```

## Handle a rate-limited API

[![reqwest-badge]][reqwest] [![hyper-badge]][hyper] [![cat-network-programming-badge]][cat-network-programming]

This example uses the [GitHub API - Rate limiting], as an example of how to
handle remote server errors.  This example uses the [`hyper::header!`][hyper::header!] macro
to parse the response header and checks for `[reqwest::StatusCode::Forbidden]`
If the response exceeds the rate limit, the example waits and retries.

```rust,editable,no_run
{{#include ../../../deps/examples/rate-limited.rs}}
```

[hyper::header!]: https://doc.servo.org/hyper/header/index.html#defining-custom-headers
[reqwest::StatusCode::Forbidden]: https://docs.rs/reqwest/*/reqwest/struct.StatusCode.html#associatedconstant.FORBIDDEN
[ClientBuilder::build]: https://docs.rs/reqwest/*/reqwest/struct.ClientBuilder.html#method.build
[Client::head]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.head
[ClientBuilder::timeout]: https://docs.rs/reqwest/*/reqwest/struct.ClientBuilder.html#method.timeout
[reqwest::Error]: https://docs.rs/reqwest/*/reqwest/struct.Error.html
[reqwest::Result]:https://docs.rs/reqwest/*/reqwest/type.Result.html
[Client::delete]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.delete
[Client::post]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.post
[RequestBuilder::basic_auth]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.basic_auth
[RequestBuilder::json]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.json
[RequestBuilder::send]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send
[reqwest::Client]: https://docs.rs/reqwest/*/reqwest/struct.Client.html
[serde_json::json!]: https://docs.rs/serde_json/*/serde_json/macro.json.html
[GitHub API]: https://developer.github.com/v3/auth/
[HTTP Basic Auth]: https://tools.ietf.org/html/rfc2617
[OAuth]: https://oauth.net/getting-started/
[reqwest::get]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[reqwest::Response]: https://docs.rs/reqwest/*/reqwest/struct.Response.html
[Response::json]: https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.json
[serde::Deserialize]: https://docs.rs/serde/*/serde/trait.Deserialize.html
[GitHub API - Rate limiting]: https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api?apiVersion=2022-11-28
[tokio::main]: https://tokio.rs/tokio/tutorial/hello-tokio#the-code
{{#include ../../refs/link-refs.md}}
