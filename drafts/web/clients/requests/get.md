## Make a HTTP GET request

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

Parses the supplied URL and makes a synchronous HTTP GET request
with [`reqwest::blocking::get`]. Prints obtained [`reqwest::blocking::Response`]
status and headers. Reads HTTP response body into an allocated [`String`]
using [`read_to_string`].

```rust,editable,no_run
{#include ../../../deps/examples/get.rs}
```

## Async

A similar approach can be used by including the [`tokio`] executor
to make the main function asynchronous, retrieving the same information.

In this example, [`tokio::main`] handles all the heavy executor setup
and allows sequential code implemented without blocking until `.await`.

Uses the asynchronous versions of [reqwest], both [`reqwest::get`] and
[`reqwest::Response`].

```rust,no_run
{#include ../../../deps/examples/get2.rs}
```

[`read_to_string`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string
[`reqwest::blocking::get`]: https://docs.rs/reqwest/*/reqwest/blocking/fn.get.html
[`reqwest::blocking::Response`]: https://docs.rs/reqwest/*/reqwest/blocking/struct.Response.html
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`reqwest::Response`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
[`tokio`]: https://docs.rs/crate/tokio/0.2.11
[`tokio::main`]: https://tokio.rs/tokio/tutorial/hello-tokio#the-code
