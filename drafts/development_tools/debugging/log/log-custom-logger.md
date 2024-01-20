## Log messages with a custom logger

[![log-badge]][log] [![cat-debugging-badge]][cat-debugging]

Implements a custom logger `ConsoleLogger` which prints to stdout.
In order to use the logging macros, `ConsoleLogger` implements
the [`log::Log`] trait and [`log::set_logger`] installs it.

```rust,editable
{#include ../../../deps/examples/log-custom-logger.rs}
```

[`log::Log`]: https://docs.rs/log/*/log/trait.Log.html
[`log::set_logger`]: https://docs.rs/log/*/log/fn.set_logger.html
