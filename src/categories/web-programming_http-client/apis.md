# Call an API

{{#include apis.incl.md}}

## Query the GitHub API {#query-the-github-api}

[![reqwest][c-reqwest-badge]][c-reqwest]{{hi:reqwest}} [![serde][c-serde-badge]][c-serde]{{hi:serde}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}} [![cat-web-programming::http-client][cat-web-programming::http-client-badge]][cat-web-programming::http-client]

Queries GitHub [stargazers API v3][stargazers]⮳ with [`reqwest::get`][c-reqwest::get]{{hi:reqwest::get}}⮳ to get list of all users who have marked a GitHub project with a star. [`reqwest::Response`][c-reqwest::Response]{{hi:reqwest::Response}}⮳ is deserialized with [`reqwest::Response::json`][c-reqwest::Response::json]{{hi:reqwest::Response::json}}⮳ into `User` objects implementing [`serde::Deserialize`][c-serde::Deserialize]{{hi:serde::Deserialize}}⮳.

[`tokio::main`][c-tokio_tutorial_hello_tokio-website]{{hi:tokio::main}} is used to set up the async executor and the process waits for [`reqwest::get`][c-reqwest::get]{{hi:reqwest::get}} to complete before processing the response into User instances.

```rust,editable
{{#include ../../../crates/ex/categories/wxyz/tests/web_programming_http_client/rest_get.rs:example}}
```

## Check if an API resource exists {#check-if-an-api-resource-exists}

[![reqwest][c-reqwest-badge]][c-reqwest]{{hi:reqwest}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}} [![cat-web-programming::http-client][cat-web-programming::http-client-badge]][cat-web-programming::http-client]{{hi:HTTP client}}

Query the GitHub Users Endpoint using a HEAD request [`reqwest::Client::head`][c-reqwest::Client::head]{{hi:reqwest::Client::head}}⮳ and then inspect the response code to determine success. This is a quick way to query a rest resource without needing to receive a body. [`reqwest::Client`][c-reqwest::Client]{{hi:reqwest::Client}}⮳ configured with [`reqwest::ClientBuilder::timeout`][c-reqwest::ClientBuilder::timeout]{{hi:reqwest::ClientBuilder::timeout}}⮳ ensures a request will not last longer than a timeout.

Due to both [`reqwest::ClientBuilder::build`][c-reqwest::ClientBuilder::build]{{hi:reqwest::ClientBuilder::build}}⮳ and [`reqwest::RequestBuilder::send`][c-reqwest::RequestBuilder::send]{{hi:reqwest::RequestBuilder::send}}⮳ returning [`reqwest::Error`][c-reqwest::Error]{{hi:reqwest::Error}}⮳ types, the shortcut [`reqwest::Result`][c-reqwest::Result]{{hi:reqwest::Result}}⮳ is used for the main function return type.

```rust,editable
{{#include ../../../crates/ex/categories/wxyz/tests/web_programming_http_client/rest_head.rs:example}}
```

## Create and delete a Gist with the GitHub API {#create-and-delete-gist-with-github-api}

[![reqwest][c-reqwest-badge]][c-reqwest]{{hi:reqwest}} [![serde][c-serde-badge]][c-serde]{{hi:serde}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}} [![cat-web-programming::http-client][cat-web-programming::http-client-badge]][cat-web-programming::http-client]

Creates a gist with POST request to GitHub [gists API v3][gist-api]⮳ using [`reqwest::Client::post`][c-reqwest::Client::post]{{hi:reqwest::Client::post}}⮳ and removes it with DELETE request using [`reqwest::Client::post`][c-reqwest::Client::post]{{hi:reqwest::Client::post}}⮳.

The [`reqwest::Client`][c-reqwest::Client]{{hi:reqwest::Client}}⮳ is responsible for details of both requests including URL, body and authentication. The POST body from [`reqwest::Client`][c-reqwest::Client]{{hi:reqwest::Client}}⮳ macro provides arbitrary JSON body. Call to [`reqwest::Client`][c-reqwest::Client]{{hi:reqwest::Client}}⮳ sets the request body. [`reqwest::Client`][c-reqwest::Client]{{hi:reqwest::Client}}⮳ handles authentication. The call to [`reqwest::Client`][c-reqwest::Client]{{hi:reqwest::Client}}⮳ synchronously executes the requests.

```rust,editable
{{#include ../../../crates/ex/categories/wxyz/tests/web_programming_http_client/rest_post.rs:example}}
```

The example uses HTTP [`basic auth`][http-basic-auth]⮳ in order to authorize access to [`GitHub API`][github-api]{{hi:GitHub API}}⮳. Typical use case would employ one of the much more complex [`OAuth`][oauth-website]{{hi:OAuth}}⮳ authorization flows.

## Consume a paginated RESTful API {#consume-a-paginated-restful-api}

[![reqwest][c-reqwest-badge]][c-reqwest]{{hi:reqwest}} [![serde][c-serde-badge]][c-serde]{{hi:serde}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}} [![cat-web-programming::http-client][cat-web-programming::http-client-badge]][cat-web-programming::http-client]

Wraps a paginated web API in a convenient Rust iterator. The iterator lazily fetches the next page of results from the remote server as it arrives at the end of each page.

```rust,editable
{{#include ../../../crates/ex/categories/wxyz/tests/web_programming_http_client/paginated.rs:example}}
```

## Handle a rate-limited API {#handle-a-rate-limited-api}

[![reqwest][c-reqwest-badge]][c-reqwest]{{hi:reqwest}} [![hyper][c-hyper-badge]][c-hyper]{{hi:hyper}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}} [![cat-web-programming::http-client][cat-web-programming::http-client-badge]][cat-web-programming::http-client]

This example uses the [`GitHub API - rate limiting`][github-api-rate-limiting]{{hi:GitHub API - Rate limiting}}⮳, as an example of how to handle remote server errors. This example uses the [`hyper::header!`][c-hyper::header!]{{hi:hyper::header!}}⮳ macro to parse the response header and checks for [`reqwest::StatusCode::FORBIDDEN`][c-reqwest::StatusCode::FORBIDDEN]{{hi:reqwest::StatusCode::FORBIDDEN}}⮳ If the response exceeds the rate limit, the example waits and retries.

```rust,editable
{{#include ../../../crates/ex/categories/wxyz/tests/web_programming_http_client/rate_limited.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P1 review](https://github.com/john-cd/rust_howto/issues/974)

</div>
