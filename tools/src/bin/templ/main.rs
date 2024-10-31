use anyhow::anyhow;
use rust_howto_tools::*;
mod cli;
use cli::*;

/// The command that the end user selected
#[derive(Default, Debug)]
pub(crate) enum Cmd {
    #[default]
    None,
    Badges(BadgeCmdArgs),
    Rbe(RbeCmdArgs),
    CategoryBadge(BadgeCmdArgs),
    Info(InfoCmdArgs),
}

fn main() -> anyhow::Result<()> {
    let (config, cmd) = cli::run()?;
    if config.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }
    match cmd {
        Cmd::Badges(b) => {
            for n in b.names {
                let badge = create_badge(&n)?;
                println!("{}", badge);
            }
        }
        Cmd::Rbe(r) => {
            for c in r.concepts {
                let badge = create_rbe_badge(&c)?;
                println!("{}", badge);
            }
        }
        Cmd::CategoryBadge(c) => {
            for n in c.names {
                let name = n.trim();
                for cat in rust_howto_tools::get_categories_for_crate(name)? {
                    let markdown =
                        create_category_badge(&cat.category, &cat.slug)?;
                    println!("{}", markdown);
                }
            }
        }
        Cmd::Info(i) => {
            for n in i.names {
                let info = get_info_for_crate(&n)?;
                println!("{:#?}", info);
            }
        }
        _ => return Err(anyhow!("You did not enter a command!")),
    }
    Ok(())
}
