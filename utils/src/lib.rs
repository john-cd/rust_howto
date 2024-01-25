/// Shared libray for utilities in src/bin/ folder
//#![allow(unused)]
mod build_book;
mod dependencies;
pub mod fs;
mod gen;
mod github;
mod link;
pub mod markdown;
mod parser;
pub mod test_markdown;
mod write_from_parser;

use std::ffi::OsStr;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use pulldown_cmark::LinkType;
use tracing::debug;

// Public Functions

// DEBUG

/// Parse Markdown from all .md files in a given source directory and
/// write all raw events to a file for debugging purposes
///
/// src_dir: &OsStr equivalent
/// dest_file_path: path to the file to create and write into
pub fn debug_parse_to<S, P>(src_dir: S, dest_file_path: P) -> Result<()>
where
    S: AsRef<OsStr>,
    P: AsRef<Path>,
{
    let src_dir_path = fs::check_is_dir(src_dir)?;
    fs::create_parent_dirs_for(dest_file_path.as_ref())?;

    let all_markdown = fs::read_to_string_all_markdown_files_in(src_dir_path)?;
    let f = File::create(dest_file_path)?;
    debug!("\nParsing markdown ---------------\n");
    let parser = parser::get_parser(all_markdown.as_ref());
    write_from_parser::write_raw_to(parser, f)?;
    Ok(())
}

// REFERENCE DEFINITIONS

/// Parse a Markdown string and write reference definitions found
/// therein to a file, given a path
///
/// markdown_input: &str equivalent
/// dest_file_path: path to the file to create and write into
pub fn write_ref_defs_to<S: AsRef<str>, P: AsRef<Path>>(
    markdown_input: S,
    dest_file_path: P,
) -> Result<()> {
    let f = File::create(dest_file_path)?;
    let parser = parser::get_parser(markdown_input.as_ref());
    write_from_parser::write_ref_defs(&parser, f)?;
    Ok(())
}

/// Get existing reference definitions from a Markdown string,
/// identify URLs that are GitHub repos, create badge URLs for these
/// links, and write to a file, given a path.
///
/// markdown_input: &str equivalent
/// dest_file_path: path to the file to create and write into
pub fn generate_badges<S: AsRef<str>, P: AsRef<Path>>(
    markdown_input: S,
    dest_file_path: P,
) -> Result<()> {
    let mut f = BufWriter::new(File::create(dest_file_path)?);
    let parser = parser::get_parser(markdown_input.as_ref());
    github::write_github_repo_badge_refdefs(&parser, &mut f)?;
    f.flush().unwrap();
    Ok(())
}

// LINKS

// TODO need to remove internal links

/// Parse a Markdown string and write all inline links and autolinks
/// (i.e., not written as reference-style links) found therein to a
/// file
///
/// markdown_input: &str equivalent
/// dest_file_path: path to the file to create and write into
pub fn write_inline_links<S: AsRef<str>, P: AsRef<Path>>(
    markdown_input: S,
    dest_file_path: P,
) -> Result<()> {
    let mut f = std::fs::File::create(dest_file_path)?;

    let parser = parser::get_parser(markdown_input.as_ref());
    let links: Vec<link::Link> = parser::extract_links(parser);
    let links: Vec<_> = links
        .into_iter()
        .filter(|l| {
            [LinkType::Inline, LinkType::Autolink]
                .iter()
                .any(|&x| l.get_link_type().unwrap() == x)
        })
        .collect();
    link::write_links_to(links, &mut f)?;
    Ok(())
}

// Write all links to a file
/// markdown_input: &str equivalent
/// dest_file_path: path to the file to create and write into
pub fn write_links<S: AsRef<str>, P: AsRef<Path>>(
    markdown_input: S,
    dest_file_path: P,
) -> Result<()> {
    let mut f = File::create(dest_file_path)?;

    let parser = parser::get_parser(markdown_input.as_ref());
    let links: Vec<link::Link> = parser::extract_links(parser);
    link::write_links_to(links, &mut f)?;
    Ok(())
}

// GENERATE REF DEFS FROM DEPENDENCIES

/// Given a Cargo.toml path, generate reference definitions from
/// dependencies and write them to a file
pub fn generate_refdefs_to<P1, P2, P3>(
    cargo_toml_dir_path: P1,
    markdown_dir_path: P2,
    refdef_dest_filepath: P3,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    P3: AsRef<Path>,
{
    let mut f = File::create(refdef_dest_filepath)?;
    // Generate ref defs from dependencies
    let deps = dependencies::get_dependencies(&cargo_toml_dir_path)?;
    // for (_, d) in &deps {
    //     println!("{:?}", d);
    // }
    let mut new_links = gen::generate_refdefs_from(deps);

    // Read existing ref defs
    // TODO can we read just the *-refs.md files?
    let all_markdown =
        fs::read_to_string_all_markdown_files_in(markdown_dir_path)?;
    let parser = parser::get_parser(all_markdown.as_ref());
    let sorted_linkdefs = parser::get_sorted_ref_defs(&parser);

    // TODO
    // let existing_links = Vec::new();

    // let links = gen::merge_links(existing_links, new_links);
    // link::write_ref_defs_to(links, &mut f)?;
    // write links
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
