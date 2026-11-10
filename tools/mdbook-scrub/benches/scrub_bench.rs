//! Criterion benchmark of the `mdbook-scrub` pre-link stage over synthetic chapters.
//!
//! TODO Reviewed ??-??-2026.

#![expect(
    clippy::expect_used,
    reason = "Benchmark fixtures treat preprocessing failures as fatal"
)]

use std::hint::black_box;

use criterion::BatchSize;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use criterion::criterion_group;
use criterion::criterion_main;
use mdbook_preprocessor::Preprocessor as _;
use mdbook_preprocessor::PreprocessorContext;
use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::book::Chapter;
use mdbook_preprocessor::config::Config;
use mdbook_scrub::Preproc;

/// Builds a book of `chapters` identical chapters, each with hidden content.
fn synthetic_book(chapters: usize) -> Book {
    let body = "Visible paragraph with `code`.\n\n<div class=\"hidden\">hidden</div>\n\n{{#include _hidden.rs}}\n\n```rust\nfn main() {}\n```\n\n"
        .repeat(20);
    let mut book = Book::new();
    for index in 0..chapters {
        book.push_item(Chapter::new(
            &format!("Chapter {index}"),
            body.clone(),
            format!("chapter_{index}.md"),
            Vec::new(),
        ));
    }
    book
}

/// Benchmarks the scrub call for several book sizes after checking its output.
///
/// Cloning the owned input book is setup work and remains outside the timed routine.
fn bench_scrub_stage(criterion: &mut Criterion) {
    let context = PreprocessorContext::new(".".into(), Config::default(), "html".into());
    let preprocessor = Preproc::new();

    let check = preprocessor
        .run(&context, synthetic_book(1))
        .expect("scrub stage should succeed");
    let content = check
        .chapters()
        .next()
        .expect("book should keep its chapter")
        .content
        .clone();
    assert!(
        !content.contains("hidden</div>") && !content.contains("_hidden.rs"),
        "scrub stage must remove hidden content"
    );
    assert!(
        content.contains("Visible paragraph"),
        "scrub stage should preserve visible prose"
    );
    assert!(
        content.contains("fn main()"),
        "scrub stage should preserve Rust code fences"
    );

    let mut group = criterion.benchmark_group("scrub_stage");
    for chapters in [1_usize, 10, 100] {
        let book = synthetic_book(chapters);
        group.throughput(Throughput::Elements(
            u64::try_from(chapters).unwrap_or(u64::MAX),
        ));
        group.bench_with_input(
            BenchmarkId::from_parameter(chapters),
            &book,
            |bencher, book| {
                bencher.iter_batched(
                    || book.clone(),
                    |input| {
                        preprocessor
                            .run(&context, black_box(input))
                            .expect("scrub stage should succeed")
                    },
                    BatchSize::LargeInput,
                );
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_scrub_stage);
criterion_main!(benches);
