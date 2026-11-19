// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Simulates preemptive fixed-priority scheduling on integer ticks without an RTOS.

/// One job with a release time, fixed priority, and remaining execution cost.
#[derive(Clone, Copy)]
struct Job {
    /// Display label used to distinguish this job from other entries.
    name: &'static str,

    /// Scheduling priority; smaller numeric values are selected first.
    priority: u8,

    /// First tick at which this job becomes eligible to execute.
    release_tick: u8,

    /// Execution ticks still required; scheduling a tick decrements this count.
    remaining_ticks: u8,
}

/// Executes at most one ready job per tick, preempting lower-priority work.
///
/// Selects the smallest priority number among released, unfinished jobs; equal
/// priorities retain slice order. Mutates each selected job's remaining cost and
/// records only busy ticks, so idle ticks produce no output entry. Jobs still
/// unfinished at `horizon` retain their remaining cost for inspection.
///
/// Scanning all jobs each tick costs O(horizon * jobs.len()) time. This is a
/// deterministic teaching simulator, rather than an RTOS timing guarantee.
fn fixed_priority_schedule(jobs: &mut [Job], horizon: u8) -> Vec<(u8, &'static str)> {
    let mut schedule = Vec::new();

    for tick in 0..horizon {
        if let Some(job) = jobs
            .iter_mut()
            .filter(|job| job.release_tick <= tick && job.remaining_ticks > 0)
            .min_by_key(|job| job.priority)
        {
            schedule.push((tick, job.name));
            job.remaining_ticks -= 1;
        }
    }

    schedule
}

/// Prints a deterministic fixed-priority schedule without starting an RTOS.
fn main() -> anyhow::Result<()> {
    run_example()
}

/// Simulates and prints the fixed-priority schedule for two example tasks.
fn run_example() -> anyhow::Result<()> {
    let mut jobs = [
        Job {
            name: "sensor read",
            priority: 2,
            release_tick: 0,
            remaining_ticks: 3,
        },
        Job {
            name: "control update",
            priority: 1,
            release_tick: 1,
            remaining_ticks: 1,
        },
    ];

    for (tick, task) in fixed_priority_schedule(&mut jobs, 4) {
        println!("tick {tick}: {task}");
    }
    Ok(())
}
// ANCHOR_END: example

/// Runs the book example.
pub fn run() -> anyhow::Result<()> {
    let entry_point = main;
    entry_point()
}
/// Verifies the example entry point prints a complete simulated schedule.
#[cfg(test)]
mod tests {
    use super::*;

    /// Schedules higher-priority work before lower-priority work when it becomes ready.
    #[test]
    fn higher_priority_job_runs_first_when_released() {
        let mut jobs = [
            Job {
                name: "sensor read",
                priority: 2,
                release_tick: 0,
                remaining_ticks: 3,
            },
            Job {
                name: "control update",
                priority: 1,
                release_tick: 1,
                remaining_ticks: 1,
            },
        ];

        assert_eq!(
            fixed_priority_schedule(&mut jobs, 4),
            [
                (0, "sensor read"),
                (1, "control update"),
                (2, "sensor read"),
                (3, "sensor read"),
            ]
        );
    }
}

/// Provides private main tests behavior for the fixed priority schedule component.
#[cfg(test)]
mod private_main_tests {
    use super::main;

    /// Checks that private main succeeds.
    #[test]
    fn private_main_succeeds() {
        let entry_point = main;
        entry_point().expect("the example entry point should succeed");
    }
}
