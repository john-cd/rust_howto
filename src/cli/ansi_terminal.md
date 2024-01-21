# ANSI Terminal

[![ansi-term-badge]][ansi-term]

This program depicts the use of [`ansi_term`][ansi_term] crate and how it is used for controlling colours and formatting, such as blue bold text or yellow underlined text, on ANSI terminals.

There are two main data structures in [`ansi_term`][ansi_term]: [`ANSIString`][ANSIString] and [`Style`][Style]. A [`Style`][Style] holds stylistic information: colours, whether the text should be bold, or blinking, or whatever. There are also Colour variants that represent simple foreground colour styles. An [`ANSIString`][ANSIString] is a string paired with a [`Style`][Style] .

**Note:** British English uses *Colour* instead of *Color*, don't get confused

## Printing colored text to the Terminal

```rust,editable
{{#include ../../deps/examples/ansi_term-basic.rs}}
```

### Bold text in Terminal

For anything more complex than plain foreground colour changes, the code
needs to construct `Style` struct. [`Style::new()`][Style::new()] creates the struct, and properties chained.

```rust,editable
{{#include ../../deps/examples/ansi_term-basic1.rs}}
```

### Bold and colored text in terminal

`Colour` implements many similar functions as `Style` and can chain methods.

```rust,editable
{{#include ../../deps/examples/ansi_term-basic2.rs}}
```

[ansi_term]: https://crates.io/crates/ansi_term
[ANSIString]: https://docs.rs/ansi_term/*/ansi_term/type.ANSIString.html
[Style]: https://docs.rs/ansi_term/*/ansi_term/struct.Style.html
[Style::new()]: https://docs.rs/ansi_term/0.11.0/ansi_term/struct.Style.html#method.new
{{#include ../refs/link-refs.md}}
