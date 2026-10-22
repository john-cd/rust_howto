//! The Rust By Example badge.
//!
//! TODO Reviewed ??-??-2026

use anyhow::Result;
use serde::Serialize;

/// Template of the Rust By Example badge and its reference definitions.
pub(super) static RBE: &str = "
[![Rust by example - {concept}][book~rust-by-example~{concept | underscored}~badge]][book~rust-by-example~{concept | underscored}]

[book~rust-by-example~{concept | underscored}~badge]: https://img.shields.io/badge/Rust_By_Example-{concept | shielded}-violet?logo=mdbook
[book~rust-by-example~{concept | underscored}]: https://doc.rust-lang.org/rust-by-example/{concept | underscored}.html
";

/// Creates a badge that links to the Rust By Example book.
///
/// The badge will be rendered with the given concept, and will link to the
/// corresponding page in the Rust By Example book.
///
/// concept: name of the language element / Rust By Example book
/// chapter e.g. "attributes"
pub fn create_rbe_badge(concept: &str) -> Result<String> {
    #[derive(Serialize)]
    struct RbeContext {
        concept: String,
    }

    let tt = super::get_template_engine()?;
    let context = RbeContext {
        concept: concept.to_owned(),
    };
    let rendered = tt.render("RBE", &context)?;
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_rbe_badge_basic() {
        let rendered = create_rbe_badge("attributes").expect("input should parse");
        let expected = "\n[![Rust by example - attributes][book~rust-by-example~attributes~badge]][book~rust-by-example~attributes]\n\n[book~rust-by-example~attributes~badge]: https://img.shields.io/badge/Rust_By_Example-attributes-violet?logo=mdbook\n[book~rust-by-example~attributes]: https://doc.rust-lang.org/rust-by-example/attributes.html\n";
        assert_eq!(rendered, expected);
    }

    #[test]
    fn test_create_rbe_badge_formatting() {
        // Test concept with hyphens, underscores, and spaces
        let rendered = create_rbe_badge("hello-world_test example").expect("input should parse");
        assert!(rendered.contains("[![Rust by example - hello-world_test example]"));
        assert!(rendered.contains("book~rust-by-example~hello_world_test example"));
        assert!(rendered.contains("https://img.shields.io/badge/Rust_By_Example-hello--world__test_example-violet?logo=mdbook"));
        assert!(
            rendered.contains(
                "https://doc.rust-lang.org/rust-by-example/hello_world_test example.html"
            )
        );
    }

    #[test]
    fn test_create_rbe_badge_empty() {
        let rendered = create_rbe_badge("").expect("input should parse");
        let expected = "\n[![Rust by example - ][book~rust-by-example~~badge]][book~rust-by-example~]\n\n[book~rust-by-example~~badge]: https://img.shields.io/badge/Rust_By_Example--violet?logo=mdbook\n[book~rust-by-example~]: https://doc.rust-lang.org/rust-by-example/.html\n";
        assert_eq!(rendered, expected);
    }
}
