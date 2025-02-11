// use anyhow::Context;
// use anyhow::Result;
// use pulldown_cmark::BrokenLink;
// use pulldown_cmark::Options;
// use pulldown_cmark::Parser;
// use std::fs::File;
// use std::io::BufWriter;
// use std::io::Write;
// use std::path::Path;
// use tracing::debug;
// use tracing::error;
// use tracing::info;
// use tracing::trace;
// use tracing::warn;
// mod fs;

// // TODO
// pub fn detect_unused_reference_definitions(content: &str) {}

// // TODO
// pub fn delete_unused_reference_definitions(
//     content: &str,
//     unused_refs: Vec<BrokenLink<'static>>,
// ) -> String {
//     content.to_string()
// }

// fn detect_broken_links(content: &str) -> Vec<BrokenLink<'static>> {
//     let mut broken_links = vec![];
//     parse_markdown(content, &mut broken_links);
//     if !broken_links.is_empty() {
//         error!("Error: {} broken links:", broken_links.len());
//     }
//     for link in &broken_links {
//         let start = link.span.start;
//         let end = link.span.end;
//         let reference = &link.reference;
//         error!("[{start}-{end}]: {reference}");
//     }
//     broken_links
// }

// fn parse_markdown(content: &str, broken_links: &mut Vec<BrokenLink<'static>>) {
//     // Create a parser with our callback function for broken links.
//     // - In case the parser encounters any potential links that have a broken
//     // reference (e.g `[foo]` when there is no `[foo]: ` entry at the bottom)
//     // the provided callback will be called with the reference name,
//     // and the returned pair will be used as the link URL and title if it is not
//     // `None`.
//     // - Options:all() includes ENABLE_TABLES, ENABLE_FOOTNOTES, ENABLE_STRIKETHROUGH, ENABLE_TASKLISTS, ENABLE_SMART_PUNCTUATION, ENABLE_HEADING_ATTRIBUTES, etc
//     let parser = pulldown_cmark::Parser::new_with_broken_link_callback(
//         content,
//         Options::all(),
//         Some(|link: BrokenLink<'_>| {
//             broken_links.push(link.into_static());
//             Some(("http://TODO".into(), ":BROKEN_LINK:".into()))
//             // or None
//         }),
//     );
//     let count = parser.count();
//     info!("Parser: {} events", count);
// }

// /// Write reference definitions parsed from a Markdown parser to a
// /// file / writer.
// ///
// /// parser: Markdown parser.
// ///
// /// w: Writer e.g. File
// pub(crate) fn write_refdefs_to<W>(parser: &mut Parser<'_>, w: &mut W) -> Result<()>
// where
//     W: Write,
// {
//     let sorted_linkdefs: std::collections::BTreeMap<_, _> =
//         parser.reference_definitions().iter().collect();

//     for (s, linkdef) in sorted_linkdefs {
//         if let Some(t) = &linkdef.title {
//             writeln!(w, "[{s}]: {} \"{t:?}\"", linkdef.dest)?;
//         } else {
//             writeln!(w, "[{s}]: {}", linkdef.dest)?;
//         }
//     }
//     Ok(())
// }

// // / Helper function:
// // /
// // / Checks if the source directory exists,
// // / create the destination directory if it doesn't exist,
// // / create the destination file,
// // / parse all the Markdown files in the source directory,
// // / and invoke a closure that uses the parser to write to the file.
// // fn helper<P1, P2, F>(src_dir_path: P1, dest_file_path: P2, func: F) -> Result<()>
// // where
// //     P1: AsRef<Path>,
// //     P2: AsRef<Path>,
// //     F: for<'a, 'b> FnOnce(&'a mut Parser<'a>, &'b mut File) -> Result<()>,
// // {
// //     let src_dir_path = fs::check_is_dir(src_dir_path)?;

// //     fs::create_parent_dir_for(dest_file_path.as_ref())?;

// //     let mut f = File::create(dest_file_path.as_ref()).with_context(|| {
// //         format!("[helper] Could not create file {}", dest_file_path.as_ref().display())
// //     })?;

// //     let all_markdown = fs::read_to_string_all_markdown_files_in(src_dir_path)?;
// //     let mut parser = parser::get_parser(all_markdown.as_ref());

// //     func(&mut parser, &mut f)?;
// //     Ok(())
// // }
