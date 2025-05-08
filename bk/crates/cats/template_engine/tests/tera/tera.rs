// ANCHOR: example
//! Example of using the Tera template engine.
use std::collections::HashMap;
use std::sync::LazyLock;

use anyhow::bail;
use serde::Serialize;

static TEMPLATES: LazyLock<tera::Result<tera::Tera>> = LazyLock::new(|| {
    tera::Tera::new("tests/tera/templates/*").map(|mut t| {
        // Configure the Tera template engine.
        // By default, Tera will auto-escape all content in files ending with
        // ".html", ".htm" and ".xml".
        // Change that with `autoescape_on`:
        // t.autoescape_on(vec![".html"]);
        // Also add filters here, for example:
        t.register_filter("underscored", underscored);
        t
    })
});

/// Filters are functions with the
/// `fn(Value, HashMap<String, Value>) -> Result<Value>`
/// definition, which can be used within templates.
///
/// The following filter replaces dashes by underscores.
pub fn underscored(
    val: &tera::Value,
    _context: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    match val {
        tera::Value::String(s) => Ok(tera::Value::String(s.replace("-", "_"))),
        _ => Ok(val.to_owned()),
    }
}

/// A simple page struct to be used as context for the template.
#[derive(Serialize, Debug)]
struct Page {
    title: String,
    heading: String,
    content: String,
}

fn main() -> anyhow::Result<()> {
    // Initialize the Tera template engine.
    let templ: &tera::Result<tera::Tera> = &TEMPLATES;
    if let Err(e) = templ {
        bail!("The Tera template engine failed to initialize: {}", e);
    }

    // Create a context with values for the placeholders.
    let page = Page {
        title: "Tera Example".to_string(),
        heading: "Welcome to Tera".to_string(),
        content: "This is a simple example of using the Tera template engine."
            .to_string(),
    };

    let context = tera::Context::from_serialize(&page)?;

    // Render the template with the context:
    let tera = (*TEMPLATES).as_ref()?;
    let rendered = tera.render("index.html", &context)?;

    // Print the rendered template:
    println!("{}", rendered);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
