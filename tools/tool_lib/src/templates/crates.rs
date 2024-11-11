use anyhow::Result;
use serde::Serialize;
use trim_in_place::TrimInPlace;

/// Markdown badges i.e. a small image (generated by shields.io)
/// linked to a website In this case, badges linked to docs.rs,
/// crates.io, github, and lib.rs
pub(super) static CRATE_BADGES: &str = concat!(
"{{ if homepage_defined }}[![{crate_name}-website][c-{crate_name | underscored}-website-badge]][c-{crate_name | underscored}-website]{{ endif }}",
"[![{crate_name}][c-{crate_name | underscored}-badge]][c-{crate_name | underscored}]",
"[![{crate_name}-crates.io][c-{crate_name | underscored}-crates.io-badge]][c-{crate_name | underscored}-crates.io]",
"[![{crate_name}-github][c-{crate_name | underscored}-github-badge]][c-{crate_name | underscored}-github]",
r"[![{crate_name}-lib.rs][c-{crate_name | underscored}-lib.rs-badge]][c-{crate_name | underscored}-lib.rs]\{\{hi:{crate_name}}}");

pub(super) static CRATE_DESCRIPTION: &str =
    "{{- if description_defined }}{description}{{ endif -}}";

/// Reference definitions for the badges above
pub(super) static CRATE_REFDEFS: &str = "\
[c-{crate_name | underscored}-badge]: https://img.shields.io/crates/v/{crate_name}?label={crate_name}
[c-{crate_name | underscored}-crates.io-badge]: https://img.shields.io/badge/crates.io-{crate_name | shielded}-crimson
[c-{crate_name | underscored}-crates.io]: https://crates.io/crates/{crate_name | underscored}
[c-{crate_name | underscored}-github-badge]: https://img.shields.io/badge/{crate_name | shielded}-steelblue?logo=github
[c-{crate_name | underscored}-github]: {{ if repository_defined }}{repository}{{ else }}https://github.com/_TODO{{ endif }}
[c-{crate_name | underscored}-lib.rs-badge]: https://img.shields.io/badge/lib.rs-{crate_name | shielded}-yellow
[c-{crate_name | underscored}-lib.rs]: https://lib.rs/crates/{crate_name}
[c-{crate_name | underscored}]: {{ if documentation_defined }}{documentation}{{ else }}https://docs.rs/{crate_name | underscored}{{ endif }}
{{ if homepage_defined }}[c-{crate_name | underscored}-website-badge]: https://img.shields.io/badge/{crate_name | shielded}-coral{{ endif }}
{{ if homepage_defined }}[c-{crate_name | underscored}-website]: {homepage}{{ endif }}";

pub enum GenerationMode {
    CrateBadges,
    CrateDescription,
    CrateRefdefs,
}

/// create_crate_badges_or_refdefs
///
/// crate_name: name of the crate (per crates.io / lib.rs)
pub fn create_crate_badges_or_refdefs(
    crate_data: &crates_io_api::Crate,
    mode: GenerationMode,
) -> Result<String> {
    #[derive(Serialize)]
    struct Context<'a> {
        crate_name: &'a str,
        description_defined: bool,
        description: &'a str,
        documentation_defined: bool,
        documentation: String, // URL e.g. https://docs.rs/{crate}
        homepage_defined: bool,
        homepage: &'a str, /* URL e.g. https://github.com/sollimann/bonsai, https://serde.rs */
        repository_defined: bool,
        repository: String, // URL e.g. https://github.com/serde-rs/serde
    }
    let tt = super::get_template_engine()?;
    let context = Context {
        crate_name: &crate_data.name,
        description_defined: crate_data.description.is_some(),
        description: crate_data.description.as_deref().unwrap_or_default(),
        documentation_defined: crate_data.documentation.is_some(),
        documentation: crate_data.documentation.clone().unwrap_or_default(),
        homepage_defined: crate_data.homepage.is_some()
            && (crate_data.homepage != crate_data.repository),
        homepage: crate_data.homepage.as_deref().unwrap_or_default(),
        repository_defined: crate_data.repository.is_some(),
        repository: crate_data
            .repository
            .clone()
            .unwrap_or_default()
            .replace(".git", ""),
    };
    let mut rendered = match mode {
        GenerationMode::CrateBadges => tt.render("CRATE_BADGES", &context)?,
        GenerationMode::CrateDescription => {
            tt.render("CRATE_DESCRIPTION", &context)?
        }
        GenerationMode::CrateRefdefs => tt.render("CRATE_REFDEFS", &context)?,
    };
    rendered.trim_in_place();
    Ok(rendered)
}
