use anyhow::Result;
use serde::Serialize;

pub(super) static INDEX_ANCHORS: &str =
    r"{{ for keyword in keywords }}\{\{hi:{keyword}}}{{ endfor }} ";

/// Returns {{hi:keyword1}}{{hi:keyword2}}... pseudo-markdown.
/// For use with `mdbook-indexing`.
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
