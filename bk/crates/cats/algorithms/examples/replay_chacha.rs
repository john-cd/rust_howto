// TODO Reviewed ??-??-2026.
//! Replays a fixed sequence of draws with a named generator.

// ANCHOR: example
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha12Rng;

/// Replays four raw draws from a deterministic test seed.
fn main() {
    let mut rng = ChaCha12Rng::from_seed([0; 32]);
    let draws: [u32; 4] = std::array::from_fn(|_| rng.next_u32());
    assert_eq!(draws, [0x6a9a_f49b, 0x53f9_5507, 0x12ce_1f81, 0xd583_265f]);

    let mut replay = ChaCha12Rng::from_seed([0; 32]);
    assert_eq!(draws, std::array::from_fn(|_| replay.next_u32()));
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Verifies the raw output and replays it from the same seed.
    #[test]
    fn replays_known_draws() {
        super::main();
    }
}
