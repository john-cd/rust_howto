<!-- TODO Reviewed ??-??-2026 -->

# User Interaction

{{#include user_interaction.incl.md}}

Interactive prompts are for people at a terminal. Keep a non-interactive path for scripts, automation, and redirected input: accept values as arguments or configuration, and provide explicit flags such as `--yes` when skipping a confirmation is safe. If a required value is missing and prompting is unavailable, report what the caller must supply.

Send diagnostics and progress to standard error so standard output can remain machine-readable. Do not rely on color, animation, or a prompt as the only way to communicate important information.

## Ask for Confirmation, Selection, Text Input {#inquire}

[![inquire][c~inquire~docs~badge]][c~inquire~docs]{{hi:inquire}}
[![inquire~crates.io][c~inquire~crates.io~badge]][c~inquire~crates.io]
[![inquire~repo][c~inquire~repo~badge]][c~inquire~repo]
[![inquire~lib.rs][c~inquire~lib.rs~badge]][c~inquire~lib.rs]
[![cat~command-line-interface][cat~command-line-interface~badge]][cat~command-line-interface]{{hi:Command-line interface}}
[![cat~value-formatting][cat~value-formatting~badge]][cat~value-formatting]{{hi:Value formatting}}

[`inquire`][c~inquire~docs]↗{{hi:inquire}} provides several different prompts, in order to interactively ask the user for information via the CLI.

It offers a range of built-in prompts for various data types, including text input, selections, confirmations, and password entry, simplifying the creation of interactive CLI applications.

Handle prompt cancellation and input errors as normal outcomes instead of panicking. Do not request a secret with a plain-text prompt; use a password prompt that disables echo, and avoid printing the returned value. Check whether a prompt is appropriate before asking for input so a command does not hang when run unattended.

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/user_interaction/inquire.rs:example}}
```

## Display Progress Bars and Spinners {#indicatif}

[![indicatif][c~indicatif~docs~badge]][c~indicatif~docs]{{hi:indicatif}}
[![indicatif~crates.io][c~indicatif~crates.io~badge]][c~indicatif~crates.io]
[![indicatif~repo][c~indicatif~repo~badge]][c~indicatif~repo]
[![indicatif~lib.rs][c~indicatif~lib.rs~badge]][c~indicatif~lib.rs]
[![cat~command-line-interface][cat~command-line-interface~badge]][cat~command-line-interface]{{hi:Command-line interface}}

[`indicatif`][c~indicatif~docs]↗{{hi:indicatif}} is a progress bar library for command line applications. It allows developers to create and manage progress bars, spinners, and other visual feedback mechanisms for long-running tasks.{{hi:Progress bars and spinners}}

Use a determinate bar when the total work is known and a spinner when it is not. Update progress at useful intervals, and finish or clear the indicator when the operation ends. Disable or simplify progress output when stderr is not a terminal; do not emit terminal control sequences into logs or data streams.

```rust,editable,noplayground
{{#include ../../../crates/cats/command_line_interface/examples/user_interaction/indicatif.rs:example}}
```

```rust,editable,noplayground
{{#include ../../../crates/cats/command_line_interface/examples/user_interaction/indicatif2.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[ansi_terminal | ANSI Terminals]].
- [[user_directories | User Directories]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  user_interaction: write
</div>
