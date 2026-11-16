// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! Describe a task as data, then schedule, expand, parse and run it.
//!
//! - `cron` computes when the task is due.
//! - `shellexpand` expands `$VARIABLE` and `~` in the command template. The lookup functions are
//!   passed in, so the result does not depend on the environment of the machine running the
//!   example.
//! - `shell-words` splits the expanded line into program and arguments using Unix shell quoting
//!   rules, without starting a shell.
//! - `duct` runs the program, captures its output and fails on a non-zero exit status.

use std::str::FromStr;

use anyhow::Context;
use anyhow::Result;
use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use cron::Schedule;

/// A task to run on a schedule.
struct Task {
    /// A `cron` expression (seconds first).
    schedule: Schedule,

    /// A command line that may contain `$VARIABLE` references.
    command: String,
}

impl Task {
    /// Creates a task from a schedule expression and a command template.
    fn new(schedule: &str, command: &str) -> Result<Self> {
        Ok(Self {
            schedule: Schedule::from_str(schedule)?,
            command: command.to_owned(),
        })
    }

    /// Returns the first time after `after` at which the task is due.
    fn next_run(&self, after: DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.schedule.after(&after).next()
    }

    /// Expands `$WHAT`, tokenizes the configured command, runs it, and returns stdout.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is empty, cannot be started, or exits unsuccessfully.
    fn run(&self, what: &str) -> Result<String> {
        let line = shellexpand::env_with_context_no_errors(&self.command, |name: &str| {
            (name == "WHAT").then_some(what)
        });
        let words = shell_words::split(&line)?;
        let (program, args) = words.split_first().context("empty command")?;
        let output = duct::cmd(program, args).read()?;
        Ok(output)
    }
}

/// Schedules a task and runs it once.
fn main() -> Result<()> {
    example()
}

/// Computes the next scheduled run and executes the configured command once.
///
/// # Errors
///
/// Returns an error if the schedule, command, or command output is invalid.
fn example() -> Result<()> {
    // Every day at 02:30:00 UTC.
    let task = Task::new("0 30 2 * * *", "rustc --print $WHAT")?;
    let now = Utc
        .with_ymd_and_hms(2026, 10, 30, 3, 0, 0)
        .single()
        .context("invalid date")?;
    let next = task.next_run(now).context("the schedule has no next run")?;
    println!("next run: {next}");
    assert_eq!(
        Some(next),
        Utc.with_ymd_and_hms(2026, 10, 31, 2, 30, 0).single()
    );

    let output = task.run("cfg")?;
    println!("{output}");
    assert!(output.contains("target_os"));
    Ok(())
}
// ANCHOR_END: example

/// Variables come from the supplied lookup, and quotes group words.
#[cfg(test)]
mod tests {
    use super::*;

    /// Variables come from the supplied lookup, and quotes group words.
    #[test]
    fn expands_variables_and_keeps_quoted_words_together() -> Result<()> {
        let task = Task::new("0 0 * * * *", "rustc --print \"$WHAT x\"")?;
        let line = shellexpand::env_with_context_no_errors(&task.command, |name: &str| {
            (name == "WHAT").then_some("hi")
        });
        assert_eq!(shell_words::split(&line)?, ["rustc", "--print", "hi x"]);
        Ok(())
    }

    /// An unterminated quote and an empty command are errors.
    #[test]
    fn rejects_bad_commands() -> Result<()> {
        assert!(Task::new("0 0 * * * *", "echo 'oops")?.run("x").is_err());
        assert!(Task::new("0 0 * * * *", "")?.run("x").is_err());
        Ok(())
    }

    /// A program that exits with a failure status is an error.
    #[test]
    fn failing_program_is_an_error() -> Result<()> {
        let task = Task::new("0 0 * * * *", "rustc --no-such-flag")?;
        assert!(task.run("x").is_err());
        Ok(())
    }

    /// Reports an error when invalid schedule occurs.
    #[test]
    fn invalid_schedule_is_an_error() {
        assert!(Task::new("not a schedule", "true").is_err());
    }
}

pub fn run() -> impl std::process::Termination {
    let entry_point = main;
    entry_point()
}

#[cfg(test)]
mod entry_point_tests {
    use super::*;

    #[test]
    fn private_main_runs() {
        let entry_point = main;
        entry_point().expect("example entry point should succeed");
    }

    #[test]
    fn next_run_is_after_the_requested_time() -> Result<()> {
        let task = Task::new("0 30 2 * * *", "rustc --print $WHAT")?;
        let after = Utc
            .with_ymd_and_hms(2026, 10, 30, 3, 0, 0)
            .single()
            .context("invalid date")?;
        let next = task.next_run(after).context("missing next run")?;

        assert_eq!(
            next,
            Utc.with_ymd_and_hms(2026, 10, 31, 2, 30, 0)
                .single()
                .expect("valid date")
        );
        Ok(())
    }
}
