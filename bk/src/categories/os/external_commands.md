# External Command

{{#include external_commands.incl.md}}

## Locate Installed Executables with `which` {#which}

[![which][c-which-badge]][c-which] [![which-crates.io][c-which-crates.io-badge]][c-which-crates.io] [![which-github][c-which-github-badge]][c-which-github] [![which-lib.rs][c-which-lib.rs-badge]][c-which-lib.rs]{{hi:which}}{{hi:Command}}{{hi:Unix}}{{hi:which}}{{hi:Which-rs}} [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}} [![cat-os][cat-os-badge]][cat-os]{{hi:Operating systems}}

[`which`][c-which]⮳{{hi:which}} is a Rust equivalent of Unix command "which". It locates installed executables in a cross-platform way.

```rust,editable,noplayground
{{#include ../../../crates/cats/os/tests/external_commands/which.rs:example}}
```

## Run an External Command and Process its `stdout` {#run-an-external-command-and-process-stdout}

[![std][c-std-badge]][c-std] [![regex][c-regex-badge]][c-regex]{{hi:regex}} [![cat-os][cat-os-badge]][cat-os]{{hi:OS}} [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}{{hi:External command}}{{hi:stdout}}

Runs `git log --oneline` as an external [`std::process::Command`][c-std::process::Command]{{hi:std::process::Command}}⮳ and inspects its [`std::process::Output`][c-std::process::Output]{{hi:std::process::Output}}⮳ using [`regex::Regex`][c-regex::Regex]{{hi:regex::Regex}}⮳ to get the hash and message of the last 5 commits.

```rust,editable
{{#include ../../../crates/cats/os/tests/external_commands/process_output.rs:example}}
```

## Run an External command, Passing it `stdin`, then Check for an Error Code {#run-an-external-command-passing-stdin-and-check-for-error-code}

[![std][c-std-badge]][c-std]{{hi:std}} [![cat-os][cat-os-badge]][cat-os]{{hi:OS}}{{hi:External command}}

Opens the `python`{{hi:python}} interpreter using an external [`std::process::Command`][c-std::process::Command]{{hi:std::process::Command}}⮳ and passes it a python statement for execution. The [`std::process::Output`][c-std::process::Output]{{hi:std::process::Output}}⮳ of statement is then parsed.

```rust,editable
{{#include ../../../crates/cats/os/tests/external_commands/send_input.rs:example}}
```

## Run Piped External Commands {#run-piped-external-commands}

[![std][c-std-badge]][c-std]{{hi:std}} [![cat-os][cat-os-badge]][cat-os]{{hi:OS}}{{hi:Piped external commands}}

Shows up to the 10<sup>th</sup> biggest files and subdirectories in the current working directory. It is equivalent to running: `du -ah . | sort -hr | head -n 10`.

[`std::process::Command`][c-std::process::Command]{{hi:std::process::Command}}⮳ represent a process{{hi:Process}}. Output of a child process is captured with a [`std::process::Stdio::piped`][c-std::process::Stdio::piped]⮳{{hi:std::process::Stdio::piped}}⮳ between parent and child.

```rust,editable
{{#include ../../../crates/cats/os/tests/external_commands/piped.rs:example}}
```

## Redirect both the `stdout` and `stderr` of a Child Process to the Same File {#redirect-both-stdout-and-stderr-of-child-process-to-the-same-file}

[![std][c-std-badge]][c-std]{{hi:std}} [![cat-os][cat-os-badge]][cat-os]{{hi:OS}}

Spawns a child process and redirects [`std::io::Stdout`][c-std::io::Stdout]{{hi:std::io::Stdout}}⮳ and [`std::io::Stderr`][c-std::io::Stderr]{{hi:std::io::Stderr}}⮳ to the same file. It follows the same idea as [run piped external commands][ex-os-run-piped-external-commands], however [`std::process::Stdio`][c-std::process::Stdio]{{hi:std::process::Stdio}}⮳ writes to a specified file. [`std::fs::File::try_clone`][c-std::fs::File::try_clone]{{hi:std::fs::File::try_clone}}⮳ references the same file handle for [`std::io::Stdout`][c-std::io::Stdout]{{hi:std::io::Stdout}}⮳ and [`std::io::Stderr`][c-std::io::Stderr]{{hi:std::io::Stderr}}⮳. It will ensure that both handles write with the same cursor position.

The below recipe is equivalent to run the [Unix][p-unix] shell command `ls . oops >out.txt 2>&1`.

```rust,editable
{{#include ../../../crates/cats/os/tests/external_commands/error_file.rs:example}}
```

## Continuously Process the Outputs of a Child Process {#continuously-process-child-process-outputs}

[![std][c-std-badge]][c-std]{{hi:std}} [![cat-os][cat-os-badge]][cat-os]{{hi:OS}}

In [Run an external command and process its `stdout`][ex-os-run-an-external-command-and-process-stdout], processing doesn't start until the external [`std::process::Command`][c-std::process::Command]{{hi:std::process::Command}} is finished. The recipe below calls [`std::process::Stdio::piped`][c-std::process::Stdio::piped]⮳{{hi:std::process::Stdio::piped}} to create a pipe, and reads
[`std::io::Stdout`][c-std::io::Stdout]{{hi:std::io::Stdout}}⮳ continuously as soon as the [`std::io::BufReader`][c-std::io::BufReader]{{hi:std::io::BufReader}}⮳ is updated.

The below recipe is equivalent to the Unix shell command `journalctl | grep usb`.

```rust,editable
{{#include ../../../crates/cats/os/tests/external_commands/continuous.rs:example}}
```

## Read an Environment Variable {#read-environment-variable}

[![std][c-std-badge]][c-std]{{hi:std}} [![cat-os][cat-os-badge]][cat-os]{{hi:OS}}

Reads an environment variable{{hi:Environment variables}} via [`std::env::var`][c-std::env::var]{{hi:std::env::var}}⮳.

```rust,editable
{{#include ../../../crates/cats/os/tests/external_commands/read_env_variable.rs:example}}
```

## Run Child Processes Using `duct` {#run-child-processes-using-duct}

[![duct][c-duct-badge]][c-duct]{{hi:duct}}
[![duct-crates.io][c-duct-crates.io-badge]][c-duct-crates.io]
[![duct-github][c-duct-github-badge]][c-duct-github]
[![duct-lib.rs][c-duct-lib.rs-badge]][c-duct-lib.rs]

[`duct`][c-duct-github]{{hi:duct}}⮳ is a library for running child processes. [`duct`][c-duct]⮳{{hi:duct}} makes it easy to build pipelines and redirect I/O like a shell. At the same time, [`duct`][c-duct]⮳{{hi:duct}} helps you write correct, portable code: whitespace is never significant, errors from child processes get reported by default, and a variety of [gotchas, bugs, and platform inconsistencies][c-duct-gotchas-github]⮳ are handled for you.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write which; review](https://github.com/john-cd/rust_howto/issues/946)
</div>
