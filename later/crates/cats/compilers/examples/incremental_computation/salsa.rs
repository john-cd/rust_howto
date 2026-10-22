// TODO Reviewed ??-??-2026
// ANCHOR: example
//! This example demonstrates incremental computation using the `salsa` crate.
//!
//! `salsa` is a framework for writing incremental, on-demand programs: programs
//! that adapt to changes in their inputs, continuously producing an up-to-date
//! output.
//!
//! The key idea is to define computations as *tracked functions*. Salsa
//! automatically records which inputs each tracked function reads. When an
//! input changes, only the affected computations are invalidated and will be
//! re-executed on the next query; everything else is served from the cache.
//!
//! This example builds a simple dependency chain:
//! - `word_count` depends on the `content` of a `SourceInput`.
//! - `summary` depends on both `content` and `word_count`.
//!
//! When `content` is updated, Salsa invalidates the cached results for
//! `word_count` and `summary`, and recomputes them the next time they are
//! queried.

use salsa::Setter as _;

/// An *input* struct. Its fields can be mutated between queries, and Salsa
/// will track which tracked functions read them so it knows what to
/// recompute.
#[salsa::input]
struct SourceInput {
    /// The text to analyze.
    content: String,
}

/// A tracked function that counts words in the input.
///
/// Salsa memoizes the return value. The next call with the same `input` (and
/// unchanged `content`) returns the cached result without re-executing.
#[salsa::tracked]
fn word_count(db: &dyn salsa::Database, input: SourceInput) -> usize {
    let content = input.content(db);
    println!("  [word_count] computing for {content:?}");
    content.split_whitespace().count()
}

/// A tracked function that builds a human-readable summary.
///
/// It depends on `word_count`, so if `word_count` changes, this is
/// recomputed as well.
#[salsa::tracked]
fn summary(db: &dyn salsa::Database, input: SourceInput) -> String {
    let content = input.content(db);
    let count = word_count(db, input);
    println!("  [summary] computing for {content:?}");
    format!("\"{content}\" — {count} word(s)")
}

/// The *database* holds all incremental state.
///
/// `salsa::Storage` manages the memo tables; `#[salsa::db]` wires up the
/// required trait impls.
#[salsa::db]
#[derive(Default)]
struct MyDatabase {
    /// The storage of the memoized results.
    storage: salsa::Storage<Self>,
}

#[salsa::db]
impl salsa::Database for MyDatabase {}

/// Runs the queries, then updates the input and runs them again.
fn main() {
    let mut db = MyDatabase::default();

    // Create an input value. `SourceInput::new` stores it in the database.
    let input = SourceInput::new(&db, "Hello Salsa world".to_owned());

    println!("=== Initial computation ===");
    println!("content  : {:?}", input.content(&db));
    println!("words    : {}", word_count(&db, input));
    println!("summary  : {}", summary(&db, input));

    println!("\n=== Same input again (fully cached) ===");
    // No "[word_count] computing" or "[summary] computing" lines printed:
    // Salsa returns cached results.
    println!("words    : {}", word_count(&db, input));
    println!("summary  : {}", summary(&db, input));

    // Mutate the input. Salsa marks the dependent memos as outdated.
    input
        .set_content(&mut db)
        .to("A different string with more words".to_owned());

    println!("\n=== After updating content ===");
    println!("content  : {:?}", input.content(&db));
    println!("words    : {}", word_count(&db, input));
    println!("summary  : {}", summary(&db, input));
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main();
        Ok(())
    }
}
