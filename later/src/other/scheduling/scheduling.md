<!-- TODO Reviewed ??-??-2026 -->
# Scheduling

{{#include scheduling.incl.md}}

## Compute the Next Fire Times of a Cron Expression {#cron-next}

[`cron`](https://docs.rs/cron)↗ parses a cron expression and iterates the instants at which it fires. The example uses a fixed `DateTime<Utc>` rather than the clock, so it is deterministic.

```rust,editable
{{#include ../../../crates/other/examples/scheduling/cron_next.rs:example}}
```

### Cron Syntax {#cron-syntax}

`cron` expects 6 or 7 fields: `sec min hour day-of-month month day-of-week [year]`. The traditional 5-field form (no seconds) must be normalized first, for example by prefixing `0 `. Expressions are evaluated in the time zone of the `DateTime` you pass in; use [`chrono-tz`](https://docs.rs/chrono-tz)↗ for a named zone, and be careful around daylight-saving transitions.

## Run a Periodic Job at a Fixed Interval {#interval-job}

`tokio::time::interval` ticks at a fixed period. Choose the `MissedTickBehavior` deliberately: `Burst` (the default) catches up, `Delay` shifts the schedule, and `Skip` stays on the original grid. Even with `Skip`, a tick missed during a run fires as soon as the run ends. Stop the loop with a `CancellationToken`.

```rust,editable
{{#include ../../../crates/other/examples/scheduling/interval_job.rs:example}}
```

## Run a Job on a Cron Schedule {#cron-loop}

Sleep until the next fire time, run the job, and recompute. Because each wake-up comes from the schedule, not from "now plus a period", the loop does not drift. Fire times that pass while the job is running are skipped.

```rust,editable
{{#include ../../../crates/other/examples/scheduling/cron_loop.rs:example}}
```

[`tokio-cron-scheduler`](https://docs.rs/tokio-cron-scheduler)↗ wraps this pattern in a job scheduler with an optional persistent store.

{{#include refs.incl.md}}
