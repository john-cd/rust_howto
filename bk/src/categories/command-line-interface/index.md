# Command Line

Techniques to help create command line interfaces{{hi:Command-line interface}}, such as argument parsers{{hi:Argument parsers}}, line editing{{hi:Line editing}}, or output coloring and formatting{{hi:Output coloring and formatting}}

## Argument Parsing

{{#include argument_parsing.incl.md}}

## ANSI Terminal Handling

{{#include ansi_terminal.incl.md}}

## Terminal User Interfaces (TUI)

{{#include tui.incl.md}}

## User Interactions

{{#include user_interaction.incl.md}}

## See also

[Command Line Applications in Rust (book)][book-rust-cli]⮳

[Code][book-command-line-rust-github]⮳ for `Command-Line Rust` (O'Reilly, 2022, ISBN 9781098109417)

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P1 review](https://github.com/john-cd/rust_howto/issues/907)

Argument Parsing: clap, structopt, argh
Interactive Prompts: dialoguer, console
Progress Bars: indicatif, pbr
Table Formatting: prettytable, term-table
Color Output: ansi_term, owo-colors
General CLI Utilities: structopt, clap (often handle more than just argument parsing)
File System Operations: std::fs, pathdiff
Process Management: std::process
Text Manipulation: regex, grep-cli, bat (for cat like functionality)
Configuration: config, serde (for serialization)
Logging: log, env_logger

</div>
