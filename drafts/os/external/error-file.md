## Redirect both stdout and stderr of child process to the same file

[![std-badge]][std] [![cat-os-badge]][cat-os]

Spawns a child process and redirects `stdout` and `stderr` to the same
file. It follows the same idea as [run piped external commands](#run-piped-external-commands), however [`process::Stdio`]
writes to a specified file.  [`File::try_clone`] references the same file handle
for `stdout` and `stderr`. It will ensure that both handles write with the same
cursor position.

The below recipe is equivalent to run the Unix shell command `ls
. oops >out.txt 2>&1`.

```rust,editable,no_run
{#include ../../../deps/examples/error-file.rs}
```

[`File::try_clone`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.try_clone
[`process::Stdio`]: https://doc.rust-lang.org/std/process/struct.Stdio.html
