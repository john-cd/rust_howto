<!-- TODO Reviewed ??-??-2026. -->

# Schedule Jobs in Rust

{{#include scheduling.incl.md}}

From `bk/crates`, run `cargo test --locked -p other --example scheduling` to exercise all three recipes.

## Compute the Next Fire Times of a Cron Expression {#cron-next}

[`cron`](https://docs.rs/cron)↗ parses a cron expression and iterates the instants at which it fires. The example uses a fixed `DateTime<Utc>` rather than the clock, so it is deterministic.

```rust,noplayground
{{#include ../../../crates/other/examples/scheduling/cron_next.rs:example}}
```

### Cron Syntax {#cron-syntax}

`cron` expects 6 or 7 fields: `sec min hour day-of-month month day-of-week [year]`. The traditional 5-field form (no seconds) must be normalized first, for example by prefixing the string `"0 "`. Expressions are evaluated in the time zone of the `DateTime` you pass in. The example starts after 09:30 UTC on Friday and finds Monday through Wednesday at 09:30 UTC. Use [`chrono-tz`](https://docs.rs/chrono-tz)↗ for a named zone. Test the transition dates explicitly when a local job can fall in a skipped or repeated hour. When importing schedules from another implementation, check the day-of-week numbering and how the two day fields combine, since adding the seconds field only adjusts the field count.

## Run a Periodic Job at a Fixed Interval {#interval-job}

The first tick of `tokio::time::interval` is immediate. Choose a
[`MissedTickBehavior`](https://docs.rs/tokio/latest/tokio/time/enum.MissedTickBehavior.html)
for later ticks. `Burst` catches up, `Delay` shifts the schedule, and `Skip`
skips intervening ticks while keeping future deadlines on the original grid.
The example awaits each job before taking the next tick, so jobs stay serial.
Its `CancellationToken` stops the loop after the current job finishes. Each process runs its own timer, so two replicas would both run the job. Use an external scheduler or coordinate ownership when only one replica should run it.

```rust,noplayground
{{#include ../../../crates/other/examples/scheduling/interval_job.rs:example}}
```

## Run a Job on a Cron Schedule {#cron-loop}

The loop computes each next deadline from the cron expression. It computes
that deadline after the previous job finishes, so fire times that pass during
a job are skipped. The example uses a fixed UTC origin and monotonic elapsed
time for reproducibility. A service that follows local wall time must also
account for clock corrections and daylight-saving changes.

```rust,noplayground
{{#include ../../../crates/other/examples/scheduling/cron_loop.rs:example}}
```

[`tokio-cron-scheduler`](https://docs.rs/tokio-cron-scheduler)↗ wraps this pattern in a job scheduler with an optional persistent store.

## Related Topics {#related-topics .skip}

- [[date-and-time | Date and Time]] for time zones and daylight-saving transitions.
- [[asynchronous | Asynchronous]] for cancellation and task lifetimes.
- [[task-automation | Task Automation]] for short-lived scheduled commands.

{{#include refs.incl.md}}
