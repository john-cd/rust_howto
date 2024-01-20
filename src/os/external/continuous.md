## Continuously process child process' outputs

[![std-badge]][std] [![cat-os-badge]][cat-os]

In [Run an external command and process stdout](#run-an-external-command-and-process-stdout),
processing doesn't start until external [`Command`] is finished.
The recipe below calls [`Stdio::piped`] to create a pipe, and reads
`stdout` continuously as soon as the [`BufReader`] is updated.

The below recipe is equivalent to the Unix shell command
`journalctl | grep usb`.

```rust,editable,no_run
{#include ../../../deps/examples/continuous.rs}
```

[`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`Stdio::piped`]: https://doc.rust-lang.org/std/process/struct.Stdio.html
