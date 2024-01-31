use std::path::PathBuf;

use anyhow::Result;
use clap::Subcommand;

use super::args::*;

/// Command-line subcommands to handle reference definitions
#[derive(Subcommand, Debug)]
pub(crate) enum RefDefsSubCommand {
    /// Write existing reference definitions to a file
    Write(MarkdownSrcDirAndDestFileArgs),

    // TODO merge with generation from dependencies?
    /// Generate badges (reference definitions) for e.g. Github links
    #[command(name="badges")]
    GenerateBadges(MarkdownSrcDirAndDestFileArgs),

    /// Generate reference definitions
    /// from the dependencies of the code examples
    /// and merge them with those found in the Markdown source directory
    #[command(name="from-dependencies")]
    GenerateFromDependencies(DependenciesDirAndDestFileArgs),
}

pub(crate) fn run(subcmd: RefDefsSubCommand) -> Result<()> {
    match subcmd {
        RefDefsSubCommand::Write(args) => {
            let markdown_src_dir_path = args
                .src
                .markdown_dir_path
                .unwrap_or(PathBuf::from("./src/"))
                .canonicalize()?;
            let refdef_dest_path = args
                .dest
                .file_path
                .unwrap_or(PathBuf::from("./book/temp/existing_refs.md"));
            println!(
                "Parsing markdown files found in {} and writing existing reference definitions to {}...",
                markdown_src_dir_path.display(),
                refdef_dest_path.display()
            );
            mdbook_utils::write_ref_defs_to(
                markdown_src_dir_path,
                refdef_dest_path,
            )?;
            println!("Done.");
        }
        RefDefsSubCommand::GenerateBadges(args) => {
            let markdown_src_dir_path = args
                .src
                .markdown_dir_path
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
            mdbook_utils::generate_badges(
                markdown_src_dir_path,
                refdef_dest_path,
            )?;
            println!("Done.");
        }
        RefDefsSubCommand::GenerateFromDependencies(args) => {
            let markdown_src_dir_path = args
                .src
                .markdown_dir_path
                .unwrap_or(PathBuf::from("./src/"))
                .canonicalize()?;
            let cargo_toml_dir_path = args
                .manifest
                .cargo_toml_dir_path
                .unwrap_or(PathBuf::from("."))
                .canonicalize()?;
            let refdef_dest_file_path = args
                .dest
                .file_path
                .unwrap_or(PathBuf::from("./book/temp/dependencies_refs.md"));
            println!(
                "Creating reference definitions in {} from the manifest in {} and Markdown sources in {}...",
                refdef_dest_file_path.display(),
                cargo_toml_dir_path.display(),
                markdown_src_dir_path.display()
            );
            mdbook_utils::generate_refdefs_to(
                cargo_toml_dir_path,
                markdown_src_dir_path,
                refdef_dest_file_path,
            )?;
            println!("Done.");
        } /* _ => {
           *     println!("NOT IMPLEMENTED");
           * } */
    }
    Ok(())
}
