use anyhow::Result;
use serde::Serialize;

pub static RBE: &str = "
[![Rust by example - {concept}][book-rust-by-example-{concept | underscored}-badge]][book-rust-by-example-{concept | underscored}]
[book-rust-by-example-{concept | underscored}-badge]: https://img.shields.io/badge/Rust_By_Example-{concept | shielded}-violet?logo=mdbook
[book-rust-by-example-{concept | underscored}]: https://doc.rust-lang.org/rust-by-example/{concept | underscored}.html
";

/// create_rbe_badge
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
        concept: concept.to_string(),
    };
    let rendered = tt.render("RBE", &context)?;
    Ok(rendered)
}