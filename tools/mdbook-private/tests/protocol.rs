// TODO Reviewed ??-??-2026
use std::io::Write;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;

use mdbook_preprocessor::PreprocessorContext;
use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::book::BookItem;
use mdbook_preprocessor::book::Chapter;
use mdbook_preprocessor::book::SectionNumber;
use mdbook_preprocessor::config::Config;
use serde_json::json;

fn chapter(name: &str, path: &str, number: Option<Vec<u32>>) -> Chapter {
    let mut chapter = Chapter::new(
        name,
        "Visible\n<!--private SECRET\n-->\nEnd".into(),
        path,
        vec![],
    );
    chapter.number = number.map(SectionNumber::new);
    chapter
}

fn invoke(settings: serde_json::Value, book: Book) -> Output {
    let mut config = Config::default();
    config.set("preprocessor.private", settings).unwrap();
    let context = PreprocessorContext::new(".".into(), config, "html".into());
    let mut child = Command::new(env!("CARGO_BIN_EXE_mdbook-private"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(&(context, book)).unwrap())
        .unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn removes_private_subtrees_and_renumbers_visible_chapters() {
    let mut public = chapter("Public", "public.md", Some(vec![2]));
    public.sub_items = vec![
        chapter("Hidden child", "dir/_child.md", Some(vec![2, 1])).into(),
        chapter("Public child", "child.md", Some(vec![2, 2])).into(),
    ];
    let draft = Chapter {
        name: "Draft".into(),
        ..Chapter::default()
    };
    let mut book = Book::new();
    book.push_item(BookItem::PartTitle("Part".into()));
    book.push_item(chapter("Hidden", "_hidden.md", Some(vec![1])));
    book.push_item(public);
    book.push_item(draft);
    let output = invoke(json!({"remove":true}), book);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let book: Book = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(book.items.len(), 3);
    let BookItem::Chapter(public) = &book.items[1] else {
        panic!("public chapter missing")
    };
    assert_eq!(public.number, Some(SectionNumber::new(vec![1])));
    assert!(!public.content.contains("SECRET"));
    assert_eq!(public.sub_items.len(), 1);
    let BookItem::Chapter(child) = &public.sub_items[0] else {
        panic!("child missing")
    };
    assert_eq!(child.number, Some(SectionNumber::new(vec![1, 1])));
    assert!(!child.content.contains("SECRET"));
    let BookItem::Chapter(draft) = &book.items[2] else {
        panic!("draft missing")
    };
    assert_eq!(draft.name, "Draft");
}

#[test]
fn retains_and_styles_private_content_when_requested() {
    let mut book = Book::new();
    book.push_item(chapter("Private", "_private.md", None));
    let output = invoke(json!({"notice":"WORK IN PROGRESS"}), book);
    assert!(output.status.success());
    let book: Book = serde_json::from_slice(&output.stdout).unwrap();
    let BookItem::Chapter(chapter) = &book.items[0] else {
        panic!("chapter missing")
    };
    assert!(chapter.content.contains("<blockquote"));
    assert!(chapter.content.contains("WORK IN PROGRESS"));
    assert!(chapter.content.contains("SECRET"));
    assert!(!chapter.content.contains("<!--private"));
}

#[test]
fn supports_custom_prefix_and_unstyled_retention() {
    let mut book = Book::new();
    book.push_item(chapter("Hidden", "internal-hidden.md", None));
    book.push_item(chapter("Public", "_public.md", None));
    let output = invoke(
        json!({"remove":true,"chapter-prefix":"internal-"}),
        book.clone(),
    );
    assert!(output.status.success());
    let filtered: Book = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(filtered.items.len(), 1);
    let output = invoke(json!({"remove":false,"style":false}), book);
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("SECRET"));
    assert!(!text.contains("blockquote"));
}

#[test]
fn invalid_privacy_configuration_fails_without_emitting_a_book() {
    let output = invoke(json!({"remove":"true"}), Book::new());
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("preprocessor.private"));
}

#[test]
fn renderer_support_uses_only_exit_status() {
    for (renderer, supported) in [("html", true), ("not-supported", false)] {
        let output = Command::new(env!("CARGO_BIN_EXE_mdbook-private"))
            .args(["supports", renderer])
            .output()
            .unwrap();
        assert_eq!(output.status.success(), supported);
        assert!(output.stdout.is_empty());
    }
}
