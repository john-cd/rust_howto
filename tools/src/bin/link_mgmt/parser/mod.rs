mod link;
mod parse;
mod rules;
mod test;
mod write;

use std::path::Path;

use anyhow::Result;
use pulldown_cmark::BrokenLink;
use pulldown_cmark::CowStr;
use pulldown_cmark::Options;
use pulldown_cmark::Parser;
pub use test::*;

// Private Functions

fn get_options() -> Options {
    // Set up options and parser.
    // Strikethroughs, etc... are not part of the CommonMark standard
    // and we therefore must enable them explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TABLES);
    options
}

// In case the parser encounters any potential links that have a
// broken reference (e.g [foo] when there is no [foo]:  entry at the
// bottom) the provided callback will be called with the
// reference name, and the returned pair will be used as the link name
// and title if it is not None.
fn _callback<'input>(
    broken_link: BrokenLink<'input>,
    markdown_input: &'input str,
) -> Option<(CowStr<'input>, CowStr<'input>)> {
    println!(
        "Issue with the markdown: reference: {}, `{}`, type: {:?}",
        broken_link.reference,
        &markdown_input[broken_link.span],
        broken_link.link_type,
    );
    Some(("https://TODO".into(), "".into()))
}

// Public Functions

// Parse the Markdown as events and print them all.
pub fn debug_parse_to<S: AsRef<str>, P: AsRef<Path>>(
    markdown_input: S,
    path: P,
) -> Result<()> {
    println!("\nParsing markdown ---------------\n");
    let f = std::fs::File::create(path)?;
    let parser = Parser::new_ext(markdown_input.as_ref(), get_options());
    parse::debug_parse_to(parser, f)?;
    Ok(())
}

pub fn write_ref_defs_to<S: AsRef<str>, P: AsRef<Path>>(
    markdown_input: S,
    path: P,
) -> Result<()> {
    let f = std::fs::File::create(path)?;
    let parser = Parser::new_ext(markdown_input.as_ref(), get_options());
    write::write_ref_defs(&parser, f)?;
    Ok(())
}

// pub fn write_inline_links<S: AsRef<str>, P: AsRef<Path>>(
//     markdown_input: S,
//     path: P,
// ) -> Result<()> {
//     let parser = Parser::new_ext(markdown_input.as_ref(),
// get_options());     // let parser = parser.filter(|event| {
//     //     match event {
//     //     }
//     // });
//     let mut f = std::fs::File::create(path)?;
//     // TODO
//     Ok(())
// }

// pub fn write_links<S: AsRef<str>, P: AsRef<Path>>(
//     markdown_input: S,
//     path: P,
// ) -> Result<()> {
//     let parser = Parser::new_ext(markdown_input.as_ref(),
// get_options());

//     let mut f = std::fs::File::create(path)?;
//     // TODO
//     let links: Vec<String> = vec![];
//     if !links.is_empty() {
//         for l in links {
//             writeln!(&mut f, "{:?}", l)?;
//         }
//     }
//     Ok(())
// }

//// Set up the parser. We can treat is as any other iterator.
//// For each event, we print its details, such as the tag or string.
// let parser = Parser::new_with_broken_link_callback(
//     markdown_input.as_ref(),
//     get_options(),
//     Some(&mut |broken_link: BrokenLink| { callback(broken_link,
// markdown_input.as_ref()) }), )
