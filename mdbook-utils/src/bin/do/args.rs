use std::path::PathBuf;

use anyhow::anyhow;
use clap::Args;

/// Destination file
#[derive(Args, Debug)]
pub(crate) struct DestFilePathArgs {
    /// Path of the file to create
    #[arg(short = 'o', long = "output", value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
    pub(crate) file_path: Option<PathBuf>,
}

/// Markdown source directory
#[derive(Args, Debug)]
pub(crate) struct MarkdownDirArgs {
    /// Source directory containing the source Markdown files
    #[arg(short, long="markdown-dir", value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    pub(crate) markdown_src_dir_path: Option<PathBuf>,
}

/// Markdown source directory and destination file
#[derive(Debug, Args)]
#[command(flatten_help = true)]
pub(crate) struct MarkdownSrcDirAndDestFileArgs {
    /// Source directory containing the Markdown files
    #[command(flatten)]
    pub(crate) src: MarkdownDirArgs,

    /// Path to the file to create
    #[command(flatten)]
    pub(crate) dest: DestFilePathArgs,
}

/// Cargo.toml-containing directory and destination file
#[derive(Debug, Args)]
#[command(flatten_help = true)]
pub(crate) struct DependenciesDirAndDestFileArgs {
    /// Path to the directory containing Cargo.toml
    #[arg(short='m', long="manifest-dir", value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    pub(crate) manifest_dir_path: Option<PathBuf>,

    /// Path to the file to create
    #[command(flatten)]
    pub(crate) dest: DestFilePathArgs,
}

/// Markdown source directory and destination directory
#[derive(Args, Debug)]
pub(crate) struct MarkdownSrcDirAndDestDirArgs {
    /// Source directory containing the Markdown files
    #[command(flatten)]
    pub(crate) src: MarkdownDirArgs,

    /// Destination directory
    #[arg(short='t', long="target-dir", value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    pub(crate) dest_dir_path: Option<PathBuf>,
}

#[derive(Debug, Args)]
#[command(flatten_help = true)]
pub(crate) struct MarkdownSrcDirUrlAndDestFileArgs {
    /// Source directory containing the source Markdown files
    #[command(flatten)]
    pub(crate) src: MarkdownDirArgs,

    #[arg(short='b', long="base-url", value_name = "URL", value_parser = parse_url)]
    pub(crate) base_url: Option<url::Url>,

    /// Path to the file to create
    #[command(flatten)]
    pub(crate) dest: DestFilePathArgs,
}

/// Parse a URL
fn parse_url(
    s: &str,
) -> Result<url::Url, Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Parse an absolute URL from a string.
    let url =
        url::Url::parse(s).map_err(|_| anyhow!("Invalid URL: {s}"))?;
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
