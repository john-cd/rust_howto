# User interaction

{{#include user_interaction.incl.md}}

## Ask for confirmation, selection, text input {#inquire}

[![inquire][c-inquire-badge]][c-inquire]{{hi:inquire}}
[![inquire-crates.io][c-inquire-crates.io-badge]][c-inquire-crates.io]
[![inquire-github][c-inquire-github-badge]][c-inquire-github]
[![inquire-lib.rs][c-inquire-lib.rs-badge]][c-inquire-lib.rs]
[![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]{{hi:Command-line interface}}
[![cat-value-formatting][cat-value-formatting-badge]][cat-value-formatting]{{hi:Value formatting}}

`inquire` provides several different prompts in order to interactively ask the user for information via the CLI.

```rust,editable
{{#include ../../../deps/tests/categories/command_line_interface/inquire.rs:example}}
```

## Display progress bars and spinners {#indicatif}

[![indicatif][c-indicatif-badge]][c-indicatif]{{hi:indicatif}}
[![indicatif-crates.io][c-indicatif-crates.io-badge]][c-indicatif-crates.io]
[![indicatif-github][c-indicatif-github-badge]][c-indicatif-github]
[![indicatif-lib.rs][c-indicatif-lib.rs-badge]][c-indicatif-lib.rs]
[![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]{{hi:Command-line interface}}

[`indicatif`][c-indicatif]{{hi:indicatif}}⮳ is a Rust library for indicating progress in command line applications to users.

This currently primarily provides progress bars and spinners as well as basic color support.{{hi:Progress bars and spinners}}

```rust,editable,noplayground
{{#include ../../../deps/tests/categories/command_line_interface/indicatif.rs:example}}
```

```rust,editable,noplayground
{{#include ../../../deps/tests/categories/command_line_interface/indicatif2.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[user_interaction: write (P1)](https://github.com/john-cd/rust_howto/issues/235)
</div>
