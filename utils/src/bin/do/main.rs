use std::borrow::Cow;
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
                RefDefsSubCommand::Write(pathargs) => {
                    // Create temp directory
                    utils::dir::create_dir("/code/book/temp/")?;
                    let pathbuf =
                        path_or(pathargs, "/code/book/temp/existing_refs.md");
                    println!(
                        "Writing existing reference definitions to {:?}",
                        pathbuf
                    );
                    let all_markdown =
                        utils::file::read_all_markdown_files_in("./src/")?;
                    utils::parser::write_ref_defs_to(all_markdown, pathbuf)?;
                }
                RefDefsSubCommand::GenerateBadges(pathargs) => {
                    let refdef_pathbuf =
                        path_or(pathargs, "/code/book/temp/badge_refs.md");

                    utils::dir::create_dir("/code/book/temp/")?;
                    println!(
                        "Writing reference definitions to {:?}",
                        refdef_pathbuf
                    );
                    let all_markdown =
                        utils::file::read_all_markdown_files_in("./src/")?;
                    utils::parser::generate_badges(
                        all_markdown,
                        refdef_pathbuf,
                    )?;
                } /* _ => {
                   *     println!("NOT IMPLEMENTED");
                   * } */
            }
        }
        Command::Links(subcmd) => {
            match subcmd {
                LinksSubCommand::WriteAll(pathargs) => {
                    // Create temp directory
                    utils::dir::create_dir("/code/book/temp/")?;
                    let pathbuf =
                        path_or(pathargs, "/code/book/temp/all_links.md");
                    println!("Writing existing links to {:?}", pathbuf);
                    let all_markdown =
                        utils::file::read_all_markdown_files_in("./src/")?;
                    utils::parser::write_links(all_markdown, pathbuf)?;
                }
                LinksSubCommand::WriteInline(pathargs) => {
                    // Create temp directory
                    utils::dir::create_dir("/code/book/temp/")?;
                    let pathbuf =
                        path_or(pathargs, "/code/book/temp/inline_links.md");
                    println!("Writing existing inline links to {:?}", pathbuf);
                    let all_markdown =
                        utils::file::read_all_markdown_files_in("./src/")?;
                    utils::parser::write_inline_links(all_markdown, pathbuf)?;
                } /* _ => {
                   *     println!("NOT IMPLEMENTED");
                   * } */
            }
        }
        Command::Markdown(subcmd) => match subcmd {
            args::MarkdownSubCommand::ExtractCodeExamples => {
                let code_dst_dir = "/code/deps/examples/temp/";
                utils::dir::create_dir(code_dst_dir)?;
                let path = "/code/drafts/";
                println!("Extracting examples from .md files in {:?}", path);
                utils::extract_code::extract_code_from_all_markdown_files_in(
                    path,
                    code_dst_dir,
                )?;
            }
            args::MarkdownSubCommand::RemoveCodeExamples => {
                let path = "/code/drafts/";
                utils::extract_code::remove_code_from_all_markdown_files_in(
                    path,
                )?;
            }
            args::MarkdownSubCommand::ReplaceIncludes => {
                let path = "/code/drafts/";
                utils::include::include_in_all_markdown_files_in(path)?;
            } /* _ => {
               *     println!("NOT IMPLEMENTED");
               * } */
        },
        Command::Debug(pathargs) => {
            // Create temp directory
            utils::dir::create_dir("/code/book/temp/")?;
            let pathbuf = path_or(pathargs, "/code/book/temp/debug.log");
            let all_markdown =
                Cow::from(utils::file::read_all_markdown_files_in("./src/")?);
            utils::parser::debug_parse_to(all_markdown, pathbuf)?;
        } /* Command::Test => {
           *     let all_markdown: String = parser::get_test_markdown();
           *     // Create temp directory
           *     utils::create_dir("/code/book/temp/")?;
           *     let md = parser::get_test_markdown();
           *     let path = "/code/book/temp/test.log";
           *     parser::debug_parse_to(md, path)?;
           * }
           * Add more subcommands here: Some(args::Commands::...) => { ... }
           * _ => {
           *     println!("NOT IMPLEMENTED");
           * } */
    }
    Ok(())
}

fn path_or(pathargs: args::PathArgs, s: &str) -> PathBuf {
    pathargs.path.unwrap_or(PathBuf::from(s))
}
