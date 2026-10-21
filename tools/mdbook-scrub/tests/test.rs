use std::io::Write;
use std::process::Command;
use std::process::Stdio;

use mdbook_driver::MDBook;
use mdbook_preprocessor::PreprocessorContext;
use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::book::Chapter;

#[test]
fn mdbook_05_preprocessors_expand_directives_after_native_includes() {
    let book = MDBook::load("test_book").unwrap();
    book.build().unwrap();
    let ch1 = std::fs::read_to_string("test_book/book/html/hidden.html").unwrap();
    assert!(!ch1.contains(r#"<div class="hidden">"#));
    assert!(!ch1.contains(r"{#include"));
    assert!(!ch1.contains(r"{#rustdoc_include"));
    assert!(!ch1.contains(r"{#playground"));

    let examples = std::fs::read_to_string("test_book/book/html/language/examples.html").unwrap();
    assert!(!examples.contains(
        "{{#example ../../crates/language/examples/constants_and_statics/statics.rs:example}}"
    ));
    assert!(examples.contains("SAMPLE_VALUE"));
    assert!(examples.contains("42"));
    assert!(!examples.contains("This setup code must not be included"));
    assert!(examples.contains("Included crate directive"));
    assert!(examples.contains("{{#crate untouched}}"));
    assert!(examples.contains("{{#example untouched.rs:example}}"));
    assert!(examples.contains("{{#include untouched.md}}"));
    assert!(!examples.contains("{{#include directive_included.md}}"));

    let export = std::fs::read_to_string("test_book/book/markdown/language/examples.md").unwrap();
    assert!(export.contains("pub static SAMPLE_VALUE: u8 = 42;"));
    assert!(export.contains("Included crate directive"));
    assert!(!export.contains("{{#include ../directive_included.md}}"));
    assert!(export.contains("{{#example untouched.rs:example}}"));

    let crate_blocks = std::fs::read_to_string("test_book/book/html/crate_blocks.html").unwrap();
    assert!(!crate_blocks.contains("{{#crate"));
    assert!(crate_blocks.contains("cat1"));
    assert!(crate_blocks.contains("Badge Migration Fixture"));
    assert!(crate_blocks.contains("Sort a Vector of Integers"));
    assert!(crate_blocks.contains("sorting.html#sort-vector-integers"));
    assert!(crate_blocks.contains("https://doc.rust-lang.org/std"));
    assert!(crate_blocks.contains("https://img.shields.io/badge/std-firebrick?logo=rust"));
    assert!(crate_blocks.contains("https://crates.io/categories/algorithms"));
    assert!(crate_blocks.contains("Algorithms"));

    let wikilinks = std::fs::read_to_string("test_book/book/html/wikilinks.html").unwrap();
    assert!(!wikilinks.contains("[["));
}

#[test]
fn preprocessor_protocol_keeps_json_on_stdout() {
    let root = std::path::Path::new("test_book").canonicalize().unwrap();
    let config = mdbook_preprocessor::config::Config::from_disk(root.join("book.toml")).unwrap();
    let context = PreprocessorContext::new(root, config, "html".into());
    let mut book = Book::new();
    book.push_item(Chapter::new(
        "Example",
        "{{#example ../../crates/language/examples/constants_and_statics/statics.rs:example}}"
            .into(),
        "language/examples.md",
        Vec::new(),
    ));
    let input = serde_json::to_vec(&(&context, &book)).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_mdbook-scrub"))
        .args(["--stage", "directives"])
        .current_dir("test_book")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(&input).unwrap();
    let output = child.wait_with_output().unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    let processed: Book = serde_json::from_slice(&output.stdout).unwrap();
    assert!(
        processed
            .chapters()
            .next()
            .unwrap()
            .content
            .contains("SAMPLE_VALUE")
    );
}
