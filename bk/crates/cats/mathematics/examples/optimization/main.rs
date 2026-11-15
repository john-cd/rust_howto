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
    brent_minimization::run();
    nelder_mead_rosenbrock::run()?;
    cmaes_sphere::run();
    genetic_algorithm::run();
    particle_swarm::run();
    Ok(())
}
