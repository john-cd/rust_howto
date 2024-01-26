use std::env;
use std::path::PathBuf;

use anyhow::Result;
use args::*;

mod args;

fn main() -> Result<()> {
    let key = "RUST_LOG";
    if env::var(key).is_err() {
        env::set_var(key, "info");
    }

    tracing_subscriber::fmt::init();

    let Cli { command: cmd } = args::parse_arguments();

    match cmd {
        Command::RefDefs(subcmd) => {
            match subcmd {
                RefDefsSubCommand::Write(args) => {
                    let markdown_src_dir_path = args
                        .src
                        .markdown_dir_path
                        .unwrap_or(PathBuf::from("./src/"));
                    let refdef_dest_path = args.dest.file_path.unwrap_or(
                        PathBuf::from("./book/temp/existing_refs.md"),
                    );
                    println!(
                        "Writing existing reference definitions to {:?}",
                        refdef_dest_path
                    );
                    utils::write_ref_defs_to(
                        markdown_src_dir_path,
                        refdef_dest_path,
                    )?;
                }
                RefDefsSubCommand::GenerateBadges(args) => {
                    let markdown_src_dir_path = args
                        .src
                        .markdown_dir_path
                        .unwrap_or(PathBuf::from("./src/"));
                    let refdef_dest_path = args
                        .dest
                        .file_path
                        .unwrap_or(PathBuf::from("./book/temp/badge_refs.md"));
                    println!(
                        "Writing new (github badge) reference definitions to {:?}",
                        refdef_dest_path
                    );
                    utils::generate_badges(
                        markdown_src_dir_path,
                        refdef_dest_path,
                    )?;
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
                        .markdown_dir_path
                        .unwrap_or(PathBuf::from("./src/"));
                    let links_dest_path = args.dest.file_path.unwrap_or(
                        PathBuf::from("/code/book/temp/all_links.md"),
                    );
                    println!("Writing existing links to {:?}", links_dest_path);
                    utils::write_links(markdown_src_dir_path, links_dest_path)?;
                }
                LinksSubCommand::WriteInline(args) => {
                    let markdown_src_dir_path = args
                        .src
                        .markdown_dir_path
                        .unwrap_or(PathBuf::from("./src/"));
                    let links_dest_path = args.dest.file_path.unwrap_or(
                        PathBuf::from("/code/book/temp/inline_links.md"),
                    );
                    println!(
                        "Writing existing inline / auto links to {:?}",
                        links_dest_path
                    );
                    utils::write_inline_links(
                        markdown_src_dir_path,
                        links_dest_path,
                    )?;
                } /* _ => {
                   *     println!("NOT IMPLEMENTED");
                   * } */
            }
        }
        Command::Markdown(subcmd) => match subcmd {
            args::MarkdownSubCommand::ExtractCodeExamples(args) => {
                let markdown_src_dir_path = args
                    .src
                    .markdown_dir_path
                    .unwrap_or(PathBuf::from("/code/drafts/"));
                println!(
                    "Extracting all the Rust code examples from Markdown files in {:?} into .rs files (no code removal)",
                    markdown_src_dir_path
                );
                let code_dest_dir_path = args
                    .dest_dir_path
                    .unwrap_or(PathBuf::from("/code/deps/examples/temp/"));
                utils::markdown::extract_code_from_all_markdown_files_in(
                    markdown_src_dir_path,
                    code_dest_dir_path,
                )?;
            }
            args::MarkdownSubCommand::ReplaceCodeExamplesByIncludes(args) => {
                // TODO confirm with user
                let markdown_src_dir_path = args
                    .markdown_dir_path
                    .unwrap_or(PathBuf::from("/code/drafts/"));
                println!(
                    "Removing Rust code examples from Markdown files in {:?}, replacing them with {{#include ... }} statements.",
                    markdown_src_dir_path
                );
                utils::markdown::remove_code_from_all_markdown_files_in(
                    markdown_src_dir_path,
                )?;
            }
            args::MarkdownSubCommand::ReplaceIncludesByContents(args) => {
                let markdown_src_dir_path = args
                    .markdown_dir_path
                    .unwrap_or(PathBuf::from("/code/drafts/"));
                println!(
                    "Replacing {{#include <file>.md}} by the corresponding file contents (excluding refs/link-refs.md and similar)"
                );
                utils::markdown::include_in_all_markdown_files_in(
                    markdown_src_dir_path,
                )?;
            } /* _ => {
               *     println!("NOT IMPLEMENTED");
               * } */
        },
        Command::Debug(args) => {
            let markdown_src_dir = args
                .src
                .markdown_dir_path
                .unwrap_or(PathBuf::from("./src/"));
            let log_dest_pathbuf = args
                .dest
                .file_path
                .unwrap_or(PathBuf::from("/code/book/temp/debug.log"));
            utils::debug_parse_to(markdown_src_dir, log_dest_pathbuf)?;
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
