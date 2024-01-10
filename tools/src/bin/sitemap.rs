//! Inspired by https://github.com/rxdn/mdbook-sitemap-generator/tree/master
//! Consider using https://docs.rs/sitewriter/1.0.4/sitewriter/.
//! or https://crates.io/crates/sitemap instead.
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;
use quick_xml::events::BytesText;
use quick_xml::writer::Writer;
use walkdir::DirEntry;
use walkdir::WalkDir;

/// True if the directory entry is hidden (starts with a `.`)
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn find_paths<P: AsRef<Path>>(root_directory: P) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    let walker = WalkDir::new(root_directory)
        .sort_by_file_name()
        .into_iter()
        // Skip hidden files and directories efficiently on unix systems
        .filter_entry(|e| !is_hidden(e));

    for entry in walker
        // Yields only the values for which the supplied closure returns Some(value)
        // Ignores WalkDir errors
        .filter_map(|res| res.ok())
        .filter(|de| de.file_type().is_file())
    {
        match entry.path().extension() {
            Some(extension) => {
                if extension == "md" {
                    // debug: println!("{}", entry.path().display());
                    paths.push(entry.into_path());
                } else {
                    println!("Not a Markdown file: {}", entry.path().display());
                }
            }
            None => {
                println!(
                    "Could not extract extension for {}",
                    entry.path().display()
                );
            }
        }
    }
    Ok(paths)
}

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

    // Locate the Markdown file
    let paths: Vec<PathBuf> = find_paths(src)?;

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
    let dest_dir = "/code/book/";
    match Path::new(dest_dir).try_exists() {
        Ok(false) => {
            std::fs::create_dir_all(dest_dir)?;
            println!("{} created", dest_dir);
        }
        Ok(true) => {
            // debug: println!("{} already exists", dest_dir);
        }
        Err(_) => {
            bail!(
                "{}'s existence can neither be confirmed nor denied.",
                dest_dir
            );
        }
    }

    // Write the sitemap
    let sitemap_full_path: String = format!("{dest_dir}sitemap.xml");

    // File::create will create a file if it does not exist, and will
    // truncate it if it does.
    write_xml(links, File::create(Path::new(sitemap_full_path.as_str()))?)?;
    println!("sitemap.xml created.");
    Ok(())
}
