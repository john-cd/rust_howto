# External Command

{{#include external.incl.md}}

## Run an external command and process stdout

[![regex][c-regex-badge]][c-regex]{{hi:regex}}  [![cat-os][cat-os-badge]][cat-os]{{hi:OS}}  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}{{hi:External command}}{{hi:stdout}}

Runs `git log --oneline` as an external [`std::process::Command`][c-std::process::Command]{{hi:std::process::Command}}⮳ and inspects its [`std::process::Output`][c-std::process::Output]{{hi:std::process::Output}}⮳ using [`regex::Regex`][c-regex::Regex]{{hi:regex::Regex}}⮳ to get the hash and message of the last 5 commits.

```rust
{{#include ../../../deps/tests/process-output.rs}}
```

## Run an external command passing it stdin and check for an error code

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-os][cat-os-badge]][cat-os]{{hi:OS}}{{hi:External command}}

Opens the `python` interpreter using an external [`std::process::Command`][c-std::process::Command]{{hi:std::process::Command}}⮳ and passes it a python statement for execution. [`std::process::Output`][c-std::process::Output]{{hi:std::process::Output}}⮳ of statement is then parsed.

```rust
{{#include ../../../deps/tests/send-input.rs}}
```

## Run piped external commands

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-os][cat-os-badge]][cat-os]{{hi:OS}}{{hi:Piped external commands}}

Shows up to the 10<sup>th</sup> biggest files and subdirectories in the current working directory. It is equivalent to running: `du -ah. | sort -hr | head -n 10`.

[`std::process::Command`][c-std::process::Command]{{hi:std::process::Command}}⮳ represent a process{{hi:Process}}. Output of a child process is captured with a
`std::process::Stdio::piped`⮳ between parent and child.

```rust
{{#include ../../../deps/tests/piped.rs}}
```

## Redirect both stdout and stderr of child process to the same file

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-os][cat-os-badge]][cat-os]{{hi:OS}}

Spawns a child process and redirects [`std::io::Stdout`][c-std::io::Stdout]{{hi:std::io::Stdout}}⮳ and [`std::io::Stderr`][c-std::io::Stderr]{{hi:std::io::Stderr}}⮳ to the same file. It follows the same idea as [run piped external commands][p-run-piped-external-commands], however [`std::process::Stdio`][c-std::process::Stdio]{{hi:std::process::Stdio}}⮳ writes to a specified file. [`std::fs::File::try_clone`][c-std::fs::File::try_clone]{{hi:std::fs::File::try_clone}}⮳ references the same file handle for [`std::io::Stdout`][c-std::io::Stdout]{{hi:std::io::Stdout}}⮳ and [`std::io::Stderr`][c-std::io::Stderr]{{hi:std::io::Stderr}}⮳. It will ensure that both handles write with the same cursor position.

The below recipe is equivalent to run the Unix shell command `ls . oops >out.txt 2>&1`.

```rust
{{#include ../../../deps/tests/error-file.rs}}
```

## Continuously process child process' outputs

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-os][cat-os-badge]][cat-os]{{hi:OS}}

In [Run an external command and process stdout][p-run-an-external-command-and-process-stdout], processing doesn't start until external [`std::process::Command`][c-std::process::Command]{{hi:std::process::Command}} is finished. The recipe below calls `std::process::Stdio::piped` to create a pipe, and reads
[`std::io::Stdout`][c-std::io::Stdout]{{hi:std::io::Stdout}}⮳ continuously as soon as the [`std::io::BufReader`][c-std::io::BufReader]{{hi:std::io::BufReader}}⮳ is updated.

The below recipe is equivalent to the Unix shell command
`journalctl | grep usb`.

```rust
{{#include ../../../deps/tests/continuous.rs}}
```

## Read Environment Variable

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-os][cat-os-badge]][cat-os]{{hi:OS}}

Reads an environment variable{{hi:Environment variables}} via [`std::env::var`][c-std::env::var]{{hi:std::env::var}}⮳.

```rust
{{#include ../../../deps/tests/read-env-variable.rs}}
```

[p-run-an-external-command-and-process-stdout]: #run-an-external-command-and-process-stdout
[p-run-piped-external-commands]: #run-piped-external-commands
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
