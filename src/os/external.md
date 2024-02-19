# External Command

## Run an external command and process stdout

[![regex-badge]][regex] [![cat-os-badge]][cat-os] [![cat-text-processing-badge]][cat-text-processing]

Runs `git log --oneline` as an external [`Command`][Command] and inspects its [`Output`][Output] using [`Regex`][Regex] to get the hash and message of the last 5 commits.

```rust,editable,no_run
{{#include ../../deps/examples/process-output.rs}}
```

## Run an external command passing it stdin and check for an error code

[![std-badge]][std] [![cat-os-badge]][cat-os]

Opens the `python` interpreter using an external [`Command`][Command] and passes it a python statement for execution. [`Output`][Output] of statement is then parsed.

```rust,editable,no_run
{{#include ../../deps/examples/send-input.rs}}
```

## Run piped external commands

[![std-badge]][std] [![cat-os-badge]][cat-os]

Shows up to the 10<sup>th</sup> biggest files and subdirectories in the current working directory. It is equivalent to running: `du -ah. | sort -hr | head -n 10`.

[`Command`][Command] represent a process. Output of a child process is captured with a
[`Stdio::piped`][Stdio::piped] between parent and child.

```rust,editable,no_run
{{#include ../../deps/examples/piped.rs}}
```

## Redirect both stdout and stderr of child process to the same file

[![std-badge]][std] [![cat-os-badge]][cat-os]

Spawns a child process and redirects `stdout` and `stderr` to the same file. It follows the same idea as [run piped external commands](#run-piped-external-commands), however [`process::Stdio`][process::Stdio] writes to a specified file.  [`File::try_clone`][File::try_clone] references the same file handle for `stdout` and `stderr`. It will ensure that both handles write with the same cursor position.

The below recipe is equivalent to run the Unix shell command `ls . oops >out.txt 2>&1`.

```rust,editable,no_run
{{#include ../../deps/examples/error-file.rs}}
```

## Continuously process child process' outputs

[![std-badge]][std] [![cat-os-badge]][cat-os]

In [Run an external command and process stdout](#run-an-external-command-and-process-stdout), processing doesn't start until external [`Command`][Command] is finished. The recipe below calls [`Stdio::piped`][Stdio::piped] to create a pipe, and reads
`stdout` continuously as soon as the [`BufReader`][BufReader] is updated.

The below recipe is equivalent to the Unix shell command
`journalctl | grep usb`.

```rust,editable,no_run
{{#include ../../deps/examples/continuous.rs}}
```

## Read Environment Variable

[![std-badge]][std] [![cat-os-badge]][cat-os]

Reads an environment variable via [std::env::var][std::env::var].

```rust,editable,no_run
{{#include ../../deps/examples/read-env-variable.rs}}
```

{{#include ../refs/link-refs.md}}
