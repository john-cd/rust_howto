//! Command-line argument parser
//!
//! Useful links:
//! <https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html>
//!
//! <https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html>
//!
//! <https://github.com/clap-rs/clap/tree/master/examples>

use std::path::PathBuf;

use anyhow::anyhow;
use clap::Args;
use clap::Parser;
use clap::Subcommand;

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
    #[command(subcommand)]
    RefDefs(RefDefsSubCommand),

    /// Manage links
    #[command(subcommand)]
    Links(LinksSubCommand),

    /// Manage code blocks (embedded examples) and includes
    #[command(subcommand)]
    Markdown(MarkdownSubCommand),

    /// Generate a sitemap.xml file from the list of Markdown files
    /// in a source directory
    SiteMap(SrcDirAndUrlAndDestFileArgs),

    /// Parse the entire Markdown code as events
    /// and write them to a file.
    Debug(SrcDirAndDestFileArgs),

    /// Test Markdown parsing
    Test,
}

/// Command-line subcommands to handle reference definitions
#[derive(Subcommand, Debug)]
pub(crate) enum RefDefsSubCommand {
    /// Write existing reference definitions to a file
    Write(SrcDirAndDestFileArgs),

    /// Generate badges (reference definitions) for e.g. Github links
    GenerateBadges(SrcDirAndDestFileArgs),
}

/// Command-line subcommands to handle links
#[derive(Subcommand, Debug)]
pub(crate) enum LinksSubCommand {
    /// Write all existing links to a Markdown file
    WriteAll(SrcDirAndDestFileArgs),

    // TODO finish
    /// Write all existing inline links and autolinks (i.e., not
    /// written as reference-style links) to a Markdown file
    WriteInline(SrcDirAndDestFileArgs),
    // // TODO
    // /// Identify duplicate links / labels
    // DuplicateLinks,

    // // TODO
    // /// Identify broken links (i.e. without reference definition)
    // BrokenLinks,
}

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
    // TODO
    // /// Generate categories.md
    // GenerateCategories,

    // TODO
    // /// Generate crates.md
    // GenerateCrates,

    // TODO autoreplace autolinks / inline links by ref links
}

#[derive(Debug, Args)]
#[command(flatten_help = true)]
pub(crate) struct SrcDirAndDestFileArgs {
    /// Source directory containing the Markdown files (optional)
    #[command(flatten)]
    pub(crate) src: MarkdownDirArgs,

    /// The path to the file to write (optional)
    #[command(flatten)]
    pub(crate) dest: DestFilePathArgs,
}

#[derive(Args, Debug)]
pub(crate) struct SrcDirAndDestDirArgs {
    /// Source directory containing the Markdown files (optional)
    #[command(flatten)]
    pub(crate) src: MarkdownDirArgs,

    /// Destination directory (optional)
    #[arg(short='t', long="target-dir", value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    pub(crate) dest_dir_path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub(crate) struct DestFilePathArgs {
    /// Specify the path of the file to create
    #[arg(short = 'o', long = "output", value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
    pub(crate) file_path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub(crate) struct MarkdownDirArgs {
    /// Specify the path to the directory containing the source
    /// Markdown files
    #[arg(short, long="markdown-dir", value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    pub(crate) markdown_src_dir_path: Option<PathBuf>,
}

#[derive(Debug, Args)]
#[command(flatten_help = true)]
pub(crate) struct SrcDirAndUrlAndDestFileArgs {
    /// Source directory containing the Markdown files (optional)
    #[command(flatten)]
    pub(crate) src: MarkdownDirArgs,

    #[arg(short='b', long="base-url", value_name = "URL", value_parser = parse_url)]
    pub(crate) base_url: Option<url::Url>,

    /// The path to the file to write (optional)
    #[command(flatten)]
    pub(crate) dest: DestFilePathArgs,
}

/// Parse a URL
fn parse_url(
    s: &str,
) -> Result<url::Url, Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Parse an absolute URL from a string.
    let url =
        url::Url::parse(s).or_else(|_| Err(anyhow!("Invalid URL: {s}")))?;
    Ok(url)
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
