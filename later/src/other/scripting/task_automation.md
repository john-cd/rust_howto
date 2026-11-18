<!-- TODO Reviewed ??-??-2026. -->

# Task Automation with Rust

{{#include task_automation.incl.md}}

Put the task in a command that runs once and exits with a failure status when something goes wrong.

## Choose Where to Schedule the Task {#scheduling}

For jobs that must run while the application is stopped, register the command with a host scheduler such as a systemd timer or Windows Task Scheduler. To run jobs within an active Rust service, see [Scheduling](../scheduling/scheduling.md).

## Run a Configured Command {#external-process-management}

For a fixed executable, start with [`std::process::Command`][c~std::process~docs]↗ and pass each argument separately. [`duct`][c~duct~docs]↗ adds a convenient way to capture stdout, check the exit status, and compose pipelines. When a command reads paths or other user values, validate them for the executable's argument syntax. A value passed as one argument can still be interpreted as an option.

## Parse a Command Template {#interact-with-shells}

If you control the configuration format, store the executable and arguments separately. For a text template such as the one below, [`shell-words`][c~shell-words~docs]↗ uses Unix quoting to split the template. Then [`shellexpand`][c~shellexpand~docs]↗ substitutes values inside each argument. This order preserves spaces and quotes in a substituted value. The template format does not implement a shell's pipelines, redirects, or quoting rules for variable expansion. Use templates from a trusted source because the first word selects the executable.

## Example: a Scheduled Task {#scheduled-task}

This example prints the next run time, then runs `rustc --print cfg` immediately to demonstrate the command. The printed time is `2026-10-31 02:30:00 UTC`, followed by the compiler's target configuration. The example needs `rustc` on `PATH`. It exits after one run, so a separate scheduler is responsible for starting it at the intended time.

```rust,noplayground
{{#include ../../../crates/other/examples/task_automation/scheduled_command.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[external_commands | External Commands]].
- [[shells | Shells]].
- [[filesystem | Filesystem]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
