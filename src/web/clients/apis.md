# Calling a Web API

## Query the GitHub API

[![reqwest][reqwest-badge]][reqwest]  [![serde][serde-badge]][serde]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Queries GitHub [stargazers API v3](https://developer.github.com/v3/activity/starring/#list-stargazers) with [`reqwest::get`][reqwest::get] to get list of all users who have marked a GitHub project with a star.
[`reqwest::Response`][reqwest::Response] is deserialized with [`Response::json`][reqwest::Response::json] into `User` objects implementing [`serde::Deserialize`][serde::Deserialize]

[`tokio::main`][tokio-tutorial-hello-tokio] is used to set up the async executor and the process waits for [`reqwest::get`][reqwest::get] to complete before processing the response into User instances.

```rust,editable,no_run
{{#include ../../../deps/tests/rest-get.rs}}
```

## Check if an API resource exists

[![reqwest][reqwest-badge]][reqwest]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]

Query the GitHub Users Endpoint using a HEAD request ([`Client::head`][reqwest::Client::head] and then inspect the response code to determine success. This is a quick way to query a rest resource without needing to receive a body. [`reqwest::Client`][reqwest::Client] configured with [`ClientBuilder::timeout`][reqwest::ClientBuilder::timeout] ensures a request will not last longer than a timeout.

Due to both [`ClientBuilder::build`][reqwest::ClientBuilder::build] and [`RequestBuilder::send`][reqwest::RequestBuilder::send] returning [`reqwest::Error`][reqwest::Error] types, the shortcut [`reqwest::Result`][reqwest::Result] is used for the main function return type.

```rust,editable,no_run
{{#include ../../../deps/tests/rest-head.rs}}
```

## Create and delete Gist with GitHub API

[![reqwest][reqwest-badge]][reqwest]  [![serde][serde-badge]][serde]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Creates a gist with POST request to GitHub [gists API v3](https://developer.github.com/v3/gists/) using [`Client::post`][reqwest::Client::post] and removes it with DELETE request using [`Client::delete`][reqwest::Client::delete]

The [`reqwest::Client`][reqwest::Client] is responsible for details of both requests including URL, body and authentication. The POST body from [`serde_json::json!`][serde_json::json] macro provides arbitrary JSON body. Call to [`RequestBuilder::json`][reqwest::RequestBuilder::json] sets the request body. [`RequestBuilder::basic_auth`][reqwest::RequestBuilder::basic_auth] handles authentication. The call to
[`RequestBuilder::send`][reqwest::RequestBuilder::send] synchronously executes the requests.

```rust,editable,no_run
{{#include ../../../deps/tests/rest-post.rs}}
```

The example uses HTTP [Basic Auth`][http-basic-auth] in order to authorize access to [`GitHub API`][github-api]. Typical use case would employ one of the much more complex [`OAuth`][oauth] authorization flows.

## Consume a paginated RESTful API

[![reqwest][reqwest-badge]][reqwest]  [![serde][serde-badge]][serde]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Wraps a paginated web API in a convenient Rust iterator. The iterator lazily fetches the next page of results from the remote server as it arrives at the end of each page.

```rust,editable,no_run
{{#include ../../../deps/tests/paginated.rs}}
```

## Handle a rate-limited API

[![reqwest][reqwest-badge]][reqwest]  [![hyper][hyper-badge]][hyper]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]

This example uses the [`GitHub API - Rate limiting`][github-api-rate-limiting], as an example of how to handle remote server errors. This example uses the [`hyper::header!`][hyper::header!] macro to parse the response header and checks for [`reqwest::StatusCode::Forbidden`][reqwest::StatusCode::FORBIDDEN] If the response exceeds the rate limit, the example waits and retries.

```rust,editable,no_run
{{#include ../../../deps/tests/rate-limited.rs}}
```

{{#include ../../refs/link-refs.md}}
