use anyhow::anyhow;
use cli::Cmd;
use rust_howto_tools::*;

mod cli;

fn main() -> anyhow::Result<()> {
    let (config, cmd) = cli::run();
    if config.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }
    match cmd {
        Cmd::Badges(b) => {
            for n in b.names {
                create_badge(&n)?;
            }
        }
        Cmd::Rbe(r) => {
            for c in r.concepts {
                create_rbe_badge(&c)?;
            }
        }
        _ => return Err(anyhow!("You did not enter a command!")),
    }
    Ok(())
}
