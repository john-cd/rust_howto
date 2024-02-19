## Create and delete Gist with GitHub API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Creates a gist with POST request to GitHub [gists API v3](https://developer.github.com/v3/gists/)
using [`Client::post`] and removes it with DELETE request using [`Client::delete`].

The [`reqwest::Client`] is responsible for details of both requests including
URL, body and authentication. The POST body from [`serde_json::json!`] macro
provides arbitrary JSON body. Call to [`RequestBuilder::json`] sets the request
body. [`RequestBuilder::basic_auth`] handles authentication. The call to
[`RequestBuilder::send`] synchronously executes the requests.

```rust,editable,no_run
{#include ../../../deps/examples/rest-post.rs}
```

The example uses [HTTP Basic Auth] in order to authorize access to [GitHub API].
Typical use case would employ one of the much more complex [OAuth] authorization
flows.

[`Client::delete`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.delete
[`Client::post`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.post
[`RequestBuilder::basic_auth`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.basic_auth
[`RequestBuilder::json`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.json
[`RequestBuilder::send`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send
[`reqwest::Client`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html
[`serde_json::json!`]: https://docs.rs/serde_json/*/serde_json/macro.json.html

[GitHub API]: https://developer.github.com/v3/auth/
[HTTP Basic Auth]: https://tools.ietf.org/html/rfc2617
[OAuth]: https://oauth.net/getting-started/
