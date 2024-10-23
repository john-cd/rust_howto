#![allow(unused)]
use anyhow::Result;
use serde::Serialize;
use tinytemplate::TinyTemplate;
use tracing::info;

#[derive(Serialize)]
struct Context {
    crate_id: String,
}

static CRATE_BADGE : &str = "[![{crate_id}][c-{crate_id | underscored}-badge]][c-{crate_id | underscored}]
[![{crate_id}-crates.io][c-{crate_id | underscored}-crates.io-badge]][c-{crate_id | underscored}-crates.io]
[![{crate_id}-github][c-{crate_id | underscored}-github-badge]][c-{crate_id | underscored}-github]
[![{crate_id}-lib.rs][c-{crate_id | underscored}-lib.rs-badge]][c-{crate_id | underscored}-lib.rs]
";

static CRATE_REFDEFS: &str = "
[c-{crate_id | underscored}-badge]: https://img.shields.io/crates/v/{crate_id}?label={crate_id}
[c-{crate_id | underscored}-crates.io-badge]: https://img.shields.io/badge/crates.io-{crate_id | shielded}-crimson
[c-{crate_id | underscored}-crates.io]: https://crates.io/crates/{crate_id | underscored}
[c-{crate_id | underscored}-github-badge]: https://img.shields.io/badge/{crate_id | shielded}-steelblue?logo=github
[c-{crate_id | underscored}-lib.rs-badge]: https://img.shields.io/badge/lib.rs-{crate_id | shielded}-yellow
[c-{crate_id | underscored}-lib.rs]: https://lib.rs/crates/{crate_id}
[c-{crate_id | underscored}]: https://docs.rs/{crate_id | underscored}
";

pub fn create_badge(crate_id: &str) -> Result<()> {
    let mut tt = TinyTemplate::new();
    tt.add_template("CRATE_BADGE", CRATE_BADGE)?;
    tt.add_template("CRATE_REFDEFS", CRATE_REFDEFS);
    // replace - by _ per the Rust convention for module names
    tt.add_formatter("underscored", |val, str| {
        info!("underscored called with str: {str}, val: {val}");
        if let Some(v) = val.as_str() {
            str.push_str(&v.replace("-", "_"));
        }
        Ok(())
    });
    // used to escape - to --, _ to __, and replace " " by _ for use in shields.io URLs
    tt.add_formatter("shielded", |val, str| {
        info!("str: {str}, val: {val}");
        if let Some(v) = val.as_str() {
            str.push_str(&v.replace("-", "--").replace("_", "__").replace(" ", "_"));
        }
        Ok(())
    });
    let context = Context {
        crate_id: crate_id.to_string(),
    };

    let rendered = tt.render("CRATE_BADGE", &context)? + tt.render("CRATE_REFDEFS", &context)?.as_str();
    println!("{}", rendered);

    Ok(())
}
