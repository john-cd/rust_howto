use std::env;
use std::path::PathBuf;

use anyhow::Result;
use args::*;
use dialoguer::Confirm;

mod args;

fn main() -> Result<()> {
    let key = "RUST_LOG";
    if env::var(key).is_err() {
        env::set_var(key, "debug");
    }

    tracing_subscriber::fmt::init();

    let Cli { command: cmd } = args::parse_arguments();

    match cmd {
        Command::RefDefs(subcmd) => {
            match subcmd {
                RefDefsSubCommand::Write(args) => {
                    let markdown_src_dir_path = args
                        .src
                        .markdown_src_dir_path
                        .unwrap_or(PathBuf::from("./src/"))
                        .canonicalize()?;
                    let refdef_dest_path = args.dest.file_path.unwrap_or(
                        PathBuf::from("./book/temp/existing_refs.md"),
                    );
                    println!(
                        "Parsing markdown files found in {} and writing existing reference definitions to {}...",
                        markdown_src_dir_path.display(),
                        refdef_dest_path.display()
                    );
                    utils::write_ref_defs_to(
                        markdown_src_dir_path,
                        refdef_dest_path,
                    )?;
                    println!("Done.");
                }
                RefDefsSubCommand::GenerateBadges(args) => {
                    let markdown_src_dir_path = args
                        .src
                        .markdown_src_dir_path
                        .unwrap_or(PathBuf::from("./src/"))
                        .canonicalize()?;
                    let refdef_dest_path = args
                        .dest
                        .file_path
                        .unwrap_or(PathBuf::from("./book/temp/badge_refs.md"));
                    println!(
                        "Parsing markdown files found in {} and writing new (github badge) reference definitions to {}...",
                        markdown_src_dir_path.display(),
                        refdef_dest_path.display()
                    );
                    utils::generate_badges(
                        markdown_src_dir_path,
                        refdef_dest_path,
                    )?;
                    println!("Done.");
                } /* _ => {
                   *     println!("NOT IMPLEMENTED");
                   * } */
            }
        }
        Command::Links(subcmd) => {
            match subcmd {
                LinksSubCommand::WriteAll(args) => {
                    let markdown_src_dir_path = args
                        .src
                        .markdown_src_dir_path
                        .unwrap_or(PathBuf::from("./src/"))
                        .canonicalize()?;
                    let links_dest_path = args.dest.file_path.unwrap_or(
                        PathBuf::from("/code/book/temp/all_links.md"),
                    );
                    println!(
                        "Parsing markdown files found in {} and writing existing links to {}...",
                        markdown_src_dir_path.display(),
                        links_dest_path.display()
                    );
                    utils::write_links(markdown_src_dir_path, links_dest_path)?;
                    println!("Done.");
                }
                LinksSubCommand::WriteInline(args) => {
                    let markdown_src_dir_path = args
                        .src
                        .markdown_src_dir_path
                        .unwrap_or(PathBuf::from("./src/"))
                        .canonicalize()?;
                    let links_dest_path = args.dest.file_path.unwrap_or(
                        PathBuf::from("/code/book/temp/inline_links.md"),
                    );
                    println!(
                        "Parsing markdown files found in {} and writing found inline / auto links to {}",
                        markdown_src_dir_path.display(),
                        links_dest_path.display()
                    );
                    utils::write_inline_links(
                        markdown_src_dir_path,
                        links_dest_path,
                    )?;
                    println!("Done.");
                } /* _ => {
                   *     println!("NOT IMPLEMENTED");
                   * } */
            }
        }
        Command::Markdown(subcmd) => match subcmd {
            args::MarkdownSubCommand::ExtractCodeExamples(args) => {
                let markdown_src_dir_path = args
                    .src
                    .markdown_src_dir_path
                    .unwrap_or(PathBuf::from("./drafts/"))
                    .canonicalize()?;
                let code_dest_dir_path =
                    args.dest_dir_path.unwrap_or(PathBuf::from("./temp/"));
                println!(
                    "Parsing Markdown files found in {} and copying found Rust code blocks to {}",
                    markdown_src_dir_path.display(),
                    code_dest_dir_path.display()
                );
                utils::markdown::extract_code_from_all_markdown_files_in(
                    markdown_src_dir_path,
                    code_dest_dir_path,
                )?;
                println!("Done.");
            }
            args::MarkdownSubCommand::ReplaceCodeExamplesByIncludes(args) => {
                let markdown_src_dir_path = args
                    .markdown_src_dir_path
                    .unwrap_or(PathBuf::from("./drafts/"))
                    .canonicalize()?;
                println!(
                    "About to remove Rust code examples from Markdown files in {}, replacing them with {{#include ... }} statements...",
                    markdown_src_dir_path.display()
                );
                let confirmation = Confirm::new()
                    .with_prompt("Do you want to continue?")
                    .default(false)
                    .interact()?;
                if confirmation {
                    utils::markdown::remove_code_from_all_markdown_files_in(
                        markdown_src_dir_path,
                    )?;
                    println!("Done.");
                } else {
                    println!("Cancelled.");
                }
            }
            args::MarkdownSubCommand::ReplaceIncludesByContents(args) => {
                let markdown_src_dir_path = args
                    .markdown_src_dir_path
                    .unwrap_or(PathBuf::from("./drafts/"))
                    .canonicalize()?;
                println!(
                    "About to parse Markdown files found in {} and replace any {{#include <file>.md}} statements by the corresponding file contents (excluding includes of *refs.md files)...",
                    markdown_src_dir_path.display()
                );
                let confirmation = Confirm::new()
                    .with_prompt("Do you want to continue?")
                    .default(false)
                    .interact()?;
                if confirmation {
                    utils::markdown::include_in_all_markdown_files_in(
                        markdown_src_dir_path,
                    )?;
                    println!("Done.");
                } else {
                    println!("Cancelled.");
                }
            } /* _ => {
               *     println!("NOT IMPLEMENTED");
               * } */
        },
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
            utils::debug_parse_to(markdown_src_dir_path, log_dest_path)?;
            println!("Done.");
        }
        Command::Test => {
            utils::test()?;
        } /* Add more subcommands here: Some(args::Commands::...) => { ... }
           * _ => {
           *     println!("NOT IMPLEMENTED");
           * } */
    }
    Ok(())
}
