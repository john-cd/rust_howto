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
