//! Criterion benchmarks for chapter parsing, include anchoring, and module graph traversal.

use std::fmt::Write as _;
use std::fs;
use std::hint::black_box;
use std::path::Path;

use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use criterion::criterion_group;
use criterion::criterion_main;
use example_integrity::build_report;
use tempfile::TempDir;

/// Writes a file, creating its parent directories.
fn write(root: &Path, path: &str, contents: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().expect("fixture path has a parent"))
        .expect("directory is writable");
    fs::write(path, contents).expect("file is writable");
}

/// Builds a book tree with `chapters` chapters. Each chapter includes an anchored example whose
/// module tree is `depth` levels deep.
fn book_tree(chapters: usize, depth: usize) -> TempDir {
    let directory = tempfile::tempdir().expect("temporary directory is available");
    let root = directory.path();
    let members: Vec<String> = (0..chapters).map(|i| format!("\"p{i}\"")).collect();
    write(
        root,
        "bk/crates/Cargo.toml",
        &format!(
            "[workspace]\nresolver = \"3\"\nmembers = [{}]\n",
            members.join(", ")
        ),
    );
    let mut summary = String::from("# Summary\n\n");
    for i in 0..chapters {
        write(
            root,
            &format!("bk/crates/p{i}/Cargo.toml"),
            &format!(
                "[package]\nname = \"p{i}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[[example]]\nname = \"demo{i}\"\npath = \"examples/main.rs\"\ntest = true\n"
            ),
        );
        // A chain of nested modules: main.rs -> m0.rs -> m1.rs -> ...
        let mut main = String::from("// ANCHOR: example\nfn main() {}\n// ANCHOR_END: example\n");
        if depth > 0 {
            main.push_str("mod m0;\n");
        }
        write(root, &format!("bk/crates/p{i}/examples/main.rs"), &main);
        for level in 0..depth {
            let next = if level + 1 < depth {
                format!("pub mod m{};\n", level + 1)
            } else {
                String::from("pub fn leaf() {}\n")
            };
            let path = (0..level).fold(String::from("examples"), |acc, l| format!("{acc}/m{l}"));
            write(root, &format!("bk/crates/p{i}/{path}/m{level}.rs"), &next);
        }
        write(
            root,
            &format!("bk/src/chapter{i}.md"),
            &format!(
                "# Chapter {i}\n\n```rust\n{{{{#include ../crates/p{i}/examples/main.rs:example}}}}\n```\n\n{{{{#example demo{i}}}}}\n"
            ),
        );
        writeln!(summary, "- [Chapter {i}](chapter{i}.md)").expect("writing to a string");
    }
    write(root, "bk/src/SUMMARY.md", &summary);
    write(root, "later/src/SUMMARY.md", "# Summary\n");
    write(root, "scripts/example_chapter_exceptions.txt", "# None\n");
    directory
}

/// Measures the full inventory over book trees of increasing size and module depth.
fn bench_build_report(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_report");
    group.sample_size(10);
    for (chapters, depth) in [(5_usize, 2_usize), (50, 2), (50, 8)] {
        let tree = book_tree(chapters, depth);
        group.throughput(Throughput::Elements(
            u64::try_from(chapters).expect("small count"),
        ));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{chapters}ch_depth{depth}")),
            tree.path(),
            |b, root| {
                b.iter(|| black_box(build_report(root).expect("fixture report builds")));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_build_report);
criterion_main!(benches);
