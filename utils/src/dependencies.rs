use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use std::result;

use anyhow::anyhow;
use anyhow::Result;
use regex::Regex;
use serde::Deserialize;
use tracing::info;

use super::link::LinkBuilder;
use crate::link::Link;

#[derive(Debug, Deserialize)]
pub struct Dependency<'a> {
    library_name: Cow<'a, str>,
    package_repo_url: Option<Cow<'a, str>>,
}

// Parse `Cargo.toml` and returns the list of dependencies:
/// Calls cargo tree --depth 1 --edges normal --prefix none --format
/// {lib},{r} --locked (immediate children, no-dev/build, flat list,
/// crate name and package repository URL)
///
/// dir_path: Path to the directory containing the Cargo.toml file.
pub fn get_dependencies<P: AsRef<Path>>(
    dir_path: P,
) -> Result<Vec<Dependency<'static>>> {
    let output = Command::new("cargo")
        .args([
            "tree",
            "--depth",
            "1",
            "--edges",
            "normal",
            "--prefix",
            "none",
            "--format",
            "{lib},{r}",
            "--locked",
        ])
        .current_dir(dir_path)
        .output()?; // returns if failed to execute Command

    write_log(&output.stdout, &output.stderr)?;

    if !output.status.success() {
        return Err(anyhow!(
            "`cargo tree` failed. Status: {}. Output: {}\n {}",
            output.status,
            String::from_utf8(output.stdout.clone())?,
            String::from_utf8(output.stderr)?
        ));
    }

    // Useful wrapper that implements Read for Vec<u8>
    let cursor = std::io::Cursor::new(output.stdout);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(cursor);

    // Read cargo tree output
    let mut results = Vec::new();

    while let Some(res) = rdr.deserialize::<Dependency>().next() {
        if let Ok(dep) = res {
            if dep.library_name != "deps" {
                results.push(dep);
            }
        } else {
            return Err(anyhow!("Failed to parse cargo tree output."));
        }
    }

    Ok(results)
}

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

/// Write e.g. stdout / stderr to a file.
fn write_log(out: &[u8], err: &[u8]) -> Result<()> {
    let mut buffer = BufWriter::new(File::create("dependencies.log")?);
    buffer.write_all(out)?;
    buffer.write_all(err)?;
    buffer.flush()?;
    Ok(())
}

// TODO finish

/// create reference definitions from dependencies, and write to a
/// writer / file.
///
/// dependencies: list of dependencies
/// w: Writer (e.g. File) to write to
fn generate_refdefs_from<W>(
    dependencies: Vec<Dependency>,
    w: &mut W,
) -> Result<()>
where
    W: Write,
{
    let sorted_deps: BTreeMap<_, _> = dependencies
        .iter()
        .map(|dep| (dep.library_name.as_ref(), dep))
        .collect();

    let mut buf = Vec::new();

    for (_, dep) in sorted_deps {
        info!("{:?}", dep);
        write_for_one_library(
            &dep.library_name,
            dep.package_repo_url.as_deref(),
            &mut buf,
        )?;
    }

    w.write_all(&buf)?;
    Ok(())
}

/// Create, for a given crate, multiple reference definitions for
/// common websites such as docs.rs, crates.io, github,
/// and th associated badge URLs
fn write_for_one_library<W>(
    library_name: &str,
    package_repo_url: Option<&str>,
    w: &mut W,
) -> Result<()>
where
    W: Write,
{
    // [arrow-rs]: https://docs.rs/arrow/
    let docs_rs_url = format!("https://docs.rs/{}", library_name);

    // [config-crate]: https://crates.io/crates/config/
    let crates_io_lbl = format!("{}-crate", library_name);
    let crates_io_url = format!("https://crates.io/crates/{}", library_name);

    // [arrow-rs-github]: https://github.com/apache/arrow-rs/
    let github_lbl = format!("{}-github", library_name);

    // info!("{}", badge_image_url.to_string());
    // let link = LinkBuilder::default()
    //     .set_label(library_name)
    //     .set_url()
    //     .set_image_url(badge_image_url.to_string())
    //     .build();

    // writeln!(w, "{}", link.to_reference_definition())?;
    // writeln!(w, "{}", link.to_badge_reference_definition())?;

    // writeln!(&mut buf, "{}", link.to_reference_link())?;
    // writeln!(&mut buf, "{}", link.to_link_with_badge())?;
    Ok(())
}

// fn append_sort_dedupe<R, W>(existing_refdefs: R, w: &mut W) ->
// Result<()> where
//     R: Read,
//     W: Write,
// {
//     let mut buf: Vec<Link> = Vec::new();

//     // Link has a custom Ord / Eq implementation, thus we can sort.
//     buf.sort();

//     // w.write_all(&buf)?;
//     Ok(())
// }
