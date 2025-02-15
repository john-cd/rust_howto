# ANSI Terminal

{{#include ansi_terminal.incl.md}}

[![ansi_term][c-ansi_term-badge]][c-ansi_term]{{hi:ansi_term}} [![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]{{hi:Command-line interface}}

This program depicts the use of [`ansi_term`][c-ansi_term-crates.io]{{hi:ansi_term}}⮳ crate and how it is used for controlling colors and formatting, such as blue bold text or yellow underlined text, on ANSI terminals{{hi:ANSI terminals}}.

There are two main data structures in [`ansi_term`][c-ansi_term-crates.io]{{hi:ansi_term}}⮳: [`ansi_term::ANSIString`][c-ansi_term::ANSIString]{{hi:ansi_term::ANSIString}}⮳ and [`Style`][c-ansi_term::Style]{{hi:ansi_term:Style}}⮳. A `Style`{{hi:ansi_term:Style}} holds stylistic information: colors, whether the text should be bold, or blinking, or whatever. There are also Color variants that represent simple foreground color styles. An [`ansi_term::ANSIString`][c-ansi_term::ANSIString]{{hi:ansi_term::ANSIString}}⮳ is a string paired with a [`ansi_term::Style`][c-ansi_term::Style]{{hi:ansi_term::Style}}⮳.

**Note:** British English uses *Color* instead of *Color*.

## Print colored text to the terminal {#colored-text}

```rust,editable
{{#include ../../../crates/cats/command_line_interface/tests/ansi_term/ansi_term_basic.rs:example}}
```

### Print bold text to the terminal {#bold-text}

[![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]{{hi:Command-line interface}}

For anything more complex than plain foreground color changes, the code needs to construct [`ansi_term::Style`][c-ansi_term::Style]{{hi:ansi_term::Style}}⮳ struct. [`ansi_term::Style::new`][c-ansi_term::Style::new]{{hi:ansi_term::Style::new}}⮳ creates the struct, and properties chained.

```rust,editable
{{#include ../../../crates/cats/command_line_interface/tests/ansi_term/ansi_term_basic1.rs:example}}
```

### Print bold and colored text to the terminal {#bold-colored-text}

[![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]{{hi:Command-line interface}}

[`ansi_term::Color`][c-ansi_term::Color]{{hi:ansi_term::Color}}⮳ implements many similar functions as [`ansi_term::Style`][c-ansi_term::Style]{{hi:ansi_term::Style}}⮳ and can chain methods.

```rust,editable
{{#include ../../../crates/cats/command_line_interface/tests/ansi_term/ansi_term_basic2.rs:example}}
```

## Manipulate the cursor, style the output, handle input events {#crossterm}

[![crossterm][c-crossterm-badge]][c-crossterm]{{hi:crossterm}}
[![crossterm-crates.io][c-crossterm-crates.io-badge]][c-crossterm-crates.io]
[![crossterm-github][c-crossterm-github-badge]][c-crossterm-github]
[![crossterm-lib.rs][c-crossterm-lib.rs-badge]][c-crossterm-lib.rs]

Low-level cross-platform terminal rendering and event handling.

`crossterm` is a pure-Rust, terminal manipulation library used to write cross-platform text-based interfaces.
It supports all UNIX and Windows terminals down to Windows 7.

- Full control over writing and flushing output buffer
- Is tty
- Cursor manipulation
- Styled output
- Terminal handling
- Events (key inputs, mouse...)

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/crossterm.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ansi_terminal: ansi_term is archived (P0)](https://github.com/john-cd/rust_howto/issues/231)

Most popular

## `termcolor` {#termcolor}

[![termcolor][c-termcolor-badge]][c-termcolor] [![termcolor-crates.io][c-termcolor-crates.io-badge]][c-termcolor-crates.io] [![termcolor-github][c-termcolor-github-badge]][c-termcolor-github] [![termcolor-lib.rs][c-termcolor-lib.rs-badge]][c-termcolor-lib.rs]{{hi:termcolor}}{{hi:Color}}{{hi:Win}}{{hi:Windows}}{{hi:Ansi}}{{hi:Console}}

`termcolor` is a simple cross platform library for writing colored text to a terminal

## `anstyle` {#anstyle}

[![anstyle-website][c-anstyle-website-badge]][c-anstyle-website] [![anstyle][c-anstyle-badge]][c-anstyle] [![anstyle-crates.io][c-anstyle-crates.io-badge]][c-anstyle-crates.io] [![anstyle-github][c-anstyle-github-badge]][c-anstyle-github] [![anstyle-lib.rs][c-anstyle-lib.rs-badge]][c-anstyle-lib.rs]{{hi:anstyle}}{{hi:Ansi}}{{hi:Color}}{{hi:No_std}}{{hi:Terminal}} [![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]{{hi:Command-line interface}}

`anstyle` provides ANSI text styling

## `anstream` {#anstream}

[![anstream-website][c-anstream-website-badge]][c-anstream-website] [![anstream][c-anstream-badge]][c-anstream] [![anstream-crates.io][c-anstream-crates.io-badge]][c-anstream-crates.io] [![anstream-github][c-anstream-github-badge]][c-anstream-github] [![anstream-lib.rs][c-anstream-lib.rs-badge]][c-anstream-lib.rs]{{hi:anstream}}{{hi:Ansi}}{{hi:Color}}{{hi:Strip}}{{hi:Terminal}}{{hi:Wincon}} [![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]{{hi:Command-line interface}}

`anstream` is a simple cross platform library for writing colored text to a terminal.

[![nu-ansi-term][c-nu_ansi_term-badge]][c-nu_ansi_term] [![nu-ansi-term-crates.io][c-nu_ansi_term-crates.io-badge]][c-nu_ansi_term-crates.io] [![nu-ansi-term-github][c-nu_ansi_term-github-badge]][c-nu_ansi_term-github] [![nu-ansi-term-lib.rs][c-nu_ansi_term-lib.rs-badge]][c-nu_ansi_term-lib.rs]{{hi:nu-ansi-term}}

`nu-ansi-term` is a library for ANSI terminal colors and styles (bold, underline)

## `ansiterm` {#ansiterm}

[![ansiterm][c-ansiterm-badge]][c-ansiterm] [![ansiterm-crates.io][c-ansiterm-crates.io-badge]][c-ansiterm-crates.io] [![ansiterm-github][c-ansiterm-github-badge]][c-ansiterm-github] [![ansiterm-lib.rs][c-ansiterm-lib.rs-badge]][c-ansiterm-lib.rs]{{hi:ansiterm}}

`ansiterm` is a library for ANSI terminal colors and styles (bold, underline)

## `console` {#console}

[![console][c-console-badge]][c-console] [![console-crates.io][c-console-crates.io-badge]][c-console-crates.io] [![console-github][c-console-github-badge]][c-console-github] [![console-lib.rs][c-console-lib.rs-badge]][c-console-lib.rs]{{hi:console}}{{hi:Ansi}}{{hi:Colors}}{{hi:console}}{{hi:Terminal}}{{hi:Cli}}

`console` is a terminal and console abstraction for Rust

## `owo-colors` {#owo-colors}

[![owo-colors][c-owo_colors-badge]][c-owo_colors] [![owo-colors-crates.io][c-owo_colors-crates.io-badge]][c-owo_colors-crates.io] [![owo-colors-github][c-owo_colors-github-badge]][c-owo_colors-github] [![owo-colors-lib.rs][c-owo_colors-lib.rs-badge]][c-owo_colors-lib.rs]{{hi:owo-colors}}{{hi:Ansi_term}}{{hi:Cli}}{{hi:Color}}{{hi:No-std}}{{hi:Terminal}}[![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]{{hi:Command-line interface}}

`owo-colors` is a zero-allocation terminal colors that will make people go owo

## `stylish` {#stylish}

[![stylish][c-stylish-badge]][c-stylish] [![stylish-crates.io][c-stylish-crates.io-badge]][c-stylish-crates.io] [![stylish-github][c-stylish-github-badge]][c-stylish-github] [![stylish-lib.rs][c-stylish-lib.rs-badge]][c-stylish-lib.rs]{{hi:stylish}}

`stylish` is another crate implementing colorized text

## `yansi` {#yansi}

[![yansi][c-yansi-badge]][c-yansi] [![yansi-crates.io][c-yansi-crates.io-badge]][c-yansi-crates.io] [![yansi-github][c-yansi-github-badge]][c-yansi-github] [![yansi-lib.rs][c-yansi-lib.rs-badge]][c-yansi-lib.rs]{{hi:yansi}}{{hi:Paint}}{{hi:Format}}{{hi:Color}}{{hi:Ansi}}{{hi:Terminal}} [![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]{{hi:Command-line interface}}

`yansi` is a simple ANSI terminal color painting library.

## `termion` {#termion}

[![termion][c-termion-badge]][c-termion] [![termion-crates.io][c-termion-crates.io-badge]][c-termion-crates.io] [![termion-github][c-termion-github-badge]][c-termion-github] [![termion-lib.rs][c-termion-lib.rs-badge]][c-termion-lib.rs]{{hi:termion}}{{hi:Color}}{{hi:Password}}{{hi:Terminal}}{{hi:Tty}}{{hi:Tui}}

`termion` is a pure Rust, bindless library for low-level handling, manipulating and reading information about terminals. This provides a full-featured alternative to Termbox.

Termion aims to be simple and yet expressive. It is bindless, meaning that it is not a front-end to some other library (e.g., ncurses or termbox), but a standalone library directly talking to the TTY.

</div>
