# Command Line

Techniques to help create command line interfaces{{hi:Command-line interface}}, such as argument parsers{{hi:Argument parsers}}, line editing{{hi:Line editing}}, or output coloring and formatting{{hi:Output coloring and formatting}}

| Topic | Relevant Rust Crates |
|---|---|
| [[argument_parsing | Argument Parsing]] | [`clap`][c-clap]⮳{{hi:clap}}, [`structopt`][c-structopt]⮳{{hi:structopt}}, [`argh`][c-argh]⮳{{hi:argh}} |
| Interactive Prompts | [`dialoguer`][c-dialoguer]⮳{{hi:dialoguer}}, [`console`][c-console]⮳{{hi:console}} | See [[user_interaction | User Interaction]]. |
| Progress Bars | [`indicatif`][c-indicatif]⮳{{hi:indicatif}}, [`pbr`][c-pbr]⮳{{hi:pbr}} | See [[command-line-interface | Command Line Interface]]. |
| Table Formatting | [`prettytable`][c-prettytable]⮳{{hi:prettytable}}, [`term-table`][c-term_table]⮳{{hi:term-table}} | |
| Color Output | [`ansi_term`][c-ansi_term]⮳{{hi:ansi_term}}, [`owo-colors`][c-owo_colors]⮳{{hi:owo-colors}} | |
| File System Operations | Use `std::fs`, [`pathdiff`][c-pathdiff]⮳{{hi:pathdiff}}. | See [[filesystem | Filesystem]]. |
| Process Management | `std::process` | See [[external_commands | External Commands]]. |
| Text Manipulation | [`regex`][c-regex]⮳{{hi:regex}}, [`grep-cli`][c-grep_cli]⮳{{hi:grep-cli}}, [`bat`][c-bat]⮳{{hi:bat}} (for `cat` like functionality) | See [[text-processing | Text Processing]]. |
| Configuration | [`config`][c-config]⮳{{hi:config}}, [`serde`][c-serde]⮳{{hi:serde}} (for serialization) | See [[config | Config]] and [[configuration | Configuration]]. |
| Logging | [`log`][c-log]⮳{{hi:log}}, [`env_logger`][c-env_logger]⮳{{hi:env_logger}} | See [[log | Log]], [[tracing | Tracing]] and [[tracing_alternatives | Tracing Alternatives]]. |

## Argument Parsing

{{#include argument_parsing.incl.md}}

## ANSI Terminal Handling

{{#include ansi_terminal.incl.md}}

## Terminal User Interfaces (TUI)

{{#include tui.incl.md}}

## User Interactions

{{#include user_interaction.incl.md}}

## Related Topics

- [[command-line-utilities | Command Line Utilities]].
- [[internationalization | Internationalization]].

## References

- [Command Line Applications in Rust (book)][book-rust-cli]⮳.
- Look to the [Code][book-command-line-rust-github]⮳ for `Command-Line Rust` (O'Reilly, 2022, ISBN 9781098109417).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/907)
</div>
