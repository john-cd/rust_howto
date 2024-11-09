use anyhow::anyhow;
use tool_lib::*;
mod cli;
mod cmd;
pub(crate) use cmd::*;

fn main() -> anyhow::Result<()> {
    let (config, cmd) = cli::run()?;

    let log_level = if config.verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };
    tracing_subscriber::fmt().with_max_level(log_level).init();

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
        Cmd::CategoriesForCrateBadge(c) => {
            for n in c.names {
                let name = n.trim();
                for cat in tool_lib::get_categories_for_crate(name)? {
                    let markdown =
                        create_category_badge(&cat.category, &cat.slug)?;
                    println!("{}", markdown);
                }
            }
        }
        Cmd::CategoryBadges(c) => {
            // for c in c.categories {
            //     let category_name = c.trim();
            //     let cat = tool_lib::get_category(category_name)?;
            //     let markdown = create_category_badge(&cat.category,
            // &cat.slug)?;         println!("{}", markdown);
            // }
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
