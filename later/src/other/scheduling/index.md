<!-- TODO Reviewed ??-??-2026 -->
# Scheduling

A long-running service often has to run jobs on a timetable: every few seconds, hourly, or at 09:30 on weekdays. This chapter shows how to compute fire times from cron expressions, run a periodic job in a Tokio service, and cope with jobs that overrun.

{{#include scheduling.incl.md}}

## Choosing a Scheduler

- **In-process** (this chapter): simplest, but jobs stop when the process stops, and every replica of a service runs its own copy.
- **The operating system**: systemd timers, `cron`, or Windows Task Scheduler survive restarts and run a short-lived binary; use them when the job does not need shared in-memory state.
- **A queue or a workflow engine**: when jobs must run exactly once across several machines, or be retried and observed.

See also the Date and Time and Asynchronous chapters.

## Related Topics

- Date and Time.
- Asynchronous: cancellation and structured concurrency.

{{#include refs.incl.md}}
