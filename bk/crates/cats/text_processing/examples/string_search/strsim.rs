// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Measuring the similarity of strings with `strsim`.

use strsim::damerau_levenshtein;
use strsim::hamming;
use strsim::jaro;
use strsim::jaro_winkler;
use strsim::levenshtein;
use strsim::normalized_levenshtein;
use strsim::sorensen_dice;
use unicode_normalization::UnicodeNormalization;
use unicode_normalization::char::is_combining_mark;

/// Returns a lowercase NFD key with combining marks removed.
fn accent_insensitive_key(text: &str) -> String {
    text.nfd()
        .filter(|character| !is_combining_mark(*character))
        .flat_map(char::to_lowercase)
        .collect()
}

/// Prints several string distances and similarities.
fn main() {
    let s1 = "aleksandr";
    let s2 = "alexander";

    // Levenshtein distance:
    println!("Levenshtein: {}", levenshtein(s1, s2));

    // Normalized Levenshtein distance (0.0 to 1.0):
    println!("Normalized Levenshtein: {}", normalized_levenshtein(s1, s2));

    // Damerau-Levenshtein distance (handles transpositions):
    println!("Damerau-Levenshtein: {}", damerau_levenshtein(s1, s2));

    // Jaro similarity (0.0 to 1.0):
    println!("Jaro: {}", jaro(s1, s2));

    // Jaro-Winkler similarity (0.0 to 1.0):
    println!("Jaro-Winkler: {}", jaro_winkler(s1, s2));

    // Sørensen-Dice coefficient (0.0 to 1.0):
    println!("Sørensen-Dice: {}", sorensen_dice(s1, s2));

    // Hamming distance (only for equal length strings):
    let s3 = "karolin";
    let s4 = "kathrin";
    match hamming(s3, s4) {
        Ok(distance) => println!("Hamming: {distance}"),
        Err(e) => println!("Error calculating Hamming distance: {e}"),
    }

    // Example: Finding closest match:
    let target = "rusty";
    let candidates = ["dusty", "rust", "trust", "crusty"];

    let closest = candidates
        .iter()
        .map(|&c| (c, jaro_winkler(target, c)))
        .max_by(|a, b| {
            a.1.partial_cmp(&b.1)
                .expect("the strsim example invariant should hold")
        })
        .expect("the strsim example invariant should hold");

    println!(
        "Closest match to '{target}' is '{}' with score {}",
        closest.0, closest.1
    );

    // Canonical decomposition removes common accents after filtering marks.
    let accented = accent_insensitive_key("Crème brûlée");
    let plain = accent_insensitive_key("creme brulee");
    println!(
        "Accent-insensitive similarity: {}",
        normalized_levenshtein(&accented, &plain)
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the strsim example through its entry point.
    #[test]
    fn test_strsim_example_entry_point() {
        main();
    }

    /// Edit distances match known values.
    #[test]
    fn edit_distances() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("same", "same"), 0);
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(damerau_levenshtein("ab", "ba"), 1);
        assert_eq!(levenshtein("ab", "ba"), 2);
    }

    /// Hamming distance requires equal lengths.
    #[test]
    fn hamming_distance() {
        assert_eq!(hamming("karolin", "kathrin"), Ok(3));
        assert!(hamming("short", "longer").is_err());
    }

    /// Similarity scores are in range, and equal strings score 1.
    #[test]
    fn similarity_scores() {
        assert!((jaro_winkler("rusty", "rusty") - 1.0).abs() < f64::EPSILON);
        assert!(
            (normalized_levenshtein("abc", "abc") - 1.0).abs() < f64::EPSILON
        );
        let score = sorensen_dice("aleksandr", "alexander");
        assert!((0.0..=1.0).contains(&score));
        assert!(jaro("abc", "xyz") < 0.01);
    }

    /// "rust" is the closest to "rusty" among the candidates.
    #[test]
    fn closest_candidate() -> Result<(), Box<dyn std::error::Error>> {
        let candidates = ["dusty", "rust", "trust", "crusty"];
        let closest = candidates
            .iter()
            .max_by(|a, b| {
                jaro_winkler("rusty", a)
                    .partial_cmp(&jaro_winkler("rusty", b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .ok_or("no candidates")?;
        assert_eq!(*closest, "rust");
        Ok(())
    }
}
