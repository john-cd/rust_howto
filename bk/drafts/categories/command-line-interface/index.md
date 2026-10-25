<!-- TODO Reviewed ??-??-2026 -->

# Command-line Interface

A command-line interface (CLI) accepts arguments and input, performs work, and presents results in a terminal. These chapters cover argument parsing, terminal output, text-based interfaces, and interactive prompts.

## Parse Command-line Arguments

Choose a parser based on the interface you need: [`clap`][c~clap~docs]↗{{hi:clap}} offers derive and builder APIs; [`lexopt`][c~lexopt~docs]↗{{hi:lexopt}} and [`pico-args`][c~pico-args~docs]↗{{hi:pico-args}} are smaller alternatives. See [[argument_parsing | Argument Parsing]].

{{#include argument_parsing.incl.md}}

## Handle Terminal Output

Use [`crossterm`][c~crossterm~docs]↗{{hi:crossterm}} for cross-platform terminal control and [`owo-colors`][c~owo-colors~docs]↗{{hi:owo-colors}} for styling text. [`termimad`][c~termimad~docs]↗{{hi:termimad}} can render Markdown in a terminal. See [[ansi_terminal | ANSI Terminal Handling]].

{{#include ansi_terminal.incl.md}}

## Build a Text User Interface

For a full-screen terminal interface, see [[tui | Terminal User Interfaces]] and [`ratatui`][c~ratatui~docs]↗{{hi:ratatui}}.

{{#include tui.incl.md}}

## Prompt Users and Report Progress

For prompts and selections, consider [`dialoguer`][c~dialoguer~docs]↗{{hi:dialoguer}} or [`inquire`][c~inquire~docs]↗{{hi:inquire}}. Use [`indicatif`][c~indicatif~docs]↗{{hi:indicatif}} for progress indicators and [`rustyline`][c~rustyline~docs]↗{{hi:rustyline}} for line editing and REPL input. See [[user_interaction | User Interaction]].

{{#include user_interaction.incl.md}}

## See Also {#see-also .skip}

- [Command Line Applications in Rust][book~rust-cli]↗.
- [Command-Line Rust][book~command-line-rust]↗ and [its code repository][book~command-line-rust~repo]↗ (O'Reilly, 2022).
- [[command-line-utilities | Command-line Utilities]].
- [[internationalization | Internationalization]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
