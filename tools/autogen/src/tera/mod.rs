use std::collections::HashMap;
use std::sync::LazyLock;

use anyhow::anyhow;
use tera::Context;
use tera::Tera;
use walkdir::DirEntry;
use walkdir::WalkDir;

use crate::model;

static TEMPLATES: LazyLock<Result<Tera, tera::Error>> = LazyLock::new(|| {
    Tera::new("assets/**/*").map(|mut t| {
        t.autoescape_on(vec![".html"]);
        // t.register_filter("do_nothing", do_nothing_filter);
        t
    })
});

/// Returns a Hashmap; key = template name; value = rendered template
pub fn render(s: model::Chapter) -> anyhow::Result<HashMap<String, String>> {
    let context = Context::from_serialize(&s)?;
    let templ: &Result<Tera, tera::Error> = &TEMPLATES;
    if let Err(e) = templ {
        return Err(anyhow::anyhow!(
            "Tera template engine failed to initialize: {}",
            e
        ));
    }
    let walker = WalkDir::new("./templates").into_iter()
        .filter_entry(|e| !is_hidden(e)) // no files / directories starting with `.`
        .filter_map(|e| e.ok()); // ignore errors

    let mut h = HashMap::new();

    for entry in walker {
        println!("Rendering {}", entry.path().display());
        let p = entry
            .path()
            .strip_prefix("assets")?
            .to_str()
            .ok_or(anyhow!("Error converting path to &str"))?;
        let rendered = templ.as_ref().unwrap().render(p, &context)?;
        h.insert(p.into(), rendered);
    }
    Ok(h)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
