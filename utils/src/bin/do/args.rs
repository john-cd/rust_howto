//! Command-line argument parser
//!
//! Useful links:
//! https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
//! https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html
//! https://github.com/clap-rs/clap/tree/master/examples

use std::path::PathBuf;

use clap::Args;
use clap::Parser;
use clap::Subcommand;

pub(crate) fn parse_arguments() -> Cli {
    Cli::parse()
}

#[derive(Args, Debug)]
pub struct PathArgs {
    // The path to the file to write (optional)
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

#[derive(Parser, Debug)]
// Reads the following attributes from the package's `Cargo.toml`
#[command(author, version, about, long_about = None)]
// Displays the help, if no arguments are provided
// #[command(arg_required_else_help = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
    // This structure allows the addition of global options, if needed
    //#[clap(flatten)]
    // global_opts: GlobalOpts,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Manage reference definitions
    #[command(subcommand)]
    RefDefs(RefDefsSubCommand),

    /// Manage links
    #[command(subcommand)]
    Links(LinksSubCommand),

    /// Manage code examples and includes
    #[command(subcommand)]
    Markdown(MarkdownSubCommand),

    /// Parse the entire Markdown code as events and print them.
    Debug(PathArgs),
    // Test,
}

#[derive(Subcommand, Debug)]
pub(crate) enum RefDefsSubCommand {
    /// Write existing reference definitions to a file
    Write(PathArgs),

    /// Generate badges (reference definitions) for e.g. Github links
    GenerateBadges(PathArgs),
}

#[derive(Subcommand, Debug)]
pub(crate) enum LinksSubCommand {
    /// Write all existing links to a Markdown file
    WriteAll(PathArgs),

    // TODO finish
    /// Write all existing inline links and autolinks (i.e., not
    /// written as reference-style links) to a Markdown file
    WriteInline(PathArgs),
    // // TODO
    // /// Identify duplicate links / labels
    // DuplicateLinks,

    // // TODO
    // /// Identify broken links (i.e. without reference definition)
    // BrokenLinks,
}

#[derive(Subcommand, Debug)]
pub(crate) enum MarkdownSubCommand {
    /// Extract Rust code examples from the Markdown
    ExtractCodeExamples,

    /// Rust code examples from the Markdown
    RemoveCodeExamples,

    /// Replace {{#include <file>.md}} by the file contents
    ReplaceIncludes,
    // TODO
    // /// Generate categories.md
    // GenerateCategories,

    // TODO
    // /// Generate crates.md
    // GenerateCrates,

    // TODO autoreplace autolinks / inline links by ref links
}

// // Example global args

// #[derive(Debug, Args)]
// struct GlobalOpts {
//     /// Color
//     #[clap(long, arg_enum, global = true, default_value_t =
// Color::Auto)]     color: Color,

//     /// Verbosity level (can be specified multiple times)
//     #[clap(long, short, global = true, parse(from_occurrences))]
//     verbose: usize,
//     //... other global options
// }

// #[derive(Clone, Debug, ArgEnum)]
// enum Color {
//     Always,
//     Auto,
//     Never,
// }
