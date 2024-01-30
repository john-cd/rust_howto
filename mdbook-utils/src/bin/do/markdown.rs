use std::path::PathBuf;

use anyhow::Result;
use clap::Subcommand;
use dialoguer::Confirm;

use super::args::*;

/// Command-line subcommands to manipulate Markdown
#[derive(Subcommand, Debug)]
pub(crate) enum MarkdownSubCommand {
    /// Copy Rust code examples from the Markdown into .rs files.
    ExtractCodeExamples(SrcDirAndDestDirArgs),

    /// Replace Rust code examples from the Markdown by {{#include
    /// ...}} statements
    ReplaceCodeExamplesByIncludes(MarkdownDirArgs),

    /// Replace {{#include file.md}} by the file contents
    ReplaceIncludesByContents(MarkdownDirArgs),

    /// Generate categories.md
    GenerateCategories,

    /// Generate crates.md
    GenerateCrates,
    // TODO autoreplace autolinks / inline links by ref links
}

pub(crate) fn run(subcmd: MarkdownSubCommand) -> Result<()> {
    match subcmd {
        MarkdownSubCommand::ExtractCodeExamples(args) => {
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
            mdbook_utils::markdown::extract_code_from_all_markdown_files_in(
                markdown_src_dir_path,
                code_dest_dir_path,
            )?;
            println!("Done.");
        }
        MarkdownSubCommand::ReplaceCodeExamplesByIncludes(args) => {
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
                mdbook_utils::markdown::remove_code_from_all_markdown_files_in(
                    markdown_src_dir_path,
                )?;
                println!("Done.");
            } else {
                println!("Cancelled.");
            }
        }
        MarkdownSubCommand::ReplaceIncludesByContents(args) => {
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
                mdbook_utils::markdown::include_in_all_markdown_files_in(
                    markdown_src_dir_path,
                )?;
                println!("Done.");
            } else {
                println!("Cancelled.");
            }
        }
        _ => {
            println!("NOT IMPLEMENTED");
        }
    }
    Ok(())
}
