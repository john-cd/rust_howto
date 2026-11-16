//! Runs the quantitative finance examples.
//!
//! TODO Reviewed ??-??-2026.

/// Price European options with the `black_scholes` crate, then recover the
mod black_scholes;
/// Estimates a European call payoff with a reproducible random stream.
mod monte_carlo;
/// Discounts a fixed sequence of cash flows with `financial`.
mod net_present_value;
/// Daily returns and a 20-day rolling standard deviation, annualized with
mod rolling_stats;
/// Price European options with the Black-Scholes formula, using `statrs` for
mod rustquant;
/// Count NYSE trading days and step forward by a number of trading days with
mod trading_days;

/// Runs the quant examples.
fn main() -> anyhow::Result<()> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("rustquant"));
    command = command.subcommand(clap::Command::new("black-scholes"));
    command = command.subcommand(clap::Command::new("rolling-stats"));
    command = command.subcommand(clap::Command::new("trading-days"));
    command = command.subcommand(clap::Command::new("net-present-value"));
    command = command.subcommand(clap::Command::new("monte-carlo"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("rustquant", _)) => {
            rustquant::run();
            Ok(())
        }
        Some(("black-scholes", _)) => {
            black_scholes::run();
            Ok(())
        }
        Some(("rolling-stats", _)) => {
            rolling_stats::run();
            Ok(())
        }
        Some(("trading-days", _)) => {
            trading_days::run()?;
            Ok(())
        }
        Some(("net-present-value", _)) => {
            net_present_value::run();
            Ok(())
        }
        Some(("monte-carlo", _)) => {
            monte_carlo::run();
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
