//! Runs the process management examples.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(windows)]
mod job_object;
mod kill_guard;
#[cfg(unix)]
mod process_group;
mod timeout_kill;

/// Runs the examples that are available on this platform.
fn main() -> anyhow::Result<()> {
    #[cfg(unix)]
    process_group::run()?;
    #[cfg(windows)]
    job_object::run()?;
    timeout_kill::run()?;
    kill_guard::run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn main_runs() {
        main().expect("the example should run");
    }
}
