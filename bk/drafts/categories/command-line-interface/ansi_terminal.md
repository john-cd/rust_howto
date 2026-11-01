<!-- TODO Reviewed ??-??-2026 -->

# ANSI Terminal

{{#include ansi_terminal.incl.md}}

## Style Text with `nu-ansi-term` {#nu-ansi-term}

[![nu-ansi-term][c~nu-ansi-term~docs~badge]][c~nu-ansi-term~docs]{{hi:nu-ansi-term}} [![cat~command-line-interface][cat~command-line-interface~badge]][cat~command-line-interface]{{hi:Command-line interface}}

The original `ansi_term` crate is archived. The examples below use [`nu-ansi-term`][c~nu-ansi-term~docs]↗{{hi:nu-ansi-term}}, which provides a similar API for styling text. If output may be redirected or piped, prefer a writer that detects terminal support, such as `anstream`, and compose styles with `anstyle`.

`nu-ansi-term` styles strings with foreground colors and attributes such as bold. ANSI escape sequences are a presentation detail: do not include them in machine-readable output, and make sure colors are not the only way to convey meaning.

### Print Colored Text to the Terminal {#colored-text}

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/ansi_term/ansi_term_basic.rs:example}}
```

### Print Bold Text to the Terminal {#bold-text}

[![cat~command-line-interface][cat~command-line-interface~badge]][cat~command-line-interface]{{hi:Command-line interface}}

For more than a foreground color, construct a `nu_ansi_term::Style` and chain the attributes to apply.

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/ansi_term/ansi_term_basic1.rs:example}}
```

### Print Bold and Colored Text to the Terminal {#bold-colored-text}

[![cat~command-line-interface][cat~command-line-interface~badge]][cat~command-line-interface]{{hi:Command-line interface}}

`nu_ansi_term::Color` also supports chained styling methods.

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/ansi_term/ansi_term_basic2.rs:example}}
```

## Manipulate the Cursor, Style the Output, Handle Input Events {#crossterm}

{{#badges crossterm}}

[`crossterm`][c~crossterm~docs]↗ is a cross-platform, low-level library for terminal control and event handling. It is useful when an application needs cursor movement, screen modes, styled output, or keyboard and mouse events.

- Direct control over terminal output and flushing.
- Terminal detection.
- Cursor manipulation.
- Styled output.
- Terminal handling.
- Events (key inputs, mouse...).

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/crossterm.rs:example}}
```

## `termcolor` {#termcolor}

