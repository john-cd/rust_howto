//! Lists and runs Docker containers, then exercises an isolated Redis service.
//!
//! TODO Reviewed ??-??-2026.

/// Runs a short-lived container with explicit cleanup using Bollard.
mod docker_client;
/// Tests a Redis service on a randomly mapped host port.
mod redis_test;

/// Executes the Docker recipes against the configured daemon.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("docker-client"));
    command = command.subcommand(clap::Command::new("redis-test"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("docker-client", _)) => {
            docker_client::run().await?;
            Ok(())
        }
        Some(("redis-test", _)) => {
            redis_test::run().await?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
