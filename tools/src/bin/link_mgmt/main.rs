#![allow(unused)]

use std::collections::HashMap;
use std::path::Path;

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

    match &cli.command {
        Some(args::Commands::Parse) => {
            let all_markdown: String =
                file::read_all_markdown_files_in("./src/")?;
            // parser::debug_parse_to_stdout(all_markdown);
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
