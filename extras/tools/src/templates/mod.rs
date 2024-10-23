#![allow(unused)]
use anyhow::Result;
use serde::Serialize;
use tinytemplate::TinyTemplate;
use tracing::info;

mod crates;
mod rbe;
pub use crates::*;
pub use rbe::*;

fn build_template_engine<'a>() -> Result<TinyTemplate<'a>> {
    let mut tt = TinyTemplate::new();
    // replace - by _ per the Rust convention for module names
    tt.add_formatter("underscored", |val, str| {
        info!("underscored called with str: {str}, val: {val}");
        if let Some(v) = val.as_str() {
            str.push_str(&v.replace("-", "_"));
        }
        Ok(())
    });
    // used to escape - to --, _ to __, and replace " " by _ for use in
    // shields.io URLs
    tt.add_formatter("shielded", |val, str| {
        info!("str: {str}, val: {val}");
        if let Some(v) = val.as_str() {
            str.push_str(
                &v.replace("-", "--").replace("_", "__").replace(" ", "_"),
            );
        }
        Ok(())
    });
    Ok(tt)
}
