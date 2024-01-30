use std::path::PathBuf;

use anyhow::Result;
use clap::Subcommand;

use super::args::*;

/// Command-line subcommands to handle reference definitions
#[derive(Subcommand, Debug)]
pub(crate) enum RefDefsSubCommand {
    /// Write existing reference definitions to a file
    Write(SrcDirAndDestFileArgs),

    /// Generate badges (reference definitions) for e.g. Github links
    GenerateBadges(SrcDirAndDestFileArgs),

    /// Generate reference definitions from code examples'
    /// dependencies
    GenerateRefDefs(SrcDirAndDestFileArgs),
}

pub(crate) fn run(subcmd: RefDefsSubCommand) -> Result<()> {
    match subcmd {
        RefDefsSubCommand::Write(args) => {
            let markdown_src_dir_path = args
                .src
                .markdown_src_dir_path
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
            mdbook_utils::generate_badges(
                markdown_src_dir_path,
                refdef_dest_path,
            )?;
            println!("Done.");
        }
        RefDefsSubCommand::GenerateRefDefs(args) => {
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
        } /* _ => {
           *     println!("NOT IMPLEMENTED");
           * } */
    }
    Ok(())
}

// fn gen_ref_defs() -> Result<()> {
//     let root_path = std::fs::canonicalize("..")?;
//     let deps_path = root_path.join("deps/");
//     if !deps_path.exists() {
//         bail!("The folder {:?} does not exist.", deps_path);
//     }
//     let refdef_dest_path = "/code/book/temp/merged_ref_defs.md";

//     mdbook_utils::generate_refdefs_to(
//         &deps_path,
//         "/code/src",
//         refdef_dest_path,
//     )?;

//     Ok(())
// }
