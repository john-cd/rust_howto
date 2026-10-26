#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Example of using the Tera template engine.

use std::sync::LazyLock;

use anyhow::bail;
use serde::Serialize;

/// The templates, which are loaded and configured on first use.
static TEMPLATES: LazyLock<tera::TeraResult<tera::Tera>> =
    LazyLock::new(|| {
        let manifest_dir = env!("CARGO_MANIFEST_DIR").replace('\\', "/");
        let template_glob = format!("{manifest_dir}/examples/tera/templates/*");
        let mut t = tera::Tera::new();
        t.load_from_glob(&template_glob)?;
        // Configure the Tera template engine.
        // By default, Tera will auto-escape all content in files ending with
        // ".html", ".htm" and ".xml".
        // Change that with `autoescape_on`:
        // t.autoescape_on(vec![".html"]);
        // Also add filters here, for example:
        t.register_filter("underscored", underscored);
        Ok(t)
    });

/// Filters are functions with the
/// `fn(Value, Kwargs, &State) -> Result` definition (the value may be any
/// type that can be extracted from a Tera `Value`, such as `&str`), which can
/// be used within templates.
///
/// The following filter replaces dashes by underscores.
pub fn underscored(
    val: &str,
    _kwargs: tera::Kwargs,
    _state: &tera::State,
) -> String {
    val.replace('-', "_")
}
/// A simple page struct to be used as context for the template.
#[derive(Serialize, Debug)]
struct Page {
    /// The title of the page.
    title: String,
    /// The heading of the page.
    heading: String,
    /// The content of the page.
    content: String,
}

/// Crate metadata used to render a Markdown badge like the book tooling emits.
#[derive(Serialize)]
struct CrateBadge {
    /// The crates.io package name.
    crate_name: String,
}

/// Renders a template with values from a context and prints the result.
fn main() -> anyhow::Result<()> {
    // Initialize the Tera template engine.
    let templ: &tera::TeraResult<tera::Tera> = &TEMPLATES;
    if let Err(e) = templ {
        bail!("The Tera template engine failed to initialize: {e}");
    }

    // Create a context with values for the placeholders.
    let page = Page {
        title: "Tera Example".to_owned(),
        heading: "Welcome to Tera".to_owned(),
        content: "This is a simple example of using the Tera template engine."
            .to_owned(),
    };

    let context = tera::Context::from_serialize(&page)?;

    // Render the template with the context:
    let tera = (*TEMPLATES).as_ref()?;
    let rendered = tera.render("index.html", &context)?;

    // Print the rendered template:
    println!("{rendered}");

    // Tera can also render Markdown fragments from the same data model.
    let badge = CrateBadge {
        crate_name: "serde".to_owned(),
    };
    let badge_context = tera::Context::from_serialize(&badge)?;
    let rendered_badge = tera.render("crate_badge.md", &badge_context)?;
    println!("{rendered_badge}");

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
