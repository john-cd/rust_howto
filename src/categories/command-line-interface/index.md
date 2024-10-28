# Command Line

Techniques to help create command line interfaces{{hi:Command line interface}}, such as argument parsers{{hi:Argument parsers}}, line editing{{hi:Line editing}}, or output coloring and formatting{{hi:Output coloring and formatting}}

{{#include arguments.incl.md}}

{{#include ansi_terminal.incl.md}}

## TUI

[![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]

### ratatui

[ratatui][c-ratatui-github]⮳

[![ratatui][c-ratatui-badge]][c-ratatui]
[![ratatui-crates.io][c-ratatui-crates.io-badge]][c-ratatui-crates.io]
[![ratatui-github][c-ratatui-github-badge]][c-ratatui-github]
[![ratatui-lib.rs][c-ratatui-lib.rs-badge]][c-ratatui-lib.rs]

A high-level TUI library with widgets, layout, etc.

### crossterm

[![crossterm][c-crossterm-badge]][c-crossterm]
[![crossterm-crates.io][c-crossterm-crates.io-badge]][c-crossterm-crates.io]
[![crossterm-github][c-crossterm-github-badge]][c-crossterm-github]
[![crossterm-lib.rs][c-crossterm-lib.rs-badge]][c-crossterm-lib.rs]

Low-level cross-platform terminal rendering and event handling

### Ask for confirmation, selection, text input

[![inquire][c-inquire-badge]][c-inquire]
[![inquire-crates.io][c-inquire-crates.io-badge]][c-inquire-crates.io]
[![inquire-github][c-inquire-github-badge]][c-inquire-github]
[![inquire-lib.rs][c-inquire-lib.rs-badge]][c-inquire-lib.rs]

## See also

[![tui][c-tui-badge]][c-tui]{{hi:tui}}

[![r3bl_tuify][c-r3bl_tuify-badge]][c-r3bl_tuify]{{hi:r3bl_tuify}}  [![r3bl_tuify-crates.io][c-r3bl_tuify-crates.io-badge]][c-r3bl_tuify-crates.io]⮳  [![blog-tuify][blog-tuify-badge]][blog-tuify]⮳

[Command Line Applications in Rust (book)][book-rust-cli]⮳

[Code][book-command-line-rust-github]⮳ for `Command-Line Rust` (O'Reilly, 2022, ISBN 9781098109417)

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: move TUI section to separate page
cover ratatui, etc
</div>
