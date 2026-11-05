//! Criterion benchmarks for the Markdown generation and link extraction helpers.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::hint::black_box;

use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use criterion::criterion_group;
use criterion::criterion_main;
use tempfile::TempDir;
use tool_lib::RecipeLink;
use tool_lib::collect_published_recipe_links;
use tool_lib::create_alphabetical_crate_page_section;
use tool_lib::create_category_badge;
use tool_lib::create_index_anchors;
use tool_lib::create_rbe_badge;
use tool_lib::detect_github_url;
use tool_lib::normalize_docs_url;
use tool_lib::process_crate_block_directives;
use tool_lib::remove_empty_lines;

/// Builds a book source tree whose recipe index includes `fragments` fragments of `rows` rows each.
fn recipe_book(fragments: usize, rows: usize) -> TempDir {
    let directory = tempfile::tempdir().expect("temporary directory is available");
    let indices = directory.path().join("src/indices");
    let recipes = directory.path().join("src/recipes");
    fs::create_dir_all(&indices).expect("directory is writable");
    fs::create_dir_all(&recipes).expect("directory is writable");

    let mut index = String::new();
    let mut references = String::new();
    for fragment in 0..fragments {
        let mut table = String::from("| Recipe | Crates |\n| --- | --- |\n");
        for row in 0..rows {
            let reference = format!("ex~bench~{fragment}_{row}");
            let name = format!("crate-{}", row.min(24));
            writeln!(
                table,
                "| [Recipe {fragment}.{row}][{reference}] | [![{name}][c~{name}~docs~badge]][c~{name}~docs] {{{{#crate other-{row} testing}}}} |"
            )
            .expect("writing to a string cannot fail");
            writeln!(references, "[{reference}]: ../recipes/r{fragment}.md#{row}")
                .expect("writing to a string cannot fail");
        }
        fs::write(recipes.join(format!("r{fragment}.incl.md")), table).expect("file is writable");
        writeln!(index, "{{{{#include ../recipes/r{fragment}.incl.md}}}}")
            .expect("writing to a string cannot fail");
    }
    writeln!(index, "{{{{#include refs.incl.md}}}}").expect("writing to a string cannot fail");
    fs::write(indices.join("examples_index.md"), index).expect("file is writable");
    fs::write(indices.join("refs.incl.md"), references).expect("file is writable");
    directory
}

/// Measures link extraction over published recipe tables of growing size.
fn bench_recipe_link_extraction(c: &mut Criterion) {
    let mut group = c.benchmark_group("published_recipes");
    for (fragments, rows) in [(2, 25), (10, 50)] {
        let book = recipe_book(fragments, rows);
        group.throughput(Throughput::Elements(
            u64::try_from(fragments * rows).expect("small count"),
        ));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{fragments}x{rows}")),
            book.path(),
            |b, path| {
                b.iter(|| black_box(collect_published_recipe_links(path).expect("book is valid")));
            },
        );
    }
    group.finish();
}

/// Measures the directive scan that runs before any network request is needed.
fn bench_crate_block_scanning(c: &mut Criterion) {
    let mut group = c.benchmark_group("crate_block_parsing");
    for paragraphs in [10_usize, 1_000] {
        let content =
            "A paragraph with `code`, a [link](https://example.com), and {{#other directive}}.\n\n"
                .repeat(paragraphs);
        group.throughput(Throughput::Bytes(
            u64::try_from(content.len()).expect("small size"),
        ));
        group.bench_with_input(
            BenchmarkId::from_parameter(paragraphs),
            &content,
            |b, content| {
                b.iter(|| {
                    black_box(
                        process_crate_block_directives(content).expect("no crates are fetched"),
                    )
                });
            },
        );
    }
    group.finish();
}

/// Measures markdown template generation and small text helpers.
fn bench_markdown_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("markdown_templates");
    group.bench_function("category_badge", |b| {
        b.iter(|| {
            black_box(create_category_badge("Command-line interface", "cli").expect("renders"))
        });
    });
    group.bench_function("rbe_badge", |b| {
        b.iter(|| black_box(create_rbe_badge("attributes").expect("renders")));
    });
    let keywords: Vec<String> = (0..200).map(|i| format!("keyword-{i}")).collect();
    group.bench_function("index_anchors_200", |b| {
        b.iter(|| {
            let refs: Vec<&str> = keywords.iter().map(String::as_str).collect();
            black_box(create_index_anchors(refs).expect("renders"))
        });
    });

    let names: Vec<String> = (0..200).map(|i| format!("crate-{i}")).collect();
    let recipes: BTreeMap<String, Vec<RecipeLink>> = names
        .iter()
        .map(|name| {
            (
                name.clone(),
                vec![RecipeLink {
                    title: "A recipe".to_owned(),
                    reference: "ex~bench~one".to_owned(),
                }],
            )
        })
        .collect();
    group.bench_function("alphabetical_section_200", |b| {
        b.iter(|| {
            let refs: Vec<&str> = names.iter().map(String::as_str).collect();
            black_box(create_alphabetical_crate_page_section("C", refs, &recipes).expect("renders"))
        });
    });

    let text = "line\n\n\nanother line\n\n".repeat(2_000);
    group.throughput(Throughput::Bytes(
        u64::try_from(text.len()).expect("small size"),
    ));
    group.bench_function("remove_empty_lines", |b| {
        b.iter(|| black_box(remove_empty_lines(&text)));
    });
    group.finish();

    c.bench_function("url_helpers", |b| {
        b.iter(|| {
            black_box(detect_github_url("https://github.com/john-cd/rust_howto"));
            black_box(normalize_docs_url("https://docs.rs/serde/latest/serde/"));
        });
    });
}

criterion_group!(
    benches,
    bench_recipe_link_extraction,
    bench_crate_block_scanning,
    bench_markdown_generation
);
criterion_main!(benches);
