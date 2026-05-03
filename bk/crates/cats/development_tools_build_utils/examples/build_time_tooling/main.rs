use clap::Parser;
use clap::Subcommand;

mod cc_bundled_cpp;
mod cc_bundled_cpp1;
mod cc_bundled_static;
mod cc_bundled_static1;
mod cc_defines;
mod cc_defines1;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "cc_bundled_cpp")]
    CcBundledCpp,
    #[command(name = "cc_bundled_cpp1")]
    CcBundledCpp1,
    #[command(name = "cc_bundled_static")]
    CcBundledStatic,
    #[command(name = "cc_bundled_static1")]
    CcBundledStatic1,
    #[command(name = "cc_defines")]
    CcDefines,
    #[command(name = "cc_defines1")]
    CcDefines1,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CcBundledCpp => {
                cc_bundled_cpp::run();
            }
            Commands::CcBundledCpp1 => {
                cc_bundled_cpp1::run();
            }
            Commands::CcBundledStatic => {
                cc_bundled_static::run();
            }
            Commands::CcBundledStatic1 => {
                cc_bundled_static1::run()?;
            }
            Commands::CcDefines => {
                cc_defines::run();
            }
            Commands::CcDefines1 => {
                cc_defines1::run();
            }
        }
    }

    Ok(())
}
