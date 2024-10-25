use anyhow::Result;
use serde::Serialize;

static CATEGORY: &str = r"| [![cat-{slug}][cat-{slug}-badge]][cat-{slug}]\{\{hi:{category}}} | {description} | {{ for name in crate_names }}[![{name}][c-{name | underscored}-badge]][c-{name | underscored}]\{\{hi:{name | underscored}}} {{ endfor }}|";

#[derive(Serialize)]
struct Context<'a> {
    category: &'a str,
    slug: &'a str,
    description: &'a str,
    crate_names: Vec<&'a str>,
}

/// create_category_and_crates
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
    let mut tt = super::build_template_engine()?;
    tt.add_template("CATEGORY", CATEGORY)?;
    let context = Context {
        category,
        slug,
        description,
        crate_names,
    };
    let rendered = tt.render("CATEGORY", &context)?;
    Ok(rendered)
}
