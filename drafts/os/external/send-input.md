## Run an external command passing it stdin and check for an error code

[![std-badge]][std] [![cat-os-badge]][cat-os]

Opens the `python` interpreter using an external [`Command`] and passes it a
python statement for execution. [`Output`] of statement is then parsed.

```rust,editable,no_run
{#include ../../../deps/examples/send-input.rs}
```

[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`Output`]: https://doc.rust-lang.org/std/process/struct.Output.html
