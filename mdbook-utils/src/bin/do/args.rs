use std::path::PathBuf;

use anyhow::anyhow;
use clap::Args;

/// Destination file
#[derive(Args, Debug)]
pub(crate) struct DestFileArgs {
    /// Path of the file to create
    #[arg(short = 'o', long = "output", value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
    pub(crate) file_path: Option<PathBuf>,
}

/// Destination directory
#[derive(Args, Debug)]
pub(crate) struct DestDirArgs {
    /// Destination directory
    #[arg(short = 't', long = "target-dir", value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    pub(crate) dir_path: Option<PathBuf>,
}

/// Markdown source directory
#[derive(Args, Debug)]
pub(crate) struct MarkdownDirArgs {
    /// Source directory containing the source Markdown files
    #[arg(short = 'm', long = "markdown-dir", value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    pub(crate) markdown_dir_path: Option<PathBuf>,
}

/// Directory containing Cargo.toml
#[derive(Args, Debug)]
pub(crate) struct CargoTomlDirArgs {
    /// Path to the directory containing Cargo.toml
    #[arg(short = 'c', long = "cargo-toml-dir", value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    pub(crate) cargo_toml_dir_path: Option<PathBuf>,
}

/// URL
#[derive(Args, Debug)]
pub(crate) struct UrlArgs {
    #[arg(short='b', long="base-url", value_name = "URL", value_parser = parse_url)]
    pub(crate) url: Option<url::Url>,
}

/// Parse a URL
fn parse_url(
    s: &str,
) -> Result<url::Url, Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Parse an absolute URL from a string.
    let url = url::Url::parse(s).map_err(|_| anyhow!("Invalid URL: {s}"))?;
    Ok(url)
}

// COMPOSITE ARGS --------------------------------

/// Markdown source directory and destination file
#[derive(Debug, Args)]
#[command(flatten_help = true)]
pub(crate) struct MarkdownSrcDirAndDestFileArgs {
    /// Source directory containing the Markdown files
    #[command(flatten)]
    pub(crate) src: MarkdownDirArgs,

    /// Path to the file to create
    #[command(flatten)]
    pub(crate) dest: DestFileArgs,
}

/// Cargo.toml-containing directory and destination file
#[derive(Debug, Args)]
#[command(flatten_help = true)]
pub(crate) struct DependenciesDirAndDestFileArgs {
    /// Source directory containing the Markdown files
    #[command(flatten)]
    pub(crate) src: MarkdownDirArgs,

    /// Source directory containing Cargo.toml
    #[command(flatten)]
    pub(crate) manifest: CargoTomlDirArgs,

    /// Path to the file to create
    #[command(flatten)]
    pub(crate) dest: DestFileArgs,
}

/// Markdown source directory and destination directory
#[derive(Args, Debug)]
pub(crate) struct MarkdownSrcDirAndDestDirArgs {
    /// Source directory containing the Markdown files
    #[command(flatten)]
    pub(crate) src: MarkdownDirArgs,

    /// Destination directory
    #[command(flatten)]
    pub(crate) dest: DestDirArgs,
}

#[derive(Debug, Args)]
#[command(flatten_help = true)]
pub(crate) struct MarkdownSrcDirUrlAndDestFileArgs {
    /// Source directory containing the source Markdown files
    #[command(flatten)]
    pub(crate) src: MarkdownDirArgs,

    /// Base URL
    #[command(flatten)]
    pub(crate) base: UrlArgs,

    /// Path to the file to create
    #[command(flatten)]
    pub(crate) dest: DestFileArgs,
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
