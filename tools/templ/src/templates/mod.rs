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

pub fn create_badge(crate_id: &str) -> Result<()> {
    let mut tt = TinyTemplate::new();
    tt.add_template("CRATE_BADGE", CRATE_BADGE)?;
    tt.add_formatter("underscored", |val, str| {
        info!("str: {str}, val: {val}");
        if let Some(v) = val.as_str() {
            str.push_str(&v.replace("-", "_"));
        }
        Ok(())
    });

    let context = Context {
        crate_id: crate_id.to_string(),
    };

    let rendered = tt.render("CRATE_BADGE", &context)?;
    println!("{}", rendered);

    Ok(())
}
