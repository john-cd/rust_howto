<!-- TODO Reviewed ??-??-2026 -->

# Call an API

{{#include apis.incl.md}}

The network examples use [`reqwest`][c~reqwest~docs]↗ for asynchronous HTTP requests; the rate-limit example isolates status and header handling without making a request. For an application that makes several requests, build one `Client` and reuse it so connection pooling and shared configuration are effective. Set a reasonable timeout, keep credentials out of source code, and handle both transport errors and non-success HTTP statuses. A successful `.send()` only means a response arrived; use `.error_for_status()` when non-2xx statuses should be errors.

## Query the GitHub API {#query-the~repo-api}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![serde][c~serde~docs~badge]][c~serde~docs]{{hi:serde}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]

Queries GitHub's stargazers endpoint to get the users who have starred a project. The example builds a reusable [`reqwest::Client`][c~reqwest::Client~docs]↗, checks for an HTTP error with `error_for_status()`, and deserializes the JSON response into `User` objects implementing [`serde::Deserialize`][c~serde::Deserialize~docs]↗.

`tokio::main`{{hi:tokio::main}} sets up the asynchronous executor. The example awaits the request and JSON body, then prints the deserialized users.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/apis/rest_get.rs:example}}
```

## Check if an API Resource Exists {#check-if-an-api-resource-exists}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]{{hi:HTTP client}}

Query the GitHub Users Endpoint using a HEAD request [`reqwest::Client::head`][c~reqwest::Client::head~docs]↗{{hi:reqwest::Client::head}} and then inspect the response code to determine success. This is a quick way to query a rest resource without needing to receive a body. [`reqwest::Client`][c~reqwest::Client~docs]↗{{hi:reqwest::Client}} configured with [`reqwest::ClientBuilder::timeout`][c~reqwest::ClientBuilder::timeout~docs]↗{{hi:reqwest::ClientBuilder::timeout}} ensures a request will not last longer than a timeout.

The example sets a timeout so a request cannot wait indefinitely. A successful HEAD response indicates that the endpoint accepted the request; a 404 often indicates a missing resource, but permissions and endpoint behavior can make status codes ambiguous. Some servers do not support HEAD and return 405, so use GET when the API requires it. Network and timeout failures are returned as `reqwest::Error`.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/apis/rest_head.rs:example}}
```

## Create and Delete a Gist with the GitHub API {#create-and-delete-gist-with~repo-api}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![serde][c~serde~docs~badge]][c~serde~docs]{{hi:serde}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]

Creates a gist with POST to the GitHub [Gists API][gist-api]↗ and deletes it with DELETE using [`reqwest::Client::delete`][c~reqwest::Client::delete~docs]↗. The example uses asynchronous requests, bearer-token authentication, and checks the HTTP status before decoding the JSON response.

The example reads a GitHub token from the `GH_TOKEN` environment variable and sends it as a bearer token. Use a token with only the permission needed to create gists; never put the token in source code or logs. The `.json()` method serializes the POST body, and each request is awaited asynchronously.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/apis/rest_post.rs:example}}
```

The example creates a public gist and then deletes it, so run it only with a token and account where that side effect is acceptable. It is an API token example; it does not implement an OAuth authorization flow.

## Consume a Paginated RESTful API {#consume-a-paginated-restful-api}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![serde][c~serde~docs~badge]][c~serde~docs]{{hi:serde}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]

Wraps a paginated web API (the crates.io reverse dependencies endpoint) in a convenient Rust iterator. The iterator lazily fetches the next page of results from the remote server as it arrives at the end of each page. Dependent crate names are looked up through `version_id` in the `versions` array, as the `dependencies` entries only carry the name of the crate being depended upon.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/apis/paginated.rs:example}}
```

## Handle a Rate-limited API {#handle-a-rate-limited-api}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![hyper][c~hyper~docs~badge]][c~hyper~docs]{{hi:hyper}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]

This example demonstrates interpreting GitHub's rate-limit response headers. It calculates and prints a suggested delay from `Retry-After` or exhausted primary-limit headers, and uses a one-minute fallback for `429` without those headers. A bare `403` can be a permissions failure or a secondary rate limit; this header-only helper returns no delay for it unless rate-limit headers identify the primary limit. It does not send a request, sleep, or retry. A real client should inspect the response body, bound retries, honor cancellation and any server-provided delay, and avoid retrying permission failures.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_client/examples/apis/rate_limited.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Make HTTP requests][p~requests].
- [HTTP client libraries][p~http-clients].
- [Parse JSON responses][p~json].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [review / fix](https://github.com/john-cd/rust_howto/issues/974)
</div>
