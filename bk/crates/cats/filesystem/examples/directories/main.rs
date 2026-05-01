use clap::Parser;
use clap::Subcommand;

mod cwd;
mod manipulate_dirs;
mod remove_dir_all;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "cwd")]
    Cwd,
    #[command(name = "manipulate_dirs")]
    ManipulateDirs,
    #[command(name = "remove_dir_all")]
    RemoveDirAll,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Cwd => {
                let _ = cwd::run();
            }
            Commands::ManipulateDirs => {
                let _ = manipulate_dirs::run();
            }
            Commands::RemoveDirAll => {
                let _ = remove_dir_all::run();
            }
        }
    }
}
