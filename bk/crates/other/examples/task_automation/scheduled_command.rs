// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Compute a task's next scheduled time and run it once on demand.
//!
//! Parse the command template before substituting values so each value stays in its own
//! argument. The executable comes directly from the trusted task definition.

use std::borrow::Cow;
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

    /// Parses the command template and substitutes `what` for `$WHAT` in arguments.
    ///
    /// # Errors
    ///
    /// Returns an error for missing commands, invalid quoting, or unknown variables.
    fn command_line(&self, what: &str) -> Result<(String, Vec<String>)> {
        let words = shell_words::split(&self.command)?;
        let mut words = words.into_iter();
        let program = words
            .next()
            .filter(|word| !word.is_empty())
            .context("empty command")?;
        let args = words
            .map(|arg| {
                shellexpand::env_with_context(&arg, |name| match name {
                    "WHAT" => Ok(Some(what)),
                    _ => Err("unknown task variable"),
                })
                .map(Cow::into_owned)
                .map_err(|error| anyhow::anyhow!("{error}"))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok((program, args))
    }

    /// Runs the configured command and returns its standard output.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid, cannot be started, exits unsuccessfully,
    /// or produces output that cannot be read as UTF-8.
    fn run(&self, what: &str) -> Result<String> {
        let (program, args) = self.command_line(what)?;
        Ok(duct::cmd(program, args).read()?)
    }
}

/// Prints a task's next scheduled time and runs the command once on demand.
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

    /// Substitution preserves spaces, quotes, and option-like text within one argument.
    #[test]
    fn expands_variables_without_splitting_arguments() -> Result<()> {
        let task = Task::new("0 0 * * * *", "rustc --print $WHAT \"two words\"")?;
        let (program, args) = task.command_line("hi \"--extra\"")?;
        assert_eq!(program, "rustc");
        assert_eq!(args, ["--print", "hi \"--extra\"", "two words"]);
        Ok(())
    }

    /// Malformed templates and unknown variables fail before launching a program.
    #[test]
    fn rejects_bad_commands() -> Result<()> {
        assert!(Task::new("0 0 * * * *", "echo 'oops")?.run("x").is_err());
        assert!(Task::new("0 0 * * * *", "")?.run("x").is_err());
        assert!(
            Task::new("0 0 * * * *", "rustc ${UNKNOWN}")?
                .command_line("x")
                .is_err()
        );
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

/// Runs the example logic exposed by this module.
pub fn run() -> impl std::process::Termination {
    let entry_point = main;
    entry_point()
}

/// Provides entry point tests behavior for the scheduled command component.
#[cfg(test)]
mod entry_point_tests {
    use super::*;

    /// Checks that private main runs.
    #[test]
    fn private_main_runs() {
        let entry_point = main;
        entry_point().expect("example entry point should succeed");
    }

    /// Checks that next run is after the requested time.
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
