## Run an external command and process stdout

[![regex-badge]][regex] [![cat-os-badge]][cat-os] [![cat-text-processing-badge]][cat-text-processing]

Runs `git log --oneline` as an external [`Command`] and inspects its [`Output`]
using [`Regex`] to get the hash and message of the last 5 commits.

```rust,editable,no_run
{#include ../../../deps/examples/process-output.rs}
```

[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`Output`]: https://doc.rust-lang.org/std/process/struct.Output.html
[`Regex`]: https://docs.rs/regex/*/regex/struct.Regex.html
