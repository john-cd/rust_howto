use anyhow::Result;
use serde::Serialize;

/// Markdown badges i.e. a small image (generated by shields.io)
/// linked to a website In this case, badges linked to docs.rs,
/// crates.io, github, and lib.rs
pub(super) static CRATE_BADGE: &str = r"[![{crate_id}][c-{crate_id | underscored}-badge]][c-{crate_id | underscored}]\{\{hi:{crate_id}}}
[![{crate_id}-crates.io][c-{crate_id | underscored}-crates.io-badge]][c-{crate_id | underscored}-crates.io]
[![{crate_id}-github][c-{crate_id | underscored}-github-badge]][c-{crate_id | underscored}-github]
[![{crate_id}-lib.rs][c-{crate_id | underscored}-lib.rs-badge]][c-{crate_id | underscored}-lib.rs]
";

/// Reference definitions for the badges above
pub(super) static CRATE_REFDEFS: &str = r"
[c-{crate_id | underscored}-badge]: https://img.shields.io/crates/v/{crate_id}?label={crate_id}
[c-{crate_id | underscored}-crates.io-badge]: https://img.shields.io/badge/crates.io-{crate_id | shielded}-crimson
[c-{crate_id | underscored}-crates.io]: https://crates.io/crates/{crate_id | underscored}
[c-{crate_id | underscored}-github-badge]: https://img.shields.io/badge/{crate_id | shielded}-steelblue?logo=github
[c-{crate_id | underscored}-github]: https://github.com/_TODO
[c-{crate_id | underscored}-lib.rs-badge]: https://img.shields.io/badge/lib.rs-{crate_id | shielded}-yellow
[c-{crate_id | underscored}-lib.rs]: https://lib.rs/crates/{crate_id}
[c-{crate_id | underscored}]: https://docs.rs/{crate_id | underscored}
";

/// create_badge
///
/// crate_id: name of the crate (per lib.rs)
pub fn create_badge(crate_id: &str) -> Result<String> {
    #[derive(Serialize)]
    struct Context {
        crate_id: String,
    }
    let tt = super::get_template_engine()?;
    let context = Context {
        crate_id: crate_id.to_string(),
    };
    let rendered = tt.render("CRATE_BADGE", &context)?
        + tt.render("CRATE_REFDEFS", &context)?.as_str();

    Ok(rendered)
}
