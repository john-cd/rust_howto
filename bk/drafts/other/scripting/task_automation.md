# Task Automation with Rust

{{#include task_automation.incl.md}}

You can build simple Rust applications to automate repetitive tasks. The following tools may help:

## Scheduling {#skip}

- `cron` (cron-like scheduling).

## External Process Management {#skip1}

- `duct`.
- `std::process` (standard library).

See [[external_commands | External Commands]].

## Interacting with shells {#skip2}

- `shell-words` process command line arguments according to the parsing rules of Unix shells.
- `shellexpand` is a library for shell-like expansion in strings. For example, it expands variables like `$A` or `${B}` into their values and to expand `~` in the beginning of a string into the home directory (given some context).

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
TODO write
where does this chapter belong?
</div>
