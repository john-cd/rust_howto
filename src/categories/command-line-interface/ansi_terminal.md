# ANSI Terminal

{{#include ansi_terminal.incl.md}}

[![ansi-term][ansi-term-badge]][ansi-term]  [![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]

This program depicts the use of [`ansi_term`][ansi-term-crate] crate and how it is used for controlling colours and formatting, such as blue bold text or yellow underlined text, on {{i:ANSI terminals}}.

There are two main data structures in [`ansi_term`][ansi-term-crate]: [`ANSIString`][ansi_term::ANSIString] and [`Style`][ansi_term::Style]. A [`Style`][ansi_term::Style] holds stylistic information: colors, whether the text should be bold, or blinking, or whatever. There are also Colour variants that represent simple foreground colour styles. An [`ANSIString`][ansi_term::ANSIString] is a string paired with a [`Style`][ansi_term::Style].

**Note:** British English uses *Colour* instead of *Color*.

## Printing colored text to the Terminal

```rust,editable
{{#include ../../../deps/tests/ansi_term-basic.rs}}
```

### Bold text in Terminal

[![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]

For anything more complex than plain foreground colour changes, the code needs to construct `Style` struct. [`Style::new()`][ansi_term::Style::new] creates the struct, and properties chained.

```rust,editable
{{#include ../../../deps/tests/ansi_term-basic1.rs}}
```

### Bold and colored text in terminal

[![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]

`Colour` implements many similar functions as `Style` and can chain methods.

```rust,editable
{{#include ../../../deps/tests/ansi_term-basic2.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
