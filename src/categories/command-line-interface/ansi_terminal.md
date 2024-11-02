# ANSI Terminal

{{#include ansi_terminal.incl.md}}

[![ansi_term][c-ansi_term-badge]][c-ansi_term]{{hi:ansi_term}}  [![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]{{hi:Command-line interface}}

This program depicts the use of [`ansi_term`][c-ansi_term-crates.io]{{hi:ansi_term}}⮳ crate and how it is used for controlling colours and formatting, such as blue bold text or yellow underlined text, on ANSI terminals{{hi:ANSI terminals}}.

There are two main data structures in [`ansi_term`][c-ansi_term-crates.io]{{hi:ansi_term}}⮳: [`ansi_term::ANSIString`][c-ansi_term::ANSIString]{{hi:ansi_term::ANSIString}}⮳ and [`Style`][c-ansi_term::Style]{{hi:ansi_term:Style}}⮳. A `Style`{{hi:ansi_term:Style}} holds stylistic information: colors, whether the text should be bold, or blinking, or whatever. There are also Colour variants that represent simple foreground colour styles. An [`ansi_term::ANSIString`][c-ansi_term::ANSIString]{{hi:ansi_term::ANSIString}}⮳ is a string paired with a [`ansi_term::Style`][c-ansi_term::Style]{{hi:ansi_term::Style}}⮳.

**Note:** British English uses *Colour* instead of *Color*.

## Printing colored text to the Terminal

```rust
{{#include ../../../deps/tests/cats/command_line_interface/ansi_term_basic.rs:example}}
```

### Bold text in Terminal

[![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]{{hi:Command-line interface}}

For anything more complex than plain foreground color changes, the code needs to construct [`ansi_term::Style`][c-ansi_term::Style]{{hi:ansi_term::Style}}⮳ struct. [`ansi_term::Style::new`][c-ansi_term::Style::new]{{hi:ansi_term::Style::new}}⮳ creates the struct, and properties chained.

```rust
{{#include ../../../deps/tests/cats/command_line_interface/ansi_term_basic1.rs:example}}
```

### Bold and colored text in terminal

[![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]{{hi:Command-line interface}}

[`ansi_term::Color`][c-ansi_term::Color]{{hi:ansi_term::Color}}⮳ implements many similar functions as [`ansi_term::Style`][c-ansi_term::Style]{{hi:ansi_term::Style}}⮳ and can chain methods.

```rust
{{#include ../../../deps/tests/cats/command_line_interface/ansi_term_basic2.rs:example}}
```

## Manipulate the cursor, style the output, handle input events

[![crossterm][c-crossterm-badge]][c-crossterm]{{hi:crossterm}}
[![crossterm-crates.io][c-crossterm-crates.io-badge]][c-crossterm-crates.io]
[![crossterm-github][c-crossterm-github-badge]][c-crossterm-github]
[![crossterm-lib.rs][c-crossterm-lib.rs-badge]][c-crossterm-lib.rs]

Low-level cross-platform terminal rendering and event handling.

Crossterm is a pure-rust, terminal manipulation library that makes it possible to write cross-platform text-based interfaces. It supports all UNIX and Windows terminals down to Windows 7

- Full control over writing and flushing output buffer
- Is tty
- Cursor manipulation
- Styled output
- Terminal handling
- Events (key inputs, mouse...)

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: ansi_term is archived?
TODO  expand crossterm
</div>
