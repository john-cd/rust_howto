// ANCHOR: example
use rand::RngExt;
use rand::SeedableRng;
use rand::rngs::StdRng;

/// A candidate solution that remembers the best position it has visited.
struct Particle {
    position: Vec<f64>,
    velocity: Vec<f64>,
    best_position: Vec<f64>,
    best_value: f64,
}

/// The Rastrigin function: many local minima, global minimum 0 at the origin.
fn rastrigin(x: &[f64]) -> f64 {
    10.0 * x.len() as f64
        + x.iter()
            .map(|v| v * v - 10.0 * (2.0 * std::f64::consts::PI * v).cos())
            .sum::<f64>()
}

/// Minimizes `f` over `[-5.12, 5.12]^dim` with particle swarm optimization.
///
/// Each velocity blends inertia, a pull toward the particle's own best
/// position, and a pull toward the swarm's best position. The seed makes the
/// run reproducible, and `iterations` caps the work.
fn pso(
    f: impl Fn(&[f64]) -> f64,
    dim: usize,
    swarm_size: usize,
    iterations: usize,
    seed: u64,
) -> (Vec<f64>, f64) {
    const BOUND: f64 = 5.12;
    const INERTIA: f64 = 0.72;
    const COGNITIVE: f64 = 1.49;
    const SOCIAL: f64 = 1.49;

    let mut rng = StdRng::seed_from_u64(seed);
    let mut swarm: Vec<Particle> = (0..swarm_size)
        .map(|_| {
            let position: Vec<f64> =
                (0..dim).map(|_| rng.random_range(-BOUND..BOUND)).collect();
            let best_value = f(&position);
            Particle {
                velocity: vec![0.0; dim],
                best_position: position.clone(),
                best_value,
                position,
            }
        })
        .collect();
    let mut global_best = swarm
        .iter()
        .min_by(|a, b| a.best_value.total_cmp(&b.best_value))
        .map_or_else(
            || (vec![0.0; dim], f64::INFINITY),
            |p| (p.best_position.clone(), p.best_value),
        );

    for _ in 0..iterations {
        let swarm_best = global_best.0.clone();
        for particle in &mut swarm {
            let dims = particle
                .position
                .iter_mut()
                .zip(&mut particle.velocity)
                .zip(&particle.best_position)
                .zip(&swarm_best);
            for (((x, v), own_best), group_best) in dims {
                let (r1, r2): (f64, f64) = (rng.random(), rng.random());
                *v = INERTIA * *v
                    + COGNITIVE * r1 * (own_best - *x)
                    + SOCIAL * r2 * (group_best - *x);
                *x = (*x + *v).clamp(-BOUND, BOUND);
            }
            let value = f(&particle.position);
            if value < particle.best_value {
                particle.best_value = value;
                particle.best_position.clone_from(&particle.position);
                if value < global_best.1 {
                    global_best = (particle.position.clone(), value);
                }
            }
        }
    }
    global_best
}

/// Prints the best point found for a two-dimensional Rastrigin function.
fn main() {
    let (best, value) = pso(rastrigin, 2, 30, 200, 42);
    println!("minimum near {best:.4?}, f = {value:.4}");
}
// ANCHOR_END: example

/// Runs the example from the chapter's example runner.
pub fn run() {
    main();
}

/// Verifies that the seeded swarm gets close to the global minimum and that
/// the same seed gives the same answer.
#[test]
fn test() {
    let first = pso(rastrigin, 2, 30, 200, 42);
    assert!(first.1 < 1.0, "value = {}", first.1);
    assert_eq!(first, pso(rastrigin, 2, 30, 200, 42));
}
