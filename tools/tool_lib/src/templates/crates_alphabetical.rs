use anyhow::Result;
use serde::Serialize;

pub(super) static ALPHABETICAL_ROW: &str = r"## {first_letter}

{{ for name in crate_names }}[![{name}][c-{name | underscored}-badge]][c-{name | underscored}]  {{ endfor }}
";

#[derive(Serialize)]
struct Context<'a> {
    first_letter: &'a str,
    crate_names: Vec<&'a str>,
}

/// Returns one section of the "crates by alphabetic order" page
pub fn create_alphabetical_crate_page_section<'a>(
    first_letter: &'a str,
    crate_names: Vec<&'a str>,
) -> Result<String> {
    let tt = super::get_template_engine()?;
    let context = Context {
        first_letter,
        crate_names,
    };
    let rendered = tt.render("ALPHABETICAL_ROW", &context)?;
    Ok(rendered)
}
