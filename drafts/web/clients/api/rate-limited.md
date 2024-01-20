## Handle a rate-limited API

[![reqwest-badge]][reqwest] [![hyper-badge]][hyper] [![cat-net-badge]][cat-net]

This example uses the [GitHub API - Rate limiting], as an example of how to
handle remote server errors.  This example uses the [`hyper::header!`] macro
to parse the response header and checks for [`reqwest::StatusCode::Forbidden`].
If the response exceeds the rate limit, the example waits and retries.

```rust,editable,no_run
{#include ../../../deps/examples/rate-limited.rs}
```

[`hyper::header!`]: https://doc.servo.org/hyper/header/index.html#defining-custom-headers
[`reqwest::StatusCode::Forbidden`]: https://docs.rs/reqwest/*/reqwest/struct.StatusCode.html#associatedconstant.FORBIDDEN
