# External Command

{{#include external.incl.md}}

## Run an external command and process stdout

[![regex][c-regex-badge]][c-regex]  [![cat-os][cat-os-badge]][cat-os]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing] {{hi:external command}}{{hi:stdout}}

Runs `git log --oneline` as an external {{hi:Command}}[`Command`][c-std::process::Command]⮳ and inspects its {{hi:Output}}[`Output`][c-std::process::Output]⮳ using {{hi:Regex}}[`Regex`][c-regex::Regex]⮳ to get the hash and message of the last 5 commits.

```rust,editable,no_run
{{#include ../../../deps/tests/process-output.rs}}
```

## Run an external command passing it stdin and check for an error code

[![std][c-std-badge]][c-std]  [![cat-os][cat-os-badge]][cat-os] {{hi:external command}}

Opens the `python` interpreter using an external {{hi:Command}}[`Command`][c-std::process::Command]⮳ and passes it a python statement for execution. {{hi:Output}}[`Output`][c-std::process::Output]⮳ of statement is then parsed.

```rust,editable,no_run
{{#include ../../../deps/tests/send-input.rs}}
```

## Run piped external commands

[![std][c-std-badge]][c-std]  [![cat-os][cat-os-badge]][cat-os] {{hi:piped external commands}}

Shows up to the 10<sup>th</sup> biggest files and subdirectories in the current working directory. It is equivalent to running: `du -ah. | sort -hr | head -n 10`.

{{hi:Command}}[`Command`][c-std::process::Command]⮳ represent a {{i:process}}. Output of a child process is captured with a
{{hi:Stdio::piped}}[`Stdio::piped`][c-std::process::Stdio::piped]⮳ between parent and child.

```rust,editable,no_run
{{#include ../../../deps/tests/piped.rs}}
```

## Redirect both stdout and stderr of child process to the same file

[![std][c-std-badge]][c-std]  [![cat-os][cat-os-badge]][cat-os]

Spawns a child process and redirects {{hi:stdout}}[`stdout`][c-std::io::Stdout]⮳ and {{hi:stderr}}[`stderr`][c-std::io::Stderr]⮳ to the same file. It follows the same idea as [run piped external commands](#run-piped-external-commands), however {{hi:process::Stdio}}[`process::Stdio`][c-std::process::Stdio]⮳ writes to a specified file. {{hi:File::try_clone}}[`File::try_clone`][c-std::fs::File::try_clone]⮳ references the same file handle for {{hi:stdout}}[`stdout`][c-std::io::Stdout]⮳ and {{hi:stderr}}[`stderr`][c-std::io::Stderr]⮳. It will ensure that both handles write with the same cursor position.

The below recipe is equivalent to run the Unix shell command `ls . oops >out.txt 2>&1`.

```rust,editable,no_run
{{#include ../../../deps/tests/error-file.rs}}
```

## Continuously process child process' outputs

[![std][c-std-badge]][c-std]  [![cat-os][cat-os-badge]][cat-os]

In [Run an external command and process stdout](#run-an-external-command-and-process-stdout), processing doesn't start until external {{hi:Command}}[`Command`][c-std::process::Command] is finished. The recipe below calls {{hi:Stdio::piped}}[`Stdio::piped`][c-std::process::Stdio::piped]⮳ to create a pipe, and reads
{{hi:stdout}}[`stdout`][c-std::io::Stdout]⮳ continuously as soon as the {{hi:BufReader}}[`BufReader`][c-std::io::BufReader]⮳ is updated.

The below recipe is equivalent to the Unix shell command
`journalctl | grep usb`.

```rust,editable,no_run
{{#include ../../../deps/tests/continuous.rs}}
```

## Read Environment Variable

[![std][c-std-badge]][c-std]  [![cat-os][cat-os-badge]][cat-os]

Reads an {{i:environment variable}} via {{hi:std::env::var}}[`std::env::var`][c-std::env::var]⮳.

```rust,editable,no_run
{{#include ../../../deps/tests/read-env-variable.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
