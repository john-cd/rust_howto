#![allow(unused)]
/// Shared library for utilities found in the src/bin/ folder
mod build_book;
mod dependencies;
pub mod fs;
mod gen;
mod link;
pub mod markdown;
mod parser;
mod sitemap;
pub mod test_markdown;
mod write_from_parser;

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use anyhow::bail;
use anyhow::Result;
use pulldown_cmark::LinkType;
use pulldown_cmark::Parser;

/// Helper function:
/// Checks the source directory exists,
/// create the destination directory if it doesn't exist,
/// create the destination file,
/// parse all the Markdown files in the source directory,
/// and invoke a closure that uses the parser to write to the file
fn helper<P1, P2, F>(
    src_dir_path: P1,
    dest_file_path: P2,
    func: F,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    F: for<'a, 'b, 'c> FnOnce(Parser<'a, 'b>, &'c mut File) -> Result<()>,
{
    let src_dir_path = fs::check_is_dir(src_dir_path)?;

    fs::create_parent_dir_for(dest_file_path.as_ref())?;

    let mut f = File::create(dest_file_path)?;

    let all_markdown = fs::read_to_string_all_markdown_files_in(src_dir_path)?;
    let parser = parser::get_parser(all_markdown.as_ref());

    func(parser, &mut f)?;
    Ok(())
}

// Public Functions

// DEBUG

/// Parse Markdown from all .md files in a given source directory and
/// write all raw events to a file for debugging purposes
///
/// src_dir_path: path to the source directory
/// dest_file_path: path to the file to create and write into
pub fn debug_parse_to<P1, P2>(
    src_dir_path: P1,
    dest_file_path: P2,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    helper(
        src_dir_path,
        dest_file_path,
        write_from_parser::write_raw_to,
    )?;
    Ok(())
}

// Test function that uses fake Markdown
pub fn test() -> Result<()> {
    fs::create_dir("./book/temp/")?;

    let dest_file_path = "./book/temp/test.log";
    let mut f = BufWriter::new(File::create(dest_file_path)?);

    let test_markdown = test_markdown::get_test_markdown();
    let parser = parser::get_parser(test_markdown.as_ref());
    write_from_parser::write_raw_to(parser, &mut f)?;
    f.flush()?;
    Ok(())
}

// REFERENCE DEFINITIONS

/// Parse Markdown from all .md files in a given source directory
/// and write reference definitions found therein to a file
///
/// src_dir_path: path to the source directory
///
/// dest_file_path: path to the file to create and write into
pub fn write_ref_defs_to<P1, P2>(
    src_dir_path: P1,
    dest_file_path: P2,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    helper(
        src_dir_path,
        dest_file_path,
        write_from_parser::write_ref_defs_to,
    )?;
    Ok(())
}

/// Parse Markdown from all .md files in a given source directory,
/// extract existing reference definitions,
/// identify URLs that are GitHub repos,
/// create badge URLs for these links,
/// and write to a file.
///
/// src_dir_path: path to the source directory
///
/// dest_file_path: path to the file to create and write into
pub fn generate_badges<P1, P2>(
    src_dir_path: P1,
    dest_file_path: P2,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    helper(
        src_dir_path,
        dest_file_path,
        write_from_parser::write_github_repo_badge_refdefs,
    )?;
    Ok(())
}

// LINKS

// TODO need to remove internal links

/// Parse Markdown from all .md files in a given source directory,
/// write all inline links and autolinks (i.e., not written as
/// reference-style links) found therein to a file
///
/// src_dir_path: path to the source directory
///
/// dest_file_path: path to the file to create and write into
pub fn write_inline_links<P1, P2>(
    src_dir_path: P1,
    dest_file_path: P2,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    helper(src_dir_path, dest_file_path, |parser, f| {
        let links: Vec<link::Link> = parser::extract_links(parser);
        let links: Vec<_> = links
            .into_iter()
            .filter(|l| {
                [LinkType::Inline, LinkType::Autolink]
                    .iter()
                    .any(|&x| l.get_link_type().unwrap() == x)
            })
            .collect();
        link::write_links_to(links, f)?;
        Ok(())
    })?;

    Ok(())
}

