<!-- TODO Reviewed ??-??-2026. -->

# Scheduling

A long-running service often has to run jobs on a timetable: every few seconds, hourly, or at 09:30 on weekdays. This chapter shows how to compute fire times from cron expressions, run a periodic job in a Tokio service, and cope with jobs that overrun.

{{#include scheduling.incl.md}}

## Choosing a Scheduler

| Need | Starting point | What to decide |
| --- | --- | --- |
| Compute calendar times | [`cron`](https://docs.rs/cron/) or [`croner`](https://docs.rs/croner/) | Check the expression dialect and the time zone. The [cron parser recipe](scheduling.md#cron-next) calculates times without running jobs. |
| Run periodically while a service is alive | [`tokio::time`](https://docs.rs/tokio/latest/tokio/time/) and [`tokio-util`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html) | Choose what happens when work overruns or shutdown begins. Each service replica has its own timer. |
| Register multiple scheduled jobs in one application | [`tokio-cron-scheduler`](https://docs.rs/tokio-cron-scheduler/) | Check storage options and how the scheduler behaves on restart or when you deploy multiple replicas. |
| Run a separate command on one host | A systemd timer, cron, or Windows Task Scheduler | Choose whether the host should run missed jobs after a restart. Settings and behavior vary by scheduler. |
| Distribute work or retry after failure | A queue and worker, or a workflow engine | Check delivery and retry guarantees. A worker can perform a side effect and fail before acknowledging the job. Use a stable job key and an idempotent operation where a retry could repeat the effect. |

For a calendar job, decide how to handle local times that occur twice or disappear
when daylight saving changes. For a periodic poll, use elapsed time and choose
whether to catch up or skip ticks.

## Related Topics

- [Date and Time][p~date-and-time] for time zones and calendar arithmetic.
- [Asynchronous][p~asynchronous] for cancellation and task lifetimes.
- [Task Automation][p~task-automation] for scheduled commands.

{{#include refs.incl.md}}
