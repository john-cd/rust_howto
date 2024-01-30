use std::env;
use std::path::PathBuf;

use anyhow::Result;
use cli::*;

mod args;
mod cli;
mod links;
mod markdown;
mod refdefs;

fn main() -> Result<()> {
    let key = "RUST_LOG";
    if env::var(key).is_err() {
        env::set_var(key, "debug");
    }

    tracing_subscriber::fmt::init();

    let Cli { command: cmd } = cli::parse_arguments();

    match cmd {
        Command::RefDefs(subcmd) => {
            refdefs::run(subcmd)?;
        }
        Command::Links(subcmd) => {
            links::run(subcmd)?;
        }
        Command::Markdown(subcmd) => {
            markdown::run(subcmd)?;
        }
        Command::SiteMap(args) => {
            let markdown_src_dir_path = args
                .src
                .markdown_src_dir_path
                .unwrap_or(PathBuf::from("./src/"))
                .canonicalize()?;

            let base_url = args
                .base_url
                .unwrap_or(url::Url::parse("https://john-cd.com/rust_howto/")?);

            let sitemap_dest_file_path = args
                .dest
                .file_path
                .unwrap_or(PathBuf::from("./book/html/sitemap.xml"));

            println!(
                "Generating {} from the list of Markdown files in {}...",
                sitemap_dest_file_path.display(),
                markdown_src_dir_path.display(),
            );
            mdbook_utils::generate_sitemap(
                markdown_src_dir_path,
                base_url,
                sitemap_dest_file_path,
            )?;
            println!("Done.");
        }
        Command::Debug(args) => {
            let markdown_src_dir_path = args
                .src
                .markdown_src_dir_path
                .unwrap_or(PathBuf::from("./src/"))
                .canonicalize()?;
            let log_dest_path = args
                .dest
                .file_path
                .unwrap_or(PathBuf::from("./book/temp/debug.log"));
            println!(
                "Parsing Markdown files found in {} and writing raw events to {}...",
                markdown_src_dir_path.display(),
                log_dest_path.display()
            );
            mdbook_utils::debug_parse_to(markdown_src_dir_path, log_dest_path)?;
            println!("Done.");
        }
        Command::Test => {
            mdbook_utils::test()?;
            println!("Done.");
        } /* Add more subcommands here: Some(args::Commands::...) => { ... }
           * _ => {
           *     println!("NOT IMPLEMENTED");
           * } */
    }
    Ok(())
}
