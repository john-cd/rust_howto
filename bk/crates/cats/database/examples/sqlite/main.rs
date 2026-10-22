//! Runs one of the `sqlite` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

#[cfg(all(target_os = "linux", feature = "sqlite"))]
use clap::Parser;
#[cfg(all(target_os = "linux", feature = "sqlite"))]
use clap::Subcommand;

#[cfg(all(target_os = "linux", feature = "sqlite"))]
mod initialization;
#[cfg(all(target_os = "linux", feature = "sqlite"))]
mod insert_select;
#[cfg(all(feature = "sqlite", any(target_os = "linux", test)))]
mod transactions;

/// Command-line arguments.
#[cfg(all(target_os = "linux", feature = "sqlite"))]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(all(target_os = "linux", feature = "sqlite"))]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `initialization` example.
    #[cfg(all(target_os = "linux", feature = "sqlite"))]
    #[command(name = "initialization")]
    Initialization,
    /// Runs the `insert_select` example.
    #[cfg(all(target_os = "linux", feature = "sqlite"))]
    #[command(name = "insert_select")]
    InsertSelect,
    /// Runs the `transactions` example.
    #[cfg(all(target_os = "linux", feature = "sqlite"))]
    #[command(name = "transactions")]
    Transactions,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    #[cfg(all(target_os = "linux", feature = "sqlite"))]
    {
        let cli = Cli::try_parse().unwrap_or(Cli { command: None });

        if let Some(command) = cli.command {
            match command {
                #[cfg(all(target_os = "linux", feature = "sqlite"))]
                Commands::Initialization => {
                    let _ = initialization::run();
                }
                #[cfg(all(target_os = "linux", feature = "sqlite"))]
                Commands::InsertSelect => {
                    let _ = insert_select::run();
                }
                #[cfg(all(target_os = "linux", feature = "sqlite"))]
                Commands::Transactions => {
                    transactions::run()?;
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
