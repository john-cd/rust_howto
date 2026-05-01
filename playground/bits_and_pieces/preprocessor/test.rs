#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn preprocessor_run() {
        let input_json = r##"[
                {
                    "root": "test_book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "pre": {}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.35"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n <div class="hidden">REMOVE</div> {{#include _hidden.rs}} {{#playground _hidden.rs}} {{#rustdoc_include  _hidden.rs:2}}"
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
        let input_json = input_json.as_bytes();

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();

        let result = Preproc::new().run(&ctx, book);
        assert!(result.is_ok());
        let actual_book = result.unwrap();
        while let Some(mdbook::BookItem::Chapter(chapter)) = actual_book.iter().next() {
            assert!(!chapter.content.contains(r#"<div class="hidden">"#));
            assert!(!chapter.content.contains("{#include"));
            assert!(!chapter.content.contains("{#rustdoc_include"));
            assert!(!chapter.content.contains("{#playground"));
        }
    }
}
