#[cfg(all(target_os = "linux", feature = "sqlite"))]
use clap::Parser;
#[cfg(all(target_os = "linux", feature = "sqlite"))]
use clap::Subcommand;

#[cfg(all(target_os = "linux", feature = "sqlite"))]
mod initialization;
#[cfg(all(target_os = "linux", feature = "sqlite"))]
mod insert_select;
#[cfg(all(target_os = "linux", feature = "sqlite"))]
mod transactions;

#[cfg(all(target_os = "linux", feature = "sqlite"))]
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[cfg(all(target_os = "linux", feature = "sqlite"))]
#[derive(Subcommand)]
enum Commands {
    #[cfg(all(target_os = "linux", feature = "sqlite"))]
    #[command(name = "initialization")]
    Initialization,
    #[cfg(all(target_os = "linux", feature = "sqlite"))]
    #[command(name = "insert_select")]
    InsertSelect,
    #[cfg(all(target_os = "linux", feature = "sqlite"))]
    #[command(name = "transactions")]
    Transactions,
}

fn main() -> anyhow::Result<()> {
    #[cfg(all(target_os = "linux", feature = "sqlite"))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(all(target_os = "linux", feature = "sqlite"))]
                Commands::Initialization => {
                    initialization::run();
                }
                #[cfg(all(target_os = "linux", feature = "sqlite"))]
                Commands::InsertSelect => {
                    insert_select::run();
                }
                #[cfg(all(target_os = "linux", feature = "sqlite"))]
                Commands::Transactions => {
                    transactions::run();
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
