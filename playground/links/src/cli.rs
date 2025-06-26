use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
pub(crate) struct Args {
    #[clap(help = "Path to directory to process")]
    pub directory: PathBuf,
}