{{#badges termcolor}}{{hi:Color}}{{hi:Win}}{{hi:Windows}}{{hi:Ansi}}{{hi:Console}}

[`termcolor`][c~termcolor~docs]↗ is a simple cross platform library for writing colored text to a terminal. It offers a straightforward way to add colored output to your terminal applications in Rust, working consistently across different operating systems. It supports various color choices, text styling, and provides options for controlling where the colored output is directed, such as standard output or standard error.

## `anstyle` {#anstyle}

[![anstyle~website][c~anstyle~website~badge]][c~anstyle~website] {{#badges anstyle}} {{hi:Ansi}} {{hi:Color}} {{hi:No_std}} {{hi:Terminal}} {{#categories command-line-interface="Command-line interface"}}

[`anstyle`][c~anstyle~docs]↗ provides composable and spec-compliant ANSI escape code manipulation for styling terminal output.

## `anstream` {#anstream}

[![anstream~website][c~anstream~website~badge]][c~anstream~website] {{#badges anstream}} {{hi:Ansi}} {{hi:Color}} {{hi:Strip}} {{hi:Terminal}} {{hi:Wincon}} {{#categories command-line-interface="Command-line interface"}}

[`anstream`][c~anstream~docs]↗ is a cross-platform library for writing colored text to a terminal. It offers a streaming API for composing and writing styled ANSI output to terminals. It facilitates efficient construction of complex styled strings via chaining, minimizing allocations and optimizing write operations. [`anstream`][c~anstream~docs]↗{{hi:anstream}} supports configurable output destinations (stdout, stderr, or custom writers) and guarantees correct ANSI escape code handling for cross-platform compatibility.

## `ansiterm` {#ansiterm}

{{#badges ansiterm}}

[`ansiterm`][c~ansiterm~docs]↗ is a library for ANSI terminal colors and styles (bold, underline). It provides ANSI escape code manipulation for terminal styling, offering a more direct and lower-level approach compared to some higher-level crates.

## `console` {#console}

{{#badges console}}{{hi:Ansi}}{{hi:Colors}}{{hi:console}}{{hi:Terminal}}{{hi:Cli}}

[`console`][c~console~docs]↗ is a terminal and console abstraction for Rust. The [`console`][c~console~docs]↗{{hi:console}} crate provides a comprehensive set of tools for building interactive console applications. It offers features such as styled text output with ANSI escape code support, progress bar rendering, user input handling (including password prompting and line editing), and terminal manipulation. console aims to simplify the development of rich command-line interfaces by abstracting away platform-specific terminal complexities and providing a consistent API for common console interactions.

## `owo-colors` {#owo-colors}

{{#badges owo-colors command-line-interface="Command-line interface"}}{{hi:Ansi_term}}{{hi:Cli}}{{hi:Color}}{{hi:No-std}}{{hi:Terminal}}

[`owo-colors`][c~owo-colors~docs]↗ is a zero-allocation terminal colors that will make people go 'owo'. It provides a simple and fast way to add color to terminal output. It leverages ANSI escape codes for styling and focuses on a concise API for common use cases, prioritizing speed and ease of use over more complex styling features. It's designed to be lightweight and efficient, minimizing overhead for applications where basic terminal coloring is sufficient.

## `stylish` {#stylish}

{{#badges stylish}}

[`stylish`][c~stylish~docs]↗ is another crate implementing colorized text.

## `yansi` {#yansi}

{{#badges yansi command-line-interface="Command-line interface"}}{{hi:Paint}}{{hi:Format}}{{hi:Color}}{{hi:Ansi}}{{hi:Terminal}}

[`yansi`][c~yansi~docs]↗ is a simple ANSI terminal color painting library. It provides an ergonomic and composable API for styling terminal output with ANSI escape codes. It emphasizes ease of use through its builder-like interface, allowing developers to chain styling methods and construct complex formatted strings.

## `termion` {#termion}

{{#badges termion}}{{hi:Color}}{{hi:Password}}{{hi:Terminal}}{{hi:Tty}}{{hi:Tui}}

[`termion`][c~termion~docs]↗ is a pure Rust, bindless library for low-level handling, manipulating and reading information about terminals. This provides a full-featured alternative to Termbox.

Termion aims to be simple and yet expressive. It is bindless, meaning that it is not a front-end to some other library (e.g., ncurses or termbox), but a standalone library directly talking to the TTY.

Termion is a pure Rust library that provides a [cross-platform][p~cross-platform] interface for controlling the terminal. It gives access to advanced terminal features like cursor manipulation, color control, and raw mode, enabling developers to create interactive command-line applications. Being pure Rust, it avoids external dependencies and offers predictable performance.

## `colored` {#colored}

{{#badges colored}}{{hi:Color}}{{hi:String}}{{hi:Term}}{{hi:Ansi_term}}{{hi:Term-painter}}

[`colored`][c~colored~docs]↗{{hi:colored}} adds color and style methods to strings through its `Colorize` trait. This example prints success, warning, and error messages with different styles:

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/ansi_term/colored.rs:example}}
```

## Print Tables with `comfy-table` {#comfy-table}

{{#badges comfy-table command-line-interface="Command-line interface"}}

[`comfy-table`][c~comfy-table~docs]↗{{hi:comfy-table}} renders rows and columns as aligned text. With `ContentArrangement::Dynamic`, long cells wrap to fit the table width, which defaults to the width of the attached terminal; the example sets an explicit width so its output is the same everywhere. A style preset, loaded with `load_style`, selects the border characters: the plain-ASCII `ASCII_FULL` below, or `UTF8_FULL` for box-drawing characters. Table output is for people: for scripts, print JSON or one record per line instead. `prettytable-rs` has not been updated since 2022, and `term-table` is smaller and simpler, so this book shows `comfy-table`; `tabled` is a popular alternative that builds tables from your structs.

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/ansi_term/comfy_table.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[user_interaction | User Interaction]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  ansi_terminal: `ansi_term` is archived; write / decide what to cover
</div>
