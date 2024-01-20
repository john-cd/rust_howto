## Log an error message to the console

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Proper error handling considers exceptions exceptional.  Here, an error logs
to stderr with `log`'s convenience macro [`log::error!`].

```rust,editable
{#include ../../../deps/examples/log-error.rs}
```

[`log::error!`]: https://docs.rs/log/*/log/macro.error.html
