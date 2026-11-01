<!-- TODO Reviewed ??-??-2026 -->

# External Command

{{#include external_commands.incl.md}}

Executing external commands and managing child processes are common tasks when building command-line tools, automation scripts, and system utilities in Rust. Choosing the right tool depends on your application's requirements and structure:

- **[`std::process::Command`][c~std::process::Command~docs]↗**: The standard library module suitable for most core application code. It provides fine-grained control over process spawning, arguments, environment variables, and standard I/O handles without external dependencies.
- **[`duct`][c~duct~docs]↗**: Ideal when you need to compose shell-like pipelines (`cmd1 | cmd2`), combine stdout and stderr, or redirect stream handles cleanly and portably across operating systems.
- **[`xshell`][c~xshell~docs]↗**: Suited for developer tools, build automation, and maintenance scripts written in Rust. Its `cmd!` macro allows concise shell-like command invocation with automatic error reporting for non-zero exit codes.
- **[`which`][c~which~docs]↗**: Useful for checking whether a specific binary (such as `git` or `rustc`) is available on the system's `PATH` before attempting execution or providing helpful setup diagnostics.
- **[`shlex`][c~shlex~docs]↗**: Essential for parsing POSIX shell command-line strings into individual argument vectors or escaping arguments for safe terminal logging and shell-copyable output.

When invoking child processes, pass each argument individually via `.arg()` or `.args()` instead of constructing shell strings to avoid injection vulnerabilities. Check `status.success()` or inspect exit codes before handling output as valid data.

## Locate Installed Executables with `which` {#which}

{{#badges which filesystem=Filesystem os="Operating systems"}}{{hi:Command}}{{hi:Unix}}{{hi:Which-rs}}

[`which`][c~which~docs]↗{{hi:which}} locates executables using the platform's search path. It can help explain why an optional tool cannot be found, but still handle errors from launching the returned path.

```rust,editable,noplayground
{{#include ../../../crates/cats/os/examples/external_commands/which.rs:example}}
```

## Run an External Command and Process its `stdout` {#run-an-external-command-and-process-stdout}

[![std][c~std~docs~badge]][c~std~docs] [![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![cat~os][cat~os~badge]][cat~os]{{hi:OS}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}{{hi:External command}}{{hi:stdout}}

Runs [`git log --oneline`][git~log~docs]↗{{hi:git}} as an external [`std::process::Command`][c~std::process::Command~docs]↗{{hi:std::process::Command}} and inspects its [`std::process::Output`][c~std::process::Output~docs]↗{{hi:std::process::Output}} using [`regex::Regex`][c~regex::Regex~docs]↗{{hi:regex::Regex}} to get the hash and message of the last 5 commits.

```rust,editable
{{#include ../../../crates/cats/os/examples/external_commands/process_output.rs:example}}
```

## Run an External Command, Passing it `stdin`, then Check for an Error Code {#run-an-external-command-passing-stdin-and-check-for-error-code}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~os][cat~os~badge]][cat~os]{{hi:OS}}{{hi:External command}}

Starts the Unix `rev` utility using [`std::process::Command`][c~std::process::Command~docs]↗{{hi:std::process::Command}}, writes text to its standard input, and parses the output after checking the exit status. The example requires `rev` to be available on the host.

```rust,editable
{{#include ../../../crates/cats/os/examples/external_commands/send_input.rs:example}}
```

## Run Piped External Commands {#run-piped-external-commands}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~os][cat~os~badge]][cat~os]{{hi:OS}}{{hi:Piped external commands}}

Shows up to the 10<sup>th</sup> biggest files and subdirectories in the current working directory. It is equivalent to running: `du -ah . | sort -hr | head -n 10`.

[`std::process::Command`][c~std::process::Command~docs]↗{{hi:std::process::Command}} represent a process{{hi:Process}}. Output of a child process is captured with a [`std::process::Stdio::piped`][c~std::process::Stdio::piped~docs]↗{{hi:std::process::Stdio::piped}} between parent and child.

```rust,editable
{{#include ../../../crates/cats/os/examples/external_commands/piped.rs:example}}
```

## Redirect both the `stdout` and `stderr` of a Child Process to the Same File {#redirect-both-stdout-and-stderr-of-child-process-to-the-same-file}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~os][cat~os~badge]][cat~os]{{hi:OS}}

Spawns a child process and redirects [`std::io::Stdout`][c~std::io::Stdout~docs]↗{{hi:std::io::Stdout}} and [`std::io::Stderr`][c~std::io::Stderr~docs]↗{{hi:std::io::Stderr}} to the same file. It follows the same idea as [run piped external commands][ex~os~run-piped-external-commands]↗, however [`std::process::Stdio`][c~std::process::Stdio~docs]↗{{hi:std::process::Stdio}} writes to a specified file. [`std::fs::File::try_clone`][c~std::fs::File::try_clone~docs]↗{{hi:std::fs::File::try_clone}} references the same file handle for [`std::io::Stdout`][c~std::io::Stdout~docs]↗{{hi:std::io::Stdout}} and [`std::io::Stderr`][c~std::io::Stderr~docs]↗{{hi:std::io::Stderr}}. It will ensure that both handles write with the same cursor position.

The below recipe is equivalent to run the [Unix][p~unix] shell command `ls . oops >out.txt 2>&1`.

```rust,editable
{{#include ../../../crates/cats/os/examples/external_commands/error_file.rs:example}}
```

## Continuously Process the Outputs of a Child Process {#continuously-process-child-process-outputs}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~os][cat~os~badge]][cat~os]{{hi:OS}}

In [Run an external command and process its `stdout`][ex~os~run-an-external-command-and-process-stdout]↗, processing doesn't start until the external [`std::process::Command`][c~std::process::Command~docs]↗{{hi:std::process::Command}} is finished. The recipe below calls [`std::process::Stdio::piped`][c~std::process::Stdio::piped~docs]↗{{hi:std::process::Stdio::piped}} to create a pipe, and reads
[`std::io::Stdout`][c~std::io::Stdout~docs]↗{{hi:std::io::Stdout}} continuously as soon as the [`std::io::BufReader`][c~std::io::BufReader~docs]↗{{hi:std::io::BufReader}} is updated.

The below recipe is equivalent to the Unix shell command `journalctl | grep usb`.

```rust,editable
{{#include ../../../crates/cats/os/examples/external_commands/continuous.rs:example}}
```

## Read an Environment Variable {#read-environment-variable}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~os][cat~os~badge]][cat~os]{{hi:OS}}

Reads an environment variable{{hi:Environment variables}} via [`std::env::var`][c~std::env::var~docs]↗{{hi:std::env::var}}.

```rust,editable
{{#include ../../../crates/cats/os/examples/external_commands/read_env_variable.rs:example}}
```

## Run Child Processes Using `duct` {#run-child-processes-using-duct}

{{#badges duct}}

[`duct`][c~duct~repo]↗{{hi:duct}} is a library for running child processes. [`duct`][c~duct~docs]↗{{hi:duct}} makes it easy to build pipelines and redirect I/O like a shell. At the same time, [`duct`][c~duct~docs]↗{{hi:duct}} helps write correct, portable code: whitespace is never significant, errors from child processes get reported by default, and a variety of [gotchas, bugs, and platform inconsistencies][c~duct~gotchas~repo]↗ are handled for you.

```rust,editable
{{#include ../../../crates/cats/os/examples/external_commands/duct_example.rs:example}}
```

## Write Shell-Oriented Scripts with `xshell` {#xshell}

[`xshell`][c~xshell~docs]↗ offers shell-like command composition and interpolation for Rust scripts. Its command helpers report spawn and nonzero-exit failures; use this style for build or maintenance scripts, while `std::process::Command` keeps application code explicit about arguments and process I/O.

```rust,editable
{{#include ../../../crates/cats/os/examples/external_commands/xshell_example.rs:example}}
```

## Split and Quote Command Lines with `shlex` {#shlex}

{{#badges shlex os="Operating systems"}}

[`shlex`][c~shlex~docs]↗ splits a string into words the way a POSIX shell would (`shlex::split`), and quotes a word so that it can be pasted into a shell (`shlex::try_quote`). Use it to parse a command line you receive as a string, or to print a copy-pasteable command in logs and error messages. `try_quote` returns an error for input it cannot quote, such as a NUL byte.

Quoting for display is not a reason to run a command through a shell: pass each argument to `Command::args` instead (see [[process_management | Process Management]]).

```rust,editable
{{#include ../../../crates/cats/os/examples/external_commands/shlex_example.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[process_management | Process Management]].
- [[command_line_interface | Command Line Interface]].
- [[command_line_utilities | Command-line Utilities Written in Rust]].
- [[shells | Shells]].
- [[os | Operating Systems]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
