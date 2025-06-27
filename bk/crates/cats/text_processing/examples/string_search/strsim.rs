#![allow(dead_code)]
// ANCHOR: example
use strsim::damerau_levenshtein;
use strsim::hamming;
use strsim::jaro;
use strsim::jaro_winkler;
use strsim::levenshtein;
use strsim::normalized_levenshtein;
use strsim::sorensen_dice;

// Add to your `Cargo.toml`:
// [dependencies]
// strsim = "0.11.1" # Or latest

fn main() {
    let s1 = "aleksandr";
    let s2 = "alexander";

    // Levenshtein distance
    println!("Levenshtein: {}", levenshtein(s1, s2));

    // Normalized Levenshtein distance (0.0 to 1.0)
    println!("Normalized Levenshtein: {}", normalized_levenshtein(s1, s2));

    // Damerau-Levenshtein distance (handles transpositions)
    println!("Damerau-Levenshtein: {}", damerau_levenshtein(s1, s2));

    // Jaro similarity (0.0 to 1.0)
    println!("Jaro: {}", jaro(s1, s2));

    // Jaro-Winkler similarity (0.0 to 1.0)
    println!("Jaro-Winkler: {}", jaro_winkler(s1, s2));

    // Sørensen-Dice coefficient (0.0 to 1.0)
    println!("Sørensen-Dice: {}", sorensen_dice(s1, s2));

    // Hamming distance (only for equal length strings)
    let s3 = "karolin";
    let s4 = "kathrin";
    match hamming(s3, s4) {
        Ok(distance) => println!("Hamming: {distance}"),
        Err(e) => println!("Error calculating Hamming distance: {e}"),
    }

    // Example: Finding closest match
    let target = "rusty";
    let candidates = ["dusty", "rust", "trust", "crusty"];

    let closest = candidates
        .iter()
        .map(|&c| (c, jaro_winkler(target, c)))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();

    println!(
        "Closest match to '{}' is '{}' with score {}",
        target, closest.0, closest.1
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
