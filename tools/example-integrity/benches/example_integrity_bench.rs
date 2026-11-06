//! Criterion benchmarks for chapter, anchor, and module inventory traversal in `example-integrity`.
//!
//! Measures:
//! - Full repository report building via `build_report` on synthetic book trees.
//! - Chapter parsing and include directive extraction across chapters and nested fragments.
//! - Named anchor regex validation and file scanning.
//! - Rust module graph traversal and conditional configuration attribute parsing.

use std::fs;
use std::hint::black_box;
use std::path::Path;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use example_integrity::build_report;
use tempfile::TempDir;

/// Helper to write files into a temporary directory tree.
fn write_fixture(root: &Path, path: &str, contents: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().expect("fixture parent directory exists"))
        .expect("creates fixture directory");
    fs::write(path, contents).expect("writes fixture file");
}

/// Creates a synthetic large book repository tree with multiple packages, chapters,
/// nested fragments, anchors, and module hierarchies.
fn create_synthetic_book_tree(num_packages: usize, num_chapters: usize) -> TempDir {
    let dir = TempDir::new().expect("creates temp dir");
    let root = dir.path();

    let members = (0..num_packages)
        .map(|i| format!("\"pkg_{i}\""))
        .collect::<Vec<_>>()
        .join(", ");
    write_fixture(
        root,
        "bk/crates/Cargo.toml",
        &format!("[workspace]\nresolver = \"3\"\nmembers = [{members}]\n"),
    );

    for i in 0..num_packages {
        write_fixture(
            root,
            &format!("bk/crates/pkg_{i}/Cargo.toml"),
            &format!(
                "[package]\nname = \"pkg_{i}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[[example]]\nname = \"demo_{i}\"\npath = \"examples/demo_{i}/main.rs\"\ntest = true\n"
            ),
        );
        write_fixture(
            root,
            &format!("bk/crates/pkg_{i}/examples/demo_{i}/main.rs"),
            "// ANCHOR: example\n#[cfg(feature = \"extra\")]\nmod sub;\nfn main() {}\n// ANCHOR_END: example\n",
        );
        write_fixture(
            root,
            &format!("bk/crates/pkg_{i}/examples/demo_{i}/sub.rs"),
            "// ANCHOR: sub\npub fn sub() {}\n// ANCHOR_END: sub\n",
        );
    }

    let mut summary = String::from("# Summary\n\n");
    for i in 0..num_chapters {
        summary.push_str(&format!("- [Chapter {i}](chapter_{i}.md)\n"));
    }
    write_fixture(root, "bk/src/SUMMARY.md", &summary);
    write_fixture(root, "bk/drafts/SUMMARY.md", "# Summary\n");
    write_fixture(root, "later/src/SUMMARY.md", "# Summary\n");
    write_fixture(
        root,
        "scripts/example_chapter_exceptions.txt",
        "# Exceptions\n",
    );

    for i in 0..num_chapters {
        let pkg_idx = i.checked_rem(num_packages).unwrap_or(0);
        let content = format!(
            "# Chapter {i}\n\n\
            {{{{#include fragments/fragment_{i}.md}}}}\n\n\
            ```rust\n\
            {{{{#include ../crates/pkg_{pkg_idx}/examples/demo_{pkg_idx}/main.rs:example}}}}\n\
            ```\n"
        );
        write_fixture(root, &format!("bk/src/chapter_{i}.md"), &content);

        let fragment = format!(
            "### Fragment {i}\n\n\
            ```rust\n\
            {{{{#include ../../crates/pkg_{pkg_idx}/examples/demo_{pkg_idx}/sub.rs:sub}}}}\n\
            ```\n"
        );
        write_fixture(
            root,
            &format!("bk/src/fragments/fragment_{i}.md"),
            &fragment,
        );
    }

    dir
}

/// Benchmarks full book report generation across synthetic book trees.
fn benchmark_report_generation(c: &mut Criterion) {
    let fixture_dir = create_synthetic_book_tree(5, 10);
    let mut group = c.benchmark_group("example_integrity_report");

    group.bench_function("build_report_small_tree", |b| {
        b.iter(|| {
            let report = build_report(black_box(fixture_dir.path())).expect("builds report");
            black_box(report);
        });
    });

    group.finish();
}

/// Benchmarks regex directive scanning and visible line filtering for Markdown chapters.
fn benchmark_chapter_directive_scanning(c: &mut Criterion) {
    let sample_chapter = "\
# Concurrency and Parallelism\n\n\
A comprehensive chapter discussing concurrent execution.\n\n\
{{#include fragments/worker_pool.md}}\n\n\
```rust\n\
{{#include ../crates/concurrency/examples/worker_threads/main.rs:example}}\n\
```\n\n\
<!-- Hidden section for draft notes -->\n\
<div class=\"hidden\">\n\
{{#include draft_notes.rs}}\n\
</div>\n\n\
{{#example custom_worker_example}}\n\
Another regular section with some markdown prose.\n";

    let include_directive =
        regex::Regex::new(r"\{\{#(?P<kind>include|rustdoc_include)\s+(?P<target>[^}\s]+)\s*\}\}")
            .expect("include regex compiles");
    let custom_example = regex::Regex::new(r"\{\{#example\s+(?P<name>[^}\s]+)\s*\}\}")
        .expect("example regex compiles");
    let rust_fence = regex::Regex::new(r"^\s*(`{3,}|~{3,})\s*(?P<language>[^\s`]*)")
        .expect("fence regex compiles");

    let mut group = c.benchmark_group("chapter_directive_scanning");

    group.bench_function("scan_directives", |b| {
        b.iter(|| {
            let text = black_box(sample_chapter);
            let mut directives = Vec::new();
            for line in text.lines() {
                for capture in include_directive.captures_iter(line) {
                    directives.push(capture["target"].to_owned());
                }
                for capture in custom_example.captures_iter(line) {
                    directives.push(capture["name"].to_owned());
                }
                let _fence = rust_fence.captures(line);
            }
            black_box(directives);
        });
    });

    group.finish();
}

/// Benchmarks regex anchor extraction and verification against source files.
fn benchmark_anchor_extraction(c: &mut Criterion) {
    let source_text = "\
// Module documentation header.
use std::sync::Arc;

// ANCHOR: setup
fn initialize() -> Arc<String> {
    Arc::new(\"state\".to_owned())
}
// ANCHOR_END: setup

// ANCHOR: run
fn execute() {
    println!(\"running\");
}
// ANCHOR_END: run

// ANCHOR: teardown
fn cleanup() {}
// ANCHOR_END: teardown
";

    let mut group = c.benchmark_group("anchor_extraction");

    group.bench_function("scan_anchors", |b| {
        b.iter(|| {
            let source = black_box(source_text);
            let mut anchors = Vec::new();
            for line in source.lines() {
                if let Some((_, anchor)) = line.split_once("ANCHOR: ") {
                    anchors.push(anchor.trim().to_owned());
                }
            }
            black_box(anchors);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_report_generation,
    benchmark_chapter_directive_scanning,
    benchmark_anchor_extraction
);
criterion_main!(benches);

#[cfg(test)]
mod tests {
    /// Runs the example_integrity benchmark entry point under test harness.
    #[test]
    fn test_example_integrity_bench_entry_point() {
        let mut criterion = criterion::Criterion::default().configure_from_args();
        super::benchmark_report_generation(&mut criterion);
        super::benchmark_chapter_directive_scanning(&mut criterion);
        super::benchmark_anchor_extraction(&mut criterion);
    }
}
