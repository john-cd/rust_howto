use mdbook_driver::MDBook;

#[test]
fn mdbook_05_preprocessor_scrubs_book_content() {
    let book = MDBook::load("test_book").unwrap();
    book.build().unwrap();
    let ch1 = std::fs::read_to_string("test_book/book/html/hidden.html").unwrap();
    assert!(!ch1.contains(r#"<div class="hidden">"#));
    assert!(!ch1.contains(r"{#include"));
    assert!(!ch1.contains(r"{#rustdoc_include"));
    assert!(!ch1.contains(r"{#playground"));

    let examples = std::fs::read_to_string("test_book/book/html/examples.html").unwrap();
    assert!(!examples.contains("{{#example"));

    let wikilinks = std::fs::read_to_string("test_book/book/html/wikilinks.html").unwrap();
    assert!(!wikilinks.contains("[["));
}
