//! Runs the optimization examples.
//!
//! TODO Reviewed ??-??-2026.

/// Minimize a one-variable function with Brent's method.
mod brent_minimization;
/// Minimize the sphere function with CMA-ES.
mod cmaes_sphere;
/// Evolve a solution with a genetic algorithm.
mod genetic_algorithm;
/// Minimize the Rosenbrock function with Nelder-Mead.
mod nelder_mead_rosenbrock;
/// Minimize a function with particle swarm optimization.
mod particle_swarm;

/// Runs the bounded function minimization, Nelder-Mead, CMA-ES and particle
/// swarm examples.
fn main() -> Result<(), argmin::core::Error> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("brent-minimization"));
    command = command.subcommand(clap::Command::new("nelder-mead-rosenbrock"));
    command = command.subcommand(clap::Command::new("cmaes-sphere"));
    command = command.subcommand(clap::Command::new("genetic-algorithm"));
    command = command.subcommand(clap::Command::new("particle-swarm"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("brent-minimization", _)) => {
            brent_minimization::run();
            Ok(())
        }
        Some(("nelder-mead-rosenbrock", _)) => {
            nelder_mead_rosenbrock::run()?;
            Ok(())
        }
        Some(("cmaes-sphere", _)) => {
            cmaes_sphere::run();
            Ok(())
        }
        Some(("genetic-algorithm", _)) => {
            genetic_algorithm::run();
            Ok(())
        }
        Some(("particle-swarm", _)) => {
            particle_swarm::run();
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
