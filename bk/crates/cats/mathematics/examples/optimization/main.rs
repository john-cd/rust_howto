//! Runs the optimization examples.
//!
//! TODO Reviewed ??-??-2026.

mod brent_minimization;
mod cmaes_sphere;
mod genetic_algorithm;
mod nelder_mead_rosenbrock;
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
