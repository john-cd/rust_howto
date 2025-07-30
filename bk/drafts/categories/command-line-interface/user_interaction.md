# User Interaction

{{#include user_interaction.incl.md}}

## Ask for Confirmation, Selection, Text Input {#inquire}

[![inquire][c~inquire~docs~badge]][c~inquire~docs]{{hi:inquire}}
[![inquire~crates.io][c~inquire~crates.io~badge]][c~inquire~crates.io]
[![inquire~github][c~inquire~github~badge]][c~inquire~github]
[![inquire~lib.rs][c~inquire~lib.rs~badge]][c~inquire~lib.rs]
[![cat~command-line-interface][cat~command-line-interface~badge]][cat~command-line-interface]{{hi:Command-line interface}}
[![cat~value-formatting][cat~value-formatting~badge]][cat~value-formatting]{{hi:Value formatting}}

[`inquire`][c~inquire~docs]⮳{{hi:inquire}} provides several different prompts, in order to interactively ask the user for information via the CLI.

It offers a range of built-in prompts for various data types, including text input, selections, confirmations, and password entry, simplifying the creation of interactive CLI applications.

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/user_interaction/inquire.rs:example}}
```

## Display Progress Bars and Spinners {#indicatif}

[![indicatif][c~indicatif~docs~badge]][c~indicatif~docs]{{hi:indicatif}}
[![indicatif~crates.io][c~indicatif~crates.io~badge]][c~indicatif~crates.io]
[![indicatif~github][c~indicatif~github~badge]][c~indicatif~github]
[![indicatif~lib.rs][c~indicatif~lib.rs~badge]][c~indicatif~lib.rs]
[![cat~command-line-interface][cat~command-line-interface~badge]][cat~command-line-interface]{{hi:Command-line interface}}

[`indicatif`][c~indicatif~docs]{{hi:indicatif}}⮳ is a progress bar library for command line applications. It allows developers to easily create and manage progress bars, spinners, and other visual feedback mechanisms to display the progress of long-running tasks.{{hi:Progress bars and spinners}}

```rust,editable,noplayground
{{#include ../../../crates/cats/command_line_interface/examples/user_interaction/indicatif.rs:example}}
```

```rust,editable,noplayground
{{#include ../../../crates/cats/command_line_interface/examples/user_interaction/indicatif2.rs:example}}
```

## Related Topics {#related-topics}

- [[ansi_terminal | ANSI Terminals]].
- [[user_directories | User Directories]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[user_interaction: write](https://github.com/john-cd/rust_howto/issues/235)
</div>
