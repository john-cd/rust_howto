//! Command-line argument parser
//!
//! Useful links:
//! <https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html>
//!
//! <https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html>
//!
//! <https://github.com/clap-rs/clap/tree/master/examples>
#![allow(dead_code)]

use clap::Parser;
use clap::Subcommand;

use super::args::*;
use crate::links_commands::LinksSubCommand;
use crate::markdown_commands::MarkdownSubCommand;
use crate::refdefs_commands::RefDefsSubCommand;

/// Parse command-line arguments
pub(crate) fn parse_arguments() -> Cli {
    Cli::parse()
}

#[derive(Parser, Debug)]
// Reads the following attributes from the package's `Cargo.toml`
#[command(author, version, about, long_about = None)]
// Displays the help, if no arguments are provided
// #[command(arg_required_else_help = true)]
/// Command-line arguments
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
    // This structure allows the addition of global options, if needed
    //#[clap(flatten)]
    // global_opts: GlobalOpts,
}

/// Command-line commands
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Manage reference definitions
    #[command(subcommand, name = "refdefs")]
    RefDefs(RefDefsSubCommand),

    /// Manage links
    #[command(subcommand)]
    Links(LinksSubCommand),

    /// Manage code blocks (embedded examples) and includes
    #[command(subcommand)]
    Markdown(MarkdownSubCommand),

    /// Generate a sitemap.xml file from the list of Markdown files
    /// in a source directory
    #[command(name = "sitemap")]
    SiteMap(MarkdownSrcDirUrlAndDestFileArgs),

    /// Parse the entire Markdown code as events
    /// and write them to a file.
    Debug(MarkdownSrcDirAndDestFileArgs),

    /// Test Markdown parsing
    #[command(skip)]
    Test,
}
