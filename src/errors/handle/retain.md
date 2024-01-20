## Avoid discarding errors during error conversions

[![error-chain-badge]][error-chain] [![cat-rust-patterns-badge]][cat-rust-patterns]

The  [error-chain] crate makes [matching] on different error types returned by
a function possible and relatively compact. [`ErrorKind`] determines the error
type.

Uses [reqwest]::[blocking] to query a random integer generator web service.  Converts
the string response into an integer. The Rust standard library,
[reqwest], and the web service can all generate errors. Well defined Rust errors
use [`foreign_links`]. An additional [`ErrorKind`] variant for the web service
error uses `errors` block of the `error_chain!` macro.

```rust,editable
{#include ../../../deps/examples/retain.rs}
```

[`ErrorKind`]: https://docs.rs/error-chain/*/error_chain/example_generated/enum.ErrorKind.html
[`foreign_links`]: https://docs.rs/error-chain/*/error_chain/#foreign-links
[blocking]: https://docs.rs/reqwest/*/reqwest/blocking/index.html
[Matching]:https://docs.rs/error-chain/*/error_chain/#matching-errors
