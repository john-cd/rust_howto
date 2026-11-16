//! Memory usage and timing examples.
//!
//! TODO Reviewed ??-??-2026.

/// Examples that analyze memory usage and measure time.
mod memory_usage_analysis {

    /// This module provides the `dhat` implementation used by its parent.
    pub mod dhat;

    /// This module provides the `measure_time` implementation used by its parent.
    pub mod measure_time;
}

/// Runs both the heap profiling and execution-time demonstrations.
fn main() {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("dhat"));
    command = command.subcommand(clap::Command::new("measure-time"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("dhat", _)) => {
            memory_usage_analysis::dhat::run();
        }
        Some(("measure-time", _)) => {
            memory_usage_analysis::measure_time::run();
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
