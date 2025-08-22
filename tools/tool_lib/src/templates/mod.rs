//! Build badges, index of the book, etc... using a template engine.

use tinytemplate::TinyTemplate;
use tracing::info;

mod category_badge;
mod crate_block_badges_or_refdefs;
mod crates_alphabetical;
mod crates_by_category;
mod detect_github_url;
mod index_anchors;
mod normalize_url;
mod rbe;
mod remove_empty_lines;

pub use category_badge::*;
pub use crate_block_badges_or_refdefs::*;
pub use crates_alphabetical::*;
pub use crates_by_category::*;
pub use detect_github_url::*;
pub use index_anchors::*;
pub use normalize_url::*;
pub use rbe::*;
pub use remove_empty_lines::*;

/// Build a template engine with the custom formatters:
/// - `underscored`: replace `-` by `_` per the Rust convention for module names
/// - `shielded`: used to escape `-` to `--`, `_` to `__`, and replace " " by `_` for use in
///   shields.io URLs
fn get_template_engine() -> anyhow::Result<TinyTemplate<'static>> {
    let mut tt = TinyTemplate::new();

    // // Replace - by _ per the Rust convention for module names:
    // tt.add_formatter("underscored", |val, str| {
    //     info!("underscored called with str: {str}, val: {val}");
    //     if let Some(v) = val.as_str() {
    //         str.push_str(&v.replace("-", "_"));
    //     }
    //     Ok(())
    // });

    // Escape - to --, _ to __, and replace " " by _ for use in
    // `shields.io` URLs.
    tt.add_formatter("shielded", |val, str| {
        info!("str: {str}, val: {val}");
        if let Some(v) = val.as_str() {
            str.push_str(&v.replace("-", "--").replace("_", "__").replace(" ", "_"));
        }
        Ok(())
    });
    tt.add_template("ALPHABETICAL_ROW", ALPHABETICAL_ROW)?;
    tt.add_template("CAT_BADGE", CAT_BADGE)?;
    tt.add_template("CATEGORY_ROW", CATEGORY_ROW)?;
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
