//! Index anchors for `mdbook-indexing`.
//!
//! TODO Reviewed ??-??-2026

use anyhow::Result;
use serde::Serialize;

pub(super) static INDEX_ANCHORS: &str =
    r"{{ for keyword in keywords }}\{\{hi:{keyword}}}{{ endfor }} ";

/// Generates a string of index anchors for use with `mdbook-indexing`.
///
/// This function takes a vector of keywords and returns a string in the format:
/// `{{hi:keyword1}}{{hi:keyword2}}...`. This format is a pseudo-markdown syntax
/// recognized by the `mdbook-indexing` preprocessor.
pub fn create_index_anchors(keywords: Vec<&str>) -> Result<String> {
    #[derive(Serialize)]
    struct Context<'a> {
        keywords: Vec<&'a str>,
    }
    let tt = super::get_template_engine()?;
    let context = Context { keywords };
    let rendered = tt.render("INDEX_ANCHORS", &context)?;

    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_index_anchors_multiple_keywords() {
        let keywords = vec!["serde", "json", "parsing"];
        let rendered = create_index_anchors(keywords).unwrap();
        assert_eq!(rendered, "{{hi:serde}}{{hi:json}}{{hi:parsing}} ");
    }

    #[test]
    fn test_create_index_anchors_single_keyword() {
        let keywords = vec!["async"];
        let rendered = create_index_anchors(keywords).unwrap();
        assert_eq!(rendered, "{{hi:async}} ");
    }

    #[test]
    fn test_create_index_anchors_empty() {
        let keywords = vec![];
        let rendered = create_index_anchors(keywords).unwrap();
        assert_eq!(rendered, " ");
    }

    #[test]
    fn test_create_index_anchors_special_characters() {
        let keywords = vec!["tokio::spawn", "hello world"];
        let rendered = create_index_anchors(keywords).unwrap();
        assert_eq!(rendered, "{{hi:tokio::spawn}}{{hi:hello world}} ");
    }
}
