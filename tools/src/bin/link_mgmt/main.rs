#![allow(unused)]

use std::path::PathBuf;

use anyhow::Result;

mod args;
mod extract;
mod file;
mod include;
mod parser;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = args::parse_arguments();

    match cli.command {
        Some(args::Commands::Debug(pathargs)) => {
            // Create temp directory
            tools::create_dir("/code/book/temp/")?;
            let pathbuf = path_or(pathargs, "/code/book/temp/debug.log");
            let all_markdown: String =
                file::read_all_markdown_files_in("./src/")?;
            parser::debug_parse_to(all_markdown, pathbuf)?;
        }
        Some(args::Commands::RefDefs(pathargs)) => {
            // Create temp directory
            tools::create_dir("/code/book/temp/")?;
            let pathbuf = path_or(pathargs, "/code/book/temp/existing_refs.md");
            println!("Writing existing reference definitions to {:?}", pathbuf);
            let all_markdown: String =
                file::read_all_markdown_files_in("./src/")?;
            parser::write_ref_defs_to(all_markdown, pathbuf)?;
        }
        Some(args::Commands::InlineLinks(pathargs)) => {
            // Create temp directory
            tools::create_dir("/code/book/temp/")?;
            let pathbuf = path_or(pathargs, "/code/book/temp/inline_links.md");
            println!("Writing existing inline links to {:?}", pathbuf);
            let all_markdown: String =
                file::read_all_markdown_files_in("./src/")?;
            parser::write_inline_links(all_markdown, pathbuf)?;
        }
        Some(args::Commands::Links(pathargs)) => {
            // Create temp directory
            tools::create_dir("/code/book/temp/")?;
            let pathbuf = path_or(pathargs, "/code/book/temp/all_links.md");
            println!("Writing existing links to {:?}", pathbuf);
            let all_markdown: String =
                file::read_all_markdown_files_in("./src/")?;
            parser::write_links(all_markdown, pathbuf)?;
        }
        Some(args::Commands::Test) => {
            let all_markdown: String = parser::get_test_markdown();
            // println!("{}", all_markdown);
            // TODO
            // Create temp directory
            tools::create_dir("/code/book/temp/")?;
            let md = parser::get_test_markdown();
            let path = "/code/book/temp/test.log";
            parser::debug_parse_to(md, path)?;
        }
        Some(args::Commands::ExtractExamples) => {
            let code_dst_dir = "/code/deps/examples/temp/";
            tools::create_dir(code_dst_dir)?;
            let path = "/code/drafts/";
            println!("Extracting examples from .md files in {:?}", path);
            extract::extract_code_from_all_markdown_files_in(
                path,
                code_dst_dir,
            )?;
        }
        Some(args::Commands::RemoveCode) => {
            let path = "/code/drafts/";
            extract::remove_code_from_all_markdown_files_in(path);
        }
        Some(args::Commands::RemoveCode) => {
            let path = "/code/drafts/";
            extract::remove_code_from_all_markdown_files_in(path);
        }
        Some(args::Commands::IncludeMarkdown) => {
            let path = "/code/src/";
            include::include_in_all_markdown_files_in(path);
        }

        // Add more subcommands here: Some(args::Commands::...) => { ... }
        _ => {}
    }
    Ok(())
}

fn path_or(pathargs: args::PathArgs, s: &str) -> PathBuf {
    pathargs.path.unwrap_or(PathBuf::from(s))
}
