use std::path::PathBuf;
use std::sync::LazyLock;

use anyhow::Result;
use anyhow::bail;
use tera::Context;
use tera::Tera;

use crate::model::book::Renderable;
use crate::read_write;

mod filters;
use filters::*;

static TEMPLATES: LazyLock<Result<Tera, tera::Error>> = LazyLock::new(|| {
    Tera::new("templates/**/*").map(|mut t| {
        // By default, Tera will auto-escape all content in files ending with
        // ".html", ".htm" and ".xml".
        // t.autoescape_on(vec![".html", ".xml"]);
        t.register_filter("underscored", underscored);
        t.register_filter("shielded", shielded);
        t
    })
});

/// Renders all the templates specified in the `Renderable` trait.
///
/// The `Renderable` trait provides the following:
/// - the list of templates to render
/// - the context to use for rendering
/// - the destination file path for each rendered template
pub fn render(r: impl Renderable) -> Result<()> {
    let templ: &Result<Tera, tera::Error> = &TEMPLATES;
    if let Err(e) = templ {
        bail!("The Tera template engine failed to initialize: {}", e);
    }
    let context = Context::from_serialize(&r)?;
    render_one(r, &context)?;
    Ok(())
}

#[allow(dead_code)]
fn render_one(r: impl Renderable, context: &Context) -> Result<()> {
    let templ = (*TEMPLATES).as_ref().unwrap();

    for renderinfo in r.get_what_to_render() {
        let template_name: &str = &renderinfo.template_name;
        let rendered = templ.render(template_name, context)?;
        let where_to_write = generate_path(&renderinfo.dest_filepath_template)?;
        read_write::backup_then_write_to(where_to_write, rendered)?;
    }
    for child in r.get_children() {
        render_one(child, context)?;
    }
    Ok(())
}

fn generate_path(path_template: &str) -> Result<PathBuf> {
    let context = Context::new();
    // context.insert("product", &product);
    // context.insert("vat_rate", &0.20);
    Ok(PathBuf::from(Tera::one_off(path_template, &context, true)?))
}

// use walkdir::DirEntry;
// use walkdir::WalkDir;
// fn build_table(templ: Tera) -> anyhow::Result<()> {
//     let walker = WalkDir::new("./templates").into_iter()
//         .filter_entry(|e| !is_hidden(e)) // no files / directories starting
// with `.`         .filter_map(|e| e.ok()); // ignore errors
//     let mut h = HashMap::new();

//     for entry in walker {
//         println!("Rendering {}", entry.path().display());
//         let p = entry
//             .path()
//             .strip_prefix("templates")?
//             .to_str()
//             .ok_or(anyhow!("Error converting path to &str"))?;
//     }
//     Ok(())
// }

// fn is_hidden(entry: &DirEntry) -> bool {
//     entry
//         .file_name()
//         .to_str()
//         .map(|s| s.starts_with("."))
//         .unwrap_or(false)
// }
