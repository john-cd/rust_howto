#![allow(unused)]

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Error;
use anyhow::Result;
use url::ParseError;
use url::Url;

mod args;
mod file;
mod link;
mod parser;
mod rules;

fn main() -> Result<()> {
    let cli = args::parse_arguments();
    // debug: println!("{:?}", cli);

    create_book_temp_dir()?;

    match cli.command {
        Some(args::Commands::Debug) => {
            let all_markdown: String =
                file::read_all_markdown_files_in("./src/")?;
            parser::debug_parse_to_stdout(all_markdown);
        }
        Some(args::Commands::RefDefs { path }) => {
            let pathbuf = path
                .unwrap_or(PathBuf::from("/code/book/temp/existing_refs.md"));
            println!("Writing existing reference definitions to {:?}", pathbuf);
            let all_markdown: String =
                file::read_all_markdown_files_in("./src/")?;
            parser::write_ref_defs_to(all_markdown, pathbuf)?;
        }
        Some(args::Commands::Test) => {
            let all_markdown: String = parser::get_test_markdown();
            // file::read_all_markdown_files_in("./src/")?;
            println!("{}", all_markdown);
            // TODO
            parser::extract_links(all_markdown);
        }
        // Add more subcommands here: Some(args::Commands::...) => { ... }
        None => {}
    }
    Ok(())
}

// Create temp directory
fn create_book_temp_dir() -> Result<()> {
    let dest_dir = "/code/book/temp/";
    tools::create_dir(dest_dir)?;
    Ok(())
}
