## Include timestamp in log messages

[![log-badge]][log] [![env_logger-badge]][env_logger] [![chrono-badge]][chrono] [![cat-debugging-badge]][cat-debugging]

Creates a custom logger configuration with [`Builder`].
Each log entry calls [`Local::now`] to get the current [`DateTime`] in local
timezone and uses [`DateTime::format`] with [`strftime::specifiers`] to format
a timestamp used in the final log.

The example calls [`Builder::format`] to set a closure which formats each
message text with timestamp, [`Record::level`] and body ([`Record::args`]).

```rust,editable
{#include ../../../deps/examples/log-timestamp.rs}
```

stderr output will contain

```rust
{#include ../../../deps/examples/log-timestamp2.rs}
```

[`DateTime::format`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.format
[`DateTime`]: https://docs.rs/chrono/*/chrono/datetime/struct.DateTime.html
[`Local::now`]: https://docs.rs/chrono/*/chrono/offset/struct.Local.html#method.now
[`Builder`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html
[`Builder::format`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.format
[`Record::args`]: https://docs.rs/log/*/log/struct.Record.html#method.args
[`Record::level`]: https://docs.rs/log/*/log/struct.Record.html#method.level
[`strftime::specifiers`]: https://docs.rs/chrono/*/chrono/format/strftime/index.html#specifiers
