//! Inspired by https://github.com/rxdn/mdbook-sitemap-generator/tree/master
//! Consider using https://docs.rs/sitewriter/1.0.4/sitewriter/.
//! or https://crates.io/crates/sitemap instead.
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use quick_xml::events::BytesText;
use quick_xml::writer::Writer;

fn write_xml(links: Vec<String>, mut dest_file: File) -> Result<()> {
    let mut writer = Writer::new_with_indent(&mut dest_file, b' ', 2);

    writer.write_bom()?;
    // Insert <?xml version="1.0" encoding="UTF-8"?>
    writer
        .get_mut()
        .write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")?;
    // <urlset>
    writer
        .create_element("urlset")
        .with_attribute((
            "xmlns",
            "http://www.sitemaps.org/schemas/sitemap/0.9",
        ))
        .write_inner_content(|writer| {
            for link in links.iter() {
                // <url><loc>
                writer.create_element("url").write_inner_content(|w| {
                    w.create_element("loc")
                        .write_text_content(BytesText::new(link.as_str()))?;
                    Ok::<_, Error>(())
                })?;
            }
            Ok::<_, Error>(())
        })?;
    Ok::<_, Error>(())
}

fn main() -> Result<()> {
    let src = Path::new("/code/src/");

    // Locate the Markdown files
    let paths: Vec<PathBuf> = tools::find_markdown_paths(src)?;

    // Remove a few exceptions
    let exclude = ["refs.md", "SUMMARY.md"];
    let l = paths.into_iter().filter(|p| {
        !exclude.iter().any(|&ex| {
            p.file_name()
                .unwrap_or(OsStr::new(""))
                .to_str()
                .unwrap_or_default()
                .ends_with(ex)
        })
    }); // p.ends_with(ex) did not work here for some reason
    // debug: let l = l.map(|path| { println!("{:?}", path); path });

    let domain = "https://john-cd.com/rust_howto/";
    let l = l.map(|p: PathBuf| {
        p.with_extension("html")
            .strip_prefix(src) // Result<&Path, _>
            .map_err(anyhow::Error::from)
            .and_then(|p| p.to_str().ok_or(anyhow!("Non UTF-8 path: {:?}", p)))
            .map(|s| format!("{domain}{s}")) // Prefix with domain
            .map(|s| s.replace("intro.html", "index.html"))
    });

    // Separate links from errors and print errors if any
    let (links, errors): (Vec<Result<_, _>>, Vec<Result<_, _>>) =
        l.partition(Result::is_ok);
    let links: Vec<String> = links.into_iter().map(Result::unwrap).collect();
    let errors: Vec<Error> =
        errors.into_iter().map(Result::unwrap_err).collect();
    // debug: println!("Links: {:?}", links);
    if !errors.is_empty() {
        println!("Errors: {:?}", errors);
    }
    // Create directory
    let dest_dir = "/code/book/html/";
    tools::create_dir(dest_dir)?;

    // Write the sitemap
    let sitemap_full_path: String = format!("{dest_dir}sitemap.xml");

    // File::create will create a file if it does not exist, and will
    // truncate it if it does.
    write_xml(links, File::create(Path::new(sitemap_full_path.as_str()))?)?;
    println!("sitemap.xml created.");
    Ok(())
}
