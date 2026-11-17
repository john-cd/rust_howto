<!-- TODO Reviewed ??-??-2026. -->

# Task Automation with Rust

{{#include task_automation.incl.md}}

The following tested example parses a cron schedule and calculates its next occurrence without waiting or starting a background task:

```rust,noplayground
{{#include ../../../crates/other/examples/scheduling/cron_next.rs:example}}
```

You can build simple Rust applications to automate repetitive tasks. The following tools may help:

## Scheduling {#scheduling}

- [`cron`][c~cron~docs]↗{{hi:cron}} (cron-like scheduling).

## External Process Management {#external-process-management}

- [`duct`][c~duct~docs]↗{{hi:duct}}.
- [`std::process`][c~std::process~docs]↗{{hi:std::process}} (standard library)↗.

See [[external_commands | External Commands]].

## Interact with Shells {#interact-with-shells}

- [`shell-words`][c~shell-words~docs]↗{{hi:shell-words}} processes command line arguments according to the [parsing][p~parsing] rules of Unix shells.
- [`shellexpand`][c~shellexpand~docs]↗{{hi:shellexpand}} is a library for shell-like expansion in strings. For example, it expands variables like `$A` or `${B}` into their values and to expand `~` in the beginning of a string into the home directory (given some context).

See also [[shells | Shells]].

## Example: a Scheduled Task {#scheduled-task}

This example describes a task as data: [`cron`][c~cron~docs]↗ finds the next run, [`shellexpand`][c~shellexpand~docs]↗ expands `$WHAT` from an explicit lookup, [`shell-words`][c~shell-words~docs]↗ splits the line without a shell, and [`duct`][c~duct~docs]↗ runs the program and fails on a non-zero exit status.

```rust,editable
{{#include ../../../crates/other/examples/task_automation/scheduled_command.rs:example}}
```

For jobs that run on a schedule, see [Scheduling](../scheduling/scheduling.md) for the scheduling side of the workflow.

## Related Topics {#related-topics .skip}

- Command Line Interface<!-- Cross-book topic: bk/src/categories/command-line-interface/index.md; not included in this preview. -->.
  - Argument Parsing<!-- Cross-book topic: bk/src/categories/command-line-interface/argument_parsing.md; not included in this preview. -->.
  - Terminal User Interfaces<!-- Cross-book topic: bk/src/categories/command-line-interface/tui.md; not included in this preview. -->.
  - User Interaction<!-- Cross-book topic: bk/src/categories/command-line-interface/user_interaction.md; not included in this preview. -->.
  - [[user_interaction | User Interaction]].
- [[filesystem | Filesystem]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
