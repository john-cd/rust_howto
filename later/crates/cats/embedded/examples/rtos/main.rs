//! TODO Reviewed ??-??-2026.

// ANCHOR: example
#[derive(Clone, Copy)]
struct Job {
    name: &'static str,
    priority: u8,
    release_tick: u8,
    remaining_ticks: u8,
}

/// Runs one discrete tick per job, always choosing the highest-priority ready
/// job.
fn fixed_priority_schedule(
    jobs: &mut [Job],
    horizon: u8,
) -> Vec<(u8, &'static str)> {
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

/// Runs the book example from the example crate's module runner.
pub fn run() -> anyhow::Result<()> {
    run_example()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the example entry point prints a complete simulated schedule.
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }

    /// Verifies a newly released high-priority job preempts lower-priority
    /// work.
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
