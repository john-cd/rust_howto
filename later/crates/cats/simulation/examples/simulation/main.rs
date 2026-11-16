//! Runs the simulation example.
//!
//! TODO Reviewed ??-??-2026.

/// Simulate a single-server bank queue with a timestamp-ordered event heap.
mod discrete_event;
/// Integrate exponential decay with a fixed-step fourth-order Runge-Kutta
mod ode_solver;

/// Runs the discrete-event and ODE simulation examples.
fn main() {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("discrete-event"));
    command = command.subcommand(clap::Command::new("ode-solver"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("discrete-event", _)) => {
            discrete_event::run();
        }
        Some(("ode-solver", _)) => {
            ode_solver::run();
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
