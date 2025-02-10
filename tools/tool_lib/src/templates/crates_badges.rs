use anyhow::Result;
use regex::Regex;
use serde::Serialize;
use trim_in_place::TrimInPlace;

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

/// Markdown badges i.e. a small image (generated by shields.io)
/// linked to a website In this case, badges linked to docs.rs,
/// crates.io, github, and lib.rs
pub(super) static CRATE_BADGES: &str = concat!(
    "{{ if homepage_defined }}[![{crate_name}-website][c-{crate_name | underscored}-website-badge]][c-{crate_name | underscored}-website] {{ endif }}",
    "[![{crate_name}][c-{crate_name | underscored}-badge]][c-{crate_name | underscored}] ",
    "[![{crate_name}-crates.io][c-{crate_name | underscored}-crates.io-badge]][c-{crate_name | underscored}-crates.io] ",
    "[![{crate_name}-github][c-{crate_name | underscored}-github-badge]][c-{crate_name | underscored}-github] ",
    r"[![{crate_name}-lib.rs][c-{crate_name | underscored}-lib.rs-badge]][c-{crate_name | underscored}-lib.rs]\{\{hi:{crate_name}}} "
);

pub(super) static CRATE_DESCRIPTION: &str =
    "{{- if description_defined }}{description}{{ endif -}}";

/// Reference definitions for the badges above
pub(super) static CRATE_REFDEFS: &str = "\
[c-{crate_name | underscored}-badge]: https://img.shields.io/crates/v/{crate_name}?label={crate_name}
[c-{crate_name | underscored}-crates.io-badge]: https://img.shields.io/badge/crates.io-{crate_name | shielded}-crimson
[c-{crate_name | underscored}-crates.io]: https://crates.io/crates/{crate_name | underscored}
[c-{crate_name | underscored}-github-badge]: https://img.shields.io/badge/{crate_name | shielded}-steelblue?logo=github
{{ if repository_defined }}[c-{crate_name | underscored}-github]: {repository}{{ endif }}
[c-{crate_name | underscored}-lib.rs-badge]: https://img.shields.io/badge/lib.rs-{crate_name | shielded}-yellow
[c-{crate_name | underscored}-lib.rs]: https://lib.rs/crates/{crate_name}
[c-{crate_name | underscored}]: {{ if documentation_defined }}{documentation}{{ else }}https://docs.rs/{crate_name}{{ endif }}
{{ if homepage_defined }}[c-{crate_name | underscored}-website-badge]: https://img.shields.io/badge/{crate_name | shielded}-coral{{ endif }}
{{ if homepage_defined }}[c-{crate_name | underscored}-website]: {homepage}{{ endif }}
";

pub enum GenerationMode {
    CrateBadges,
    CrateDescription,
    CrateRefdefs,
}

#[derive(Serialize)]
struct Context<'a> {
    crate_name: &'a str,
    description_defined: bool,
    description: &'a str,
    documentation_defined: bool,
    documentation: &'a str, // URL e.g. https://docs.rs/{crate}
    homepage_defined: bool,
    homepage: &'a str, /* URL e.g. https://github.com/sollimann/bonsai, https://serde.rs */
    repository_defined: bool,
    repository: String, // URL e.g. https://github.com/serde-rs/serde
}

/// create_crate_badges_or_refdefs
///
/// crate_name: name of the crate (per crates.io / lib.rs)
pub fn create_crate_badges_or_refdefs(
    crate_data: &crates_io_api::Crate,
    mode: GenerationMode,
) -> Result<String> {
    let tt = super::get_template_engine()?;

    // Normalize URL
    let documentation = crate_data
        .documentation
        .as_ref()
        .map(|s| normalize_docs_url(s))
        .unwrap_or_default();

    let context = Context {
        crate_name: &crate_data.name.to_lowercase(),
        description_defined: crate_data.description.is_some(),
        description: crate_data.description.as_deref().unwrap_or_default(),
        documentation_defined: crate_data.documentation.is_some(),
        documentation: documentation.trim_end_matches('/'),
        homepage_defined: crate_data.homepage.is_some()
            && (crate_data.homepage != crate_data.repository),
        homepage: crate_data
            .homepage
            .as_deref()
            .unwrap_or_default()
            .trim_end_matches('/'),
        repository_defined: crate_data.repository.is_some(),
        repository: crate_data
            .repository
            .as_deref()
            .unwrap_or_default()
            .replace(".git", "")
            .trim_end_matches('/')
            .to_string(),
    };
    let mut rendered = match mode {
        GenerationMode::CrateBadges => tt.render("CRATE_BADGES", &context)?,
        GenerationMode::CrateDescription => tt.render("CRATE_DESCRIPTION", &context)?,
        GenerationMode::CrateRefdefs => tt.render("CRATE_REFDEFS", &context)?,
    };
    rendered.trim_in_place();
    Ok(rendered)
}

// Get rid of repeats in e.g. https://docs.rs/quote/latest/quote
// Normalize e.g. https://docs.rs/crate/termbook to https://docs.rs/termbook
// Normalize e.g. https://docs.rs/tungstenite/0.24.0 to https://docs.rs/tungstenite
// Normalize e.g. https://docs.rs/watchmaker/0.1.0/watchmaker/fn.solve.html to https://docs.rs/watchmaker
fn normalize_docs_url(url: &str) -> std::borrow::Cow<str> {
    let re1: &Regex = regex!(r"http(s)?://docs.rs/(?:crate/)?(?<crt>[A-Za-z0-9_-]+)(?:/.*)?");
    // If no match is found, then the haystack is returned unchanged.
    re1.replace(url, "https://docs.rs/${crt}")
}

#[test]
fn test() {
    assert_eq!(
        normalize_docs_url("https://docs.rs/approx"),
        "https://docs.rs/approx"
    );
    assert_eq!(
        normalize_docs_url("http://docs.rs/prost"),
        "https://docs.rs/prost"
    );
    assert_eq!(
        normalize_docs_url("https://docs.rs/serde/"),
        "https://docs.rs/serde"
    );
    assert_eq!(
        normalize_docs_url("https://docs.rs/crate/termbook"),
        "https://docs.rs/termbook"
    );
    assert_eq!(
        normalize_docs_url("http://docs.rs/crate/doc-comment"),
        "https://docs.rs/doc-comment"
    );
    assert_eq!(
        normalize_docs_url("https://docs.rs/tungstenite/0.24.0"),
        "https://docs.rs/tungstenite"
    );
    assert_eq!(
        normalize_docs_url("https://docs.rs/quote/latest/quote"),
        "https://docs.rs/quote"
    );
    assert_eq!(
        normalize_docs_url("https://docs.rs/watchmaker/0.1.0/watchmaker/fn.solve.html"),
        "https://docs.rs/watchmaker"
    );
}
