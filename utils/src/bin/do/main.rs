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
                    utils::fs::create_dir("/code/book/temp/")?;
                    let pathbuf =
                        path_or(pathargs, "/code/book/temp/existing_refs.md");
                    println!(
                        "Writing existing reference definitions to {:?}",
                        pathbuf
                    );
                    let all_markdown =
                        utils::fs::read_to_string_all_markdown_files_in(
                            "./src/",
                        )?;
                    utils::write_ref_defs_to(all_markdown, pathbuf)?;
                }
                RefDefsSubCommand::GenerateBadges(pathargs) => {
                    let refdef_pathbuf =
                        path_or(pathargs, "/code/book/temp/badge_refs.md");

                    utils::fs::create_dir("/code/book/temp/")?;
                    println!(
                        "Writing reference definitions to {:?}",
                        refdef_pathbuf
                    );
                    let all_markdown =
                        utils::fs::read_to_string_all_markdown_files_in(
                            "./src/",
                        )?;
                    utils::generate_badges(all_markdown, refdef_pathbuf)?;
                } /* _ => {
                   *     println!("NOT IMPLEMENTED");
                   * } */
            }
        }
        Command::Links(subcmd) => {
            match subcmd {
                LinksSubCommand::WriteAll(pathargs) => {
                    // Create temp directory
                    utils::fs::create_dir("/code/book/temp/")?;
                    let pathbuf =
                        path_or(pathargs, "/code/book/temp/all_links.md");
                    println!("Writing existing links to {:?}", pathbuf);
                    let all_markdown =
                        utils::fs::read_to_string_all_markdown_files_in(
                            "./src/",
                        )?;
                    utils::write_links(all_markdown, pathbuf)?;
                }
                LinksSubCommand::WriteInline(pathargs) => {
                    // Create temp directory
                    utils::fs::create_dir("/code/book/temp/")?;
                    let pathbuf =
                        path_or(pathargs, "/code/book/temp/inline_links.md");
                    println!("Writing existing inline links to {:?}", pathbuf);
                    let all_markdown =
                        utils::fs::read_to_string_all_markdown_files_in(
                            "./src/",
                        )?;
                    utils::write_inline_links(all_markdown, pathbuf)?;
                } /* _ => {
                   *     println!("NOT IMPLEMENTED");
                   * } */
            }
        }
        Command::Markdown(subcmd) => match subcmd {
            args::MarkdownSubCommand::ExtractCodeExamples => {
                let code_dst_dir = "/code/deps/examples/temp/";
                utils::fs::create_dir(code_dst_dir)?;
                let src_dir = "/code/drafts/";
                println!("Extracting examples from .md files in {:?}", src_dir);
                utils::markdown::extract_code_from_all_markdown_files_in(
                    src_dir,
                    code_dst_dir,
                )?;
            }
            args::MarkdownSubCommand::RemoveCodeExamples => {
                let path = "/code/drafts/";
                utils::markdown::remove_code_from_all_markdown_files_in(path)?;
            }
            args::MarkdownSubCommand::ReplaceIncludes => {
                let path = "/code/drafts/";
                utils::markdown::include_in_all_markdown_files_in(path)?;
            } /* _ => {
               *     println!("NOT IMPLEMENTED");
               * } */
        },
        Command::Debug(pathargs) => {
            let src_dir = "./src/";
            let dest_pathbuf = path_or(pathargs, "/code/book/temp/debug.log");
            utils::debug_parse_to(src_dir, dest_pathbuf)?;
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