/// Parse Markdown from all .md files in a given source directory,
/// write all links found therein to a file
///
/// src_dir_path: path to the source directory
///
/// dest_file_path: path to the file to create and write into
pub fn write_links<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    helper(src_dir_path, dest_file_path, |parser, f| {
        let links: Vec<link::Link> = parser::extract_links(parser);
        link::write_links_to(links, f)?;
        Ok(())
    })?;

    Ok(())
}

// GENERATE REF DEFS FROM DEPENDENCIES

/// Given a Cargo.toml path,
/// generate reference definitions from code dependencies
/// and write them to a file
///
/// cargo_toml_dir_path: path to the directory containing `Cargo.toml`
///
/// markdown_dir_path: path to the directory containing Markdown files
///
/// refdef_dest_file_path: path to the file to create and
/// write into
pub fn generate_refdefs_to<P1, P2, P3>(
    cargo_toml_dir_path: P1,
    markdown_dir_path: P2,
    refdef_dest_file_path: P3,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    P3: AsRef<Path>,
{
    // fs::create_dir("/code/book/temp/")?;
    // let mut f = File::create(refdef_dest_file_path)?;
    // // Generate ref defs from dependencies
    // let deps = dependencies::get_dependencies(&cargo_toml_dir_path)?;
    // // for (_, d) in &deps {
    // //     tracing::info!("{:?}", d);
    // // }
    // let mut new_links = gen::generate_refdefs_from(deps);

    // // Read existing ref defs
    // // TODO can we read just the *-refs.md files?
    // let all_markdown =
    //     fs::read_to_string_all_markdown_files_in(markdown_dir_path)?;
    // let parser = parser::get_parser(all_markdown.as_ref());
    // let sorted_linkdefs = parser::get_sorted_ref_defs(parser);

    // TODO
    // let existing_links = Vec::new();

    // let links = gen::merge_links(existing_links, new_links);
    // link::write_ref_defs_to(links, &mut f)?;
    // write links
    Ok(())
}

// SITEMAP

/// Create a sitemap.xml file from the list of Markdown files in a
/// source directory
///
/// src_dir_path: path to the source directory
///
/// domain: base URL e.g. <https://john-cd.com/rust_howto/>
///
/// dest_file_path: the path to the destination file e.g.
/// book/html/sitemap.xml
pub fn generate_sitemap<P1, P2>(
    src_dir_path: P1,
    base_url: url::Url,
    dest_file_path: P2,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    // Verify source path
    let src_dir_path = fs::check_is_dir(src_dir_path)?;

    // Returns an error whether this URL is a cannot-be-a-base URL,
    // meaning that parsing a relative URL string with this URL
    // as the base will return an error.
    if (base_url.cannot_be_a_base()) {
        bail!("Invalid URL - cannot be a base: {}", base_url)
    }

    // Create the parent folders of the destination file, if needed
    fs::create_parent_dir_for(dest_file_path.as_ref())?;

    // File::create will create a file if it does not exist,
    // and will truncate it if it does.
    let mut f = File::create(dest_file_path)?;

    crate::sitemap::generate_sitemap(src_dir_path, base_url, &mut f)?;

    Ok(())
}

// TODO

// let markdown_input_length = markdown_input.as_ref().len();
// write_markdown_to(parser, markdown_input_length, f)?;

// TODO

//// Set up the parser. We can treat is as any other iterator.
//// For each event, we print its details, such as the tag or string.
// let parser = Parser::new_with_broken_link_callback(
//     markdown_input.as_ref(),
//     get_options(),
//     Some(&mut |broken_link: BrokenLink| { callback(broken_link,
// markdown_input.as_ref()) }), )
