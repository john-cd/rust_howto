#![allow(unused)]

use std::path::PathBuf;

use anyhow::Result;

mod args;
mod file;
mod parser;
mod extract;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = args::parse_arguments();

    // Create temp directory
    tools::create_dir("/code/book/temp/")?;

    let all_markdown: String = file::read_all_markdown_files_in("./src/")?;

    match cli.command {
        Some(args::Commands::Debug(pathargs)) => {
            let pathbuf = path_or(pathargs, "/code/book/temp/debug.log");
            parser::debug_parse_to(all_markdown, pathbuf)?;
        }
        Some(args::Commands::RefDefs(pathargs)) => {
            let pathbuf = path_or(pathargs, "/code/book/temp/existing_refs.md");
            println!("Writing existing reference definitions to {:?}", pathbuf);
            parser::write_ref_defs_to(all_markdown, pathbuf)?;
        }
        Some(args::Commands::InlineLinks(pathargs)) => {
            let pathbuf = path_or(pathargs, "/code/book/temp/inline_links.md");
            println!("Writing existing inline links to {:?}", pathbuf);
            parser::write_inline_links(all_markdown, pathbuf)?;
        }
        Some(args::Commands::Links(pathargs)) => {
            let pathbuf = path_or(pathargs, "/code/book/temp/all_links.md");
            println!("Writing existing links to {:?}", pathbuf);
            parser::write_links(all_markdown, pathbuf)?;
        }
        Some(args::Commands::Test) => {
            let all_markdown: String = parser::get_test_markdown();
            // println!("{}", all_markdown);
            // TODO
            let md = parser::get_test_markdown();
            let path = "/code/book/temp/test.log";
            parser::debug_parse_to(md, path)?;
        }
        Some(args::Commands::ExtractExamples) => {
            let path = "/code/drafts/";
            println!("Extracting examples from .md files in {:?}", path);
            extract::extract_code_from_all_markdown_files_in(path)?;
        }
        // Add more subcommands here: Some(args::Commands::...) => { ... }
        _ => {}
    }
    Ok(())
}

fn path_or(pathargs: args::PathArgs, s: &str) -> PathBuf {
    pathargs.path.unwrap_or(PathBuf::from(s))
}
