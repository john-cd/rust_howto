//! End-to-end tests of the `mdbook-scrub` preprocessor.
//!
//! TODO Reviewed ??-??-2026.

use std::io::Write as _;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;
use std::str::FromStr as _;

use mdbook_driver::MDBook;
use mdbook_preprocessor::Preprocessor as _;
use mdbook_preprocessor::PreprocessorContext;
use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::book::Chapter;
use mdbook_scrub::Preproc;
use tempfile::tempdir;

/// Runs the directive preprocessor through mdBook's JSON protocol.
fn run_directive_protocol(context: PreprocessorContext, book: Book) -> Output {
    let input = serde_json::to_vec(&(&context, &book))
        .expect("the preprocessor context and book should serialize");
    let mut child = Command::new(env!("CARGO_BIN_EXE_mdbook-scrub"))
        .args(["--stage", "directives"])
        .current_dir("test_book")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the mdbook-scrub executable should start");
    child
        .stdin
        .take()
        .expect("the child should accept protocol input")
        .write_all(&input)
        .expect("the protocol input should be written");

    child
        .wait_with_output()
        .expect("the mdbook-scrub process should finish")
}

/// Builds the test book and checks native and custom directive output.
#[test]
fn mdbook_05_preprocessors_expand_directives_after_native_includes() {
    let book = MDBook::load("test_book").expect("input should parse");
    book.build().expect("input should parse");
    let ch1 =
        std::fs::read_to_string("test_book/book/html/hidden.html").expect("input should parse");
    assert!(!ch1.contains(r#"<div class="hidden">"#));
    assert!(!ch1.contains("{#include"));
    assert!(!ch1.contains("{#rustdoc_include"));
    assert!(!ch1.contains("{#playground"));

    let examples = std::fs::read_to_string("test_book/book/html/language/examples.html")
        .expect("input should parse");
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

    let export = std::fs::read_to_string("test_book/book/markdown/language/examples.md")
        .expect("input should parse");
    assert!(export.contains("pub static SAMPLE_VALUE: u8 = 42;"));
    assert!(export.contains("Included crate directive"));
    assert!(!export.contains("{{#include ../directive_included.md}}"));
    assert!(export.contains("{{#example untouched.rs:example}}"));

    let crate_blocks = std::fs::read_to_string("test_book/book/html/crate_blocks.html")
        .expect("input should parse");
    assert!(!crate_blocks.contains("{{#crate"));
    assert!(crate_blocks.contains("cat1"));
    assert!(crate_blocks.contains("Badge Migration Fixture"));
    assert!(crate_blocks.contains("Sort a Vector of Integers"));
    assert!(crate_blocks.contains("sorting.html#sort-vector-integers"));
    assert!(crate_blocks.contains("https://doc.rust-lang.org/std"));
    assert!(crate_blocks.contains("https://img.shields.io/badge/std-firebrick?logo=rust"));
    assert!(crate_blocks.contains("https://crates.io/categories/algorithms"));
    assert!(crate_blocks.contains("Algorithms"));

    let wikilinks =
        std::fs::read_to_string("test_book/book/html/wikilinks.html").expect("input should parse");
    assert!(!wikilinks.contains("[["));
}

/// Reports supported and unsupported renderers through the executable CLI.
#[test]
fn supports_renderer_subcommand_matches_mdbook_protocol() {
    let supported = Command::new(env!("CARGO_BIN_EXE_mdbook-scrub"))
        .args(["--stage", "directives", "supports", "html"])
        .output()
        .expect("the mdbook-scrub executable should start");
    assert!(supported.status.success());

    let unsupported = Command::new(env!("CARGO_BIN_EXE_mdbook-scrub"))
        .args(["--stage", "directives", "supports", "not-supported"])
        .output()
        .expect("the mdbook-scrub executable should start");
    assert!(!unsupported.status.success());
}

/// Lets renderer capability checks succeed when the log destination is unavailable.
#[test]
fn supports_renderer_does_not_require_a_writable_log_destination() {
    let directory = tempdir().expect("a temporary directory should be available");
    let log_path = directory.path().join("mdbook-scrub.log");
    std::fs::create_dir(&log_path).expect("a directory should block the log file path");

    let output = Command::new(env!("CARGO_BIN_EXE_mdbook-scrub"))
        .args(["supports", "html"])
        .current_dir(directory.path())
        .output()
        .expect("the mdbook-scrub executable should start");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
    assert!(log_path.is_dir());
}

/// Processes valid stdin when the log destination is unavailable and keeps JSON on stdout.
#[test]
fn preprocessing_falls_back_to_stderr_when_log_destination_is_unavailable() {
    let directory = tempdir().expect("a temporary directory should be available");
    let log_path = directory.path().join("mdbook-scrub.log");
    std::fs::create_dir(&log_path).expect("a directory should block the log file path");

    let root = std::path::Path::new("test_book")
        .canonicalize()
        .expect("the test book root should exist");
    let config = mdbook_preprocessor::config::Config::from_disk(root.join("book.toml"))
        .expect("the test book configuration should parse");
    let context = PreprocessorContext::new(root, config, "html".into());
    let mut book = Book::new();
    book.push_item(Chapter::new(
        "Example",
        "Valid book content".into(),
        "example.md",
        Vec::new(),
    ));
    let input = serde_json::to_vec(&(&context, &book))
        .expect("the preprocessor context and book should serialize");

    let mut child = Command::new(env!("CARGO_BIN_EXE_mdbook-scrub"))
        .current_dir(directory.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the mdbook-scrub executable should start");
    child
        .stdin
        .take()
        .expect("the child should accept protocol input")
        .write_all(&input)
        .expect("the protocol input should be written");
    let output = child
        .wait_with_output()
        .expect("the mdbook-scrub process should finish");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let processed: Book = serde_json::from_slice(&output.stdout)
        .expect("preprocessor stdout should contain only book JSON");
    assert_eq!(
        processed
            .chapters()
            .next()
            .expect("the processed book should retain its chapter")
            .content,
        "Valid book content"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("could not open mdbook-scrub.log"),
        "{stderr}"
    );
    assert!(stderr.contains("falling back to stderr"), "{stderr}");
}

/// Keeps JSON on stdout while expanding a valid Rust example directive.
#[test]
fn preprocessor_protocol_keeps_json_on_stdout() {
    let root = std::path::Path::new("test_book")
        .canonicalize()
        .expect("input should parse");
    let config = mdbook_preprocessor::config::Config::from_disk(root.join("book.toml"))
        .expect("input should parse");
    let context = PreprocessorContext::new(root, config, "html".into());
    let mut book = Book::new();
    book.push_item(Chapter::new(
        "Example",
        "{{#example ../../crates/language/examples/constants_and_statics/statics.rs:example}}"
            .into(),
        "language/examples.md",
        Vec::new(),
    ));
    let output = run_directive_protocol(context, book);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    let processed: Book = serde_json::from_slice(&output.stdout).expect("input should parse");
    assert!(
        processed
            .chapters()
            .next()
            .expect("input should parse")
            .content
            .contains("SAMPLE_VALUE")
    );
}

/// Fails with a useful diagnostic when directive configuration has the wrong type.
#[test]
fn malformed_directive_configuration_fails_through_the_protocol() {
    let root = std::path::Path::new("test_book")
        .canonicalize()
        .expect("the test book root should exist");
    let config = mdbook_preprocessor::config::Config::from_str(
        "[preprocessor.directives]\nprocess-crate-block-directives = \"yes\"\n",
    )
    .expect("the malformed directive value should still be valid TOML");
    let context = PreprocessorContext::new(root, config, "html".into());
    let book = Book::new();
    let output = run_directive_protocol(context, book);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to deserialize"), "{stderr}");
}

/// Rejects directives left unresolved when their processing is disabled.
#[test]
fn disabled_directive_configuration_fails_through_the_protocol() {
    let root = std::path::Path::new("test_book")
        .canonicalize()
        .expect("the test book root should exist");
    let config = mdbook_preprocessor::config::Config::from_str(
        "[preprocessor.directives]\nprocess-crate-block-directives = false\n",
    )
    .expect("the disabled directive configuration should parse");
    let context = PreprocessorContext::new(root, config, "html".into());
    let mut book = Book::new();
    book.push_item(Chapter::new(
        "Crate Blocks",
        "{{#crate sample}}".into(),
        "crate_blocks.md",
        Vec::new(),
    ));
    let output = run_directive_protocol(context, book);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Failed to process custom directives"),
        "{stderr}"
    );
}

/// Leaves Rust code spans and already expanded directives unchanged on a second pass.
#[test]
fn directive_processing_is_idempotent_and_preserves_rust_snippets() {
    let root = std::path::Path::new("test_book")
        .canonicalize()
        .expect("input should parse");
    let config = mdbook_preprocessor::config::Config::from_disk(root.join("book.toml"))
        .expect("input should parse");
    let context = PreprocessorContext::new(root.clone(), config, "html".into());
    let mut content =
        std::fs::read_to_string(root.join("src/crate_blocks.md")).expect("input should parse");
    let rust_snippet = "```rust\nlet directive = \"{{#crate untouched}}\";\n```";
    content.push('\n');
    content.push_str(rust_snippet);

    let mut book = Book::new();
    book.push_item(Chapter::new(
        "Crate Blocks",
        content,
        "crate_blocks.md",
        Vec::new(),
    ));

    let preprocessor = Preproc::directives();
    let once = preprocessor
        .run(&context, book)
        .expect("input should parse");
    let once_content = once
        .chapters()
        .next()
        .expect("input should parse")
        .content
        .clone();
    assert!(once_content.contains(rust_snippet));

    let twice = preprocessor
        .run(&context, once)
        .expect("input should parse");
    let twice_content = twice
        .chapters()
        .next()
        .expect("input should parse")
        .content
        .clone();
    assert_eq!(once_content, twice_content);
}
