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

Argument Parsing: [`clap`][c-clap]⮳{{hi:clap}}, [`structopt`][c-structopt]⮳{{hi:structopt}}, [`argh`][c-argh]⮳{{hi:argh}}
[[argument_parsing | Argument Parsing]]

Interactive Prompts: [`dialoguer`][c-dialoguer]⮳{{hi:dialoguer}}, [`console`][c-console]⮳{{hi:console}}
[[user_interaction | User Interaction]]

Progress Bars: [`indicatif`][c-indicatif]⮳{{hi:indicatif}}, [`pbr`][c-pbr]⮳{{hi:pbr}}
[[command-line-interface | Command Line Interface]]

Table Formatting: [`prettytable`][c-prettytable]⮳{{hi:prettytable}}, [`term-table`][c-term_table]⮳{{hi:term-table}}

Color Output: [`ansi_term`][c-ansi_term]⮳{{hi:ansi_term}}, [`owo-colors`][c-owo_colors]⮳{{hi:owo-colors}}

General CLI Utilities: [`structopt`][c-structopt]⮳{{hi:structopt}}, [`clap`][c-clap]⮳{{hi:clap}} (often handle more than just argument parsing)

File System Operations: `std::fs`, [`pathdiff`][c-pathdiff]⮳{{hi:pathdiff}}
[[filesystem | Filesystem]]

Process Management: `std::process`
[[external_commands | External Commands]]

Text Manipulation: [`regex`][c-regex]⮳{{hi:regex}}, [`grep-cli`][c-grep_cli]⮳{{hi:grep-cli}}, [`bat`][c-bat]⮳{{hi:bat}} (for `cat` like functionality)
[[text-processing | Text Processing]]

Configuration: [`config`][c-config]⮳{{hi:config}}, [`serde`][c-serde]⮳{{hi:serde}} (for serialization)
[[config | Config]]
[[configuration | Configuration]]

Logging: [`log`][c-log]⮳{{hi:log}}, [`env_logger`][c-env_logger]⮳{{hi:env_logger}}
[[log | Log]]
[[tracing | Tracing]]
[[tracing_alternatives | Tracing Alternatives]]

---

## `colored` {#colored}

[![colored][c-colored-badge]][c-colored] [![colored-crates.io][c-colored-crates.io-badge]][c-colored-crates.io] [![colored-github][c-colored-github-badge]][c-colored-github] [![colored-lib.rs][c-colored-lib.rs-badge]][c-colored-lib.rs]{{hi:colored}}{{hi:Color}}{{hi:String}}{{hi:Term}}{{hi:Ansi_term}}{{hi:Term-painter}}

The most simple way to add colors in your terminal

</div>
