use anyhow::Result;
use serde::Serialize;

pub(super) static CATEGORY_ROW: &str = r"| [![cat~{slug}][cat~{slug}~badge]][cat~{slug}]\{\{hi:{category}}} | {description} | {{ for name in crate_names }}[![{name}][c~{name}~docs~badge]][c~{name}~docs]\{\{hi:{name}}} {{ endfor }}|";

/// Context for rendering a category row in the crates by category table.
///
/// category: category name e.g. Encoding
/// slug: category slug used by the crates.io website e.g. encoding
/// description: description of the category
/// crate_names: list of the names of the crates to include in the category (per
/// lib.rs)
#[derive(Serialize)]
struct Context<'a> {
    category: &'a str,
    slug: &'a str,
    description: &'a str,
    crate_names: Vec<&'a str>,
}

/// Create_category_and_crates
/// Returns one line in the "crates organized by category" table:
/// | <category badge> | <description> | <crate badge> <crate badge>... |
///
/// category: category name e.g. Encoding
/// slug: category slug used by the crates.io website e.g. encoding
/// description: description of the category
/// crate_names: list of the names of the crates to include in the category (per
/// lib.rs)
pub fn create_category_and_crates(
    category: &str,
    slug: &str,
    description: &str,
    crate_names: Vec<&str>,
) -> Result<String> {
    let tt = super::get_template_engine()?;
    let context = Context {
        category: category.trim(),
        slug: slug.trim(),
        description: description.trim(),
        crate_names,
    };
    let rendered = tt.render("CATEGORY_ROW", &context)?;
    Ok(rendered)
}
