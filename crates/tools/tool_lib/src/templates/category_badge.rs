use anyhow::Result;
use serde::Serialize;

/// Category badge
pub(super) static CAT_BADGE: &str =
    r"[![cat-{slug}][cat-{slug}-badge]][cat-{slug}]\{\{hi:{category}}} ";

/// create_category_badge
///
/// category: category name e.g. Encoding
/// slug: category slug used by the crates.io website e.g. encoding
pub fn create_category_badge(category: &str, slug: &str) -> Result<String> {
    #[derive(Serialize)]
    struct Context<'a> {
        category: &'a str,
        slug: &'a str,
    }
    let tt = super::get_template_engine()?;
    let context = Context {
        category: category.trim(),
        slug: slug.trim(),
    };
    let rendered = tt.render("CAT_BADGE", &context)?;

    Ok(rendered)
}
