use clap::Parser;
use clap::Subcommand;

mod tar_compress;
mod tar_decompress;
mod tar_strip_prefix;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "tar_compress")]
    TarCompress,
    #[command(name = "tar_decompress")]
    TarDecompress,
    #[command(name = "tar_strip_prefix")]
    TarStripPrefix,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::TarCompress => {
                tar_compress::run();
            }
            Commands::TarDecompress => {
                tar_decompress::run();
            }
            Commands::TarStripPrefix => {
                tar_strip_prefix::run();
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
