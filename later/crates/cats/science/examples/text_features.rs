// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Fits a training-only vocabulary and ranks text with count-vector cosine similarity.

use std::collections::BTreeSet;
use std::collections::HashMap;

use ndarray_016::Array1;

/// A deterministic feature order learned only from the training documents.
struct Vocabulary {
    /// Maps each training token to its fixed feature column.
    columns: HashMap<String, usize>,
}

/// Lowercases Unicode alphanumeric words without claiming pretrained tokenization.
fn words(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
}

impl Vocabulary {
    /// Learns sorted vocabulary columns from a nonempty set of usable words.
    fn fit(documents: &[&str]) -> Result<Self, &'static str> {
        let terms: BTreeSet<_> = documents.iter().flat_map(|text| words(text)).collect();
        if terms.is_empty() {
            return Err("empty training vocabulary");
        }
        let columns = terms
            .into_iter()
            .enumerate()
            .map(|(i, term)| (term, i))
            .collect();
        Ok(Self { columns })
    }

    /// Counts known terms without adding columns for unseen inference words.
    fn transform(&self, text: &str) -> Array1<f64> {
        let mut counts = Array1::zeros(self.columns.len());
        for word in words(text) {
            if let Some(&column) = self.columns.get(&word) {
                counts[column] += 1.0;
            }
        }
        counts
    }
}

/// Computes finite count-vector similarity, treating an all-unknown query as no match.
fn cosine(left: &Array1<f64>, right: &Array1<f64>) -> Result<f64, &'static str> {
    if left.len() != right.len() || left.iter().chain(right).any(|v| !v.is_finite()) {
        return Err("mismatched or non-finite feature vectors");
    }
    let norm = left.dot(left).sqrt() * right.dot(right).sqrt();
    if !norm.is_finite() {
        return Err("feature norm overflow");
    }
    if norm == 0.0 {
        return Ok(0.0);
    }
    Ok((left.dot(right) / norm).clamp(-1.0, 1.0))
}

/// Prints a deterministic lexical retrieval baseline with no downloaded models.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let documents = ["rust async systems", "audio signal spectrum"];
    let vocabulary = Vocabulary::fit(&documents)?;
    let query = vocabulary.transform("Rust systems unknown");
    for document in documents {
        println!(
            "{:.3}: {document}",
            cosine(&query, &vocabulary.transform(document))?
        );
    }
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Inference-only words cannot mutate the vocabulary or its feature order.
    #[test]
    fn held_out_words_do_not_add_columns() {
        let vocabulary = Vocabulary::fit(&["Rust audio"]).expect("training words");
        assert_eq!(vocabulary.transform("RUST unseen").to_vec(), vec![0.0, 1.0]);
        assert_eq!(vocabulary.columns.len(), 2);
        assert_eq!(vocabulary.transform("audio").to_vec(), vec![1.0, 0.0]);
    }

    /// An unknown query has zero similarity instead of a division-by-zero result.
    #[test]
    fn unknown_query_has_no_match() {
        let vocabulary = Vocabulary::fit(&["rust systems"]).expect("training words");
        assert_eq!(
            cosine(
                &vocabulary.transform("unknown"),
                &vocabulary.transform("rust")
            ),
            Ok(0.0)
        );
    }

    /// Exact token matching separates overlapping words and ranks a relevant document first.
    #[test]
    fn lexical_ranking_is_reproducible() {
        let vocabulary = Vocabulary::fit(&["rust systems", "rusty audio"]).expect("training words");
        let query = vocabulary.transform("rust");
        assert!(
            cosine(&query, &vocabulary.transform("rust systems")).expect("valid vectors") > 0.7
        );
        assert_eq!(
            cosine(&query, &vocabulary.transform("rusty audio")),
            Ok(0.0)
        );
        assert!(Vocabulary::fit(&["   "]).is_err());
    }
}
