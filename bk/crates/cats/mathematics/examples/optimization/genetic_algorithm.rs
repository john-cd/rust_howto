// ANCHOR: example
use radiate::prelude::*;

/// Number of bits in each candidate bit string.
const BITS: usize = 32;

/// Evolves a bit string toward all ones ("OneMax"): the fitness is the number
/// of set bits. The seed makes the run reproducible.
fn evolve() -> usize {
    random_provider::scoped_seed(42, || {
        let engine = GeneticEngine::builder()
            .codec(BitCodec::vector(BITS))
            .fitness_fn(|genotype: Vec<bool>| {
                genotype.into_iter().filter(|bit| *bit).count()
            })
            .build();
        let result = engine.iter().until_score(BITS).take(1000).last();
        result.map_or(0, |generation| generation.score().as_usize())
    })
}

/// Prints the best fitness reached.
fn main() {
    println!("best fitness: {} of {BITS}", evolve());
}
// ANCHOR_END: example

/// Runs the example from the chapter's example runner.
pub fn run() {
    main();
}

/// Verifies that the genetic algorithm reaches the maximum fitness.
#[test]
fn test() {
    assert_eq!(evolve(), BITS);
}
