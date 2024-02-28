# External Command

{{#include external.incl.md}}

## Run an {{i:external command}} and process {{i:stdout}}

[![regex][regex-badge]][regex]  [![cat-os][cat-os-badge]][cat-os]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Runs `git log --oneline` as an external [`Command`][std::process::Command] and inspects its [`Output`][std::process::Output] using [`Regex`][regex::Regex] to get the hash and message of the last 5 commits.

```rust,editable,no_run
{{#include ../../../deps/tests/process-output.rs}}
```

## Run an {{i:external command}} passing it stdin and check for an error code

[![std][std-badge]][std]  [![cat-os][cat-os-badge]][cat-os]

Opens the `python` interpreter using an external [`Command`][std::process::Command] and passes it a python statement for execution. [`Output`][std::process::Output] of statement is then parsed.

```rust,editable,no_run
{{#include ../../../deps/tests/send-input.rs}}
```

## Run {{i:piped external commands}}

[![std][std-badge]][std]  [![cat-os][cat-os-badge]][cat-os]

Shows up to the 10<sup>th</sup> biggest files and subdirectories in the current working directory. It is equivalent to running: `du -ah. | sort -hr | head -n 10`.

[`Command`][std::process::Command] represent a {{i:process}}. Output of a child process is captured with a
[`Stdio::piped`][std::process::Stdio::piped] between parent and child.

```rust,editable,no_run
{{#include ../../../deps/tests/piped.rs}}
```

## Redirect both stdout and stderr of child process to the same file

[![std][std-badge]][std]  [![cat-os][cat-os-badge]][cat-os]

Spawns a child process and redirects {{i:`stdout`}} and {{i:`stderr`}} to the same file. It follows the same idea as [run piped external commands](#run-ipiped-external-commands), however [`process::Stdio`][std::process::Stdio] writes to a specified file. [`File::try_clone`][std::fs::File::try_clone] references the same file handle for `stdout` and `stderr`. It will ensure that both handles write with the same cursor position.

The below recipe is equivalent to run the Unix shell command `ls . oops >out.txt 2>&1`.

```rust,editable,no_run
{{#include ../../../deps/tests/error-file.rs}}
```

## Continuously process child process' outputs

[![std][std-badge]][std]  [![cat-os][cat-os-badge]][cat-os]

In [Run an external command and process stdout](#run-an-external-command-and-process-stdout), processing doesn't start until external [`Command`][std::process::Command] is finished. The recipe below calls [`Stdio::piped`][std::process::Stdio::piped] to create a pipe, and reads
`stdout` continuously as soon as the [`BufReader`][std::io::BufReader] is updated.

The below recipe is equivalent to the Unix shell command
`journalctl | grep usb`.

```rust,editable,no_run
{{#include ../../../deps/tests/continuous.rs}}
```

## Read Environment Variable

[![std][std-badge]][std]  [![cat-os][cat-os-badge]][cat-os]

Reads an {{i:environment variable}} via [`std::env::var`][std::env::var].

```rust,editable,no_run
{{#include ../../../deps/tests/read-env-variable.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
