# Task Automation with Rust

{{#include task_automation.incl.md}}

You can build simple Rust applications to automate repetitive tasks. The following tools may help:

## Scheduling {#skip}

- [`cron`][c~cron~docs]↗{{hi:cron}} (cron-like scheduling).

## External Process Management {#skip1}

- [`duct`][c~duct~docs]↗{{hi:duct}}.
- [`std::process`]( )↗{{hi: }} (standard library)↗.

See [[external_commands | External Commands]].

## Interact with Shells {#skip2}

- [`shell-words`]( )↗{{hi: }} process command line arguments according to the [parsing][p~parsing] rules of Unix shells.
- [`shellexpand`]( )↗{{hi: }} is a library for shell-like expansion in strings. For example, it expands variables like `$A` or `${B}` into their values and to expand `~` in the beginning of a string into the home directory (given some context).

See also [[shells | Shells]].

## Related Topics {#skip4}

- [[command-line-interface | Command Line Interface]].
  - [[argument_parsing | Argument Parsing]].
  - [[tui | TUI]].
  - [[user_interaction | User Interaction]].
- [[filesystem | Filesystem]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write; where does this chapter belong](https://github.com/john-cd/rust_howto/issues/1232)?
</div>
