//! Index anchors for `mdbook-indexing`.
//!
//! TODO Reviewed ??-??-2026

use anyhow::Result;
use serde::Serialize;

/// Template of the index anchors.
pub(super) static INDEX_ANCHORS: &str =
    r"{{ for keyword in keywords }}\{\{hi:{keyword}}}{{ endfor }} ";

/// Generates a string of index anchors for use with `mdbook-indexing`.
///
/// This function takes a vector of keywords and returns a string in the format:
/// `{{hi:keyword1}}{{hi:keyword2}}...`. This format is a pseudo-markdown syntax
/// recognized by the `mdbook-indexing` preprocessor.
///
/// # Arguments
///
/// * `keywords` - Keywords to render, in the order they should appear.
///
/// # Errors
///
/// Returns an error if the index-anchor template cannot be loaded or rendered.
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

    /// Tests `create_index_anchors` with multiple distinct keyword strings.
    #[test]
    fn test_create_index_anchors_multiple_keywords() {
        let keywords = vec!["serde", "json", "parsing"];
        let rendered = create_index_anchors(keywords)
            .expect("the template should render anchors for all supplied keywords");
        assert_eq!(rendered, "{{hi:serde}}{{hi:json}}{{hi:parsing}} ");
    }

    /// Tests `create_index_anchors` with a single keyword string.
    #[test]
    fn test_create_index_anchors_single_keyword() {
        let keywords = vec!["async"];
        let rendered = create_index_anchors(keywords)
            .expect("the template should render an anchor for one keyword");
        assert_eq!(rendered, "{{hi:async}} ");
    }

    /// Tests `create_index_anchors` with an empty vector of keywords.
    #[test]
    fn test_create_index_anchors_empty() {
        let keywords = vec![];
        let rendered = create_index_anchors(keywords)
            .expect("the template should render an empty anchor list");
        assert_eq!(rendered, " ");
    }

    /// Tests `create_index_anchors` with keywords containing special character sequences like colons and spaces.
    #[test]
    fn test_create_index_anchors_special_characters() {
        let keywords = vec!["tokio::spawn", "hello world"];
        let rendered = create_index_anchors(keywords)
            .expect("the template should render keywords containing spaces and colons");
        assert_eq!(rendered, "{{hi:tokio::spawn}}{{hi:hello world}} ");
    }

    /// Tests `create_index_anchors` with an empty string keyword.
    #[test]
    fn test_create_index_anchors_empty_string_keyword() {
        let keywords = vec![""];
        let rendered = create_index_anchors(keywords)
            .expect("the template should render an empty-string keyword");
        assert_eq!(rendered, "{{hi:}} ");
    }

    /// Tests `create_index_anchors` with keywords containing leading, trailing, and tab/newline whitespace.
    #[test]
    fn test_create_index_anchors_whitespace_keywords() {
        let keywords = vec!["  leading", "trailing  ", "\ttab\n"];
        let rendered = create_index_anchors(keywords)
            .expect("the template should render whitespace characters in keywords");
        assert_eq!(rendered, "{{hi:  leading}}{{hi:trailing  }}{{hi:\ttab\n}} ");
    }

    /// Tests `create_index_anchors` with non-ASCII and multibyte Unicode keywords.
    #[test]
    fn test_create_index_anchors_unicode_keywords() {
        let keywords = vec!["🦀", "并发", "éxito"];
        let rendered = create_index_anchors(keywords)
            .expect("the template should render Unicode keyword values");
        assert_eq!(rendered, "{{hi:🦀}}{{hi:并发}}{{hi:éxito}} ");
    }

    /// Tests `create_index_anchors` with keywords containing braces and special characters.
    #[test]
    fn test_create_index_anchors_brace_and_html_characters() {
        let keywords = vec!["{foo}", "bar<baz>", "a & b"];
        let rendered = create_index_anchors(keywords)
            .expect("the template should escape HTML while preserving braces");
        assert_eq!(
            rendered,
            "{{hi:{foo}}}{{hi:bar&lt;baz&gt;}}{{hi:a &amp; b}} "
        );
    }
}
