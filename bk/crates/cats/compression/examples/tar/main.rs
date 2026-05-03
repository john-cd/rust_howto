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
    Compress,
    #[command(name = "tar_decompress")]
    Decompress,
    #[command(name = "tar_strip_prefix")]
    StripPrefix,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Compress => {
                tar_compress::run()?;
            }
            Commands::Decompress => {
                tar_decompress::run()?;
            }
            Commands::StripPrefix => {
                tar_strip_prefix::run()?;
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
