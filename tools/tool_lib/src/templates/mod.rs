//! Build badges, index of the book, etc... using a template engine.
//!
//! TODO Reviewed ??-??-2026.

use tinytemplate::TinyTemplate;
use tracing::info;

mod category_badge;
mod crate_block_badges_or_refdefs;
mod crates_alphabetical;
mod crates_by_category;
mod detect_github_url;
mod index_anchors;
mod normalize_url;
mod published_recipes;
mod rbe;
mod remove_empty_lines;

pub use category_badge::*;
pub use crate_block_badges_or_refdefs::*;
pub use crates_alphabetical::*;
pub use crates_by_category::*;
pub use detect_github_url::*;
pub use index_anchors::*;
pub use normalize_url::*;
pub use published_recipes::*;
pub use rbe::*;
pub use remove_empty_lines::*;

/// Build a template engine with the custom formatters:
/// - `underscored`: replace `-` by `_` per the Rust convention for module names
/// - `shielded`: used to escape `-` to `--`, `_` to `__`, and replace " " by `_` for use in
///   shields.io URLs
fn get_template_engine() -> anyhow::Result<TinyTemplate<'static>> {
    let mut tt = TinyTemplate::new();

    // Replace - by _ per the Rust convention for module names:
    tt.add_formatter("underscored", |val, str| {
        info!("underscored called with str: {str}, val: {val}");
        if let Some(v) = val.as_str() {
            str.push_str(&v.replace('-', "_"));
        }
        Ok(())
    });

    // Escape - to --, _ to __, and replace " " by _ for use in
    // `shields.io` URLs.
    tt.add_formatter("shielded", |val, str| {
        info!("str: {str}, val: {val}");
        if let Some(v) = val.as_str() {
            str.push_str(&v.replace('-', "--").replace('_', "__").replace(' ', "_"));
        }
        Ok(())
    });
    tt.add_template("CAT_BADGE", CAT_BADGE)?;
    tt.add_template("CRATE_BLOCK_BADGES", CRATE_BLOCK_BADGES)?;
    tt.add_template("CRATE_DESCRIPTION", CRATE_DESCRIPTION)?;
    tt.add_template("CRATE_BLOCK_REFDEFS", CRATE_BLOCK_REFDEFS)?;
    tt.add_template("RBE", RBE)?;
    tt.add_template("INDEX_ANCHORS", INDEX_ANCHORS)?;
    Ok(tt)
}

// problem: TinyTemplate is not Sync + Send
// can't do
// use std::sync::OnceLock;
// fn get_template_engine() -> &'static TinyTemplate<'static> {
//   static INSTANCE: OnceLock<TinyTemplate<'static>> = OnceLock::new();
//   let  t = INSTANCE.get_or_init(|| { build_template_engine() });
//   t
// }

// [finish; unit tests; tinytemplate vs tera?](https://github.com/john-cd/rust_howto/issues/1361)

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::*;

    /// Context holding a single string value for formatter tests.
    #[derive(Serialize)]
    struct Value {
        /// The vector value stored by this example.
        v: &'static str,
    }

    /// Renders `template` against `input` using the shared engine's formatters.
    fn render(template: &'static str, input: &'static str) -> String {
        let mut tt = get_template_engine().expect("engine should build");
        tt.add_template("TEST", template)
            .expect("template should parse");
        tt.render("TEST", &Value { v: input })
            .expect("template should render")
    }

    /// The `underscored` formatter replaces every hyphen with an underscore.
    #[test]
    fn underscored_formatter_replaces_hyphens() {
        assert_eq!(render("{v | underscored}", "a-b-c"), "a_b_c");
    }

    /// The `shielded` formatter escapes `-` and `_` and replaces spaces.
    #[test]
    fn shielded_formatter_escapes_for_shields_io() {
        assert_eq!(render("{v | shielded}", "a-b_c d"), "a--b__c_d");
    }

    /// Formatters leave the output untouched when the value is not a string.
    #[test]
    fn formatters_ignore_non_string_values() {
        let mut tt = get_template_engine().expect("engine should build");
        tt.add_template("TEST", "[{v | underscored}{v | shielded}]")
            .expect("template should parse");
        let rendered = tt
            .render("TEST", &serde_json::json!({ "v": 42 }))
            .expect("template should render");
        assert_eq!(rendered, "[]");
    }
}
