<!-- TODO Reviewed ??-??-2026 -->

| Task                                | Useful tools                                                   | Notes                                                                                                             |
| ----------------------------------- | -------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| Run a task on a schedule            | [`cron`][c~cron~docs]↗                                         | Choose a scheduler that matches the deployment environment; system cron is commonly available on Unix-like hosts. |
| Start and supervise a child process | [`std::process`][c~std::process~docs]↗, [`duct`][c~duct~docs]↗ | Pass arguments separately, check the exit status, and avoid invoking a shell for fixed commands.                  |
| Parse shell-style words             | [`shell-words`][c~shell-words~docs]↗                           | This parses Unix shell word syntax; it does not execute commands or provide Windows command-line parsing.         |
| Expand environment variables or `~` | [`shellexpand`][c~shellexpand~docs]↗                           | Supply the environment/context explicitly when deterministic behavior is needed.                                  |

For portable automation, keep task logic in the Rust program and delegate scheduling and service supervision to the host environment. Treat file paths and process output as untrusted input, and propagate failures instead of silently ignoring them.
