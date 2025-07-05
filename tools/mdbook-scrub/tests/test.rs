use mdbook::MDBook;

#[test]
fn hidden_sections_and_hidden_files_includes_are_removed() {
    let book = MDBook::load("test_book").unwrap();
    book.build().unwrap();
    let ch1 = std::fs::read_to_string("test_book/book/html/hidden.html").unwrap();
    assert!(!ch1.contains(r#"<div class="hidden">"#));
    assert!(!ch1.contains(r"{#include"));
    assert!(!ch1.contains(r"{#rustdoc_include"));
    assert!(!ch1.contains(r"{#playground"));
}
