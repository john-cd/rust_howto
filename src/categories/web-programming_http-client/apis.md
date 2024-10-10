# Calling a Web API

{{#include apis.incl.md}}

## Query the GitHub API

[![reqwest][reqwest-badge]][c-reqwest]  [![serde][serde-badge]][c-serde]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-encoding][cat-encoding-badge]][cat-encoding]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-client][cat-http-client-badge]][cat-http-client]

Queries GitHub [stargazers API v3](https://developer.github.com/v3/activity/starring/#list-stargazers)⮳ with [`{{i:reqwest::get}}`][c-reqwest::get]⮳ to get list of all users who have marked a GitHub project with a star. [`{{i:reqwest::Response}}`][c-reqwest::Response]⮳ is deserialized with [`{{i:Response::json}}`][c-reqwest::Response::json]⮳ into `User` objects implementing [`{{i:serde::Deserialize}}`][c-serde::Deserialize]⮳.

[`{{i:tokio::main}}`][c-tokio-tutorial-hello-tokio] is used to set up the async executor and the process waits for [`{{i:reqwest::get}}`][c-reqwest::get] to complete before processing the response into User instances.

```rust,editable,no_run
{{#include ../../../deps/tests/rest-get.rs}}
```

## Check if an API resource exists

[![reqwest][reqwest-badge]][c-reqwest]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-client][cat-http-client-badge]][cat-http-client]

Query the GitHub Users Endpoint using a HEAD request [`{{i:Client::head}}`][c-reqwest::Client::head]⮳ and then inspect the response code to determine success. This is a quick way to query a rest resource without needing to receive a body. [`{{i:reqwest::Client}}`][c-reqwest::Client]⮳ configured with [`{{i:ClientBuilder::timeout}}`][c-reqwest::ClientBuilder::timeout]⮳ ensures a request will not last longer than a timeout.

Due to both [`{{i:ClientBuilder::build}}`][c-reqwest::ClientBuilder::build]⮳ and [`{{i:RequestBuilder::send}}`][c-reqwest::RequestBuilder::send]⮳ returning [`{{i:reqwest::Error}}`][c-reqwest::Error]⮳ types, the shortcut [`{{i:reqwest::Result}}`][c-reqwest::Result]⮳ is used for the main function return type.

```rust,editable,no_run
{{#include ../../../deps/tests/rest-head.rs}}
```

## Create and delete Gist with GitHub API

[![reqwest][reqwest-badge]][c-reqwest]  [![serde][serde-badge]][c-serde]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-encoding][cat-encoding-badge]][cat-encoding]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-client][cat-http-client-badge]][cat-http-client]

Creates a gist with POST request to GitHub [gists API v3](https://developer.github.com/v3/gists/)⮳ using [`{{i:Client::post}}`][c-reqwest::Client::post]⮳ and removes it with DELETE request using [`{{i:Client::delete}}`][c-reqwest::Client::post]⮳.

The [`{{i:reqwest::Client}}`][c-reqwest::Client]⮳ is responsible for details of both requests including URL, body and authentication. The POST body from [`{{i:serde_json::json!}}`][c-reqwest::Client]⮳ macro provides arbitrary JSON body. Call to [`{{i:RequestBuilder::json}}`][c-reqwest::Client]⮳ sets the request body. [`{{i:RequestBuilder::basic_auth}}`][c-reqwest::Client]⮳ handles authentication. The call to [`{{i:RequestBuilder::send}}`][c-reqwest::Client]⮳ synchronously executes the requests.

```rust,editable,no_run
{{#include ../../../deps/tests/rest-post.rs}}
```

The example uses HTTP [`basic auth`][http-basic-auth]⮳ in order to authorize access to [`{{i:GitHub API}}`][github-api]⮳. Typical use case would employ one of the much more complex [`{{i:OAuth}}`][oauth-website]⮳ authorization flows.

## Consume a paginated RESTful API

[![reqwest][reqwest-badge]][c-reqwest]  [![serde][serde-badge]][c-serde]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-encoding][cat-encoding-badge]][cat-encoding]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-client][cat-http-client-badge]][cat-http-client]

Wraps a paginated web API in a convenient Rust iterator. The iterator lazily fetches the next page of results from the remote server as it arrives at the end of each page.

```rust,editable,no_run
{{#include ../../../deps/tests/paginated.rs}}
```

## Handle a rate-limited API

[![reqwest][reqwest-badge]][c-reqwest]  [![hyper][hyper-badge]][c-hyper]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]  [![cat-http-client][cat-http-client-badge]][cat-http-client]

This example uses the [`{{i:GitHub API - Rate limiting}}`][github-api-rate-limiting]⮳, as an example of how to handle remote server errors. This example uses the [`{{i:hyper::header!}}`][c-hyper::header!]⮳ macro to parse the response header and checks for [`{{i:reqwest::StatusCode::Forbidden}}`][c-reqwest::StatusCode::FORBIDDEN]⮳ If the response exceeds the rate limit, the example waits and retries.

```rust,editable,no_run
{{#include ../../../deps/tests/rate-limited.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
