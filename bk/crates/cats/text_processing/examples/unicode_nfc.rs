// TODO Reviewed ??-??-2026.
//! Compares canonically equivalent text after NFC normalization.

// ANCHOR: example
use icu_normalizer::ComposingNormalizerBorrowed;

/// Compares the precomposed and decomposed forms of é before and after NFC.
fn main() {
    let precomposed = "é";
    let decomposed = "e\u{301}";
    assert_ne!(precomposed, decomposed);

    let nfc = ComposingNormalizerBorrowed::new_nfc();
    assert_eq!(nfc.normalize(precomposed), nfc.normalize(decomposed));
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Confirms NFC makes canonically equivalent spellings equal.
    #[test]
    fn normalizes_canonically_equivalent_text() {
        super::main();
    }
}
