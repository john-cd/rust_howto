mod cli;

use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;
use rust_howto_tools::*;

fn main() -> anyhow::Result<()> {
    match cli::get_cmd()? {
        cli::Cmd::None => {
            return Err(anyhow::anyhow!("You did not enter a command!"));
        }
        cli::Cmd::CategoryPage(c) => {
            let crates_with_categories: Result<Vec<(String, Vec<Cat>)>> = c
                .names
                .into_iter()
                .map(|name| {
                    let cats = get_categories_for_crate(&name)?;
                    Ok((name, cats))
                })
                .collect();

            // Flatten the Vec((String, Vec<Cat>)) into Vec<(Cat, String)>,
            // sort and group by category
            let category_and_crate: HashMap<Cat, Vec<String>> =
                crates_with_categories?
                    .into_iter()
                    .flat_map(|(name, cats)| {
                        cats.into_iter().map(move |cat| (cat, name.clone()))
                    })
                    .sorted()           // needs Itertools
                    .into_group_map(); // needs Itertools

            for (cat, crate_names) in category_and_crate {
                let markdown = create_category_and_crates(
                    &cat.category,
                    &cat.slug,
                    &cat.description,
                    crate_names.iter().map(AsRef::as_ref).collect(), /* convert Vec<String> into Vec<&str> */
                )?;
                println!("{}", markdown);
            }
        }
    }
    Ok(())
}
